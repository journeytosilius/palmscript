//! Event-memory helper state.

use std::collections::VecDeque;

use crate::diagnostic::RuntimeError;
use crate::types::Value;
use crate::vm::SeriesBuffer;

#[derive(Clone, Debug)]
pub(crate) struct BarsSinceState {
    seen_true: bool,
    bars_since: usize,
    last_seen_version: u64,
    cached_output: Value,
}

#[derive(Clone, Debug)]
pub(crate) struct ValueWhenState {
    occurrence: usize,
    last_seen_version: u64,
    cached_output: Value,
    matches: VecDeque<Value>,
}

impl BarsSinceState {
    pub(crate) fn new() -> Self {
        Self {
            seen_true: false,
            bars_since: 0,
            last_seen_version: 0,
            cached_output: Value::NA,
        }
    }

    pub(crate) fn update(
        &mut self,
        condition: &SeriesBuffer,
        pc: usize,
    ) -> Result<Value, RuntimeError> {
        if condition.version() == self.last_seen_version {
            return Ok(self.cached_output.clone());
        }
        self.last_seen_version = condition.version();
        match condition.get(0) {
            Value::Bool(true) => {
                self.seen_true = true;
                self.bars_since = 0;
                self.cached_output = Value::F64(0.0);
            }
            Value::Bool(false) => {
                if self.seen_true {
                    self.bars_since += 1;
                    self.cached_output = Value::F64(self.bars_since as f64);
                } else {
                    self.cached_output = Value::NA;
                }
            }
            Value::NA => {
                if self.seen_true {
                    self.bars_since += 1;
                }
                self.cached_output = Value::NA;
            }
            other => {
                return Err(RuntimeError::TypeMismatch {
                    pc,
                    expected: "bool",
                    found: other.type_name(),
                });
            }
        }
        Ok(self.cached_output.clone())
    }
}

impl ValueWhenState {
    pub(crate) fn new(occurrence: usize) -> Self {
        Self {
            occurrence,
            last_seen_version: 0,
            cached_output: Value::NA,
            matches: VecDeque::with_capacity(occurrence + 1),
        }
    }

    pub(crate) fn update(
        &mut self,
        condition: &SeriesBuffer,
        source: &SeriesBuffer,
        pc: usize,
    ) -> Result<Value, RuntimeError> {
        if condition.version() == self.last_seen_version {
            return Ok(self.cached_output.clone());
        }
        self.last_seen_version = condition.version();

        match condition.get(0) {
            Value::Bool(true) => {
                let current = source.get(0);
                match current {
                    Value::F64(_) | Value::Bool(_) | Value::NA => {
                        if self.matches.len() == self.occurrence + 1 {
                            self.matches.pop_front();
                        }
                        self.matches.push_back(current);
                        self.cached_output = self.lookup_occurrence();
                    }
                    other => {
                        return Err(RuntimeError::TypeMismatch {
                            pc,
                            expected: "f64-or-bool",
                            found: other.type_name(),
                        });
                    }
                }
            }
            Value::Bool(false) => {
                self.cached_output = self.lookup_occurrence();
            }
            Value::NA => {
                self.cached_output = Value::NA;
            }
            other => {
                return Err(RuntimeError::TypeMismatch {
                    pc,
                    expected: "bool",
                    found: other.type_name(),
                });
            }
        }

        Ok(self.cached_output.clone())
    }

    fn lookup_occurrence(&self) -> Value {
        if self.matches.len() <= self.occurrence {
            Value::NA
        } else {
            self.matches[self.matches.len() - 1 - self.occurrence].clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{BarsSinceState, ValueWhenState};
    use crate::types::Value;
    use crate::vm::SeriesBuffer;

    #[test]
    fn barssince_tracks_true_events() {
        let mut state = BarsSinceState::new();
        let mut cond = SeriesBuffer::new(8);

        cond.push(Value::Bool(false));
        assert_eq!(state.update(&cond, 0).unwrap(), Value::NA);
        cond.push(Value::Bool(true));
        assert_eq!(state.update(&cond, 0).unwrap(), Value::F64(0.0));
        cond.push(Value::Bool(false));
        assert_eq!(state.update(&cond, 0).unwrap(), Value::F64(1.0));
    }

    #[test]
    fn valuewhen_returns_recent_matches_by_occurrence() {
        let mut state = ValueWhenState::new(1);
        let mut cond = SeriesBuffer::new(8);
        let mut source = SeriesBuffer::new(8);

        source.push(Value::F64(1.0));
        cond.push(Value::Bool(true));
        assert_eq!(state.update(&cond, &source, 0).unwrap(), Value::NA);

        source.push(Value::F64(2.0));
        cond.push(Value::Bool(false));
        assert_eq!(state.update(&cond, &source, 0).unwrap(), Value::NA);

        source.push(Value::F64(3.0));
        cond.push(Value::Bool(true));
        assert_eq!(state.update(&cond, &source, 0).unwrap(), Value::F64(1.0));
    }

    #[test]
    fn valuewhen_propagates_na_condition_without_erasing_history() {
        let mut state = ValueWhenState::new(0);
        let mut cond = SeriesBuffer::new(8);
        let mut source = SeriesBuffer::new(8);

        source.push(Value::Bool(true));
        cond.push(Value::Bool(true));
        assert_eq!(state.update(&cond, &source, 0).unwrap(), Value::Bool(true));

        source.push(Value::Bool(false));
        cond.push(Value::NA);
        assert_eq!(state.update(&cond, &source, 0).unwrap(), Value::NA);

        source.push(Value::Bool(false));
        cond.push(Value::Bool(false));
        assert_eq!(state.update(&cond, &source, 0).unwrap(), Value::Bool(true));
    }
}
