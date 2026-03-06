//! Input preparation helpers for source adapters such as CLI CSV mode.
//!
//! This module infers a raw bar interval, validates strategy interval
//! requirements, and prepares base/supplemental feeds for the existing runtime.

use std::collections::BTreeSet;

use crate::compiler::CompiledProgram;
use crate::diagnostic::DataPrepError;
use crate::runtime::{Bar, IntervalFeed, MultiIntervalConfig};
use crate::Interval;

#[derive(Clone, Debug, PartialEq)]
pub struct PreparedInputs {
    pub raw_interval: Interval,
    pub base_bars: Vec<Bar>,
    pub config: MultiIntervalConfig,
}

pub fn prepare_csv_inputs_for_program(
    compiled: &CompiledProgram,
    raw_bars: Vec<Bar>,
) -> Result<PreparedInputs, DataPrepError> {
    let base_interval = compiled
        .program
        .base_interval
        .ok_or(DataPrepError::MissingBaseIntervalDeclaration)?;
    let raw_interval = infer_input_interval(&raw_bars)?;

    let mut required = BTreeSet::new();
    required.insert(base_interval);
    required.extend(compiled.program.declared_intervals.iter().copied());

    let base_bars = prepare_interval(raw_interval, base_interval, &raw_bars)?;
    let supplemental = compiled
        .program
        .declared_intervals
        .iter()
        .copied()
        .map(|interval| {
            Ok(IntervalFeed {
                interval,
                bars: prepare_interval(raw_interval, interval, &raw_bars)?,
            })
        })
        .collect::<Result<Vec<_>, DataPrepError>>()?;

    for interval in required {
        if raw_interval > interval {
            return Err(DataPrepError::RawIntervalTooCoarse {
                raw: raw_interval,
                required: interval,
            });
        }
    }

    Ok(PreparedInputs {
        raw_interval,
        base_bars,
        config: MultiIntervalConfig {
            base_interval,
            supplemental,
        },
    })
}

pub fn infer_input_interval(bars: &[Bar]) -> Result<Interval, DataPrepError> {
    validate_raw_bars(bars)?;
    if bars.len() < 2 {
        return Err(DataPrepError::CannotInferInputInterval);
    }

    for candidate in crate::INTERVAL_SPECS.iter().map(|spec| spec.interval) {
        if candidate_matches_bars(candidate, bars)? {
            return Ok(candidate);
        }
    }

    Err(DataPrepError::CannotInferInputInterval)
}

fn prepare_interval(
    raw_interval: Interval,
    target: Interval,
    bars: &[Bar],
) -> Result<Vec<Bar>, DataPrepError> {
    if raw_interval > target {
        return Err(DataPrepError::RawIntervalTooCoarse {
            raw: raw_interval,
            required: target,
        });
    }
    if bars.is_empty() {
        return Err(DataPrepError::InsufficientDataForInterval { interval: target });
    }
    if raw_interval == target {
        return Ok(bars.to_vec());
    }

    let raw_times = bars
        .iter()
        .map(|bar| open_time_ms(bar.time))
        .collect::<Result<Vec<_>, _>>()?;
    let mut index = 0usize;
    let mut rolled = Vec::new();
    let mut bucket_open =
        target
            .bucket_open_time(raw_times[0])
            .ok_or(DataPrepError::UnsupportedRollupPath {
                raw: raw_interval,
                target,
            })?;

    while index < bars.len() {
        let expected = expected_source_opens(raw_interval, target, bucket_open)?;
        let found = count_matching_bars(&raw_times[index..], &expected);
        if found != expected.len() {
            return Err(DataPrepError::IncompleteRollupBucket {
                raw: raw_interval,
                target,
                bucket_open_time: bucket_open,
                expected: expected.len(),
                found,
            });
        }
        rolled.push(aggregate_bucket(
            &bars[index..index + expected.len()],
            bucket_open,
        ));
        index += expected.len();
        bucket_open =
            target
                .next_open_time(bucket_open)
                .ok_or(DataPrepError::UnsupportedRollupPath {
                    raw: raw_interval,
                    target,
                })?;
    }

    if rolled.is_empty() {
        return Err(DataPrepError::InsufficientDataForInterval { interval: target });
    }

    Ok(rolled)
}

fn validate_raw_bars(bars: &[Bar]) -> Result<(), DataPrepError> {
    let mut previous = None;
    for bar in bars {
        let time = open_time_ms(bar.time)?;
        if let Some(prev) = previous {
            if time == prev {
                return Err(DataPrepError::DuplicateInputBarTime { time });
            }
            if time < prev {
                return Err(DataPrepError::UnsortedInputBars {
                    previous_time: prev,
                    current_time: time,
                });
            }
        }
        previous = Some(time);
    }
    Ok(())
}

