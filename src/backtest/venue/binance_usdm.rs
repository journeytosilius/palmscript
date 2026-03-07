use crate::bytecode::OrderDecl;
use crate::order::{OrderKind, TimeInForce, TriggerReference};

pub(super) fn validate(order: &OrderDecl) -> Result<(), String> {
    if matches!(
        order.kind,
        OrderKind::StopMarket
            | OrderKind::StopLimit
            | OrderKind::TakeProfitMarket
            | OrderKind::TakeProfitLimit
    ) && !matches!(
        order.trigger_ref,
        Some(TriggerReference::Last | TriggerReference::Mark)
    ) {
        return Err(
            "Binance USD-M trigger orders support trigger_ref.last and trigger_ref.mark"
                .to_string(),
        );
    }
    if order.post_only {
        if !matches!(
            order.kind,
            OrderKind::Limit | OrderKind::StopLimit | OrderKind::TakeProfitLimit
        ) {
            return Err(
                "Binance USD-M post_only is only supported for limit-family orders".to_string(),
            );
        }
        if order.tif != Some(TimeInForce::Gtc) {
            return Err("Binance USD-M post_only requires tif.gtc".to_string());
        }
    }
    Ok(())
}
