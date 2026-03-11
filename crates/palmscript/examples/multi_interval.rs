#[path = "support/mod.rs"]
mod support;

use palmscript::{compile, run_with_sources, Interval, VmLimits};

fn main() {
    let source = "interval 1d\nsource spot = binance.spot(\"BTCUSDT\")\nuse spot 1w\nlet weekly_basis = ema(spot.1w.close, 2)\nif spot.close > weekly_basis { plot(1) } else { plot(0) }";
    let compiled = compile(source).expect("script compiles");
    let base_bars = support::flat_bars(
        support::JAN_1_2024_UTC_MS,
        support::DAY_MS,
        &[
            100.0, 101.0, 102.0, 103.0, 104.0, 105.0, 106.0, 107.0, 108.0, 109.0, 110.0, 111.0,
            112.0, 113.0, 114.0, 115.0, 116.0, 117.0, 118.0, 119.0, 120.0,
        ],
    );
    let outputs = run_with_sources(
        &compiled,
        support::source_runtime_config(
            Interval::Day1,
            base_bars,
            vec![support::weekly_feed(
                0,
                support::JAN_1_2024_UTC_MS,
                &[90.0, 95.0, 105.0],
            )],
        ),
        VmLimits::default(),
    )
    .expect("runs");

    println!("script:\n{source}");
    support::print_step_values("weekly basis gating daily execution:", &outputs);
    support::print_outputs(&outputs);
}
