# Glossary

## Base Interval

The execution interval declared by `interval <...>`.

## Declared Interval

An interval explicitly introduced through `use <alias> <...>` for a named source.

## Source-Aware Script

A script that declares at least one `source`.

## Source Template

A built-in exchange/venue constructor such as `binance.spot`, `bybit.usdt_perps`, `gate.spot`, or `hyperliquid.perps`.

## Venue-Native Symbol

The exchange-specific symbol string used in a source declaration, such as `BTCUSDT` on Bybit or `BTC_USDT` on Gate.

## Market Mode

Execution against historical market-backed source feeds.

## No Lookahead

The guarantee that a higher-interval sample becomes visible only after that candle fully closes.

## Output Series

A named per-step result emitted by `export` or `trigger`.

## Union Of Base Timestamps

The execution timeline built from the union of all declared-source base-interval candle open times.
