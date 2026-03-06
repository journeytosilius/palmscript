use palmscript::{
    bytecode::Program, compile, prepare_csv_inputs_for_program, Bar, CompiledProgram, DataPrepError,
};

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

fn bar(time: i64, close: f64) -> Bar {
    minute_bar(time, close)
}

fn compiled(source: &str) -> CompiledProgram {
    compile(source).expect("script compiles")
}

// Reachable public DataPrepError catalog:
// - CannotInferInputInterval
// - MissingBaseIntervalDeclaration
// - RawIntervalTooCoarse
// - UnsupportedRollupPath
// - IncompleteRollupBucket
// - UnsortedInputBars
// - DuplicateInputBarTime
// - InvalidInputBarTime
//
// Internal-only with today's public preparation entrypoint:
// - InsufficientDataForInterval
//   Reason: `prepare_csv_inputs_for_program` fails earlier with interval inference or
//   incomplete-bucket errors before this variant is exposed.

#[test]
fn data_prep_error_catalog_matches_contract() {
    let empty_compiled = CompiledProgram {
        program: Program::default(),
        source: String::new(),
    };
    let cases: [(&str, Result<(), DataPrepError>); 8] = [
        (
            "missing_base_interval",
            prepare_csv_inputs_for_program(
                &empty_compiled,
                vec![minute_bar(1_704_067_200_000, 1.0)],
            )
            .map(|_| ()),
        ),
        (
            "cannot_infer_input_interval",
            prepare_csv_inputs_for_program(
                &compiled("interval 1m\nplot(close)"),
                vec![minute_bar(1_704_067_200_000, 1.0)],
            )
            .map(|_| ()),
        ),
        (
            "raw_interval_too_coarse",
            prepare_csv_inputs_for_program(
                &compiled("interval 1m\nplot(close)"),
                vec![
                    minute_bar(1_704_067_200_000, 1.0),
                    minute_bar(1_704_153_600_000, 2.0),
                ],
            )
            .map(|_| ()),
        ),
        (
            "unsupported_rollup_path",
            prepare_csv_inputs_for_program(
                &compiled("interval 1w\nuse 1M\nplot(1M.close)"),
                vec![
                    bar(1_704_067_200_000, 1.0),
                    bar(1_704_672_000_000, 2.0),
                    bar(1_705_276_800_000, 3.0),
                    bar(1_705_881_600_000, 4.0),
                    bar(1_706_486_400_000, 5.0),
                ],
            )
            .map(|_| ()),
        ),
        (
            "incomplete_rollup_bucket",
            prepare_csv_inputs_for_program(
                &compiled("interval 1d\nuse 1w\nplot(1w.close)"),
                (0..6)
                    .map(|index| {
                        minute_bar(1_704_067_200_000 + 86_400_000 * index, index as f64 + 1.0)
                    })
                    .collect(),
            )
            .map(|_| ()),
        ),
        (
            "unsorted_input_bars",
            prepare_csv_inputs_for_program(
                &compiled("interval 1m\nplot(close)"),
                vec![
                    minute_bar(1_704_067_260_000, 2.0),
                    minute_bar(1_704_067_200_000, 1.0),
                ],
            )
            .map(|_| ()),
        ),
        (
            "duplicate_input_bar_time",
            prepare_csv_inputs_for_program(
                &compiled("interval 1m\nplot(close)"),
                vec![
                    minute_bar(1_704_067_200_000, 1.0),
                    minute_bar(1_704_067_200_000, 2.0),
                ],
            )
            .map(|_| ()),
        ),
        (
            "invalid_input_bar_time",
            prepare_csv_inputs_for_program(
                &compiled("interval 1m\nplot(close)"),
                vec![
                    minute_bar(1_704_067_200_000, 1.0),
                    Bar {
                        time: 1_704_067_260_000.5,
                        ..minute_bar(1_704_067_260_000, 2.0)
                    },
                ],
            )
            .map(|_| ()),
        ),
    ];

    let expected = [
        "compiled strategy is missing a base interval declaration",
        "CSV mode could not infer an input interval from bar timestamps",
        "raw input interval Day1 is too coarse for required interval Min1",
        "cannot roll raw input interval Week1 into target interval Month1",
        "incomplete rollup bucket for Week1 at open time 1704067200000: expected 7 Day1 bar(s), found 6",
        "input bars are not strictly increasing: previous time 1704067260000, current time 1704067200000",
        "input bars contain a duplicate timestamp at 1704067200000",
        "input bar time 1704067260000 is invalid",
    ];

    for ((name, result), expected_message) in cases.into_iter().zip(expected) {
        let err = result.expect_err(name);
        assert_eq!(err.to_string(), expected_message, "{name}");
    }
}