fn candidate_matches_bars(candidate: Interval, bars: &[Bar]) -> Result<bool, DataPrepError> {
    let mut saw_exact_gap = false;
    for window in bars.windows(2) {
        let prev = open_time_ms(window[0].time)?;
        let current = open_time_ms(window[1].time)?;
        if !candidate.is_aligned(prev) || !candidate.is_aligned(current) {
            return Ok(false);
        }
        if !gap_matches(candidate, prev, current, &mut saw_exact_gap)? {
            return Ok(false);
        }
    }

    Ok(saw_exact_gap)
}

fn gap_matches(
    interval: Interval,
    previous: i64,
    current: i64,
    saw_exact_gap: &mut bool,
) -> Result<bool, DataPrepError> {
    if current <= previous {
        return Ok(false);
    }

    if let Some(duration) = interval.fixed_duration_ms() {
        let delta = current - previous;
        if delta % duration != 0 {
            return Ok(false);
        }
        if delta == duration {
            *saw_exact_gap = true;
        }
        return Ok(true);
    }

    let mut next = interval
        .next_open_time(previous)
        .ok_or(DataPrepError::CannotInferInputInterval)?;
    let mut count = 1usize;
    while next < current {
        next = interval
            .next_open_time(next)
            .ok_or(DataPrepError::CannotInferInputInterval)?;
        count += 1;
    }
    if next != current {
        return Ok(false);
    }
    if count == 1 {
        *saw_exact_gap = true;
    }
    Ok(true)
}

fn expected_source_opens(
    raw: Interval,
    target: Interval,
    bucket_open: i64,
) -> Result<Vec<i64>, DataPrepError> {
    if !raw.is_aligned(bucket_open) {
        return Err(DataPrepError::UnsupportedRollupPath { raw, target });
    }

    let bucket_close = target
        .next_open_time(bucket_open)
        .ok_or(DataPrepError::UnsupportedRollupPath { raw, target })?;
    let mut opens = vec![bucket_open];
    let mut current = bucket_open;

    loop {
        let next = raw
            .next_open_time(current)
            .ok_or(DataPrepError::UnsupportedRollupPath { raw, target })?;
        if next == bucket_close {
            return Ok(opens);
        }
        if next > bucket_close {
            return Err(DataPrepError::UnsupportedRollupPath { raw, target });
        }
        opens.push(next);
        current = next;
    }
}

fn count_matching_bars(actual: &[i64], expected: &[i64]) -> usize {
    actual
        .iter()
        .zip(expected.iter())
        .take_while(|(actual, expected)| actual == expected)
        .count()
}

fn aggregate_bucket(bars: &[Bar], bucket_open: i64) -> Bar {
    let first = bars[0];
    let last = bars[bars.len() - 1];
    let mut high = first.high;
    let mut low = first.low;
    let mut volume = 0.0;

    for bar in bars {
        high = high.max(bar.high);
        low = low.min(bar.low);
        volume += bar.volume;
    }

    Bar {
        open: first.open,
        high,
        low,
        close: last.close,
        volume,
        time: bucket_open as f64,
    }
}

fn open_time_ms(time: f64) -> Result<i64, DataPrepError> {
    if !time.is_finite() || time.fract() != 0.0 {
        return Err(DataPrepError::InvalidInputBarTime { time: time as i64 });
    }
    Ok(time as i64)
}

#[cfg(test)]
mod tests {
    use super::{infer_input_interval, prepare_csv_inputs_for_program};
    use crate::{compile, Bar, DataPrepError, Interval};

    const SECOND_MS: i64 = 1_000;
    const MINUTE_MS: i64 = 60 * SECOND_MS;
    const HOUR_MS: i64 = 60 * MINUTE_MS;
    const DAY_MS: i64 = 24 * HOUR_MS;
    const WEEK_MS: i64 = 7 * DAY_MS;
    const JAN_1_2024_UTC_MS: i64 = 1_704_067_200_000;
    const FEB_1_2024_UTC_MS: i64 = 1_706_745_600_000;
    const MAR_1_2024_UTC_MS: i64 = 1_709_251_200_000;

    fn bars_with_spacing(start_ms: i64, spacing_ms: i64, closes: &[f64]) -> Vec<Bar> {
        closes
            .iter()
            .enumerate()
            .map(|(index, close)| Bar {
                open: *close - 0.5,
                high: *close + 1.0,
                low: *close - 1.0,
                close: *close,
                volume: 100.0 + index as f64,
                time: (start_ms + spacing_ms * index as i64) as f64,
            })
            .collect()
    }

