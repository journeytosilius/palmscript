# Language Overview

PalmScript strategies are top-level source files made of declarations and statements.

Common building blocks:

- `interval <...>` for the base execution clock
- optional `source` declarations for exchange-backed markets
- optional `use` declarations for higher or equal intervals
- top-level functions
- `let`, `export`, and `trigger`
- `if / else if / else`
- expressions built from operators, calls, and indexing
- helper builtins such as `crossover`, `highest`, `barssince`, and `valuewhen`

## Two Script Styles

### Source-Less Scripts

These run against one raw market feed, usually through CSV mode:

```palmscript
interval 1m
let basis = ema(close, 20)
plot(close - basis)
```

### Source-Aware Scripts

These name exchange-backed markets explicitly:

```palmscript
interval 1m
source bn = binance.spot("BTCUSDT")
source hl = hyperliquid.perps("BTC")

plot(bn.close - hl.close)
```

## Mental Model

- the script always has exactly one base interval
- series values evolve over time
- higher intervals update only when those candles fully close
- missing history or missing aligned source data appears as `na`
- `plot`, `export`, and `trigger` emit results after each execution step

## Where To Go For Exact Rules

- syntax and tokens: [Lexical Structure](../reference/lexical-structure.md) and [Grammar](../reference/grammar.md)
- declarations and visibility: [Declarations and Scope](../reference/declarations-and-scope.md)
- expressions and runtime meaning: [Evaluation Semantics](../reference/evaluation-semantics.md)
- market series rules: [Intervals and Sources](../reference/intervals-and-sources.md)
- outputs: [Outputs](../reference/outputs.md)
