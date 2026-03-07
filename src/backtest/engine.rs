use crate::backtest::bridge::{capture_request, PreparedBacktest};
use crate::backtest::orders::{
    adjusted_price, close_position, close_trade, empty_request_slots, evaluate_active_order,
    fill_action_for_role, is_attached_exit_role, missing_field_reason, open_position,
    position_side_for_entry, role_applicable, role_index, unrealized_pnl_for_position,
    update_open_trade_excursions, ActiveOrder, CapturedOrderRequest, FillExecutionContext,
    OpenTrade, PositionState, TradeEntryContext, WorkingState, ROLE_COUNT, ROLE_PRIORITY,
};
use crate::backtest::{
    BacktestConfig, BacktestDiagnosticSummary, BacktestDiagnostics, BacktestError, BacktestResult,
    BacktestSummary, EquityPoint, FeatureSnapshot, FeatureValue, Fill, OrderDiagnostic,
    OrderEndReason, OrderKindDiagnosticSummary, OrderRecord, OrderStatus, PositionSnapshot,
    SideDiagnosticSummary, Trade, TradeDiagnostic, TradeExitClassification,
};
use crate::bytecode::{PositionFieldDecl, SignalRole};
use crate::order::OrderKind;
use crate::output::StepOutput;
use crate::position::{PositionField, PositionSide};
use crate::runtime::{Bar, RuntimeStep, RuntimeStepper};
use crate::types::Value;

pub(crate) struct OrderRecordUpdate {
    pub trigger_time: Option<f64>,
    pub fill_bar_index: Option<usize>,
    pub fill_time: Option<f64>,
    pub raw_price: Option<f64>,
    pub fill_price: Option<f64>,
    pub status: OrderStatus,
    pub end_reason: Option<OrderEndReason>,
}

#[derive(Clone, Debug, Default)]
struct OrderDiagnosticContext {
    signal_snapshot: Option<FeatureSnapshot>,
    placed_snapshot: Option<FeatureSnapshot>,
    fill_snapshot: Option<FeatureSnapshot>,
    placed_position: Option<PositionSnapshot>,
    fill_position: Option<PositionSnapshot>,
}

