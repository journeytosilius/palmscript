use crate::backtest::bridge::PreparedBacktest;
use crate::backtest::orders::{
    adjusted_price, close_position, close_trade, empty_request_slots, evaluate_active_order,
    fill_action_for_role, missing_field_reason, open_position, position_side_for_entry,
    role_applicable, role_index, unrealized_pnl_for_position, ActiveOrder, OpenTrade,
    PositionState, WorkingState, ROLE_PRIORITY,
};
use crate::backtest::{
    BacktestConfig, BacktestError, BacktestResult, BacktestSummary, EquityPoint, OrderEndReason,
    OrderRecord, OrderStatus, PositionSide,
};
use crate::bytecode::SignalRole;
use crate::output::Outputs;
use crate::runtime::Bar;

pub(crate) struct OrderRecordUpdate {
    pub trigger_time: Option<f64>,
    pub fill_bar_index: Option<usize>,
    pub fill_time: Option<f64>,
    pub raw_price: Option<f64>,
    pub fill_price: Option<f64>,
    pub status: OrderStatus,
    pub end_reason: Option<OrderEndReason>,
}

pub(crate) fn simulate_backtest(
    outputs: Outputs,
    execution_bars: Vec<Bar>,
    config: &BacktestConfig,
    prepared: PreparedBacktest,
) -> Result<BacktestResult, BacktestError> {
    let fee_rate = config.fee_bps / crate::backtest::BPS_SCALE;
    let slippage_rate = config.slippage_bps / crate::backtest::BPS_SCALE;
    let mut cash = config.initial_capital;
    let mut position = None::<PositionState>;
    let mut open_trade = None::<OpenTrade>;
    let mut fills = Vec::new();
    let mut trades = Vec::new();
    let mut orders = Vec::<OrderRecord>::new();
    let mut equity_curve = Vec::with_capacity(execution_bars.len());
    let mut active_orders: [Option<ActiveOrder>; 4] = [None, None, None, None];
    let mut pending_requests = empty_request_slots();
    let mut pending_conflict_time = None::<f64>;
    let mut batch_cursor = 0usize;
    let mut total_realized_pnl = 0.0;
    let mut max_gross_exposure = 0.0_f64;
    let mut peak_equity = config.initial_capital;
    let mut max_drawdown = 0.0_f64;

    for (bar_index, bar) in execution_bars.iter().copied().enumerate() {
        while batch_cursor < prepared.signal_batches.len()
            && prepared.signal_batches[batch_cursor].time < bar.time
        {
            let batch = &prepared.signal_batches[batch_cursor];
            accumulate_pending_requests(
                &mut pending_requests,
                batch.requests,
                &mut pending_conflict_time,
            );
            batch_cursor += 1;
        }

        if position.is_none()
            && pending_requests[role_index(SignalRole::LongEntry)].is_some()
            && pending_requests[role_index(SignalRole::ShortEntry)].is_some()
        {
            return Err(BacktestError::ConflictingSignals {
                time: pending_conflict_time.unwrap_or(bar.time),
            });
        }

        for role in ROLE_PRIORITY {
            let slot = role_index(role);
            let Some(request) = pending_requests[slot].take() else {
                continue;
            };
            if !role_applicable(role, position.as_ref()) {
                continue;
            }
            if let Some(existing) = active_orders[slot].take() {
                update_order_record(
                    &mut orders[existing.record_index],
                    OrderRecordUpdate {
                        trigger_time: None,
                        fill_bar_index: None,
                        fill_time: None,
                        raw_price: None,
                        fill_price: None,
                        status: OrderStatus::Cancelled,
                        end_reason: Some(OrderEndReason::Replaced),
                    },
                );
            }

            let mut record =
                crate::backtest::orders::order_record(request, bar_index, bar.time, orders.len());
            if let Some(reason) = missing_field_reason(request) {
                record.status = OrderStatus::Rejected;
                record.end_reason = Some(reason);
                orders.push(record);
                continue;
            }

            let record_index = orders.len();
            orders.push(record);
            active_orders[slot] = Some(ActiveOrder {
                request,
                record_index,
                first_eval_done: false,
                state: WorkingState::Ready,
            });
        }
        pending_conflict_time = None;

        let mut filled_this_bar = false;
        for role in ROLE_PRIORITY {
            if filled_this_bar {
                break;
            }
            let slot = role_index(role);
            let Some(mut active) = active_orders[slot].take() else {
                continue;
            };

            let evaluation = evaluate_active_order(&active, bar.time, bar.open, bar.high, bar.low);
            active.first_eval_done = true;

            match evaluation {
                crate::backtest::orders::Evaluation::None => {
                    active_orders[slot] = Some(active);
                }
                crate::backtest::orders::Evaluation::Expire => {
                    update_order_record(
                        &mut orders[active.record_index],
                        OrderRecordUpdate {
                            trigger_time: None,
                            fill_bar_index: None,
                            fill_time: None,
                            raw_price: None,
                            fill_price: None,
                            status: OrderStatus::Expired,
                            end_reason: None,
                        },
                    );
                }
                crate::backtest::orders::Evaluation::Cancel(reason) => {
                    update_order_record(
                        &mut orders[active.record_index],
                        OrderRecordUpdate {
                            trigger_time: None,
                            fill_bar_index: None,
                            fill_time: None,
                            raw_price: None,
                            fill_price: None,
                            status: OrderStatus::Cancelled,
                            end_reason: Some(reason),
                        },
                    );
                }
                crate::backtest::orders::Evaluation::MoveToRestingLimit {
                    active_after_time,
                    trigger_time,
                } => {
                    orders[active.record_index].trigger_time = Some(trigger_time);
                    active.state = WorkingState::RestingLimit { active_after_time };
                    active_orders[slot] = Some(active);
                }
                crate::backtest::orders::Evaluation::Fill(execution) => {
                    let action = fill_action_for_role(role);
                    let execution_price =
                        if matches!(active.request.kind, crate::order::OrderKind::Market) {
                            adjusted_price(execution.raw_price, action, slippage_rate)
                        } else {
                            execution.price
                        };

                    maybe_close_position_for_role(
                        role,
                        bar_index,
                        bar.time,
                        execution.raw_price,
                        execution_price,
                        fee_rate,
                        &mut cash,
                        &mut position,
                        &mut open_trade,
                        &mut fills,
                        &mut trades,
                        &mut total_realized_pnl,
                    );

                    if let Some(next_side) = position_side_for_entry(role) {
                        let (next_position, next_trade, entry_fill) = open_position(
                            bar_index,
                            bar.time,
                            execution.raw_price,
                            execution_price,
                            next_side,
                            fee_rate,
                            &mut cash,
                        );
                        fills.push(entry_fill);
                        position = Some(next_position);
                        open_trade = Some(next_trade);
                    }

                    update_order_record(
                        &mut orders[active.record_index],
                        OrderRecordUpdate {
                            trigger_time: execution.trigger_time,
                            fill_bar_index: Some(bar_index),
                            fill_time: Some(bar.time),
                            raw_price: Some(execution.raw_price),
                            fill_price: Some(execution_price),
                            status: OrderStatus::Filled,
                            end_reason: None,
                        },
                    );

                    invalidate_inapplicable_orders(
                        &mut active_orders,
                        position.as_ref(),
                        &mut orders,
                    );
                    filled_this_bar = true;
                }
            }
        }

        let quantity = position.as_ref().map_or(0.0, |state| state.quantity);
        let gross_exposure = quantity.abs() * bar.close;
        max_gross_exposure = max_gross_exposure.max(gross_exposure);
        let equity = cash + quantity * bar.close;
        peak_equity = peak_equity.max(equity);
        max_drawdown = max_drawdown.max(peak_equity - equity);
        equity_curve.push(EquityPoint {
            bar_index,
            time: bar.time,
            cash,
            equity,
            position_side: position.as_ref().map(|state| state.side),
            quantity,
            mark_price: bar.close,
            gross_exposure,
        });
    }

    let ending_equity = equity_curve
        .last()
        .map_or(config.initial_capital, |point| point.equity);
    let unrealized_pnl = ending_equity - config.initial_capital - total_realized_pnl;
    let winning_trade_count = trades
        .iter()
        .filter(|trade| trade.realized_pnl > 0.0)
        .count();
    let losing_trade_count = trades
        .iter()
        .filter(|trade| trade.realized_pnl < 0.0)
        .count();
    let trade_count = trades.len();
    let win_rate = if trade_count == 0 {
        0.0
    } else {
        winning_trade_count as f64 / trade_count as f64
    };

    let open_position = match (position, equity_curve.last()) {
        (Some(position), Some(last_point)) => Some(crate::backtest::PositionSnapshot {
            side: position.side,
            quantity: position.quantity.abs(),
            entry_bar_index: position.entry_bar_index,
            entry_time: position.entry_time,
            entry_price: position.entry_price,
            market_price: last_point.mark_price,
            market_time: last_point.time,
            unrealized_pnl: unrealized_pnl_for_position(&position, last_point.mark_price),
        }),
        _ => None,
    };

    Ok(BacktestResult {
        outputs,
        orders,
        fills,
        trades,
        equity_curve,
        summary: BacktestSummary {
            starting_equity: config.initial_capital,
            ending_equity,
            realized_pnl: total_realized_pnl,
            unrealized_pnl,
            total_return: (ending_equity - config.initial_capital) / config.initial_capital,
            trade_count,
            winning_trade_count,
            losing_trade_count,
            win_rate,
            max_drawdown,
            max_gross_exposure,
        },
        open_position,
    })
}

