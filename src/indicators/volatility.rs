//! Volatility helpers used by low-state TA-Lib builtins.

use crate::diagnostic::RuntimeError;
use crate::types::Value;
use crate::vm::SeriesBuffer;

pub(crate) fn calculate_trange(
    high: &SeriesBuffer,
    low: &SeriesBuffer,
    close: &SeriesBuffer,
    pc: usize,
) -> Result<Value, RuntimeError> {
    if high.is_empty() || low.is_empty() || close.len() < 2 {
        return Ok(Value::NA);
    }

    let current_high = expect_buffer_value(high, 0, pc)?;
    let current_low = expect_buffer_value(low, 0, pc)?;
    let previous_close = expect_buffer_value(close, 1, pc)?;
    let Some(current_high) = current_high else {
        return Ok(Value::NA);
    };
    let Some(current_low) = current_low else {
        return Ok(Value::NA);
    };
    let Some(previous_close) = previous_close else {
        return Ok(Value::NA);
    };

    let mut greatest = current_high - current_low;
    let distance_to_high = (previous_close - current_high).abs();
    if distance_to_high > greatest {
        greatest = distance_to_high;
    }
    let distance_to_low = (previous_close - current_low).abs();
    if distance_to_low > greatest {
        greatest = distance_to_low;
    }
    Ok(Value::F64(greatest))
}

fn expect_buffer_value(
    buffer: &SeriesBuffer,
    offset: usize,
    pc: usize,
) -> Result<Option<f64>, RuntimeError> {
    match buffer.get(offset) {
        Value::F64(value) => Ok(Some(value)),
        Value::NA => Ok(None),
        other => Err(RuntimeError::TypeMismatch {
            pc,
            expected: "f64",
            found: other.type_name(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::calculate_trange;
    use crate::types::Value;
    use crate::vm::SeriesBuffer;

    #[test]
    fn trange_uses_prior_close_and_skips_first_bar() {
        let mut high = SeriesBuffer::new(8);
        let mut low = SeriesBuffer::new(8);
        let mut close = SeriesBuffer::new(8);

        high.push(Value::F64(11.0));
        low.push(Value::F64(9.0));
        close.push(Value::F64(10.0));
        assert_eq!(calculate_trange(&high, &low, &close, 0).unwrap(), Value::NA);

        high.push(Value::F64(13.0));
        low.push(Value::F64(11.0));
        close.push(Value::F64(12.0));
        assert_eq!(
            calculate_trange(&high, &low, &close, 0).unwrap(),
            Value::F64(3.0)
        );
    }
}
