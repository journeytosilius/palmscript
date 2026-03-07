//! Builtin function identifiers and metadata shared across the compiler, IDE,
//! and VM.
//!
//! The builtin registry is the source of truth for reserved names, callable
//! surface, signatures, and broad implementation class.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BuiltinKind {
    MarketSeries,
    Plot,
    Indicator,
    MovingAverage,
    IndicatorTuple,
    Relation2,
    Relation3,
    Cross,
    Change,
    Roc,
    Highest,
    Lowest,
    Rising,
    Falling,
    BarsSince,
    ValueWhen,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u16)]
pub enum BuiltinId {
    Open = 0,
    High = 1,
    Low = 2,
    Close = 3,
    Volume = 4,
    Time = 5,
    Sma = 6,
    Ema = 7,
    Rsi = 8,
    Plot = 9,
    Above = 10,
    Below = 11,
    Between = 12,
    Outside = 13,
    Cross = 14,
    Crossover = 15,
    Crossunder = 16,
    Change = 17,
    Roc = 18,
    Highest = 19,
    Lowest = 20,
    Rising = 21,
    Falling = 22,
    BarsSince = 23,
    ValueWhen = 24,
    Ma = 25,
    Macd = 26,
}

impl BuiltinId {
    pub const RESERVED: [Self; 27] = [
        Self::Open,
        Self::High,
        Self::Low,
        Self::Close,
        Self::Volume,
        Self::Time,
        Self::Sma,
        Self::Ema,
        Self::Rsi,
        Self::Plot,
        Self::Above,
        Self::Below,
        Self::Between,
        Self::Outside,
        Self::Cross,
        Self::Crossover,
        Self::Crossunder,
        Self::Change,
        Self::Roc,
        Self::Highest,
        Self::Lowest,
        Self::Rising,
        Self::Falling,
        Self::BarsSince,
        Self::ValueWhen,
        Self::Ma,
        Self::Macd,
    ];

