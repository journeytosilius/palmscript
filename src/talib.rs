//! TA-Lib-specific typed metadata shared across compiler, IDE, and runtime.
//!
//! This module intentionally keeps the user-facing TA-Lib surface typed instead
//! of lowering optional enum parameters to strings or ad hoc numeric codes.

use serde::{Deserialize, Serialize};

pub const TALIB_UPSTREAM_COMMIT: &str = "1bdf54384036852952b8b4cb97c09359ae407bd0";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MaType {
    Sma,
    Ema,
    Wma,
    Dema,
    Tema,
    Trima,
    Kama,
    Mama,
    T3,
}

impl MaType {
    pub const ALL: [Self; 9] = [
        Self::Sma,
        Self::Ema,
        Self::Wma,
        Self::Dema,
        Self::Tema,
        Self::Trima,
        Self::Kama,
        Self::Mama,
        Self::T3,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Sma => "sma",
            Self::Ema => "ema",
            Self::Wma => "wma",
            Self::Dema => "dema",
            Self::Tema => "tema",
            Self::Trima => "trima",
            Self::Kama => "kama",
            Self::Mama => "mama",
            Self::T3 => "t3",
        }
    }

    pub fn from_variant(variant: &str) -> Option<Self> {
        match variant {
            "sma" => Some(Self::Sma),
            "ema" => Some(Self::Ema),
            "wma" => Some(Self::Wma),
            "dema" => Some(Self::Dema),
            "tema" => Some(Self::Tema),
            "trima" => Some(Self::Trima),
            "kama" => Some(Self::Kama),
            "mama" => Some(Self::Mama),
            "t3" => Some(Self::T3),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TalibGroup {
    OverlapStudies,
    MomentumIndicators,
    VolumeIndicators,
    VolatilityIndicators,
    PriceTransform,
    CycleIndicators,
    StatisticFunctions,
    MathTransform,
    MathOperators,
    PatternRecognition,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TalibFlag {
    Overlap,
    UnstablePeriod,
    Candlestick,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize)]
pub struct TalibFunctionMetadata {
    pub name: &'static str,
    pub group: TalibGroup,
    pub output_count: u8,
    pub flags: &'static [TalibFlag],
}

pub const TALIB_METADATA_SNAPSHOT: &[TalibFunctionMetadata] = &[
    TalibFunctionMetadata {
        name: "ma",
        group: TalibGroup::OverlapStudies,
        output_count: 1,
        flags: &[TalibFlag::Overlap],
    },
    TalibFunctionMetadata {
        name: "macd",
        group: TalibGroup::MomentumIndicators,
        output_count: 3,
        flags: &[],
    },
];

#[cfg(test)]
mod tests {
    use super::{MaType, TALIB_METADATA_SNAPSHOT, TALIB_UPSTREAM_COMMIT};

    #[test]
    fn ma_type_variants_round_trip() {
        for ty in MaType::ALL {
            assert_eq!(MaType::from_variant(ty.as_str()), Some(ty));
        }
        assert_eq!(MaType::from_variant("missing"), None);
    }

    #[test]
    fn metadata_snapshot_is_pinned() {
        assert_eq!(TALIB_UPSTREAM_COMMIT.len(), 40);
        assert!(TALIB_METADATA_SNAPSHOT
            .iter()
            .any(|entry| entry.name == "ma"));
        assert!(TALIB_METADATA_SNAPSHOT
            .iter()
            .any(|entry| entry.name == "macd"));
    }
}