fn accumulate_pending_requests(
    pending_requests: &mut [Option<crate::backtest::orders::CapturedOrderRequest>; 4],
    requests: [Option<crate::backtest::orders::CapturedOrderRequest>; 4],
    pending_conflict_time: &mut Option<f64>,
) {
    for request in requests.into_iter().flatten() {
        let slot = role_index(request.role);
        pending_requests[slot] = Some(request);
        if matches!(request.role, SignalRole::LongEntry | SignalRole::ShortEntry) {
            *pending_conflict_time = Some(request.signal_time);
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn maybe_close_position_for_role(
    role: SignalRole,
    bar_index: usize,
    time: f64,
    raw_price: f64,
    execution_price: f64,
    fee_rate: f64,
    cash: &mut f64,
    position: &mut Option<PositionState>,
    open_trade: &mut Option<OpenTrade>,
    fills: &mut Vec<crate::backtest::Fill>,
    trades: &mut Vec<crate::backtest::Trade>,
    total_realized_pnl: &mut f64,
) {
    let should_close = matches!(
        (position.as_ref().map(|state| state.side), role),
        (
            Some(PositionSide::Long),
            SignalRole::LongExit | SignalRole::ShortEntry
        ) | (
            Some(PositionSide::Short),
            SignalRole::ShortExit | SignalRole::LongEntry
        )
    );
    if !should_close {
        return;
    }

    let closed_position = position.take().expect("open position should exist");
    let exit_fill = close_position(
        bar_index,
        time,
        raw_price,
        execution_price,
        fee_rate,
        cash,
        &closed_position,
    );
    let trade = close_trade(
        open_trade.take().expect("open trade should exist"),
        exit_fill.clone(),
    );
    *total_realized_pnl += trade.realized_pnl;
    fills.push(exit_fill);
    trades.push(trade);
}

fn invalidate_inapplicable_orders(
    active_orders: &mut [Option<ActiveOrder>; 4],
    position: Option<&PositionState>,
    orders: &mut [OrderRecord],
) {
    for slot in active_orders.iter_mut() {
        let Some(active) = slot.as_ref() else {
            continue;
        };
        if role_applicable(active.request.role, position) {
            continue;
        }
        let record_index = active.record_index;
        *slot = None;
        update_order_record(
            &mut orders[record_index],
            OrderRecordUpdate {
                trigger_time: None,
                fill_bar_index: None,
                fill_time: None,
                raw_price: None,
                fill_price: None,
                status: OrderStatus::Cancelled,
                end_reason: Some(OrderEndReason::RoleInvalidated),
            },
        );
    }
}

fn update_order_record(record: &mut OrderRecord, update: OrderRecordUpdate) {
    if let Some(trigger_time) = update.trigger_time {
        record.trigger_time = Some(trigger_time);
    }
    record.fill_bar_index = update.fill_bar_index;
    record.fill_time = update.fill_time;
    record.raw_price = update.raw_price;
    record.fill_price = update.fill_price;
    record.status = update.status;
    record.end_reason = update.end_reason;
}
