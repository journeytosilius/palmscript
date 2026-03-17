import { useEffect, useState } from "react";

import {
  fetchPaperOverview,
  fetchPaperSessionDetail,
  fetchPaperSessionLogs,
} from "./api";
import {
  formatDateLabel,
  formatNumber,
  formatPercent,
  formatTimeLabel,
} from "./formatters";
import type {
  ArbitragePairDiagnosticSummary,
  BacktestDiagnostics,
  BacktestResult,
  BacktestSummary,
  ExecutionDaemonStatus,
  PaperDashboardOverview,
  PaperDashboardSession,
  PaperFeedSnapshot,
  PaperSessionDetailResponse,
  PaperSessionExport,
  PaperSessionLogEvent,
  PaperSessionLogsResponse,
  PaperSessionSnapshot,
  PositionSnapshot,
  TransferRouteDiagnosticSummary,
} from "./types";
import { LineChart, MetricCard } from "./ui";

const POLL_MS = 3_000;
const STRATEGY_STORAGE_KEY = "palmscript.paper.strategy";
const SESSION_STORAGE_KEY = "palmscript.paper.session";

interface StrategyGroup {
  key: string;
  label: string;
  sessions: PaperDashboardSession[];
  health: string;
  liveCount: number;
  failedCount: number;
  updatedAtMs: number;
}

