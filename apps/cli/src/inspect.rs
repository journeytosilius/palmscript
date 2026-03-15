use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use palmscript::{
    BacktestResult, OutputKind, OutputSeries, OutputValue, Outputs, PaperSessionExport,
};
use serde::Serialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactKind {
    Outputs,
    BacktestResult,
    PaperSessionExport,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExportSeriesSummary {
    pub name: String,
    pub kind: OutputKind,
    pub point_count: usize,
    pub na_count: usize,
    pub true_count: usize,
    pub false_count: usize,
    pub numeric_count: usize,
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
    pub mean_value: Option<f64>,
    pub first_bar_index: Option<usize>,
    pub last_bar_index: Option<usize>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExportListSummary {
    pub artifact_kind: ArtifactKind,
    pub export_count: usize,
    pub exports: Vec<ExportSeriesSummary>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct BoolExportOverlapSummary {
    pub artifact_kind: ArtifactKind,
    pub left: String,
    pub right: String,
    pub point_count: usize,
    pub comparable_point_count: usize,
    pub na_count: usize,
    pub left_true_count: usize,
    pub right_true_count: usize,
    pub both_true_count: usize,
    pub either_true_count: usize,
    pub left_only_true_count: usize,
    pub right_only_true_count: usize,
    pub both_false_count: usize,
}

struct LoadedOutputs {
    artifact_kind: ArtifactKind,
    outputs: Outputs,
}

pub fn inspect_exports(path: &Path) -> Result<ExportListSummary, String> {
    let loaded = load_outputs(path)?;
    let exports = loaded
        .outputs
        .exports
        .iter()
        .map(summarize_export)
        .collect::<Vec<_>>();
    Ok(ExportListSummary {
        artifact_kind: loaded.artifact_kind,
        export_count: exports.len(),
        exports,
    })
}

pub fn inspect_export(path: &Path, name: &str) -> Result<ExportSeriesSummary, String> {
    let loaded = load_outputs(path)?;
    let series = find_export(&loaded.outputs, name)?;
    Ok(summarize_export(series))
}

pub fn inspect_overlap(
    path: &Path,
    left: &str,
    right: &str,
) -> Result<BoolExportOverlapSummary, String> {
    let loaded = load_outputs(path)?;
    let left_series = find_export(&loaded.outputs, left)?;
    let right_series = find_export(&loaded.outputs, right)?;
    let left_values = bool_points(left_series)?;
    let right_values = bool_points(right_series)?;
    let mut all_indices = BTreeSet::new();
    all_indices.extend(left_values.keys().copied());
    all_indices.extend(right_values.keys().copied());

    let mut summary = BoolExportOverlapSummary {
        artifact_kind: loaded.artifact_kind,
        left: left.to_string(),
        right: right.to_string(),
        point_count: all_indices.len(),
        comparable_point_count: 0,
        na_count: 0,
        left_true_count: 0,
        right_true_count: 0,
        both_true_count: 0,
        either_true_count: 0,
        left_only_true_count: 0,
        right_only_true_count: 0,
        both_false_count: 0,
    };

    for bar_index in all_indices {
        let left_value = left_values.get(&bar_index).copied().flatten();
        let right_value = right_values.get(&bar_index).copied().flatten();
        match (left_value, right_value) {
            (Some(left_bool), Some(right_bool)) => {
                summary.comparable_point_count += 1;
                if left_bool {
                    summary.left_true_count += 1;
                }
                if right_bool {
                    summary.right_true_count += 1;
                }
                match (left_bool, right_bool) {
                    (true, true) => summary.both_true_count += 1,
                    (true, false) => summary.left_only_true_count += 1,
                    (false, true) => summary.right_only_true_count += 1,
                    (false, false) => summary.both_false_count += 1,
                }
            }
            _ => {
                summary.na_count += 1;
            }
        }
    }

    summary.either_true_count =
        summary.both_true_count + summary.left_only_true_count + summary.right_only_true_count;

    Ok(summary)
}

fn load_outputs(path: &Path) -> Result<LoadedOutputs, String> {
    let raw = fs::read_to_string(path)
        .map_err(|err| format!("failed to read artifact `{}`: {err}", path.display()))?;

    if let Ok(backtest) = serde_json::from_str::<BacktestResult>(&raw) {
        return Ok(LoadedOutputs {
            artifact_kind: ArtifactKind::BacktestResult,
            outputs: backtest.outputs,
        });
    }
    if let Ok(export) = serde_json::from_str::<PaperSessionExport>(&raw) {
        let result = export.latest_result.ok_or_else(|| {
            format!(
                "artifact `{}` is a paper session export without `latest_result` outputs",
                path.display()
            )
        })?;
        return Ok(LoadedOutputs {
            artifact_kind: ArtifactKind::PaperSessionExport,
            outputs: result.outputs,
        });
    }
    if let Ok(outputs) = serde_json::from_str::<Outputs>(&raw) {
        return Ok(LoadedOutputs {
            artifact_kind: ArtifactKind::Outputs,
            outputs,
        });
    }

    Err(format!(
        "artifact `{}` is not a supported PalmScript outputs artifact; expected backtest result, paper export, or raw outputs JSON",
        path.display()
    ))
}

fn find_export<'a>(outputs: &'a Outputs, name: &str) -> Result<&'a OutputSeries, String> {
    outputs
        .exports
        .iter()
        .find(|series| series.name == name)
        .ok_or_else(|| format!("export `{name}` not found in artifact"))
}

fn summarize_export(series: &OutputSeries) -> ExportSeriesSummary {
    let mut na_count = 0;
    let mut true_count = 0;
    let mut false_count = 0;
    let mut numeric_count = 0;
    let mut numeric_sum = 0.0;
    let mut min_value = None::<f64>;
    let mut max_value = None::<f64>;

    for point in &series.points {
        match point.value {
            OutputValue::NA => na_count += 1,
            OutputValue::Bool(value) => {
                if value {
                    true_count += 1;
                } else {
                    false_count += 1;
                }
            }
            OutputValue::F64(value) => {
                numeric_count += 1;
                numeric_sum += value;
                min_value = Some(min_value.map_or(value, |current| current.min(value)));
                max_value = Some(max_value.map_or(value, |current| current.max(value)));
            }
        }
    }

    ExportSeriesSummary {
        name: series.name.clone(),
        kind: series.kind,
        point_count: series.points.len(),
        na_count,
        true_count,
        false_count,
        numeric_count,
        min_value,
        max_value,
        mean_value: (numeric_count > 0).then_some(numeric_sum / numeric_count as f64),
        first_bar_index: series.points.first().map(|point| point.bar_index),
        last_bar_index: series.points.last().map(|point| point.bar_index),
    }
}

fn bool_points(series: &OutputSeries) -> Result<BTreeMap<usize, Option<bool>>, String> {
    let mut values = BTreeMap::new();
    for point in &series.points {
        let value = match point.value {
            OutputValue::Bool(value) => Some(value),
            OutputValue::NA => None,
            OutputValue::F64(_) => {
                return Err(format!(
                    "export `{}` is numeric; `inspect overlap` requires boolean exports",
                    series.name
                ));
            }
        };
        values.insert(point.bar_index, value);
    }
    Ok(values)
}

#[cfg(test)]
mod tests {
    use super::{inspect_exports, inspect_overlap, ArtifactKind};
    use palmscript::bytecode::OutputKind;
    use palmscript::{OutputSample, OutputSeries, OutputValue, Outputs};
    use std::fs;
    use tempfile::tempdir;

    fn sample_outputs() -> Outputs {
        Outputs {
            exports: vec![
                OutputSeries {
                    id: 0,
                    name: "setup".to_string(),
                    kind: OutputKind::ExportSeries,
                    points: vec![
                        OutputSample {
                            output_id: 0,
                            name: "setup".to_string(),
                            bar_index: 0,
                            time: Some(10.0),
                            value: OutputValue::Bool(true),
                        },
                        OutputSample {
                            output_id: 0,
                            name: "setup".to_string(),
                            bar_index: 1,
                            time: Some(11.0),
                            value: OutputValue::Bool(false),
                        },
                        OutputSample {
                            output_id: 0,
                            name: "setup".to_string(),
                            bar_index: 2,
                            time: Some(12.0),
                            value: OutputValue::NA,
                        },
                    ],
                },
                OutputSeries {
                    id: 1,
                    name: "time_ok".to_string(),
                    kind: OutputKind::ExportSeries,
                    points: vec![
                        OutputSample {
                            output_id: 1,
                            name: "time_ok".to_string(),
                            bar_index: 0,
                            time: Some(10.0),
                            value: OutputValue::Bool(true),
                        },
                        OutputSample {
                            output_id: 1,
                            name: "time_ok".to_string(),
                            bar_index: 1,
                            time: Some(11.0),
                            value: OutputValue::Bool(true),
                        },
                        OutputSample {
                            output_id: 1,
                            name: "time_ok".to_string(),
                            bar_index: 2,
                            time: Some(12.0),
                            value: OutputValue::NA,
                        },
                    ],
                },
            ],
            ..Outputs::default()
        }
    }

    #[test]
    fn inspect_exports_summarizes_counts() {
        let dir = tempdir().expect("tempdir");
        let artifact = dir.path().join("outputs.json");
        fs::write(
            &artifact,
            serde_json::to_string_pretty(&sample_outputs()).expect("json"),
        )
        .expect("write");

        let summary = inspect_exports(&artifact).expect("summary");
        assert_eq!(summary.artifact_kind, ArtifactKind::Outputs);
        assert_eq!(summary.export_count, 2);
        assert_eq!(summary.exports[0].true_count, 1);
        assert_eq!(summary.exports[0].false_count, 1);
        assert_eq!(summary.exports[0].na_count, 1);
    }

    #[test]
    fn inspect_overlap_reports_joint_true_counts() {
        let dir = tempdir().expect("tempdir");
        let artifact = dir.path().join("outputs.json");
        fs::write(
            &artifact,
            serde_json::to_string_pretty(&sample_outputs()).expect("json"),
        )
        .expect("write");

        let overlap = inspect_overlap(&artifact, "setup", "time_ok").expect("overlap");
        assert_eq!(overlap.both_true_count, 1);
        assert_eq!(overlap.left_true_count, 1);
        assert_eq!(overlap.right_true_count, 2);
        assert_eq!(overlap.na_count, 1);
    }
}
