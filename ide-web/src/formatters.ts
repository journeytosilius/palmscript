const DAY_MS = 24 * 60 * 60 * 1000;

export const DEFAULT_SOURCE = `interval 4h
source spot = binance.spot("BTCUSDT")
use spot 1d
use spot 1w

let fast = ema(spot.close, 13)
let slow = ema(spot.close, 89)
let daily_fast = ema(spot.1d.close, 30)
let daily_slow = ema(spot.1d.close, 80)
let weekly_fast = ema(spot.1w.close, 5)
let weekly_slow = ema(spot.1w.close, 13)

entry long = above(fast, slow) and above(daily_fast, daily_slow) and above(weekly_fast, weekly_slow)
exit long = below(fast, slow)

plot(fast - slow)
export trend_long_state = above(fast, slow)
`;

export function dateInputValue(timeMs: number): string {
  return new Date(timeMs).toISOString().slice(0, 10);
}

export function parseDateInput(value: string): number {
  return Date.parse(`${value}T00:00:00Z`);
}

export function defaultWindowForDataset(dataset: {
  from: number;
  to: number;
}): { from: string; to: string } {
  const datasetEnd = dataset.to - DAY_MS;
  const yearWindowStart = Math.max(dataset.from, dataset.to - 365 * DAY_MS);
  return {
    from: dateInputValue(yearWindowStart),
    to: dateInputValue(datasetEnd),
  };
}

export function formatNumber(value: number, digits = 2): string {
  return new Intl.NumberFormat("en-US", {
    minimumFractionDigits: digits,
    maximumFractionDigits: digits,
  }).format(value);
}

export function formatPercent(value: number): string {
  return `${formatNumber(value, 2)}%`;
}

export function formatDateLabel(timeMs: number): string {
  return new Intl.DateTimeFormat("en-US", {
    year: "numeric",
    month: "short",
    day: "2-digit",
    timeZone: "UTC",
  }).format(timeMs);
}

export function formatTimeLabel(timeMs: number): string {
  return new Intl.DateTimeFormat("en-US", {
    year: "numeric",
    month: "short",
    day: "2-digit",
    hour: "2-digit",
    minute: "2-digit",
    timeZone: "UTC",
  }).format(timeMs);
}

export function clampWindow(
  dataset: { from: number; to: number },
  from: string,
  to: string,
): { fromMs: number; toMs: number } {
  const fromMs = parseDateInput(from);
  const toMs = parseDateInput(to) + DAY_MS;

  if (!Number.isFinite(fromMs) || !Number.isFinite(toMs)) {
    throw new Error("Choose a valid From and To date.");
  }
  if (fromMs >= toMs) {
    throw new Error("The From date must be before the To date.");
  }
  if (fromMs < dataset.from || toMs > dataset.to) {
    throw new Error(
      `The selected window must stay inside ${formatDateLabel(dataset.from)} to ${formatDateLabel(dataset.to - DAY_MS)}.`,
    );
  }
  return { fromMs, toMs };
}