export function PaperDashboard() {
  const [overview, setOverview] = useState<PaperDashboardOverview | null>(null);
  const [selectedStrategyKey, setSelectedStrategyKey] = useState<string | null>(() =>
    window.localStorage.getItem(STRATEGY_STORAGE_KEY),
  );
  const [selectedSessionId, setSelectedSessionId] = useState<string | null>(null);
  const [detail, setDetail] = useState<PaperSessionDetailResponse | null>(null);
  const [logs, setLogs] = useState<PaperSessionLogsResponse | null>(null);
  const [status, setStatus] = useState("Loading paper sessions...");
  const [overviewLoading, setOverviewLoading] = useState(true);
  const [detailLoading, setDetailLoading] = useState(false);

  const strategyGroups = groupSessionsByStrategy(overview?.sessions ?? []);
  const selectedStrategy =
    strategyGroups.find((strategy) => strategy.key === selectedStrategyKey) ?? null;
  const strategySessions = selectedStrategy?.sessions ?? [];

  useEffect(() => {
    document.title = "PalmScript Paper Dashboard";
  }, []);

  useEffect(() => {
    let cancelled = false;

    async function loadOverview() {
      try {
        const nextOverview = await fetchPaperOverview();
        if (cancelled) {
          return;
        }
        setOverview(nextOverview);
        setOverviewLoading(false);
        setStatus(
          buildOverviewStatus(
            nextOverview.daemon,
            groupSessionsByStrategy(nextOverview.sessions).length,
            nextOverview.sessions.length,
          ),
        );
      } catch (error) {
        if (!cancelled) {
          setOverviewLoading(false);
          setStatus(error instanceof Error ? error.message : "Failed to load paper sessions.");
        }
      }
    }

    void loadOverview();
    const handle = window.setInterval(() => {
      void loadOverview();
    }, POLL_MS);
    return () => {
      cancelled = true;
      window.clearInterval(handle);
    };
  }, []);

  useEffect(() => {
    if (!strategyGroups.length) {
      setSelectedStrategyKey(null);
      setSelectedSessionId(null);
      return;
    }
    setSelectedStrategyKey((current) => {
      if (current && strategyGroups.some((strategy) => strategy.key === current)) {
        return current;
      }
      return strategyGroups[0].key;
    });
  }, [strategyGroups]);

  useEffect(() => {
    if (!selectedStrategyKey) {
      setSelectedSessionId(null);
      return;
    }
    const nextSessions =
      overview?.sessions.filter((session) => strategyKey(session) === selectedStrategyKey) ?? [];
    setSelectedSessionId((current) => {
      if (
        current &&
        nextSessions.some((session) => session.manifest.session_id === current)
      ) {
        return current;
      }
      const storedSession = window.localStorage.getItem(SESSION_STORAGE_KEY);
      if (
        storedSession &&
        nextSessions.some((session) => session.manifest.session_id === storedSession)
      ) {
        return storedSession;
      }
      return nextSessions[0]?.manifest.session_id ?? null;
    });
  }, [overview, selectedStrategyKey]);

  useEffect(() => {
    if (selectedStrategyKey) {
      window.localStorage.setItem(STRATEGY_STORAGE_KEY, selectedStrategyKey);
    } else {
      window.localStorage.removeItem(STRATEGY_STORAGE_KEY);
    }
  }, [selectedStrategyKey]);

  useEffect(() => {
    if (selectedSessionId) {
      window.localStorage.setItem(SESSION_STORAGE_KEY, selectedSessionId);
    } else {
      window.localStorage.removeItem(SESSION_STORAGE_KEY);
    }
  }, [selectedSessionId]);

  useEffect(() => {
    if (!selectedSessionId) {
      setDetail(null);
      setLogs(null);
      return;
    }
    const sessionId = selectedSessionId;

    let cancelled = false;
    setDetail(null);
    setLogs(null);

    async function loadDetail() {
      setDetailLoading(true);
      try {
        const [nextDetail, nextLogs] = await Promise.all([
          fetchPaperSessionDetail(sessionId),
          fetchPaperSessionLogs(sessionId),
        ]);
        if (cancelled) {
          return;
        }
        setDetail(nextDetail);
        setLogs(nextLogs);
      } catch (error) {
        if (!cancelled) {
          setStatus(error instanceof Error ? error.message : "Failed to load session detail.");
        }
      } finally {
        if (!cancelled) {
          setDetailLoading(false);
        }
      }
    }

    void loadDetail();
    const handle = window.setInterval(() => {
      void loadDetail();
    }, POLL_MS);
    return () => {
      cancelled = true;
      window.clearInterval(handle);
    };
  }, [selectedSessionId]);

  const selectedSession =
    strategySessions.find((session) => session.manifest.session_id === selectedSessionId) ?? null;
  const activeDetail =
    detail?.export.manifest.session_id === selectedSessionId ? detail : null;
  const activeLogs = logs?.session_id === selectedSessionId ? logs : null;
  const exportData = activeDetail?.export ?? null;
  const manifest = exportData?.manifest ?? selectedSession?.manifest ?? null;
  const snapshot = exportData?.snapshot ?? selectedSession?.snapshot ?? null;
  const result = exportData?.latest_result ?? null;
  const summary = snapshot?.summary ?? result?.summary ?? null;
  const diagnostics = result?.diagnostics ?? null;
  const displayedStatus = snapshot?.status ?? manifest?.status ?? null;
  const displayedHealth = snapshot?.health ?? manifest?.health ?? null;
  const displayedFailureMessage =
    snapshot?.failure_message ?? manifest?.failure_message ?? null;
  const displayedFeedSummary = snapshot?.feed_summary ?? manifest?.feed_summary ?? null;
  const displayedFeedSnapshots = snapshot?.feed_snapshots ?? manifest?.required_feeds ?? [];
  const displayedOpenPositions = snapshot?.open_positions ?? [];
  const displayedTradeCount = snapshot?.trade_count ?? 0;
  const displayedOpenOrderCount = snapshot?.open_order_count ?? 0;

  return (
    <div className="app-shell app-shell--paper">
      <header className="app-header app-header--paper">
        <div className="app-header__brand app-header__brand--paper">
          <img
            className="app-header__logo"
            src="./ide/palmscript-logo.png"
            alt="PalmScript"
          />
          <div className="paper-heading">
            <span className="paper-heading__eyebrow">Paper Trading Monitor</span>
            <h1 className="paper-heading__title">Live strategy dashboard</h1>
          </div>
        </div>
        <div className="app__status">{status}</div>
      </header>

      <main className="paper-dashboard">
        <section className="panel paper-overview-panel">
          <div className="panel__titlebar">
            <h2 className="panel__title">Overview</h2>
            <span className="panel__meta">
              {overviewLoading
                ? "Refreshing"
                : overview?.daemon
                  ? `updated ${formatTimeLabel(overview.daemon.updated_at_ms)}`
                  : "No daemon status"}
            </span>
          </div>
          <div className="paper-overview-grid">
            <article className="paper-overview-item">
              <span className="paper-overview-item__label">Daemon</span>
              <strong
                className={`paper-overview-item__value paper-overview-item__value--${overview?.daemon?.running ? "positive" : "negative"}`}
              >
                {daemonHeadline(overview?.daemon ?? null)}
              </strong>
              <span className="paper-overview-item__detail">
                {overview?.daemon ? `poll ${overview.daemon.poll_interval_ms} ms` : "idle"}
              </span>
            </article>
            <article className="paper-overview-item">
              <span className="paper-overview-item__label">Strategies</span>
              <strong className="paper-overview-item__value">{strategyGroups.length}</strong>
              <span className="paper-overview-item__detail">
                {selectedStrategy?.label ?? "no selection"}
              </span>
            </article>
            <article className="paper-overview-item">
              <span className="paper-overview-item__label">Runs</span>
              <strong className="paper-overview-item__value">{overview?.sessions.length ?? 0}</strong>
              <span className="paper-overview-item__detail">
                {selectedStrategy ? `${strategySessions.length} for selected` : "no strategy"}
              </span>
            </article>
            <article className="paper-overview-item">
              <span className="paper-overview-item__label">Feed Hub</span>
              <strong className="paper-overview-item__value">
                {overview?.daemon?.subscription_count ?? 0}
              </strong>
              <span className="paper-overview-item__detail">
                {overview?.daemon ? `${overview.daemon.armed_feed_count} armed` : "no daemon"}
              </span>
            </article>
          </div>
        </section>

        <section className="panel paper-strategy-panel">
          <div className="panel__titlebar">
            <h2 className="panel__title">Strategies</h2>
            <span className="panel__meta">
              {overviewLoading
                ? "Refreshing"
                : strategyGroups.length
                  ? `${strategyGroups.length} configured`
                  : "No configured strategies"}
            </span>
          </div>
          {strategyGroups.length ? (
            <div className="paper-strategy-accordion">
              {strategyGroups.map((strategy) => {
                const active = strategy.key === selectedStrategyKey;
                return (
                  <details
                    key={strategy.key}
                    className={`paper-strategy-disclosure${active ? " paper-strategy-disclosure--active" : ""}`}
                    open={active}
                  >
                    <summary
                      className="paper-strategy-summary"
                      onClick={(event) => {
                        event.preventDefault();
                        setSelectedStrategyKey(strategy.key);
                        setSelectedSessionId(strategy.sessions[0]?.manifest.session_id ?? null);
                      }}
                    >
                      <div className="paper-strategy-summary__copy">
                        <strong>{strategy.label}</strong>
                        <span className="paper-session-card__meta">
                          {strategy.sessions.length} run{strategy.sessions.length === 1 ? "" : "s"} · updated{" "}
                          {formatTimeLabel(strategy.updatedAtMs)}
                        </span>
                      </div>
                      <div className="paper-strategy-summary__meta">
                        <span className="paper-session-card__stats">
                          <span>{strategy.liveCount} live</span>
                          <span>{strategy.failedCount} failed</span>
                        </span>
                        <span className={`status-pill status-pill--${toneForStatus(strategy.health)}`}>
                          {strategy.health}
                        </span>
                      </div>
                    </summary>
                    <div className="paper-strategy-disclosure__body">
                      <div className="paper-run-list">
                        {strategy.sessions.map((session) => {
                          const activeRun = session.manifest.session_id === selectedSessionId;
                          const sessionSummary = session.snapshot?.summary ?? null;
                          return (
                            <button
                              key={session.manifest.session_id}
                              className={`paper-run-button${activeRun ? " paper-run-button--active" : ""}`}
                              type="button"
                              onClick={() => {
                                setSelectedStrategyKey(strategy.key);
                                setSelectedSessionId(session.manifest.session_id);
                              }}
                            >
                              <span className="paper-run-button__name">{runLabel(session)}</span>
                              <span className="paper-run-button__meta">
                                {formatTimeLabel(session.manifest.start_time_ms)}
                              </span>
                              <span className={`status-pill status-pill--${toneForStatus(session.manifest.health)}`}>
                                {session.manifest.health}
                              </span>
                              <span className="paper-run-button__metric">
                                {sessionSummary
                                  ? formatPercent(sessionSummary.total_return * 100)
                                  : "No snapshot"}
                              </span>
                            </button>
                          );
                        })}
                      </div>
                    </div>
                  </details>
                );
              })}
            </div>
          ) : (
            <div className="empty-state">No paper sessions have been submitted.</div>
          )}
        </section>

        <section className="panel paper-detail-panel">
          {selectedSession ? (
            <>
              <div className="panel__titlebar">
                <div>
                  <h2 className="panel__title">{selectedStrategy?.label ?? sessionLabel(selectedSession)}</h2>
                  <span className="panel__meta">
                    {runLabel(selectedSession)} · {selectedSession.manifest.base_interval} · started{" "}
                    {formatTimeLabel(selectedSession.manifest.start_time_ms)}
                  </span>
                </div>
                <div className="paper-status-row">
                  {displayedHealth ? (
                    <span className={`status-pill status-pill--${toneForStatus(displayedHealth)}`}>
                      {displayedHealth}
                    </span>
                  ) : null}
                  {displayedStatus ? (
                    <span className={`status-pill status-pill--${toneForStatus(displayedStatus)}`}>
                      {displayedStatus}
                    </span>
                  ) : null}
                  {detailLoading ? <span className="panel__meta">Updating…</span> : null}
                </div>
              </div>
              {displayedFailureMessage ? (
                <div className="empty-state empty-state--paper-failure">
                  {displayedFailureMessage}
                </div>
              ) : null}
              <div className="paper-detail-stack">
                <section className="paper-detail-section">
                  <div className="paper-detail-section__header">
                    <h3 className="panel__title">Summary</h3>
                    <span className="panel__meta">live session snapshot</span>
                  </div>
                <div className="summary-grid summary-grid--paper">
                  <MetricCard label="Ending Equity" value={summary ? formatNumber(summary.ending_equity) : "NA"} />
                  <MetricCard
                    label="Total Return"
                    value={summary ? formatPercent(summary.total_return * 100) : "NA"}
                    tone={summary && summary.total_return >= 0 ? "positive" : "negative"}
                  />
                  <MetricCard
                    label="Realized PnL"
                    value={summary?.realized_pnl !== undefined ? formatSigned(summary.realized_pnl) : "NA"}
                    tone={summary?.realized_pnl !== undefined && summary.realized_pnl >= 0 ? "positive" : "negative"}
                  />
                  <MetricCard
                    label="Unrealized PnL"
                    value={summary?.unrealized_pnl !== undefined ? formatSigned(summary.unrealized_pnl) : "NA"}
                    tone={summary?.unrealized_pnl !== undefined && summary.unrealized_pnl >= 0 ? "positive" : "negative"}
                  />
                  <MetricCard label="Trades" value={String(displayedTradeCount)} />
                  <MetricCard label="Win Rate" value={summary ? formatPercent(summary.win_rate * 100) : "NA"} />
                  <MetricCard
                    label="Sharpe"
                    value={summary?.sharpe_ratio !== undefined && summary?.sharpe_ratio !== null ? formatNumber(summary.sharpe_ratio, 3) : "NA"}
                  />
                  <MetricCard
                    label="Max Drawdown"
                    value={summary ? formatNumber(summary.max_drawdown) : "NA"}
                    tone="negative"
                  />
                  <MetricCard label="Open Positions" value={String(displayedOpenPositions.length)} />
                  <MetricCard label="Open Orders" value={String(displayedOpenOrderCount)} />
                  <MetricCard
                    label="Fill Rate"
                    value={
                      diagnostics?.summary.order_fill_rate !== undefined
                        ? formatPercent(diagnostics.summary.order_fill_rate * 100)
                        : "NA"
                    }
                  />
                  <MetricCard
                    label="Avg Hold"
                    value={
                      diagnostics?.summary.average_bars_held !== undefined
                        ? `${formatNumber(diagnostics.summary.average_bars_held)} bars`
                        : "NA"
                    }
                  />
                </div>
                </section>

                <section className="paper-detail-section paper-detail-section--split">
                  <div className="paper-detail-column">
                    <div className="paper-detail-section__header">
                      <h3 className="panel__title">Equity Curve</h3>
                      <span className="panel__meta">
                        {result?.equity_curve.length ?? 0} points
                      </span>
                    </div>
                    {result?.equity_curve && result.equity_curve.length > 1 ? (
                      <LineChart
                        series={[
                          {
                            values: result.equity_curve.map((point) => point.equity),
                            stroke: "#1f8de1",
                            fill: "rgba(31, 141, 225, 0.14)",
                          },
                        ]}
                      />
                    ) : (
                      <div className="empty-state">No equity curve yet.</div>
                    )}
                  </div>
                  <div className="paper-detail-column">
                    <div className="paper-detail-section__header">
                      <h3 className="panel__title">Exposure</h3>
                      <span className="panel__meta">
                        max gross {summary?.max_gross_exposure !== undefined ? formatNumber(summary.max_gross_exposure) : "NA"}
                      </span>
                    </div>
                    {result?.equity_curve && result.equity_curve.length > 1 ? (
                      <>
                        <LineChart
                          series={[
                            {
                              values: result.equity_curve.map((point) => point.gross_exposure ?? 0),
                              stroke: "#f59e0b",
                            },
                            {
                              values: result.equity_curve.map((point) => Math.abs(point.net_exposure ?? 0)),
                              stroke: "#ef4444",
                            },
                          ]}
                        />
                        <div className="legend-row">
                          <span><i className="legend-swatch legend-swatch--amber" /> Gross</span>
                          <span><i className="legend-swatch legend-swatch--red" /> Net</span>
                        </div>
                      </>
                    ) : (
                      <div className="empty-state">No exposure curve yet.</div>
                    )}
                  </div>
                </section>

                <section className="paper-detail-section paper-detail-section--split">
                  <div className="paper-detail-column">
                    <div className="paper-detail-section__header">
                      <h3 className="panel__title">Feed Health</h3>
                      <span className="panel__meta">
                        {displayedFeedSummary
                          ? `${displayedFeedSummary.live_ready_feeds}/${displayedFeedSummary.total_feeds} live`
                          : "NA"}
                      </span>
                    </div>
                    <div className="summary-grid">
                      <MetricCard
                        label="History Ready"
                        value={displayedFeedSummary ? String(displayedFeedSummary.history_ready_feeds) : "NA"}
                      />
                      <MetricCard
                        label="Live Ready"
                        value={displayedFeedSummary ? String(displayedFeedSummary.live_ready_feeds) : "NA"}
                      />
                      <MetricCard
                        label="Failed"
                        value={displayedFeedSummary ? String(displayedFeedSummary.failed_feeds) : "NA"}
                        tone={displayedFeedSummary && displayedFeedSummary.failed_feeds > 0 ? "negative" : "neutral"}
                      />
                      <MetricCard
                        label="Latest Closed Bar"
                        value={
                          snapshot?.latest_closed_bar_time_ms
                            ? formatTimeLabel(snapshot.latest_closed_bar_time_ms)
                            : "NA"
                        }
                      />
                    </div>
                    <div className="list">
                      {displayedFeedSnapshots.length ? (
                        displayedFeedSnapshots.map((feed, index) => (
                          <article className="list-card" key={`${feed.execution_alias}-${index}`}>
                            <strong>
                              {feed.execution_alias} · {feed.symbol}
                            </strong>
                            <span>
                              {feed.template} · {feed.interval ?? selectedSession.manifest.base_interval} · {feed.arming_state ?? "n/a"}
                            </span>
                            <span>
                              top {feed.top_of_book ? formatNumber(feed.top_of_book.mid_price, 4) : "NA"} · last{" "}
                              {feed.last_price ? formatNumber(feed.last_price.price, 4) : "NA"} · mark{" "}
                              {feed.mark_price ? formatNumber(feed.mark_price.price, 4) : "NA"}
                            </span>
                            {feed.failure_message ? <span>{feed.failure_message}</span> : null}
                          </article>
                        ))
                      ) : (
                        <div className="empty-state">No feed snapshots yet.</div>
                      )}
                    </div>
                  </div>
                  <div className="paper-detail-column">
                    <div className="paper-detail-section__header">
                      <h3 className="panel__title">Open Positions</h3>
                      <span className="panel__meta">{displayedOpenPositions.length}</span>
                    </div>
                    <div className="list">
                      {displayedOpenPositions.length ? (
                        displayedOpenPositions.map((position, index) => (
                          <article className="list-card" key={`${position.execution_alias}-${index}`}>
                            <strong>
                              {position.execution_alias} · {position.side} · {formatNumber(position.quantity, 4)}
                            </strong>
                            <span>
                              entry {formatNumber(position.entry_price)} · mark {formatNumber(position.market_price)}
                            </span>
                            <span>unrealized {formatSigned(position.unrealized_pnl)}</span>
                            {renderMarginLine(position)}
                          </article>
                        ))
                      ) : (
                        <div className="empty-state">No open positions.</div>
                      )}
                    </div>
                  </div>
                </section>

                <section className="paper-detail-section paper-detail-section--split">
                  <div className="paper-detail-column">
                    <div className="paper-detail-section__header">
                      <h3 className="panel__title">Recent Trades</h3>
                      <span className="panel__meta">{result?.trades.length ?? displayedTradeCount}</span>
                    </div>
                    <div className="list">
                      {result?.trades?.length ? (
                        [...result.trades]
                          .slice(-50)
                          .reverse()
                          .map((trade, index) => (
                            <article className="list-card" key={index}>
                              <strong>
                                {trade.execution_alias ?? "session"} · {trade.side} · {trade.entry_module ?? "entry"}
                              </strong>
                              <span>
                                {formatTimeLabel(trade.entry.time)} → {formatTimeLabel(trade.exit.time)}
                              </span>
                              <span>
                                entry {formatNumber(trade.entry.price)} · exit {formatNumber(trade.exit.price)} · pnl{" "}
                                {formatSigned(trade.realized_pnl)}
                              </span>
                            </article>
                          ))
                      ) : (
                        <div className="empty-state">No trades yet.</div>
                      )}
                    </div>
                  </div>
                  <div className="paper-detail-column">
                    <div className="paper-detail-section__header">
                      <h3 className="panel__title">Orders</h3>
                      <span className="panel__meta">{result?.orders.length ?? displayedOpenOrderCount}</span>
                    </div>
                    <div className="list">
                      {result?.orders?.length ? (
                        [...result.orders]
                          .slice(-50)
                          .reverse()
                          .map((order, index) => (
                            <article className="list-card" key={index}>
                              <strong>
                                {order.execution_alias ?? "session"} · {order.role} · {order.kind}
                              </strong>
                              <span>
                                {order.status}
                                {order.end_reason ? ` · ${order.end_reason}` : ""}
                              </span>
                              <span>
                                placed {formatTimeLabel(order.placed_time)} · fill{" "}
                                {order.fill_time ? formatTimeLabel(order.fill_time) : "NA"} · px{" "}
                                {order.fill_price === null ? "NA" : formatNumber(order.fill_price)}
                              </span>
                            </article>
                          ))
                      ) : (
                        <div className="empty-state">No order history yet.</div>
                      )}
                    </div>
                  </div>
                </section>

                <section className="paper-detail-section paper-detail-section--split">
                  <div className="paper-detail-column">
                    <div className="paper-detail-section__header">
                      <h3 className="panel__title">Diagnostics</h3>
                      <span className="panel__meta">cohorts and risk</span>
                    </div>
                    {diagnostics ? (
                      <div className="paper-diagnostics">
                        <div className="summary-grid">
                          <MetricCard label="Average Bars To Fill" value={formatMetric(diagnostics.summary.average_bars_to_fill)} />
                          <MetricCard label="Average MAE %" value={formatMetric(diagnostics.summary.average_mae_pct)} />
                          <MetricCard label="Average MFE %" value={formatMetric(diagnostics.summary.average_mfe_pct)} />
                          <MetricCard label="Signal Exits" value={String(diagnostics.summary.signal_exit_count ?? 0)} />
                        </div>
                        <DiagnosticTable
                          title="By Side"
                          rows={(diagnostics.cohorts?.by_side ?? []).map((entry) => [
                            entry.side,
                            String(entry.trade_count),
                            formatPercent(entry.win_rate * 100),
                            formatNumber(entry.average_realized_pnl),
                          ])}
                          headers={["Side", "Trades", "Win", "Avg PnL"]}
                        />
                        <DiagnosticTable
                          title="Exit Classes"
                          rows={(diagnostics.cohorts?.by_exit_classification ?? []).map((entry) => [
                            entry.classification,
                            String(entry.trade_count),
                            formatPercent(entry.win_rate * 100),
                            formatNumber(entry.average_realized_pnl),
                          ])}
                          headers={["Exit", "Trades", "Win", "Avg PnL"]}
                        />
                        <DiagnosticTable
                          title="Weekday"
                          rows={(diagnostics.cohorts?.by_weekday_utc ?? []).map((entry) => [
                            `UTC ${entry.weekday_utc}`,
                            String(entry.trade_count),
                            formatPercent(entry.win_rate * 100),
                            formatNumber(entry.total_realized_pnl),
                          ])}
                          headers={["Bucket", "Trades", "Win", "Total PnL"]}
                        />
                      </div>
                    ) : (
                      <div className="empty-state">Detailed diagnostics are not available yet.</div>
                    )}
                  </div>
                  <div className="paper-detail-column">
                    <div className="paper-detail-section__header">
                      <h3 className="panel__title">Drawdown and Hints</h3>
                      <span className="panel__meta">
                        {diagnostics?.overfitting_risk?.level ?? "unknown"} risk
                      </span>
                    </div>
                    {diagnostics ? (
                      <div className="paper-diagnostics">
                        <div className="summary-grid">
                          <MetricCard label="Longest Drawdown" value={formatBars(diagnostics.drawdown?.longest_drawdown_bars)} />
                          <MetricCard label="Current Drawdown" value={formatBars(diagnostics.drawdown?.current_drawdown_bars)} />
                          <MetricCard label="Stagnation" value={formatBars(diagnostics.drawdown?.longest_stagnation_bars)} />
                          <MetricCard label="Recovery" value={formatBarsFloat(diagnostics.drawdown?.average_recovery_bars)} />
                        </div>
                        <div className="list">
                          {(diagnostics.hints ?? []).length ? (
                            diagnostics.hints?.map((hint, index) => (
                              <article className="list-card" key={index}>
                                <strong>{hint.kind}</strong>
                                <span>
                                  {hint.metric ?? "metric"} {hint.value !== null && hint.value !== undefined ? formatNumber(hint.value) : "NA"}
                                </span>
                              </article>
                            ))
                          ) : (
                            <div className="empty-state">No improvement hints.</div>
                          )}
                        </div>
                      </div>
                    ) : (
                      <div className="empty-state">No drawdown diagnostics yet.</div>
                    )}
                  </div>
                </section>

                <section className="paper-detail-section paper-detail-section--split">
                  <div className="paper-detail-column">
                    <div className="paper-detail-section__header">
                      <h3 className="panel__title">Transfer and Arbitrage</h3>
                      <span className="panel__meta">portfolio extras</span>
                    </div>
                    {diagnostics ? (
                      <div className="paper-diagnostics">
                        <DiagnosticTable
                          title="Transfers"
                          rows={(diagnostics.transfer_summary?.by_route ?? []).map((route) =>
                            transferRouteRow(route),
                          )}
                          headers={["Route", "Count", "Completed", "Fee"]}
                        />
                        <DiagnosticTable
                          title="Arbitrage"
                          rows={(diagnostics.arbitrage?.by_pair ?? []).map((pair) =>
                            arbitragePairRow(pair),
                          )}
                          headers={["Pair", "Baskets", "Completed", "PnL"]}
                        />
                      </div>
                    ) : (
                      <div className="empty-state">No transfer or arbitrage diagnostics.</div>
                    )}
                  </div>
                  <div className="paper-detail-column">
                    <div className="paper-detail-section__header">
                      <h3 className="panel__title">Session Log</h3>
                      <span className="panel__meta">{activeLogs?.logs.length ?? 0} events</span>
                    </div>
                    <div className="list">
                      {activeLogs?.logs.length ? (
                        [...activeLogs.logs]
                          .slice(-40)
                          .reverse()
                          .map((event: PaperSessionLogEvent, index) => (
                            <article className="list-card" key={index}>
                              <strong>{event.message}</strong>
                              <span>
                                {formatTimeLabel(event.time_ms)} · {event.status} · {event.health}
                              </span>
                            </article>
                          ))
                      ) : (
                        <div className="empty-state">No log events yet.</div>
                      )}
                    </div>
                  </div>
                </section>
              </div>
            </>
          ) : (
            <div className="empty-state empty-state--large">
              Select a running strategy to inspect live paper metrics.
            </div>
          )}
        </section>
      </main>
    </div>
  );
}

