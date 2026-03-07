//! Stateless statistical helpers used by TA-Lib rolling statistics builtins.

use std::f64::consts::PI;

use crate::diagnostic::RuntimeError;
use crate::types::Value;
use crate::vm::SeriesBuffer;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum RegressionOutput {
    Value,
    Angle,
    Intercept,
    Slope,
    Forecast,
}

pub(crate) fn calculate_var(
    buffer: &SeriesBuffer,
    window: usize,
    _deviations: f64,
    pc: usize,
) -> Result<Value, RuntimeError> {
    match rolling_moments(buffer, window, pc)? {
        Some((sum, sum_sq)) => {
            let mean = sum / window as f64;
            let mean_sq = sum_sq / window as f64;
            Ok(Value::F64(mean_sq - mean * mean))
        }
        None => Ok(Value::NA),
    }
}

pub(crate) fn calculate_stddev(
    buffer: &SeriesBuffer,
    window: usize,
    deviations: f64,
    pc: usize,
) -> Result<Value, RuntimeError> {
    let variance = match calculate_var(buffer, window, deviations, pc)? {
        Value::F64(value) => value,
        Value::NA => return Ok(Value::NA),
        other => {
            return Err(RuntimeError::TypeMismatch {
                pc,
                expected: "f64",
                found: other.type_name(),
            });
        }
    };

    if variance > 0.0 {
        Ok(Value::F64(variance.sqrt() * deviations))
    } else {
        Ok(Value::F64(0.0))
    }
}

pub(crate) fn calculate_linear_regression(
    buffer: &SeriesBuffer,
    window: usize,
    output: RegressionOutput,
    pc: usize,
) -> Result<Value, RuntimeError> {
    let Some((slope, intercept)) = regression_coefficients(buffer, window, pc)? else {
        return Ok(Value::NA);
    };

    let value = match output {
        RegressionOutput::Value => intercept + slope * (window - 1) as f64,
        RegressionOutput::Angle => slope.atan() * (180.0 / PI),
        RegressionOutput::Intercept => intercept,
        RegressionOutput::Slope => slope,
        RegressionOutput::Forecast => intercept + slope * window as f64,
    };
    Ok(Value::F64(value))
}

fn rolling_moments(
    buffer: &SeriesBuffer,
    window: usize,
    pc: usize,
) -> Result<Option<(f64, f64)>, RuntimeError> {
    if buffer.len() < window {
        return Ok(None);
    }

    let mut sum = 0.0;
    let mut sum_sq = 0.0;
    for value in buffer.iter_recent(window) {
        match value {
            Value::F64(value) => {
                sum += value;
                sum_sq += value * value;
            }
            Value::NA => return Ok(None),
            other => {
                return Err(RuntimeError::TypeMismatch {
                    pc,
                    expected: "f64",
                    found: other.type_name(),
                });
            }
        }
    }

    Ok(Some((sum, sum_sq)))
}

fn regression_coefficients(
    buffer: &SeriesBuffer,
    window: usize,
    pc: usize,
) -> Result<Option<(f64, f64)>, RuntimeError> {
    if buffer.len() < window {
        return Ok(None);
    }

    let mut sum_xy = 0.0;
    let mut sum_y = 0.0;
    for x in 0..window {
        let offset = window - 1 - x;
        match buffer.get(offset) {
            Value::F64(value) => {
                sum_y += value;
                sum_xy += x as f64 * value;
            }
            Value::NA => return Ok(None),
            other => {
                return Err(RuntimeError::TypeMismatch {
                    pc,
                    expected: "f64",
                    found: other.type_name(),
                });
            }
        }
    }

    let n = window as f64;
    let sum_x = n * (n - 1.0) / 2.0;
    let sum_x_sq = n * (n - 1.0) * (2.0 * n - 1.0) / 6.0;
    let divisor = n * sum_x_sq - sum_x * sum_x;
    if divisor == 0.0 {
        return Ok(Some((0.0, sum_y / n)));
    }

    let slope = (n * sum_xy - sum_x * sum_y) / divisor;
    let intercept = (sum_y - slope * sum_x) / n;
    Ok(Some((slope, intercept)))
}

#[cfg(test)]
mod tests {
    use super::{calculate_linear_regression, calculate_stddev, calculate_var, RegressionOutput};
    use crate::types::Value;
    use crate::vm::SeriesBuffer;

    #[test]
    fn variance_matches_population_variance() {
        let mut buffer = SeriesBuffer::new(8);
        for value in [1.0, 2.0, 3.0, 4.0, 5.0] {
            buffer.push(Value::F64(value));
        }

        assert_eq!(calculate_var(&buffer, 5, 3.0, 0).unwrap(), Value::F64(2.0));
    }

    #[test]
    fn stddev_applies_deviation_multiplier() {
        let mut buffer = SeriesBuffer::new(8);
        for value in [1.0, 2.0, 3.0, 4.0, 5.0] {
            buffer.push(Value::F64(value));
        }

        assert_eq!(
            calculate_stddev(&buffer, 5, 2.0, 0).unwrap(),
            Value::F64(2.0 * 2.0_f64.sqrt())
        );
    }

    #[test]
    fn linear_regression_family_matches_simple_line() {
        let mut buffer = SeriesBuffer::new(8);
        for value in [1.0, 2.0, 3.0, 4.0, 5.0] {
            buffer.push(Value::F64(value));
        }

        assert_eq!(
            calculate_linear_regression(&buffer, 5, RegressionOutput::Value, 0).unwrap(),
            Value::F64(5.0)
        );
        assert_eq!(
            calculate_linear_regression(&buffer, 5, RegressionOutput::Intercept, 0).unwrap(),
            Value::F64(1.0)
        );
        assert_eq!(
            calculate_linear_regression(&buffer, 5, RegressionOutput::Slope, 0).unwrap(),
            Value::F64(1.0)
        );
        assert_eq!(
            calculate_linear_regression(&buffer, 5, RegressionOutput::Forecast, 0).unwrap(),
            Value::F64(6.0)
        );
        assert_eq!(
            calculate_linear_regression(&buffer, 5, RegressionOutput::Angle, 0).unwrap(),
            Value::F64(45.0)
        );
    }
}
