use palmscript::{compile, run_with_sources, Bar, SourceFeed, SourceRuntimeConfig, VmLimits};

fn minute_bar(time: i64, close: f64) -> Bar {
    Bar {
        open: close - 0.5,
        high: close + 0.5,
        low: close - 1.0,
        close,
        volume: 10.0,
        time: time as f64,
    }
}

#[test]
fn source_aware_scripts_require_qualified_market_series() {
    let err = compile("interval 1m\nsource a = binance.spot(\"BTCUSDT\")\nplot(close)")
        .expect_err("bare series should reject");
    assert!(err.diagnostics.iter().any(|diagnostic| diagnostic
        .message
        .contains("scripts require source-qualified market series; found `close`")));
}

#[test]
fn source_aware_runtime_uses_union_of_base_timestamps() {
    let compiled = compile(
        "interval 1m\nsource spot = binance.spot(\"BTCUSDT\")\nsource perp = binance.usdm(\"BTCUSDT\")\nplot(spot.close - perp.close)",
    )
    .expect("compile");
    let outputs = run_with_sources(
        &compiled,
        SourceRuntimeConfig {
            base_interval: palmscript::Interval::Min1,
            feeds: vec![
                SourceFeed {
                    source_id: 0,
                    interval: palmscript::Interval::Min1,
                    bars: vec![
                        minute_bar(1_704_067_200_000, 10.0),
                        minute_bar(1_704_067_260_000, 11.0),
                    ],
                },
                SourceFeed {
                    source_id: 1,
                    interval: palmscript::Interval::Min1,
                    bars: vec![
                        minute_bar(1_704_067_260_000, 7.0),
                        minute_bar(1_704_067_320_000, 8.0),
                    ],
                },
            ],
        },
        VmLimits::default(),
    )
    .expect("runtime");
    let points = &outputs.plots[0].points;
    assert_eq!(points.len(), 3);
    assert_eq!(points[0].value, None);
    assert_eq!(points[1].value, Some(4.0));
    assert_eq!(points[2].value, None);
}

#[test]
fn source_interval_references_require_source_scoped_use_declarations() {
    let err = compile("interval 1m\nsource a = hyperliquid.perps(\"BTC\")\nplot(a.1h.close)")
        .expect_err("missing use should fail");
    assert!(err.diagnostics.iter().any(|diagnostic| diagnostic
        .message
        .contains("must be declared with `use a 1h`")));
}
