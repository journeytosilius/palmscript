# Intervals and Sources

This page defines the normative interval and source rules for PalmScript.

## Supported Intervals

PalmScript accepts the interval literals listed in [Interval Table](intervals.md). Intervals are case-sensitive.

## Base Interval

Every script declares exactly one base interval:

```palmscript
interval 1m
```

The base interval defines the execution clock.

## Named Sources

Executable scripts declare one or more named exchange-backed sources:

```palmscript
interval 1m
source hl = hyperliquid.perps("BTC")
source bn = binance.spot("BTCUSDT")
use hl 1h

plot(bn.close - hl.1h.close)
```

Rules:

- at least one `source` declaration is required
- market series must be source-qualified
- each declared source contributes a base feed on the script base interval
- `use <alias> <interval>` declares an additional interval for that source
- `<alias>.<field>` refers to that source on the base interval
- `<alias>.<interval>.<field>` refers to that source on the named interval
- lower-than-base interval references are rejected

## Supported Source Templates

PalmScript currently supports these first-class templates:

- `binance.spot("<symbol>")`
- `binance.usdm("<symbol>")`
- `hyperliquid.spot("<symbol>")`
- `hyperliquid.perps("<symbol>")`

Interval support is template-specific:

- `binance.spot` accepts all supported PalmScript intervals
- `binance.usdm` accepts all supported PalmScript intervals
- `hyperliquid.spot` rejects `1s` and `6h`
- `hyperliquid.perps` rejects `1s` and `6h`

Operational fetch constraints are also template-specific:

- Hyperliquid REST only exposes the most recent `5000` candles per feed
- market mode rejects any Hyperliquid feed request that exceeds that retention window
- Binance feeds are paginated internally and do not have the same whole-window retention cap

## Source Field Set

All source templates are normalized into the same canonical market fields:

- `time`
- `open`
- `high`
- `low`
- `close`
- `volume`

Rules:

- `time` is the candle open time in Unix milliseconds UTC
- price and volume fields are numeric
- venue-specific extra fields are not exposed in the language

## Equal, Higher, and Lower Intervals

PalmScript distinguishes three cases for a referenced interval relative to the base interval:

- equal interval: valid
- higher interval: valid if declared with `use <alias> <interval>`
- lower interval: rejected

## Runtime Semantics

In market mode:

- PalmScript fetches the required `(source, interval)` feeds directly from the venues
- the base execution timeline is the union of all declared-source base-interval bar open times
- if one source has no base bar at a timeline step, that source contributes `na` for that step
- slower source intervals retain their last fully closed value until their next close boundary

## No-Lookahead Guarantee

PalmScript must not expose a higher-interval candle before that candle is fully closed.

This applies to source-aware qualified intervals such as `hl.1h.close`.

## Runtime Alignment Rules

Prepared feeds must be aligned to their declared intervals.

The runtime rejects feeds that are:

- misaligned to the interval boundary
- unsorted
- duplicated at one interval open time
