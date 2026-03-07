//! Volume indicator state.

use crate::diagnostic::RuntimeError;
use crate::types::Value;
use crate::vm::SeriesBuffer;

#[derive(Clone, Debug)]
pub(crate) struct ObvState {
    initialized: bool,
    last_price_version: u64,
    last_volume_version: u64,
    last_close: f64,
    value: f64,
    cached_output: Value,
}

impl ObvState {
    pub(crate) fn new() -> Self {
        Self {
            initialized: false,
            last_price_version: 0,
            last_volume_version: 0,
            last_close: 0.0,
            value: 0.0,
            cached_output: Value::NA,
        }
    }

    pub(crate) fn update(
        &mut self,
        price_buffer: &SeriesBuffer,
        volume_buffer: &SeriesBuffer,
        pc: usize,
    ) -> Result<Value, RuntimeError> {
        if price_buffer.version() == self.last_price_version
            && volume_buffer.version() == self.last_volume_version
        {
            return Ok(self.cached_output.clone());
        }
        self.last_price_version = price_buffer.version();
        self.last_volume_version = volume_buffer.version();

        let current_close = match price_buffer.get(0) {
            Value::F64(value) => value,
            Value::NA => {
                self.cached_output = Value::NA;
                return Ok(Value::NA);
            }
            other => {
                return Err(RuntimeError::TypeMismatch {
                    pc,
                    expected: "f64",
                    found: other.type_name(),
                });
            }
        };
        let current_volume = match volume_buffer.get(0) {
            Value::F64(value) => value,
            Value::NA => {
                self.cached_output = Value::NA;
                return Ok(Value::NA);
            }
            other => {
                return Err(RuntimeError::TypeMismatch {
                    pc,
                    expected: "f64",
                    found: other.type_name(),
                });
            }
        };

        if !self.initialized {
            self.initialized = true;
            self.last_close = current_close;
            self.value = current_volume;
            self.cached_output = Value::F64(self.value);
            return Ok(self.cached_output.clone());
        }

        if current_close > self.last_close {
            self.value += current_volume;
        } else if current_close < self.last_close {
            self.value -= current_volume;
        }
        self.last_close = current_close;
        self.cached_output = Value::F64(self.value);
        Ok(self.cached_output.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::ObvState;
    use crate::types::Value;
    use crate::vm::SeriesBuffer;

    #[test]
    fn obv_seeds_from_first_volume_and_accumulates_directionally() {
        let mut state = ObvState::new();
        let mut price = SeriesBuffer::new(8);
        let mut volume = SeriesBuffer::new(8);

        price.push(Value::F64(10.0));
        volume.push(Value::F64(100.0));
        assert_eq!(state.update(&price, &volume, 0).unwrap(), Value::F64(100.0));

        price.push(Value::F64(11.0));
        volume.push(Value::F64(50.0));
        assert_eq!(state.update(&price, &volume, 0).unwrap(), Value::F64(150.0));

        price.push(Value::F64(9.0));
        volume.push(Value::F64(25.0));
        assert_eq!(state.update(&price, &volume, 0).unwrap(), Value::F64(125.0));
    }
}
