# Glossary

## Base Interval

The execution interval declared by `interval <...>`.

## Declared Interval

An interval explicitly introduced through `use <...>` in a source-less script or `use <alias> <...>` in a source-aware script.

## Source-Aware Script

A script that declares at least one `source`.

## Source-Less Script

A script that declares no `source`.

## Source Template

A built-in exchange/venue constructor such as `binance.spot` or `hyperliquid.perps`.

## CSV Mode

The CLI execution mode invoked as `palmscript run csv ...`.

## Market Mode

The CLI execution mode invoked as `palmscript run market ...`.

## No Lookahead

The guarantee that a higher-interval sample becomes visible only after that candle fully closes.

## Output Series

A named per-step result emitted by `export` or `trigger`.

## Raw Interval

The interval inferred from an input CSV file before roll-up.

## Roll-Up

The deterministic aggregation of a finer CSV feed into a coarser interval feed.
