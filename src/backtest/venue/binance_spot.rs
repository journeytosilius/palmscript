use crate::bytecode::OrderDecl;
use crate::order::{OrderKind, TimeInForce, TriggerReference};

pub(super) fn validate(order: &OrderDecl) -> Result<(), String> {
    if matches!(
        order.kind,
        OrderKind::StopMarket
            | OrderKind::StopLimit
            | OrderKind::TakeProfitMarket
            | OrderKind::TakeProfitLimit
    ) && order.trigger_ref != Some(TriggerReference::Last)
    {
        return Err("Binance spot trigger orders only support trigger_ref.last".to_string());
    }
    if matches!(order.tif, Some(TimeInForce::Gtd)) {
        return Err("Binance spot does not support tif.gtd in this backtester".to_string());
    }
    if order.post_only {
        if !matches!(order.kind, OrderKind::Limit) {
            return Err("Binance spot post_only is only supported for limit orders".to_string());
        }
        if order.tif != Some(TimeInForce::Gtc) {
            return Err("Binance spot post_only requires tif.gtc".to_string());
        }
    }
    Ok(())
}
