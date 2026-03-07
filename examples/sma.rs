#[path = "support/mod.rs"]
mod support;

use palmscript::{compile, run_with_sources, Interval, VmLimits};

fn main() {
    let source = "interval 1m\nsource spot = binance.spot(\"BTCUSDT\")\nplot(sma(spot.close, 5))";
    let compiled = compile(source).expect("script compiles");
    let bars = support::fixture_bars(12);
    let outputs = run_with_sources(
        &compiled,
        support::source_runtime_config(Interval::Min1, bars, vec![]),
        VmLimits::default(),
    )
    .expect("script runs");

    println!("script:\n{source}");
    support::print_outputs(&outputs);
}
