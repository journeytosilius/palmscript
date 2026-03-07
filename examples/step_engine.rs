#[path = "support/mod.rs"]
mod support;

use palmscript::{compile, run_with_sources, Interval, VmLimits};

fn main() {
    let source = "interval 1m\nsource spot = binance.spot(\"BTCUSDT\")\nif spot.close > ema(spot.close, 3) { plot(1) } else { plot(0) }";
    let compiled = compile(source).expect("script compiles");
    let bars = support::fixture_bars(8);
    let outputs = run_with_sources(
        &compiled,
        support::source_runtime_config(Interval::Min1, bars, vec![]),
        VmLimits::default(),
    )
    .expect("script runs");

    println!("script:\n{source}");
    for point in &outputs.plots[0].points {
        println!("bar {} -> plot {:?}", point.bar_index, point.value);
    }
    support::print_outputs(&outputs);
}