function buildOverviewStatus(
  daemon: ExecutionDaemonStatus | null | undefined,
  strategyCount: number,
  sessionCount: number,
): string {
  if (!daemon) {
    return sessionCount === 0
      ? "No paper daemon status found. Submit a session and start `execution serve`."
      : "Sessions are persisted, but the daemon has not written status yet.";
  }
  return daemon.running
    ? `Daemon online with ${daemon.subscription_count} subscriptions, ${strategyCount} strategy group(s), and ${sessionCount} tracked session(s).`
    : "Daemon status file exists, but the process is not currently running.";
}

function daemonHeadline(daemon: ExecutionDaemonStatus | null): string {
  if (!daemon) {
    return "offline";
  }
  return daemon.running ? "running" : "stopped";
}

function sessionLabel(session: PaperDashboardSession): string {
  const script = session.manifest.script_path?.split("/").pop();
  if (script) {
    return script;
  }
  const primary = session.manifest.execution_sources[0];
  return primary ? `${primary.template}:${primary.symbol}` : session.manifest.session_id;
}

function runLabel(session: PaperDashboardSession): string {
  return session.manifest.session_id.slice(0, 8);
}

function strategyKey(session: PaperDashboardSession): string {
  return session.manifest.script_path ?? sessionLabel(session);
}

