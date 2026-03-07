# Glossary

## Base Interval

The execution interval declared by `interval <...>`.

## Declared Interval

An interval explicitly introduced through `use <alias> <...>` for a named source.

## Source-Aware Script

A script that declares at least one `source`.

## Source Template

A built-in exchange/venue constructor such as `binance.spot` or `hyperliquid.perps`.

## Market Mode

The CLI execution mode invoked as `palmscript run market ...`.

## No Lookahead

The guarantee that a higher-interval sample becomes visible only after that candle fully closes.

## Output Series

A named per-step result emitted by `export` or `trigger`.

## Union Of Base Timestamps

The market-mode execution timeline built from the union of all declared-source base-interval candle open times.
