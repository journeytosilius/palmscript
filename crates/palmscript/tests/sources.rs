use palmscript::{compile, run_with_sources, Bar, SourceFeed, SourceRuntimeConfig, VmLimits};

fn minute_bar(time: i64, close: f64) -> Bar {
    Bar {
        open: close - 0.5,
        high: close + 0.5,
        low: close - 1.0,
        close,
        volume: 10.0,
        time: time as f64,
        funding_rate: None,
        open_interest: None,
        mark_price: None,
        index_price: None,
        premium_index: None,
        basis: None,
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
    let err = compile("interval 1m\nsource a = gate.usdt_perps(\"BTC_USDT\")\nplot(a.1h.close)")
        .expect_err("missing use should fail");
    assert!(err.diagnostics.iter().any(|diagnostic| diagnostic
        .message
        .contains("must be declared with `use a 1h`")));
}

#[test]
fn runtime_loads_binance_usdm_auxiliary_source_fields() {
    let compiled = compile(
        "interval 1h\nsource perp = binance.usdm(\"BTCUSDT\")\nplot(perp.mark_price + perp.funding_rate + perp.open_interest)",
    )
    .expect("compile");
    let outputs = run_with_sources(
        &compiled,
        SourceRuntimeConfig {
            base_interval: palmscript::Interval::Hour1,
            feeds: vec![SourceFeed {
                source_id: 0,
                interval: palmscript::Interval::Hour1,
                bars: vec![Bar {
                    open: 100.0,
                    high: 101.0,
                    low: 99.0,
                    close: 100.5,
                    volume: 10.0,
                    time: 1_704_067_200_000.0,
                    funding_rate: Some(0.01),
                    open_interest: Some(200.0),
                    mark_price: Some(100.25),
                    index_price: Some(100.1),
                    premium_index: Some(0.001),
                    basis: Some(0.15),
                }],
            }],
        },
        VmLimits::default(),
    )
    .expect("runtime");
    assert_eq!(outputs.plots[0].points.len(), 1);
    assert_eq!(outputs.plots[0].points[0].value, Some(300.26));
}