pub(crate) fn simulate_backtest(
    mut stepper: RuntimeStepper,
    execution_bars: Vec<Bar>,
    config: &BacktestConfig,
    prepared: PreparedBacktest,
) -> Result<BacktestResult, BacktestError> {
    let fee_rate = config.fee_bps / crate::backtest::BPS_SCALE;
    let slippage_rate = config.slippage_bps / crate::backtest::BPS_SCALE;
    let mut cash = config.initial_capital;
    let mut position = None::<PositionState>;
    let mut open_trade = None::<OpenTrade>;
    let mut fills = Vec::<Fill>::new();
    let mut trades = Vec::<Trade>::new();
    let mut trade_diagnostics = Vec::<TradeDiagnostic>::new();
    let mut orders = Vec::<OrderRecord>::new();
    let mut order_contexts = Vec::<OrderDiagnosticContext>::new();
    let mut equity_curve = Vec::with_capacity(execution_bars.len());
    let mut active_orders: [Option<ActiveOrder>; ROLE_COUNT] =
        [None, None, None, None, None, None, None, None];
    let mut pending_requests = empty_request_slots();
    let mut pending_snapshots: [Option<FeatureSnapshot>; ROLE_COUNT] =
        [None, None, None, None, None, None, None, None];
    let mut pending_conflict_time = None::<f64>;
    let mut total_realized_pnl = 0.0;
    let mut max_gross_exposure = 0.0_f64;
    let mut peak_equity = config.initial_capital;
    let mut max_drawdown = 0.0_f64;
    let mut execution_cursor = 0usize;
    let mut last_mark_price = None::<f64>;

    while let Some(open_time) = stepper.peek_open_time() {
        let next_execution = execution_bars.get(execution_cursor).copied();
        let current_execution =
            next_execution.filter(|bar| bar.time.is_finite() && bar.time == open_time as f64);
        let overrides = build_position_overrides(
            &prepared.position_fields,
            position.as_ref(),
            open_trade.as_ref(),
            current_execution.map(|bar| bar.close).or(last_mark_price),
            open_time as f64,
            current_execution.map(|_| execution_cursor),
        );
        let RuntimeStep { output, .. } = stepper
            .step_with_overrides(&overrides)
            .map_err(BacktestError::Runtime)?
            .expect("peeked runtime step should exist");
        let step_time = open_time as f64;
        let snapshot = snapshot_from_step(&output, step_time);

        let position_before_step = position.clone();
        if let Some(bar) = current_execution {
            if let Some(open_trade) = open_trade.as_mut() {
                update_open_trade_excursions(open_trade, bar.high, bar.low);
            }

            if position.is_none()
                && pending_requests[role_index(SignalRole::LongEntry)].is_some()
                && pending_requests[role_index(SignalRole::ShortEntry)].is_some()
            {
                return Err(BacktestError::ConflictingSignals {
                    time: pending_conflict_time.unwrap_or(bar.time),
                });
            }

            place_pending_requests(
                &mut pending_requests,
                &mut pending_snapshots,
                &mut active_orders,
                &mut orders,
                &mut order_contexts,
                position.as_ref(),
                snapshot.clone(),
                current_position_snapshot(position.as_ref(), bar.close, bar.time),
                execution_cursor,
                bar.time,
            );
            pending_conflict_time = None;

            let fill_snapshot = snapshot.clone();
            let fill_position = current_position_snapshot(position.as_ref(), bar.close, bar.time);
            let mut filled_this_bar = false;
            for role in ROLE_PRIORITY {
                if filled_this_bar {
                    break;
                }
                let slot = role_index(role);
                let Some(mut active) = active_orders[slot].take() else {
                    continue;
                };

                let evaluation =
                    evaluate_active_order(&active, bar.time, bar.open, bar.high, bar.low);
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
                        let execution_price = if matches!(active.request.kind, OrderKind::Market) {
                            adjusted_price(execution.raw_price, action, slippage_rate)
                        } else {
                            execution.price
                        };
                        order_contexts[active.record_index].fill_snapshot = fill_snapshot.clone();
                        order_contexts[active.record_index].fill_position = fill_position.clone();

                        let closed_side = position.as_ref().and_then(|state| {
                            if closes_existing_position(role, state.side) {
                                Some(state.side)
                            } else {
                                None
                            }
                        });

                        maybe_close_position_for_role(
                            role,
                            active.record_index,
                            active.request.kind,
                            fill_snapshot.clone(),
                            execution_cursor,
                            bar.time,
                            execution.raw_price,
                            execution_price,
                            fee_rate,
                            &mut cash,
                            &mut position,
                            &mut open_trade,
                            &mut fills,
                            &mut trades,
                            &mut trade_diagnostics,
                            &mut total_realized_pnl,
                        );

                        if let Some(side) = closed_side {
                            cancel_orders_for_closed_side(
                                &mut active_orders,
                                side,
                                role,
                                &mut orders,
                            );
                        }

                        if let Some(next_side) = position_side_for_entry(role) {
                            let (next_position, mut next_trade, entry_fill) = open_position(
                                FillExecutionContext {
                                    bar_index: execution_cursor,
                                    time: bar.time,
                                    raw_price: execution.raw_price,
                                    execution_price,
                                },
                                next_side,
                                TradeEntryContext {
                                    order_id: active.record_index,
                                    role,
                                    kind: active.request.kind,
                                    snapshot: fill_snapshot.clone(),
                                },
                                fee_rate,
                                &mut cash,
                            );
                            update_open_trade_excursions(&mut next_trade, bar.high, bar.low);
                            fills.push(entry_fill);
                            position = Some(next_position);
                            open_trade = Some(next_trade);
                        }

                        update_order_record(
                            &mut orders[active.record_index],
                            OrderRecordUpdate {
                                trigger_time: execution.trigger_time,
                                fill_bar_index: Some(execution_cursor),
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
                bar_index: execution_cursor,
                time: bar.time,
                cash,
                equity,
                position_side: position.as_ref().map(|state| state.side),
                quantity,
                mark_price: bar.close,
                gross_exposure,
            });
            last_mark_price = Some(bar.close);
            execution_cursor += 1;
        }

        enqueue_signal_requests(
            &output,
            step_time,
            &prepared,
            &mut pending_requests,
            &mut pending_snapshots,
            &mut pending_conflict_time,
            snapshot.clone(),
        );
        enqueue_attached_requests(
            step_time,
            &output,
            &prepared,
            position_before_step.as_ref(),
            position.as_ref(),
            &mut pending_requests,
            &mut pending_snapshots,
            snapshot,
        );
    }

    let outputs = stepper.finish();
    let order_diagnostics = build_order_diagnostics(&orders, &order_contexts);
    let diagnostics_summary = build_diagnostics_summary(&order_diagnostics, &trade_diagnostics);
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
        (Some(position), Some(last_point)) => Some(PositionSnapshot {
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
        diagnostics: BacktestDiagnostics {
            order_diagnostics,
            trade_diagnostics,
            summary: diagnostics_summary,
        },
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

fn build_position_overrides(
    fields: &[PositionFieldDecl],
    position: Option<&PositionState>,
    open_trade: Option<&OpenTrade>,
    market_price: Option<f64>,
    _market_time: f64,
    current_bar_index: Option<usize>,
) -> Vec<(u16, Value)> {
    fields
        .iter()
        .map(|decl| {
            let value = match (decl.field, position, open_trade) {
                (PositionField::IsLong, Some(state), _) => {
                    Value::Bool(state.side == PositionSide::Long)
                }
                (PositionField::IsLong, None, _) => Value::Bool(false),
                (PositionField::IsShort, Some(state), _) => {
                    Value::Bool(state.side == PositionSide::Short)
                }
                (PositionField::IsShort, None, _) => Value::Bool(false),
                (PositionField::Side, Some(state), _) => Value::PositionSide(state.side),
                (PositionField::Side, None, _) => Value::NA,
                (PositionField::EntryPrice, Some(state), _) => Value::F64(state.entry_price),
                (PositionField::EntryTime, Some(state), _) => Value::F64(state.entry_time),
                (PositionField::EntryBarIndex, Some(state), _) => {
                    Value::F64(state.entry_bar_index as f64)
                }
                (PositionField::BarsHeld, Some(state), _) => Value::F64(
                    current_bar_index
                        .map(|index| index.saturating_sub(state.entry_bar_index) as f64)
                        .unwrap_or(0.0),
                ),
                (PositionField::MarketPrice, Some(_), _) => {
                    market_price.map(Value::F64).unwrap_or(Value::NA)
                }
                (PositionField::UnrealizedPnl, Some(state), _) => market_price
                    .map(|price| Value::F64(unrealized_pnl_for_position(state, price)))
                    .unwrap_or(Value::NA),
                (PositionField::UnrealizedReturn, Some(state), _) => market_price
                    .map(|price| {
                        let pnl = unrealized_pnl_for_position(state, price);
                        let notional = state.entry_price * state.quantity.abs();
                        if notional.abs() < crate::backtest::EPSILON {
                            Value::F64(0.0)
                        } else {
                            Value::F64(pnl / notional)
                        }
                    })
                    .unwrap_or(Value::NA),
                (PositionField::Mae, Some(_), Some(trade)) => Value::F64(trade.mae_price_delta),
                (PositionField::Mfe, Some(_), Some(trade)) => Value::F64(trade.mfe_price_delta),
                _ => Value::NA,
            };
            (decl.slot, value)
        })
        .collect()
}

fn enqueue_signal_requests(
    output: &StepOutput,
    signal_time: f64,
    prepared: &PreparedBacktest,
    pending_requests: &mut [Option<CapturedOrderRequest>; ROLE_COUNT],
    pending_snapshots: &mut [Option<FeatureSnapshot>; ROLE_COUNT],
    pending_conflict_time: &mut Option<f64>,
    snapshot: Option<FeatureSnapshot>,
) {
    for event in &output.trigger_events {
        let Some(role) = prepared.signal_roles.get(&event.output_id).copied() else {
            continue;
        };
        let Some(template) = prepared.order_templates.get(&role).copied() else {
            continue;
        };
        let slot = role_index(role);
        pending_requests[slot] = Some(capture_request(template, signal_time, output));
        pending_snapshots[slot] = snapshot.clone();
        if matches!(role, SignalRole::LongEntry | SignalRole::ShortEntry) {
            *pending_conflict_time = Some(signal_time);
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn enqueue_attached_requests(
    signal_time: f64,
    output: &StepOutput,
    prepared: &PreparedBacktest,
    position_before_step: Option<&PositionState>,
    position_after_step: Option<&PositionState>,
    pending_requests: &mut [Option<CapturedOrderRequest>; ROLE_COUNT],
    pending_snapshots: &mut [Option<FeatureSnapshot>; ROLE_COUNT],
    snapshot: Option<FeatureSnapshot>,
) {
    let (Some(before), Some(after)) = (position_before_step, position_after_step) else {
        return;
    };
    if before.side != after.side {
        return;
    }

    let roles = match before.side {
        PositionSide::Long => [SignalRole::ProtectLong, SignalRole::TargetLong],
        PositionSide::Short => [SignalRole::ProtectShort, SignalRole::TargetShort],
    };
    for role in roles {
        let Some(template) = prepared.order_templates.get(&role).copied() else {
            continue;
        };
        let slot = role_index(role);
        pending_requests[slot] = Some(capture_request(template, signal_time, output));
        pending_snapshots[slot] = snapshot.clone();
    }
}

#[allow(clippy::too_many_arguments)]
fn place_pending_requests(
    pending_requests: &mut [Option<CapturedOrderRequest>; ROLE_COUNT],
    pending_snapshots: &mut [Option<FeatureSnapshot>; ROLE_COUNT],
    active_orders: &mut [Option<ActiveOrder>; ROLE_COUNT],
    orders: &mut Vec<OrderRecord>,
    order_contexts: &mut Vec<OrderDiagnosticContext>,
    position: Option<&PositionState>,
    placed_snapshot: Option<FeatureSnapshot>,
    placed_position: Option<PositionSnapshot>,
    bar_index: usize,
    time: f64,
) {
    for role in ROLE_PRIORITY {
        let slot = role_index(role);
        let Some(request) = pending_requests[slot].take() else {
            continue;
        };
        let signal_snapshot = pending_snapshots[slot].take();
        if !role_applicable(role, position) {
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
                    end_reason: Some(if is_attached_exit_role(role) {
                        OrderEndReason::Rearmed
                    } else {
                        OrderEndReason::Replaced
                    }),
                },
            );
        }

        let mut record =
            crate::backtest::orders::order_record(request, bar_index, time, orders.len());
        let record_index = orders.len();
        order_contexts.push(OrderDiagnosticContext {
            signal_snapshot,
            placed_snapshot: placed_snapshot.clone(),
            fill_snapshot: None,
            placed_position: placed_position.clone(),
            fill_position: None,
        });

        if let Some(reason) = missing_field_reason(request) {
            record.status = OrderStatus::Rejected;
            record.end_reason = Some(reason);
            orders.push(record);
            continue;
        }

        orders.push(record);
        active_orders[slot] = Some(ActiveOrder {
            request,
            record_index,
            first_eval_done: false,
            state: WorkingState::Ready,
        });
    }
}

fn closes_existing_position(role: SignalRole, side: PositionSide) -> bool {
    matches!(
        (side, role),
        (
            PositionSide::Long,
            SignalRole::LongExit
                | SignalRole::ProtectLong
                | SignalRole::TargetLong
                | SignalRole::ShortEntry
        ) | (
            PositionSide::Short,
            SignalRole::ShortExit
                | SignalRole::ProtectShort
                | SignalRole::TargetShort
                | SignalRole::LongEntry
        )
    )
}

fn cancel_orders_for_closed_side(
    active_orders: &mut [Option<ActiveOrder>; ROLE_COUNT],
    side: PositionSide,
    filled_role: SignalRole,
    orders: &mut [OrderRecord],
) {
    let (signal_role, protect_role, target_role) = match side {
        PositionSide::Long => (
            SignalRole::LongExit,
            SignalRole::ProtectLong,
            SignalRole::TargetLong,
        ),
        PositionSide::Short => (
            SignalRole::ShortExit,
            SignalRole::ProtectShort,
            SignalRole::TargetShort,
        ),
    };

    cancel_active_role(
        active_orders,
        signal_role,
        orders,
        OrderEndReason::PositionClosed,
    );
    match filled_role {
        role if role == protect_role => {
            cancel_active_role(
                active_orders,
                target_role,
                orders,
                OrderEndReason::OcoCancelled,
            );
        }
        role if role == target_role => {
            cancel_active_role(
                active_orders,
                protect_role,
                orders,
                OrderEndReason::OcoCancelled,
            );
        }
        _ => {
            cancel_active_role(
                active_orders,
                protect_role,
                orders,
                OrderEndReason::PositionClosed,
            );
            cancel_active_role(
                active_orders,
                target_role,
                orders,
                OrderEndReason::PositionClosed,
            );
        }
    }
}

fn cancel_active_role(
    active_orders: &mut [Option<ActiveOrder>; ROLE_COUNT],
    role: SignalRole,
    orders: &mut [OrderRecord],
    reason: OrderEndReason,
) {
    let slot = role_index(role);
    let Some(active) = active_orders[slot].take() else {
        return;
    };
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

#[allow(clippy::too_many_arguments)]
fn maybe_close_position_for_role(
    role: SignalRole,
    order_id: usize,
    order_kind: OrderKind,
    exit_snapshot: Option<FeatureSnapshot>,
    bar_index: usize,
    time: f64,
    raw_price: f64,
    execution_price: f64,
    fee_rate: f64,
    cash: &mut f64,
    position: &mut Option<PositionState>,
    open_trade: &mut Option<OpenTrade>,
    fills: &mut Vec<Fill>,
    trades: &mut Vec<Trade>,
    trade_diagnostics: &mut Vec<TradeDiagnostic>,
    total_realized_pnl: &mut f64,
) {
    let should_close = matches!(
        (position.as_ref().map(|state| state.side), role),
        (
            Some(PositionSide::Long),
            SignalRole::LongExit
                | SignalRole::ProtectLong
                | SignalRole::TargetLong
                | SignalRole::ShortEntry
        ) | (
            Some(PositionSide::Short),
            SignalRole::ShortExit
                | SignalRole::ProtectShort
                | SignalRole::TargetShort
                | SignalRole::LongEntry
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
    let open_trade = open_trade.take().expect("open trade should exist");
    let trade = close_trade(open_trade.clone(), exit_fill.clone());
    *total_realized_pnl += trade.realized_pnl;
    fills.push(exit_fill.clone());
    trade_diagnostics.push(TradeDiagnostic {
        trade_id: trades.len(),
        side: open_trade.side,
        entry_order_id: open_trade.entry_order_id,
        exit_order_id: order_id,
        entry_role: open_trade.entry_role,
        exit_role: role,
        entry_kind: open_trade.entry_kind,
        exit_kind: order_kind,
        exit_classification: classify_exit(role),
        entry_snapshot: open_trade.entry_snapshot,
        exit_snapshot,
        bars_held: exit_fill
            .bar_index
            .saturating_sub(open_trade.entry.bar_index),
        duration_ms: exit_fill.time - open_trade.entry.time,
        realized_pnl: trade.realized_pnl,
        mae_price_delta: open_trade.mae_price_delta,
        mfe_price_delta: open_trade.mfe_price_delta,
        mae_pct: pct_delta(open_trade.mae_price_delta, open_trade.entry.price),
        mfe_pct: pct_delta(open_trade.mfe_price_delta, open_trade.entry.price),
    });
    trades.push(trade);
}

fn invalidate_inapplicable_orders(
    active_orders: &mut [Option<ActiveOrder>; ROLE_COUNT],
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
        let role = active.request.role;
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
                end_reason: Some(if is_attached_exit_role(role) {
                    OrderEndReason::PositionClosed
                } else {
                    OrderEndReason::RoleInvalidated
                }),
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

fn build_order_diagnostics(
    orders: &[OrderRecord],
    contexts: &[OrderDiagnosticContext],
) -> Vec<OrderDiagnostic> {
    orders
        .iter()
        .zip(contexts)
        .map(|(order, context)| OrderDiagnostic {
            order_id: order.id,
            role: order.role,
            kind: order.kind,
            status: order.status,
            end_reason: order.end_reason,
            signal_snapshot: context.signal_snapshot.clone(),
            placed_snapshot: context.placed_snapshot.clone(),
            fill_snapshot: context.fill_snapshot.clone(),
            placed_position: context.placed_position.clone(),
            fill_position: context.fill_position.clone(),
            bars_to_fill: order
                .fill_bar_index
                .map(|fill_bar_index| fill_bar_index.saturating_sub(order.placed_bar_index)),
            time_to_fill_ms: order
                .fill_time
                .map(|fill_time| fill_time - order.placed_time),
        })
        .collect()
}

fn build_diagnostics_summary(
    order_diagnostics: &[OrderDiagnostic],
    trade_diagnostics: &[TradeDiagnostic],
) -> BacktestDiagnosticSummary {
    let order_fill_rate = ratio(
        order_diagnostics
            .iter()
            .filter(|diagnostic| matches!(diagnostic.status, OrderStatus::Filled))
            .count(),
        order_diagnostics.len(),
    );
    let average_bars_to_fill = average(
        order_diagnostics
            .iter()
            .filter_map(|diagnostic| diagnostic.bars_to_fill.map(|bars| bars as f64)),
    );
    let average_bars_held = average(
        trade_diagnostics
            .iter()
            .map(|diagnostic| diagnostic.bars_held as f64),
    );
    let average_mae_pct = average(
        trade_diagnostics
            .iter()
            .map(|diagnostic| diagnostic.mae_pct),
    );
    let average_mfe_pct = average(
        trade_diagnostics
            .iter()
            .map(|diagnostic| diagnostic.mfe_pct),
    );

    let mut by_order_kind = Vec::new();
    for kind in [
        OrderKind::Market,
        OrderKind::Limit,
        OrderKind::StopMarket,
        OrderKind::StopLimit,
        OrderKind::TakeProfitMarket,
        OrderKind::TakeProfitLimit,
    ] {
        let matching = order_diagnostics
            .iter()
            .filter(|diagnostic| diagnostic.kind == kind)
            .collect::<Vec<_>>();
        if matching.is_empty() {
            continue;
        }
        let placed_count = matching.len();
        let filled_count = matching
            .iter()
            .filter(|diagnostic| matches!(diagnostic.status, OrderStatus::Filled))
            .count();
        let cancelled_count = matching
            .iter()
            .filter(|diagnostic| matches!(diagnostic.status, OrderStatus::Cancelled))
            .count();
        let rejected_count = matching
            .iter()
            .filter(|diagnostic| matches!(diagnostic.status, OrderStatus::Rejected))
            .count();
        let expired_count = matching
            .iter()
            .filter(|diagnostic| matches!(diagnostic.status, OrderStatus::Expired))
            .count();
        by_order_kind.push(OrderKindDiagnosticSummary {
            kind,
            placed_count,
            filled_count,
            cancelled_count,
            rejected_count,
            expired_count,
            fill_rate: ratio(filled_count, placed_count),
            average_bars_to_fill: average(
                matching
                    .iter()
                    .filter_map(|diagnostic| diagnostic.bars_to_fill.map(|bars| bars as f64)),
            ),
        });
    }

    let mut by_side = Vec::new();
    for side in [PositionSide::Long, PositionSide::Short] {
        let matching = trade_diagnostics
            .iter()
            .filter(|diagnostic| diagnostic.side == side)
            .collect::<Vec<_>>();
        if matching.is_empty() {
            continue;
        }
        by_side.push(SideDiagnosticSummary {
            side,
            trade_count: matching.len(),
            win_rate: ratio(
                matching
                    .iter()
                    .filter(|diagnostic| diagnostic.realized_pnl > 0.0)
                    .count(),
                matching.len(),
            ),
            average_realized_pnl: average(
                matching.iter().map(|diagnostic| diagnostic.realized_pnl),
            ),
            average_bars_held: average(
                matching
                    .iter()
                    .map(|diagnostic| diagnostic.bars_held as f64),
            ),
            average_mae_pct: average(matching.iter().map(|diagnostic| diagnostic.mae_pct)),
            average_mfe_pct: average(matching.iter().map(|diagnostic| diagnostic.mfe_pct)),
        });
    }

    BacktestDiagnosticSummary {
        order_fill_rate,
        average_bars_to_fill,
        average_bars_held,
        average_mae_pct,
        average_mfe_pct,
        signal_exit_count: trade_diagnostics
            .iter()
            .filter(|diagnostic| {
                matches!(
                    diagnostic.exit_classification,
                    TradeExitClassification::Signal
                )
            })
            .count(),
        protect_exit_count: trade_diagnostics
            .iter()
            .filter(|diagnostic| {
                matches!(
                    diagnostic.exit_classification,
                    TradeExitClassification::Protect
                )
            })
            .count(),
        target_exit_count: trade_diagnostics
            .iter()
            .filter(|diagnostic| {
                matches!(
                    diagnostic.exit_classification,
                    TradeExitClassification::Target
                )
            })
            .count(),
        reversal_exit_count: trade_diagnostics
            .iter()
            .filter(|diagnostic| {
                matches!(
                    diagnostic.exit_classification,
                    TradeExitClassification::Reversal
                )
            })
            .count(),
        by_order_kind,
        by_side,
    }
}

fn classify_exit(role: SignalRole) -> TradeExitClassification {
    match role {
        SignalRole::LongEntry | SignalRole::ShortEntry => TradeExitClassification::Reversal,
        SignalRole::ProtectLong | SignalRole::ProtectShort => TradeExitClassification::Protect,
        SignalRole::TargetLong | SignalRole::TargetShort => TradeExitClassification::Target,
        SignalRole::LongExit | SignalRole::ShortExit => TradeExitClassification::Signal,
    }
}

fn snapshot_from_step(step: &StepOutput, time: f64) -> Option<FeatureSnapshot> {
    let bar_index = step
        .exports
        .first()
        .map(|sample| sample.bar_index)
        .unwrap_or_default();
    let values = step
        .exports
        .iter()
        .map(|sample| FeatureValue {
            name: sample.name.clone(),
            value: sample.value.clone(),
        })
        .collect::<Vec<_>>();
    if values.is_empty() {
        None
    } else {
        Some(FeatureSnapshot {
            bar_index,
            time,
            values,
        })
    }
}

fn current_position_snapshot(
    position: Option<&PositionState>,
    mark_price: f64,
    market_time: f64,
) -> Option<PositionSnapshot> {
    position.map(|state| PositionSnapshot {
        side: state.side,
        quantity: state.quantity.abs(),
        entry_bar_index: state.entry_bar_index,
        entry_time: state.entry_time,
        entry_price: state.entry_price,
        market_price: mark_price,
        market_time,
        unrealized_pnl: unrealized_pnl_for_position(state, mark_price),
    })
}

fn pct_delta(delta: f64, price: f64) -> f64 {
    if price.abs() < crate::backtest::EPSILON {
        0.0
    } else {
        delta / price
    }
}

fn average(values: impl IntoIterator<Item = f64>) -> f64 {
    let mut count = 0usize;
    let mut sum = 0.0;
    for value in values {
        sum += value;
        count += 1;
    }
    if count == 0 {
        0.0
    } else {
        sum / count as f64
    }
}

fn ratio(numerator: usize, denominator: usize) -> f64 {
    if denominator == 0 {
        0.0
    } else {
        numerator as f64 / denominator as f64
    }
}
