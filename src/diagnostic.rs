//! Compile-time and runtime error types used across the crate.
//!
//! Diagnostics preserve spans for source-level failures, while runtime errors
//! report VM faults such as stack underflow, type mismatches, and invalid
//! program state.

use crate::bytecode::OpCode;
use crate::span::Span;
use crate::Interval;
use thiserror::Error;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DiagnosticKind {
    Lex,
    Parse,
    Type,
    Compile,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Diagnostic {
    pub kind: DiagnosticKind,
    pub message: String,
    pub span: Span,
}

impl Diagnostic {
    pub fn new(kind: DiagnosticKind, message: impl Into<String>, span: Span) -> Self {
        Self {
            kind,
            message: message.into(),
            span,
        }
    }
}

#[derive(Debug, Error)]
#[error("compile failed with {diagnostics_len} diagnostic(s)")]
pub struct CompileError {
    pub diagnostics: Vec<Diagnostic>,
    diagnostics_len: usize,
}

impl CompileError {
    pub fn new(diagnostics: Vec<Diagnostic>) -> Self {
        let diagnostics_len = diagnostics.len();
        Self {
            diagnostics,
            diagnostics_len,
        }
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DataPrepError {
    #[error("CSV mode could not infer an input interval from bar timestamps")]
    CannotInferInputInterval,
    #[error("compiled strategy is missing a base interval declaration")]
    MissingBaseIntervalDeclaration,
    #[error("raw input interval {raw:?} is too coarse for required interval {required:?}")]
    RawIntervalTooCoarse { raw: Interval, required: Interval },
    #[error("cannot roll raw input interval {raw:?} into target interval {target:?}")]
    UnsupportedRollupPath { raw: Interval, target: Interval },
    #[error("input data does not contain enough complete bars for interval {interval:?}")]
    InsufficientDataForInterval { interval: Interval },
    #[error("incomplete rollup bucket for {target:?} at open time {bucket_open_time}: expected {expected} {raw:?} bar(s), found {found}")]
    IncompleteRollupBucket {
        raw: Interval,
        target: Interval,
        bucket_open_time: i64,
        expected: usize,
        found: usize,
    },
    #[error("input bars are not strictly increasing: previous time {previous_time}, current time {current_time}")]
    UnsortedInputBars {
        previous_time: i64,
        current_time: i64,
    },
    #[error("input bars contain a duplicate timestamp at {time}")]
    DuplicateInputBarTime { time: i64 },
    #[error("input bar time {time} is invalid")]
    InvalidInputBarTime { time: i64 },
}

#[derive(Debug, Error, PartialEq)]
pub enum RuntimeError {
    #[error("instruction budget exhausted at bar {bar_index}, pc {pc}")]
    InstructionBudgetExceeded { bar_index: usize, pc: usize },
    #[error("stack underflow at pc {pc} while executing {opcode:?}")]
    StackUnderflow { pc: usize, opcode: OpCode },
    #[error("type mismatch at pc {pc}: expected {expected}, found {found}")]
    TypeMismatch {
        pc: usize,
        expected: &'static str,
        found: &'static str,
    },
    #[error("arity mismatch for builtin {builtin}: expected {expected}, found {found}")]
    ArityMismatch {
        builtin: &'static str,
        expected: usize,
        found: usize,
    },
    #[error("unknown builtin id {builtin_id}")]
    UnknownBuiltin { builtin_id: u16 },
    #[error("invalid jump target {target} at pc {pc}")]
    InvalidJump { pc: usize, target: usize },
    #[error("invalid local slot {slot}")]
    InvalidLocalSlot { slot: usize },
    #[error("invalid series slot {slot}")]
    InvalidSeriesSlot { slot: usize },
    #[error("external input count mismatch: expected {expected}, found {found}")]
    ExternalInputArityMismatch { expected: usize, found: usize },
    #[error("external input `{name}` expected {expected}, found {found}")]
    ExternalInputTypeMismatch {
        name: String,
        expected: &'static str,
        found: &'static str,
    },
    #[error("output `{name}` expected {expected}, found {found}")]
    OutputTypeMismatch {
        name: String,
        expected: &'static str,
        found: &'static str,
    },
    #[error("script requires multi-interval runtime configuration")]
    MissingIntervalConfig,
    #[error("missing interval feed for {interval:?}")]
    MissingIntervalFeed { interval: Interval },
    #[error("duplicate interval feed for {interval:?}")]
    DuplicateIntervalFeed { interval: Interval },
    #[error("unexpected interval feed for {interval:?}")]
    UnexpectedIntervalFeed { interval: Interval },
    #[error("lower interval reference {referenced:?} is not allowed with base interval {base:?}")]
    LowerIntervalReference {
        base: Interval,
        referenced: Interval,
    },
    #[error("bar open time {open_time} is not aligned to interval {interval:?}")]
    InvalidIntervalAlignment { interval: Interval, open_time: i64 },
    #[error("interval feed {interval:?} is not strictly increasing at open time {open_time}")]
    UnsortedIntervalFeed { interval: Interval, open_time: i64 },
    #[error("interval feed {interval:?} contains a duplicate bar at open time {open_time}")]
    DuplicateIntervalBar { interval: Interval, open_time: i64 },
    #[error("required history {required} for slot {slot} exceeds max_history_capacity {limit}")]
    HistoryCapacityExceeded {
        slot: usize,
        required: usize,
        limit: usize,
    },
    #[error("pipeline graph contains a cycle")]
    PipelineCycle,
    #[error("pipeline node `{node}` is duplicated")]
    DuplicatePipelineNode { node: String },
    #[error("pipeline node `{node}` is missing")]
    MissingPipelineNode { node: String },
    #[error("pipeline nodes must share the same base interval; `{node}` uses {interval:?} but expected {expected:?}")]
    PipelineIntervalMismatch {
        node: String,
        interval: Interval,
        expected: Interval,
    },
    #[error("pipeline input `{input}` on node `{node}` is missing a producer")]
    MissingPipelineInput { node: String, input: String },
    #[error("pipeline input `{input}` on node `{node}` has multiple producers")]
    DuplicatePipelineInput { node: String, input: String },
    #[error("pipeline output `{output}` not found on node `{node}`")]
    MissingPipelineOutput { node: String, output: String },
    #[error("pipeline input `{input}` on node `{node}` expected {expected} but producer provides {found}")]
    PipelineInputTypeMismatch {
        node: String,
        input: String,
        expected: &'static str,
        found: &'static str,
    },
}
