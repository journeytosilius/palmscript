//! Runtime output structures produced while executing scripts.
//!
//! Outputs are grouped into per-step values and accumulated series so callers
//! can inspect plot data, exported series, trigger events, and alerts after VM
//! execution.

use crate::bytecode::OutputKind;
use crate::{OrderFieldKind, SignalRole};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PlotPoint {
    #[serde(skip)]
    pub plot_id: usize,
    pub bar_index: usize,
    pub time: Option<f64>,
    pub value: Option<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PlotSeries {
    pub id: usize,
    pub name: Option<String>,
    pub points: Vec<PlotPoint>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Alert {
    pub bar_index: usize,
    pub message: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum OutputValue {
    F64(f64),
    Bool(bool),
    NA,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OutputSample {
    #[serde(skip)]
    pub output_id: usize,
    pub name: String,
    pub bar_index: usize,
    pub time: Option<f64>,
    pub value: OutputValue,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct OutputSeries {
    pub id: usize,
    pub name: String,
    pub kind: OutputKind,
    pub points: Vec<OutputSample>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TriggerEvent {
    pub output_id: usize,
    pub name: String,
    pub bar_index: usize,
    pub time: Option<f64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderFieldSample {
    #[serde(skip)]
    pub field_id: usize,
    pub name: String,
    pub role: SignalRole,
    pub kind: OrderFieldKind,
    pub bar_index: usize,
    pub time: Option<f64>,
    pub value: OutputValue,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct OrderFieldSeries {
    pub id: usize,
    pub name: String,
    pub role: SignalRole,
    pub kind: OrderFieldKind,
    pub points: Vec<OrderFieldSample>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct StepOutput {
    pub plots: Vec<PlotPoint>,
    pub exports: Vec<OutputSample>,
    pub triggers: Vec<OutputSample>,
    pub order_fields: Vec<OrderFieldSample>,
    pub trigger_events: Vec<TriggerEvent>,
    pub alerts: Vec<Alert>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Outputs {
    pub plots: Vec<PlotSeries>,
    pub exports: Vec<OutputSeries>,
    pub triggers: Vec<OutputSeries>,
    pub order_fields: Vec<OrderFieldSeries>,
    pub trigger_events: Vec<TriggerEvent>,
    pub alerts: Vec<Alert>,
}

#[cfg(test)]
mod tests {
    use super::{
        OrderFieldSample, OrderFieldSeries, OutputSample, OutputSeries, OutputValue, Outputs,
        PlotPoint, PlotSeries, StepOutput,
    };
    use crate::bytecode::{OutputKind, SignalRole};
    use crate::OrderFieldKind;

    #[test]
    fn default_outputs_and_step_outputs_are_empty() {
        assert_eq!(StepOutput::default().plots.len(), 0);
        assert_eq!(StepOutput::default().exports.len(), 0);
        assert_eq!(Outputs::default().plots.len(), 0);
        assert_eq!(Outputs::default().order_fields.len(), 0);
        assert_eq!(Outputs::default().alerts.len(), 0);
    }

    #[test]
    fn output_structs_preserve_named_series_data() {
        let sample = OutputSample {
            output_id: 2,
            name: "trend".to_string(),
            bar_index: 5,
            time: Some(10.0),
            value: OutputValue::Bool(true),
        };
        let series = OutputSeries {
            id: 2,
            name: "trend".to_string(),
            kind: OutputKind::ExportSeries,
            points: vec![sample.clone()],
        };
        let plot = PlotSeries {
            id: 1,
            name: Some("price".to_string()),
            points: vec![PlotPoint {
                plot_id: 1,
                bar_index: 5,
                time: Some(10.0),
                value: Some(11.0),
            }],
        };
        let order_field = OrderFieldSeries {
            id: 3,
            name: "__order.long_entry.price".to_string(),
            role: SignalRole::LongEntry,
            kind: OrderFieldKind::Price,
            points: vec![OrderFieldSample {
                field_id: 3,
                name: "__order.long_entry.price".to_string(),
                role: SignalRole::LongEntry,
                kind: OrderFieldKind::Price,
                bar_index: 5,
                time: Some(10.0),
                value: OutputValue::F64(11.0),
            }],
        };
        assert_eq!(series.points[0], sample);
        assert_eq!(plot.points[0].value, Some(11.0));
        assert_eq!(order_field.points[0].value, OutputValue::F64(11.0));
    }
}
