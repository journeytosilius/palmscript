//! Position- and exit-scoped types shared by the compiler, runtime, and
//! backtester.
//!
//! These types model position-aware fields exposed to attached exits, recent
//! closed-trade fields exposed to strategy logic, and the shared enum types
//! reused across the public API and backtest internals.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PositionSide {
    Long,
    Short,
}

impl PositionSide {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Long => "long",
            Self::Short => "short",
        }
    }

    pub fn from_variant(variant: &str) -> Option<Self> {
        match variant {
            "long" => Some(Self::Long),
            "short" => Some(Self::Short),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ExitKind {
    Protect,
    Target,
    Signal,
    Reversal,
}

impl ExitKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Protect => "protect",
            Self::Target => "target",
            Self::Signal => "signal",
            Self::Reversal => "reversal",
        }
    }

    pub fn from_variant(variant: &str) -> Option<Self> {
        match variant {
            "protect" => Some(Self::Protect),
            "target" => Some(Self::Target),
            "signal" => Some(Self::Signal),
            "reversal" => Some(Self::Reversal),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PositionField {
    EntryPrice,
    EntryTime,
    EntryBarIndex,
    BarsHeld,
    IsLong,
    IsShort,
    Side,
    MarketPrice,
    UnrealizedPnl,
    UnrealizedReturn,
    Mae,
    Mfe,
}

impl PositionField {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EntryPrice => "entry_price",
            Self::EntryTime => "entry_time",
            Self::EntryBarIndex => "entry_bar_index",
            Self::BarsHeld => "bars_held",
            Self::IsLong => "is_long",
            Self::IsShort => "is_short",
            Self::Side => "side",
            Self::MarketPrice => "market_price",
            Self::UnrealizedPnl => "unrealized_pnl",
            Self::UnrealizedReturn => "unrealized_return",
            Self::Mae => "mae",
            Self::Mfe => "mfe",
        }
    }

    pub fn parse(name: &str) -> Option<Self> {
        match name {
            "entry_price" => Some(Self::EntryPrice),
            "entry_time" => Some(Self::EntryTime),
            "entry_bar_index" => Some(Self::EntryBarIndex),
            "bars_held" => Some(Self::BarsHeld),
            "is_long" => Some(Self::IsLong),
            "is_short" => Some(Self::IsShort),
            "side" => Some(Self::Side),
            "market_price" => Some(Self::MarketPrice),
            "unrealized_pnl" => Some(Self::UnrealizedPnl),
            "unrealized_return" => Some(Self::UnrealizedReturn),
            "mae" => Some(Self::Mae),
            "mfe" => Some(Self::Mfe),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PositionEventField {
    LongEntryFill,
    ShortEntryFill,
    LongExitFill,
    ShortExitFill,
    LongProtectFill,
    ShortProtectFill,
    LongTargetFill,
    ShortTargetFill,
    LongSignalExitFill,
    ShortSignalExitFill,
    LongReversalExitFill,
    ShortReversalExitFill,
}

impl PositionEventField {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LongEntryFill => "long_entry_fill",
            Self::ShortEntryFill => "short_entry_fill",
            Self::LongExitFill => "long_exit_fill",
            Self::ShortExitFill => "short_exit_fill",
            Self::LongProtectFill => "long_protect_fill",
            Self::ShortProtectFill => "short_protect_fill",
            Self::LongTargetFill => "long_target_fill",
            Self::ShortTargetFill => "short_target_fill",
            Self::LongSignalExitFill => "long_signal_exit_fill",
            Self::ShortSignalExitFill => "short_signal_exit_fill",
            Self::LongReversalExitFill => "long_reversal_exit_fill",
            Self::ShortReversalExitFill => "short_reversal_exit_fill",
        }
    }

    pub fn parse(name: &str) -> Option<Self> {
        match name {
            "long_entry_fill" => Some(Self::LongEntryFill),
            "short_entry_fill" => Some(Self::ShortEntryFill),
            "long_exit_fill" => Some(Self::LongExitFill),
            "short_exit_fill" => Some(Self::ShortExitFill),
            "long_protect_fill" => Some(Self::LongProtectFill),
            "short_protect_fill" => Some(Self::ShortProtectFill),
            "long_target_fill" => Some(Self::LongTargetFill),
            "short_target_fill" => Some(Self::ShortTargetFill),
            "long_signal_exit_fill" => Some(Self::LongSignalExitFill),
            "short_signal_exit_fill" => Some(Self::ShortSignalExitFill),
            "long_reversal_exit_fill" => Some(Self::LongReversalExitFill),
            "short_reversal_exit_fill" => Some(Self::ShortReversalExitFill),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LastExitField {
    Kind,
    Side,
    Price,
    Time,
    BarIndex,
    RealizedPnl,
    RealizedReturn,
    BarsHeld,
}

impl LastExitField {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Kind => "kind",
            Self::Side => "side",
            Self::Price => "price",
            Self::Time => "time",
            Self::BarIndex => "bar_index",
            Self::RealizedPnl => "realized_pnl",
            Self::RealizedReturn => "realized_return",
            Self::BarsHeld => "bars_held",
        }
    }

    pub fn parse(name: &str) -> Option<Self> {
        match name {
            "kind" => Some(Self::Kind),
            "side" => Some(Self::Side),
            "price" => Some(Self::Price),
            "time" => Some(Self::Time),
            "bar_index" => Some(Self::BarIndex),
            "realized_pnl" => Some(Self::RealizedPnl),
            "realized_return" => Some(Self::RealizedReturn),
            "bars_held" => Some(Self::BarsHeld),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LastExitScope {
    Global,
    Long,
    Short,
}

impl LastExitScope {
    pub const fn namespace(self) -> &'static str {
        match self {
            Self::Global => "last_exit",
            Self::Long => "last_long_exit",
            Self::Short => "last_short_exit",
        }
    }

    pub fn from_namespace(namespace: &str) -> Option<Self> {
        match namespace {
            "last_exit" => Some(Self::Global),
            "last_long_exit" => Some(Self::Long),
            "last_short_exit" => Some(Self::Short),
            _ => None,
        }
    }
}
