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
    #[error("expected tuple of length {expected} at pc {pc}, found {found}")]
    TupleArityMismatch {
        pc: usize,
        expected: usize,
        found: &'static str,
    },
    #[error("output `{name}` expected {expected}, found {found}")]
    OutputTypeMismatch {
        name: String,
        expected: &'static str,
        found: &'static str,
    },
    #[error("missing base feed for source {source_id}")]
    MissingSourceBaseFeed { source_id: u16 },
    #[error("duplicate base feed for source {source_id}")]
    DuplicateSourceBaseFeed { source_id: u16 },
    #[error("missing feed for source {source_id} interval {interval:?}")]
    MissingSourceIntervalFeed { source_id: u16, interval: Interval },
    #[error("duplicate feed for source {source_id} interval {interval:?}")]
    DuplicateSourceIntervalFeed { source_id: u16, interval: Interval },
    #[error("unexpected feed for source {source_id} interval {interval:?}")]
    UnexpectedSourceFeed { source_id: u16, interval: Interval },
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
    #[error("unsupported ma_type `{ma_type}` for builtin `{builtin}`")]
    UnsupportedMaType {
        builtin: &'static str,
        ma_type: &'static str,
    },
}

#[cfg(test)]
mod tests {
    use super::{CompileError, Diagnostic, DiagnosticKind, RuntimeError};
    use crate::bytecode::OpCode;
    use crate::span::{Position, Span};

    #[test]
    fn diagnostic_and_compile_error_preserve_message_and_count() {
        let diagnostic = Diagnostic::new(
            DiagnosticKind::Parse,
            "expected expression",
            Span::new(Position::new(1, 1, 2), Position::new(2, 1, 3)),
        );
        let error = CompileError::new(vec![diagnostic.clone()]);
        assert_eq!(error.diagnostics, vec![diagnostic]);
        assert_eq!(error.to_string(), "compile failed with 1 diagnostic(s)");
    }

    #[test]
    fn runtime_error_messages_include_context() {
        let stack = RuntimeError::StackUnderflow {
            pc: 4,
            opcode: OpCode::Add,
        };
        assert!(stack.to_string().contains("pc 4"));
        assert!(stack.to_string().contains("Add"));
    }
}
