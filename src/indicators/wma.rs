//! Weighted moving average helper.

use crate::diagnostic::RuntimeError;
use crate::types::Value;
use crate::vm::SeriesBuffer;

pub(crate) fn calculate(
    buffer: &SeriesBuffer,
    window: usize,
    pc: usize,
) -> Result<Value, RuntimeError> {
    if buffer.len() < window {
        return Ok(Value::NA);
    }

    let mut weighted_sum = 0.0;
    let mut weight_total = 0.0;
    for (index, value) in buffer.iter_recent(window).enumerate() {
        let weight = (window - index) as f64;
        match value {
            Value::F64(value) => {
                weighted_sum += value * weight;
                weight_total += weight;
            }
            Value::NA => return Ok(Value::NA),
            other => {
                return Err(RuntimeError::TypeMismatch {
                    pc,
                    expected: "f64",
                    found: other.type_name(),
                });
            }
        }
    }

    Ok(Value::F64(weighted_sum / weight_total))
}

#[cfg(test)]
mod tests {
    use super::calculate;
    use crate::types::Value;
    use crate::vm::SeriesBuffer;

    #[test]
    fn calculates_weighted_average() {
        let mut buffer = SeriesBuffer::new(8);
        buffer.push(Value::F64(1.0));
        buffer.push(Value::F64(2.0));
        buffer.push(Value::F64(3.0));

        assert_eq!(calculate(&buffer, 3, 0).unwrap(), Value::F64(14.0 / 6.0));
    }
}