function groupSessionsByStrategy(sessions: PaperDashboardSession[]): StrategyGroup[] {
  const groups = new Map<string, StrategyGroup>();

  for (const session of sessions) {
    const key = strategyKey(session);
    const existing = groups.get(key);
    if (existing) {
      existing.sessions.push(session);
      existing.liveCount += sessionHealthValue(session) === "live" ? 1 : 0;
      existing.failedCount += sessionHealthValue(session) === "failed" ? 1 : 0;
      existing.updatedAtMs = Math.max(existing.updatedAtMs, sessionUpdatedAt(session));
      existing.health = combineHealth(existing.health, sessionHealthValue(session));
      continue;
    }

    groups.set(key, {
      key,
      label: sessionLabel(session),
      sessions: [session],
      health: sessionHealthValue(session),
      liveCount: sessionHealthValue(session) === "live" ? 1 : 0,
      failedCount: sessionHealthValue(session) === "failed" ? 1 : 0,
      updatedAtMs: sessionUpdatedAt(session),
    });
  }

  for (const group of groups.values()) {
    group.sessions.sort((left, right) => sessionUpdatedAt(right) - sessionUpdatedAt(left));
  }

  return [...groups.values()].sort((left, right) => right.updatedAtMs - left.updatedAtMs);
}