    #[test]
    fn infers_minute_daily_and_monthly_input_intervals() {
        assert_eq!(
            infer_input_interval(&bars_with_spacing(
                JAN_1_2024_UTC_MS,
                MINUTE_MS,
                &[1.0, 2.0]
            ))
            .expect("minute"),
            Interval::Min1
        );
        assert_eq!(
            infer_input_interval(&bars_with_spacing(JAN_1_2024_UTC_MS, DAY_MS, &[1.0, 2.0]))
                .expect("day"),
            Interval::Day1
        );
        let monthly = vec![
            bars_with_spacing(JAN_1_2024_UTC_MS, DAY_MS, &[1.0])[0],
            bars_with_spacing(FEB_1_2024_UTC_MS, DAY_MS, &[2.0])[0],
            bars_with_spacing(MAR_1_2024_UTC_MS, DAY_MS, &[3.0])[0],
        ];
        assert_eq!(
            infer_input_interval(&monthly).expect("month"),
            Interval::Month1
        );
    }

    #[test]
    fn rejects_unsorted_duplicate_and_non_inferable_inputs() {
        let duplicate = vec![
            bars_with_spacing(JAN_1_2024_UTC_MS, MINUTE_MS, &[1.0])[0],
            bars_with_spacing(JAN_1_2024_UTC_MS, MINUTE_MS, &[2.0])[0],
        ];
        assert_eq!(
            infer_input_interval(&duplicate).expect_err("duplicate"),
            DataPrepError::DuplicateInputBarTime {
                time: JAN_1_2024_UTC_MS,
            }
        );

        let unsorted = vec![
            bars_with_spacing(JAN_1_2024_UTC_MS + MINUTE_MS, MINUTE_MS, &[1.0])[0],
            bars_with_spacing(JAN_1_2024_UTC_MS, MINUTE_MS, &[2.0])[0],
        ];
        assert_eq!(
            infer_input_interval(&unsorted).expect_err("unsorted"),
            DataPrepError::UnsortedInputBars {
                previous_time: JAN_1_2024_UTC_MS + MINUTE_MS,
                current_time: JAN_1_2024_UTC_MS,
            }
        );

        let single = bars_with_spacing(JAN_1_2024_UTC_MS, MINUTE_MS, &[1.0]);
        assert_eq!(
            infer_input_interval(&single).expect_err("single"),
            DataPrepError::CannotInferInputInterval
        );
    }

    #[test]
    fn rolls_up_minute_to_hour_and_day_to_week_and_month() {
        let hourly = prepare_csv_inputs_for_program(
            &compile("interval 1h\nplot(close)").expect("compile"),
            bars_with_spacing(JAN_1_2024_UTC_MS, MINUTE_MS, &[1.0; 120]),
        )
        .expect("hourly");
        assert_eq!(hourly.raw_interval, Interval::Min1);
        assert_eq!(hourly.base_bars.len(), 2);

        let weekly = prepare_csv_inputs_for_program(
            &compile("interval 1d\nuse 1w\nplot(1w.close)").expect("compile"),
            bars_with_spacing(JAN_1_2024_UTC_MS, DAY_MS, &[1.0; 14]),
        )
        .expect("weekly");
        assert_eq!(weekly.config.supplemental[0].bars.len(), 2);

        let monthly = prepare_csv_inputs_for_program(
            &compile("interval 1d\nuse 1M\nplot(1M.close)").expect("compile"),
            bars_with_spacing(JAN_1_2024_UTC_MS, DAY_MS, &[1.0; 60]),
        )
        .expect("monthly");
        assert_eq!(monthly.config.supplemental[0].bars.len(), 2);
        assert_eq!(monthly.base_bars.len(), 60);
    }

    #[test]
    fn rejects_incomplete_and_too_coarse_rollups() {
        let incomplete = prepare_csv_inputs_for_program(
            &compile("interval 1d\nuse 1w\nplot(1w.close)").expect("compile"),
            bars_with_spacing(JAN_1_2024_UTC_MS, DAY_MS, &[1.0; 6]),
        )
        .expect_err("incomplete");
        assert!(matches!(
            incomplete,
            DataPrepError::IncompleteRollupBucket {
                raw: Interval::Day1,
                target: Interval::Week1,
                ..
            }
        ));

        let too_coarse = prepare_csv_inputs_for_program(
            &compile("interval 1m\nplot(close)").expect("compile"),
            bars_with_spacing(JAN_1_2024_UTC_MS, DAY_MS, &[1.0, 2.0]),
        )
        .expect_err("coarse");
        assert_eq!(
            too_coarse,
            DataPrepError::RawIntervalTooCoarse {
                raw: Interval::Day1,
                required: Interval::Min1,
            }
        );
    }

    #[test]
    fn rejects_unsupported_rollup_paths() {
        let weekly_raw = bars_with_spacing(JAN_1_2024_UTC_MS, WEEK_MS, &[1.0, 2.0, 3.0, 4.0, 5.0]);
        let err = prepare_csv_inputs_for_program(
            &compile("interval 1w\nuse 1M\nplot(1M.close)").expect("compile"),
            weekly_raw,
        )
        .expect_err("unsupported");
        assert_eq!(
            err,
            DataPrepError::UnsupportedRollupPath {
                raw: Interval::Week1,
                target: Interval::Month1,
            }
        );
    }
}
