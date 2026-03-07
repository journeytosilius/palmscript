use palmscript::{compile, run, Bar, OutputValue, VmLimits, TALIB_UPSTREAM_COMMIT};
use serde::Deserialize;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

const FIXTURE_PATH: &str = "tests/data/ta_lib/implemented_oracle.json";

#[derive(Clone, Debug, Deserialize)]
struct FixtureDocument {
    upstream_commit: String,
    datasets: BTreeMap<String, FixtureDataset>,
    cases: Vec<FixtureCase>,
}

#[derive(Clone, Debug, Deserialize)]
struct FixtureDataset {
    bars: Vec<Bar>,
}

#[derive(Clone, Debug, Deserialize)]
struct FixtureCase {
    name: String,
    dataset: String,
    script: String,
    epsilon: f64,
    expected_exports: BTreeMap<String, Vec<Option<f64>>>,
}

#[test]
fn committed_talib_oracle_matches_runtime_outputs() {
    let fixtures = load_fixtures();
    assert_eq!(fixtures.upstream_commit, TALIB_UPSTREAM_COMMIT);

    for case in &fixtures.cases {
        let dataset = fixtures
            .datasets
            .get(&case.dataset)
            .unwrap_or_else(|| panic!("missing dataset {} for {}", case.dataset, case.name));
        let compiled =
            compile(&case.script).unwrap_or_else(|err| panic!("{:?}: {}", case.name, err));
        let outputs = run(&compiled, &dataset.bars, VmLimits::default())
            .unwrap_or_else(|err| panic!("{} failed to run: {err}", case.name));

        let actual_exports = outputs
            .exports
            .iter()
            .map(|series| {
                (
                    series.name.clone(),
                    series
                        .points
                        .iter()
                        .map(|point| match point.value {
                            OutputValue::F64(value) => Some(value),
                            OutputValue::NA => None,
                            OutputValue::Bool(_) => {
                                panic!(
                                    "{} export {} unexpectedly returned bool",
                                    case.name, series.name
                                )
                            }
                        })
                        .collect::<Vec<_>>(),
                )
            })
            .collect::<BTreeMap<_, _>>();

        assert_eq!(
            actual_exports.len(),
            case.expected_exports.len(),
            "{} export count mismatch",
            case.name
        );

        for (export_name, expected) in &case.expected_exports {
            let actual = actual_exports
                .get(export_name)
                .unwrap_or_else(|| panic!("{} missing export {}", case.name, export_name));
            assert_series_close(&case.name, export_name, actual, expected, case.epsilon);
        }
    }
}

fn load_fixtures() -> FixtureDocument {
    let path = Path::new(FIXTURE_PATH);
    let text = fs::read_to_string(path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()));
    serde_json::from_str(&text)
        .unwrap_or_else(|err| panic!("failed to parse {}: {err}", path.display()))
}

fn assert_series_close(
    case_name: &str,
    export_name: &str,
    actual: &[Option<f64>],
    expected: &[Option<f64>],
    epsilon: f64,
) {
    assert_eq!(
        actual.len(),
        expected.len(),
        "{} export {} length mismatch",
        case_name,
        export_name
    );
    for (index, (actual, expected)) in actual.iter().zip(expected.iter()).enumerate() {
        match (actual, expected) {
            (None, None) => {}
            (Some(actual), Some(expected)) => {
                let delta = (actual - expected).abs();
                assert!(
                    delta <= epsilon,
                    "{} export {} differs at index {}: actual={} expected={} delta={}",
                    case_name,
                    export_name,
                    index,
                    actual,
                    expected,
                    delta
                );
            }
            _ => panic!(
                "{} export {} NA mismatch at index {}: actual={:?} expected={:?}",
                case_name, export_name, index, actual, expected
            ),
        }
    }
}
