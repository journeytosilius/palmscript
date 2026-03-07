//! MACD tuple state built from EMA sub-components.

use crate::diagnostic::RuntimeError;
use crate::indicators::EmaState;
use crate::types::Value;
use crate::vm::SeriesBuffer;

#[derive(Clone, Debug)]
pub(crate) struct MacdState {
    fast: EmaState,
    slow: EmaState,
    signal: EmaState,
    macd_buffer: SeriesBuffer,
    last_seen_version: u64,
    cached_output: Value,
}

impl MacdState {
    pub(crate) fn new(fast: usize, slow: usize, signal: usize) -> Self {
        Self {
            fast: EmaState::new(fast),
            slow: EmaState::new(slow),
            signal: EmaState::new(signal),
            macd_buffer: SeriesBuffer::new(signal.max(2) + 1),
            last_seen_version: 0,
            cached_output: Value::Tuple3([
                Box::new(Value::NA),
                Box::new(Value::NA),
                Box::new(Value::NA),
            ]),
        }
    }

    pub(crate) fn update(
        &mut self,
        price_buffer: &SeriesBuffer,
        pc: usize,
    ) -> Result<Value, RuntimeError> {
        let version = price_buffer.version();
        if version == self.last_seen_version {
            return Ok(self.cached_output.clone());
        }
        self.last_seen_version = version;

        let fast = self.fast.update(price_buffer, pc)?;
        let slow = self.slow.update(price_buffer, pc)?;
        let macd_line = match (&fast, &slow) {
            (Value::F64(fast), Value::F64(slow)) => Value::F64(fast - slow),
            (Value::NA, _) | (_, Value::NA) => Value::NA,
            (left, right) => {
                return Err(RuntimeError::TypeMismatch {
                    pc,
                    expected: "f64",
                    found: if !matches!(left, Value::F64(_) | Value::NA) {
                        left.type_name()
                    } else {
                        right.type_name()
                    },
                });
            }
        };
        self.macd_buffer.push(macd_line.clone());
        let signal = self.signal.update(&self.macd_buffer, pc)?;
        let histogram = match (&macd_line, &signal) {
            (Value::F64(macd), Value::F64(signal)) => Value::F64(macd - signal),
            (Value::NA, _) | (_, Value::NA) => Value::NA,
            (left, right) => {
                return Err(RuntimeError::TypeMismatch {
                    pc,
                    expected: "f64",
                    found: if !matches!(left, Value::F64(_) | Value::NA) {
                        left.type_name()
                    } else {
                        right.type_name()
                    },
                });
            }
        };
        self.cached_output =
            Value::Tuple3([Box::new(macd_line), Box::new(signal), Box::new(histogram)]);
        Ok(self.cached_output.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::MacdState;
    use crate::types::Value;
    use crate::vm::SeriesBuffer;

    #[test]
    fn produces_tuple_output() {
        let mut state = MacdState::new(3, 5, 2);
        let mut buffer = SeriesBuffer::new(16);
        for price in [1.0, 2.0, 3.0, 4.0, 5.0, 6.0] {
            buffer.push(Value::F64(price));
        }
        let output = state.update(&buffer, 0).unwrap();
        assert_eq!(output.tuple_len(), Some(3));
    }
}