    pub const CALLABLE: [Self; 21] = [
        Self::Sma,
        Self::Ema,
        Self::Rsi,
        Self::Plot,
        Self::Above,
        Self::Below,
        Self::Between,
        Self::Outside,
        Self::Cross,
        Self::Crossover,
        Self::Crossunder,
        Self::Change,
        Self::Roc,
        Self::Highest,
        Self::Lowest,
        Self::Rising,
        Self::Falling,
        Self::BarsSince,
        Self::ValueWhen,
        Self::Ma,
        Self::Macd,
    ];

    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "open" => Some(Self::Open),
            "high" => Some(Self::High),
            "low" => Some(Self::Low),
            "close" => Some(Self::Close),
            "volume" => Some(Self::Volume),
            "time" => Some(Self::Time),
            "sma" => Some(Self::Sma),
            "ema" => Some(Self::Ema),
            "rsi" => Some(Self::Rsi),
            "plot" => Some(Self::Plot),
            "above" => Some(Self::Above),
            "below" => Some(Self::Below),
            "between" => Some(Self::Between),
            "outside" => Some(Self::Outside),
            "cross" => Some(Self::Cross),
            "crossover" => Some(Self::Crossover),
            "crossunder" => Some(Self::Crossunder),
            "change" => Some(Self::Change),
            "roc" => Some(Self::Roc),
            "highest" => Some(Self::Highest),
            "lowest" => Some(Self::Lowest),
            "rising" => Some(Self::Rising),
            "falling" => Some(Self::Falling),
            "barssince" => Some(Self::BarsSince),
            "valuewhen" => Some(Self::ValueWhen),
            "ma" => Some(Self::Ma),
            "macd" => Some(Self::Macd),
            _ => None,
        }
    }

    pub fn from_u16(id: u16) -> Option<Self> {
        match id {
            0 => Some(Self::Open),
            1 => Some(Self::High),
            2 => Some(Self::Low),
            3 => Some(Self::Close),
            4 => Some(Self::Volume),
            5 => Some(Self::Time),
            6 => Some(Self::Sma),
            7 => Some(Self::Ema),
            8 => Some(Self::Rsi),
            9 => Some(Self::Plot),
            10 => Some(Self::Above),
            11 => Some(Self::Below),
            12 => Some(Self::Between),
            13 => Some(Self::Outside),
            14 => Some(Self::Cross),
            15 => Some(Self::Crossover),
            16 => Some(Self::Crossunder),
            17 => Some(Self::Change),
            18 => Some(Self::Roc),
            19 => Some(Self::Highest),
            20 => Some(Self::Lowest),
            21 => Some(Self::Rising),
            22 => Some(Self::Falling),
            23 => Some(Self::BarsSince),
            24 => Some(Self::ValueWhen),
            25 => Some(Self::Ma),
            26 => Some(Self::Macd),
            _ => None,
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::High => "high",
            Self::Low => "low",
            Self::Close => "close",
            Self::Volume => "volume",
            Self::Time => "time",
            Self::Sma => "sma",
            Self::Ema => "ema",
            Self::Rsi => "rsi",
            Self::Plot => "plot",
            Self::Above => "above",
            Self::Below => "below",
            Self::Between => "between",
            Self::Outside => "outside",
            Self::Cross => "cross",
            Self::Crossover => "crossover",
            Self::Crossunder => "crossunder",
            Self::Change => "change",
            Self::Roc => "roc",
            Self::Highest => "highest",
            Self::Lowest => "lowest",
            Self::Rising => "rising",
            Self::Falling => "falling",
            Self::BarsSince => "barssince",
            Self::ValueWhen => "valuewhen",
            Self::Ma => "ma",
            Self::Macd => "macd",
        }
    }

    pub const fn kind(self) -> BuiltinKind {
        match self {
            Self::Open | Self::High | Self::Low | Self::Close | Self::Volume | Self::Time => {
                BuiltinKind::MarketSeries
            }
            Self::Plot => BuiltinKind::Plot,
            Self::Sma | Self::Ema | Self::Rsi => BuiltinKind::Indicator,
            Self::Ma => BuiltinKind::MovingAverage,
            Self::Macd => BuiltinKind::IndicatorTuple,
            Self::Above | Self::Below => BuiltinKind::Relation2,
            Self::Between | Self::Outside => BuiltinKind::Relation3,
            Self::Cross | Self::Crossover | Self::Crossunder => BuiltinKind::Cross,
            Self::Change => BuiltinKind::Change,
            Self::Roc => BuiltinKind::Roc,
            Self::Highest => BuiltinKind::Highest,
            Self::Lowest => BuiltinKind::Lowest,
            Self::Rising => BuiltinKind::Rising,
            Self::Falling => BuiltinKind::Falling,
            Self::BarsSince => BuiltinKind::BarsSince,
            Self::ValueWhen => BuiltinKind::ValueWhen,
        }
    }

    pub const fn is_callable(self) -> bool {
        !matches!(self.kind(), BuiltinKind::MarketSeries)
    }

    pub const fn arity(self) -> Option<usize> {
        match self.kind() {
            BuiltinKind::MarketSeries => None,
            BuiltinKind::Plot | BuiltinKind::BarsSince => Some(1),
            BuiltinKind::Indicator
            | BuiltinKind::Relation2
            | BuiltinKind::Cross
            | BuiltinKind::Change
            | BuiltinKind::Roc
            | BuiltinKind::Highest
            | BuiltinKind::Lowest
            | BuiltinKind::Rising
            | BuiltinKind::Falling => Some(2),
            BuiltinKind::MovingAverage | BuiltinKind::Relation3 | BuiltinKind::ValueWhen => Some(3),
            BuiltinKind::IndicatorTuple => Some(4),
        }
    }

    pub const fn signature(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::High => "high",
            Self::Low => "low",
            Self::Close => "close",
            Self::Volume => "volume",
            Self::Time => "time",
            Self::Sma => "sma(series, length)",
            Self::Ema => "ema(series, length)",
            Self::Rsi => "rsi(series, length)",
            Self::Plot => "plot(value)",
            Self::Above => "above(a, b)",
            Self::Below => "below(a, b)",
            Self::Between => "between(x, low, high)",
            Self::Outside => "outside(x, low, high)",
            Self::Cross => "cross(a, b)",
            Self::Crossover => "crossover(a, b)",
            Self::Crossunder => "crossunder(a, b)",
            Self::Change => "change(series, length)",
            Self::Roc => "roc(series, length)",
            Self::Highest => "highest(series, length)",
            Self::Lowest => "lowest(series, length)",
            Self::Rising => "rising(series, length)",
            Self::Falling => "falling(series, length)",
            Self::BarsSince => "barssince(condition)",
            Self::ValueWhen => "valuewhen(condition, source, occurrence)",
            Self::Ma => "ma(series, length, ma_type)",
            Self::Macd => "macd(series, fast_length, slow_length, signal_length)",
        }
    }

    pub const fn summary(self) -> &'static str {
        match self {
            Self::Open => "series<float> for the base-interval open.",
            Self::High => "series<float> for the base-interval high.",
            Self::Low => "series<float> for the base-interval low.",
            Self::Close => "series<float> for the base-interval close.",
            Self::Volume => "series<float> for the base-interval volume.",
            Self::Time => "series<float> for the base-interval candle open time.",
            Self::Sma => "Simple moving average.",
            Self::Ema => "Exponential moving average.",
            Self::Rsi => "Relative strength index.",
            Self::Plot => "Emit a plot output for the current bar.",
            Self::Above => "True when `a > b`.",
            Self::Below => "True when `a < b`.",
            Self::Between => "True when `low < x` and `x < high`.",
            Self::Outside => "True when `x < low` or `x > high`.",
            Self::Cross => "True when `a` crosses `b` in either direction.",
            Self::Crossover => "True when `a` crosses above `b`.",
            Self::Crossunder => "True when `a` crosses below `b`.",
            Self::Change => "Difference between the current sample and a prior sample.",
            Self::Roc => "Rate of change in percent.",
            Self::Highest => "Highest value over a trailing window including the current sample.",
            Self::Lowest => "Lowest value over a trailing window including the current sample.",
            Self::Rising => "True when the current sample is strictly greater than every prior sample in the trailing window.",
            Self::Falling => "True when the current sample is strictly less than every prior sample in the trailing window.",
            Self::BarsSince => "Bars since the last true condition on the condition's update clock.",
            Self::ValueWhen => "Captured source value from the Nth most recent true condition.",
            Self::Ma => "TA-Lib moving average with typed ma_type selection.",
            Self::Macd => "Moving average convergence/divergence tuple (macd, signal, histogram).",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::BuiltinId;

    #[test]
    fn builtin_name_and_numeric_lookups_round_trip() {
        for builtin in BuiltinId::RESERVED {
            assert_eq!(BuiltinId::from_name(builtin.as_str()), Some(builtin));
            assert_eq!(BuiltinId::from_u16(builtin as u16), Some(builtin));
        }
        assert_eq!(BuiltinId::from_name("missing"), None);
        assert_eq!(BuiltinId::from_u16(99), None);
    }

    #[test]
    fn callable_builtins_have_arity_and_market_series_do_not() {
        for builtin in BuiltinId::CALLABLE {
            assert!(builtin.is_callable());
            assert!(builtin.arity().is_some(), "{builtin:?}");
        }
        for builtin in [
            BuiltinId::Open,
            BuiltinId::High,
            BuiltinId::Low,
            BuiltinId::Close,
            BuiltinId::Volume,
            BuiltinId::Time,
        ] {
            assert!(!builtin.is_callable());
            assert_eq!(builtin.arity(), None);
        }
    }
}
