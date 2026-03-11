use crate::bytecode::OrderDecl;
use crate::order::{OrderKind, TimeInForce, TriggerReference};

pub(super) fn validate(order: &OrderDecl) -> Result<(), String> {
    if matches!(
        order.kind,
        OrderKind::StopMarket
            | OrderKind::StopLimit
            | OrderKind::TakeProfitMarket
            | OrderKind::TakeProfitLimit
    ) && order.trigger_ref != Some(TriggerReference::Mark)
    {
        return Err("Hyperliquid perps trigger orders only support trigger_ref.mark".to_string());
    }
    if matches!(order.tif, Some(TimeInForce::Fok | TimeInForce::Gtd)) {
        return Err(
            "Hyperliquid perps only supports tif.gtc and tif.ioc in this backtester".to_string(),
        );
    }
    if order.post_only {
        if !matches!(
            order.kind,
            OrderKind::Limit | OrderKind::StopLimit | OrderKind::TakeProfitLimit
        ) {
            return Err(
                "Hyperliquid perps post_only is only supported for limit-family orders".to_string(),
            );
        }
        if order.tif != Some(TimeInForce::Gtc) {
            return Err("Hyperliquid perps post_only requires tif.gtc".to_string());
        }
    }
    Ok(())
}
