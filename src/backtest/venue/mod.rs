mod binance_spot;
mod binance_usdm;
mod hyperliquid_perps;
mod hyperliquid_spot;

use crate::backtest::BacktestError;
use crate::bytecode::OrderDecl;
use crate::interval::SourceTemplate;

#[derive(Clone, Copy, Debug)]
pub(crate) enum VenueOrderProfile {
    BinanceSpot,
    BinanceUsdm,
    HyperliquidSpot,
    HyperliquidPerps,
}

impl VenueOrderProfile {
    pub(crate) const fn from_template(template: SourceTemplate) -> Self {
        match template {
            SourceTemplate::BinanceSpot => Self::BinanceSpot,
            SourceTemplate::BinanceUsdm => Self::BinanceUsdm,
            SourceTemplate::HyperliquidSpot => Self::HyperliquidSpot,
            SourceTemplate::HyperliquidPerps => Self::HyperliquidPerps,
        }
    }

    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::BinanceSpot => "binance.spot",
            Self::BinanceUsdm => "binance.usdm",
            Self::HyperliquidSpot => "hyperliquid.spot",
            Self::HyperliquidPerps => "hyperliquid.perps",
        }
    }
}

pub(crate) fn validate_order_for_template(
    profile: VenueOrderProfile,
    alias: &str,
    order: &OrderDecl,
) -> Result<(), BacktestError> {
    let result = match profile {
        VenueOrderProfile::BinanceSpot => binance_spot::validate(order),
        VenueOrderProfile::BinanceUsdm => binance_usdm::validate(order),
        VenueOrderProfile::HyperliquidSpot => hyperliquid_spot::validate(order),
        VenueOrderProfile::HyperliquidPerps => hyperliquid_perps::validate(order),
    };
    result.map_err(|reason| BacktestError::UnsupportedOrderForVenue {
        alias: alias.to_string(),
        venue: profile.as_str().to_string(),
        role: order.role,
        kind: order.kind,
        reason,
    })
}
