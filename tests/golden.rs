use serde_json::json;
use tradelang::{compile, run, Bar, VmLimits};

fn fixture_bars() -> Vec<Bar> {
    (0..20)
        .map(|index| {
            let close = 100.0 + index as f64;
            Bar {
                open: close - 0.5,
                high: close + 1.0,
                low: close - 1.0,
                close,
                volume: 1_000.0 + index as f64,
                time: 1_700_000_000_000.0 + index as f64 * 60_000.0,
            }
        })
        .collect()
}

#[test]
fn golden_sma_shape_matches() {
    let compiled = compile("plot(sma(close, 14))").expect("script compiles");
    let outputs = run(&compiled, &fixture_bars(), VmLimits::default()).expect("script runs");
    let json = serde_json::to_value(outputs).expect("json");
    assert_eq!(json["plots"][0]["id"], json!(0));
    assert_eq!(json["plots"][0]["points"].as_array().unwrap().len(), 20);
    assert_eq!(
        json["plots"][0]["points"][0]["value"],
        serde_json::Value::Null
    );
    assert!(json["plots"][0]["points"][13]["value"].is_number());
}

#[test]
fn golden_close_index_shape_matches() {
    let compiled = compile("plot(close[1])").expect("script compiles");
    let outputs = run(&compiled, &fixture_bars(), VmLimits::default()).expect("script runs");
    let json = serde_json::to_value(outputs).expect("json");
    assert_eq!(
        json["plots"][0]["points"][0]["value"],
        serde_json::Value::Null
    );
    assert!(json["plots"][0]["points"][1]["value"].is_number());
}

#[test]
fn golden_if_else_shape_matches() {
    let compiled =
        compile("if close > sma(close, 14) { plot(1) } else { plot(0) }").expect("script compiles");
    let outputs = run(&compiled, &fixture_bars(), VmLimits::default()).expect("script runs");
    let json = serde_json::to_value(outputs).expect("json");
    assert_eq!(json["plots"][0]["points"].as_array().unwrap().len(), 20);
    assert_eq!(
        json["plots"][0]["points"][0]["value"],
        serde_json::json!(0.0)
    );
    assert!(json["plots"][0]["points"][14]["value"].is_number());
}