function sessionUpdatedAt(session: PaperDashboardSession): number {
  return session.snapshot?.updated_at_ms ?? session.manifest.updated_at_ms;
}

function sessionHealthValue(session: PaperDashboardSession): string {
  return (session.snapshot?.health ?? session.manifest.health).toLowerCase();
}

function combineHealth(current: string, next: string): string {
  const ranking = ["failed", "degraded", "stopped", "live", "running"];
  const currentRank = ranking.indexOf(current);
  const nextRank = ranking.indexOf(next);
  if (currentRank === -1) {
    return next;
  }
  if (nextRank === -1) {
    return current;
  }
  return nextRank < currentRank ? next : current;
}

function toneForStatus(status: string): "positive" | "negative" | "neutral" {
  const value = status.toLowerCase();
  if (value === "live" || value === "running") {
    return "positive";
  }
  if (value === "failed" || value === "stopped" || value === "degraded") {
    return "negative";
  }
  return "neutral";
}

function formatSigned(value: number): string {
  const prefix = value >= 0 ? "+" : "";
  return `${prefix}${formatNumber(value)}`;
}

function renderMarginLine(position: PositionSnapshot) {
  if (
    position.free_collateral === undefined &&
    position.isolated_margin === undefined &&
    position.liquidation_price === undefined
  ) {
    return null;
  }
  return (
    <span>
      collateral {formatMaybe(position.free_collateral)} · isolated {formatMaybe(position.isolated_margin)} · liq{" "}
      {formatMaybe(position.liquidation_price)}
    </span>
  );
}

