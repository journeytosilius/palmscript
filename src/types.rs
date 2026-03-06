//! Core value and type definitions shared by the compiler and runtime.
//!
//! These enums define the typed boundary for scalar values, series references,
//! and local slot kinds used throughout the VM.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Type {
    F64,
    Bool,
    SeriesF64,
    SeriesBool,
    Void,
}

impl Type {
    pub const fn is_series(self) -> bool {
        matches!(self, Self::SeriesF64 | Self::SeriesBool)
    }

    pub const fn scalar(self) -> Option<Self> {
        match self {
            Self::SeriesF64 => Some(Self::F64),
            Self::SeriesBool => Some(Self::Bool),
            Self::F64 | Self::Bool | Self::Void => Some(self),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SlotKind {
    Scalar,
    Series,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Value {
    F64(f64),
    Bool(bool),
    NA,
    Void,
    SeriesRef(usize),
}

impl Value {
    pub const fn type_name(&self) -> &'static str {
        match self {
            Self::F64(_) => "f64",
            Self::Bool(_) => "bool",
            Self::NA => "na",
            Self::Void => "void",
            Self::SeriesRef(_) => "series-ref",
        }
    }

    pub const fn is_na(&self) -> bool {
        matches!(self, Self::NA)
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Self::F64(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(value) => Some(*value),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Type, Value};

    #[test]
    fn type_helpers_distinguish_series_and_scalar_forms() {
        assert!(Type::SeriesF64.is_series());
        assert!(Type::SeriesBool.is_series());
        assert!(!Type::F64.is_series());
        assert_eq!(Type::SeriesF64.scalar(), Some(Type::F64));
        assert_eq!(Type::SeriesBool.scalar(), Some(Type::Bool));
        assert_eq!(Type::Bool.scalar(), Some(Type::Bool));
        assert_eq!(Type::Void.scalar(), Some(Type::Void));
    }

    #[test]
    fn value_accessors_and_type_names_match_variants() {
        assert_eq!(Value::F64(1.5).type_name(), "f64");
        assert_eq!(Value::Bool(true).type_name(), "bool");
        assert_eq!(Value::NA.type_name(), "na");
        assert_eq!(Value::Void.type_name(), "void");
        assert_eq!(Value::SeriesRef(3).type_name(), "series-ref");

        assert_eq!(Value::F64(1.5).as_f64(), Some(1.5));
        assert_eq!(Value::Bool(true).as_bool(), Some(true));
        assert_eq!(Value::Bool(true).as_f64(), None);
        assert_eq!(Value::F64(1.5).as_bool(), None);
        assert!(Value::NA.is_na());
        assert!(!Value::Void.is_na());
    }
}
