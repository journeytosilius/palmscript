#![allow(dead_code)]

use palmscript::{Bar, Interval, Outputs, SourceFeed, SourceRuntimeConfig};

pub const SECOND_MS: i64 = 1_000;
pub const MINUTE_MS: i64 = 60 * SECOND_MS;
pub const HOUR_MS: i64 = 60 * MINUTE_MS;
pub const DAY_MS: i64 = 24 * HOUR_MS;
pub const WEEK_MS: i64 = 7 * DAY_MS;
pub const JAN_1_2024_UTC_MS: i64 = 1_704_067_200_000;
pub const FEB_1_2024_UTC_MS: i64 = 1_706_745_600_000;
pub const MAR_1_2024_UTC_MS: i64 = 1_709_251_200_000;

pub fn fixture_bars(len: usize) -> Vec<Bar> {
    rising_bars(JAN_1_2024_UTC_MS, MINUTE_MS, len, 100.0)
}

pub fn rising_bars(start_ms: i64, spacing_ms: i64, len: usize, start_close: f64) -> Vec<Bar> {
    (0..len)
        .map(|index| {
            let close = start_close + index as f64;
            Bar {
                open: close - 0.5,
                high: close + 1.0,
                low: close - 1.0,
                close,
                volume: 1_000.0 + index as f64,
                time: (start_ms + spacing_ms * index as i64) as f64,
                funding_rate: None,
                open_interest: None,
                mark_price: None,
                index_price: None,
                premium_index: None,
                basis: None,
            }
        })
        .collect()
}

pub fn flat_bars(start_ms: i64, spacing_ms: i64, closes: &[f64]) -> Vec<Bar> {
    closes
        .iter()
        .enumerate()
        .map(|(index, close)| Bar {
            open: *close - 0.5,
            high: *close + 1.0,
            low: *close - 1.0,
            close: *close,
            volume: 1_000.0 + index as f64,
            time: (start_ms + spacing_ms * index as i64) as f64,
            funding_rate: None,
            open_interest: None,
            mark_price: None,
            index_price: None,
            premium_index: None,
            basis: None,
        })
        .collect()
}

pub fn source_feed(source_id: u16, interval: Interval, bars: Vec<Bar>) -> SourceFeed {
    SourceFeed {
        source_id,
        interval,
        bars,
    }
}

pub fn source_runtime_config(
    base_interval: Interval,
    base_bars: Vec<Bar>,
    supplemental: Vec<SourceFeed>,
) -> SourceRuntimeConfig {
    let mut feeds = Vec::with_capacity(1 + supplemental.len());
    feeds.push(source_feed(0, base_interval, base_bars));
    feeds.extend(supplemental);
    SourceRuntimeConfig {
        base_interval,
        feeds,
    }
}

pub fn monthly_feed(source_id: u16, closes: &[f64]) -> SourceFeed {
    let mut bars = Vec::with_capacity(closes.len());
    let month_starts = [JAN_1_2024_UTC_MS, FEB_1_2024_UTC_MS, MAR_1_2024_UTC_MS];
    for (open_time, close) in month_starts.into_iter().zip(closes.iter().copied()) {
        bars.push(Bar {
            open: close - 1.0,
            high: close + 2.0,
            low: close - 2.0,
            close,
            volume: 10_000.0,
            time: open_time as f64,
            funding_rate: None,
            open_interest: None,
            mark_price: None,
            index_price: None,
            premium_index: None,
            basis: None,
        });
    }
    source_feed(source_id, palmscript::Interval::Month1, bars)
}

pub fn weekly_feed(source_id: u16, start_ms: i64, closes: &[f64]) -> SourceFeed {
    source_feed(
        source_id,
        palmscript::Interval::Week1,
        flat_bars(start_ms, WEEK_MS, closes),
    )
}

pub fn daily_feed(source_id: u16, start_ms: i64, closes: &[f64]) -> SourceFeed {
    source_feed(
        source_id,
        palmscript::Interval::Day1,
        flat_bars(start_ms, DAY_MS, closes),
    )
}

pub fn hourly_feed(source_id: u16, start_ms: i64, closes: &[f64]) -> SourceFeed {
    source_feed(
        source_id,
        palmscript::Interval::Hour1,
        flat_bars(start_ms, HOUR_MS, closes),
    )
}

pub fn minute_feed(source_id: u16, start_ms: i64, closes: &[f64]) -> SourceFeed {
    source_feed(
        source_id,
        palmscript::Interval::Min1,
        flat_bars(start_ms, MINUTE_MS, closes),
    )
}

pub fn print_step_values(label: &str, outputs: &Outputs) {
    println!("{label}");
    for point in &outputs.plots[0].points {
        println!("bar {} -> {:?}", point.bar_index, point.value);
    }
}

pub fn print_outputs(outputs: &Outputs) {
    let json = serde_json::to_string_pretty(outputs).expect("outputs serialize to json");
    println!("{json}");
}