function formatMaybe(value: number | null | undefined): string {
  return value === null || value === undefined ? "NA" : formatNumber(value);
}

function formatMetric(value: number | null | undefined): string {
  return value === null || value === undefined ? "NA" : formatNumber(value);
}

function formatBars(value: number | null | undefined): string {
  return value === null || value === undefined ? "NA" : `${value} bars`;
}

function formatBarsFloat(value: number | null | undefined): string {
  return value === null || value === undefined ? "NA" : `${formatNumber(value)} bars`;
}

function transferRouteRow(route: TransferRouteDiagnosticSummary): string[] {
  return [
    `${route.from_alias} → ${route.to_alias}`,
    String(route.transfer_count),
    String(route.completed_transfer_count),
    formatNumber(route.total_fee),
  ];
}

function arbitragePairRow(pair: ArbitragePairDiagnosticSummary): string[] {
  return [
    `${pair.buy_alias} / ${pair.sell_alias}`,
    String(pair.basket_count),
    String(pair.completed_basket_count),
    formatNumber(pair.total_realized_pnl),
  ];
}

function DiagnosticTable({
  title,
  headers,
  rows,
}: {
  title: string;
  headers: string[];
  rows: string[][];
}) {
  return (
    <div className="diagnostic-table">
      <div className="diagnostic-table__title">{title}</div>
      {rows.length ? (
        <table>
          <thead>
            <tr>
              {headers.map((header) => (
                <th key={header}>{header}</th>
              ))}
            </tr>
          </thead>
          <tbody>
            {rows.map((row, index) => (
              <tr key={index}>
                {row.map((cell, cellIndex) => (
                  <td key={cellIndex}>{cell}</td>
                ))}
              </tr>
            ))}
          </tbody>
        </table>
      ) : (
        <div className="empty-state">No data.</div>
      )}
    </div>
  );
}
