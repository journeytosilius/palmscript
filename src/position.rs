//! Position-scoped types shared by the compiler, runtime, and backtester.
//!
//! These types model position-aware fields exposed to attached exits and the
//! position side enum reused across the public API and backtest internals.

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
}

impl PositionEventField {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LongEntryFill => "long_entry_fill",
            Self::ShortEntryFill => "short_entry_fill",
            Self::LongExitFill => "long_exit_fill",
            Self::ShortExitFill => "short_exit_fill",
        }
    }

    pub fn parse(name: &str) -> Option<Self> {
        match name {
            "long_entry_fill" => Some(Self::LongEntryFill),
            "short_entry_fill" => Some(Self::ShortEntryFill),
            "long_exit_fill" => Some(Self::LongExitFill),
            "short_exit_fill" => Some(Self::ShortExitFill),
            _ => None,
        }
    }
}
