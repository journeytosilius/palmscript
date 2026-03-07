# Reference Overview

This section is the normative definition of PalmScript as implemented in this repository.

If a guide page and a reference page ever differ, the reference page is authoritative.

This section defines the language that exists today. It is not a design document for planned syntax.

## What This Section Defines

- lexical structure
- grammar
- declarations and scope rules
- types and values
- series and indexing semantics
- evaluation semantics
- interval and source rules
- builtin and indicator contracts
- output semantics
- diagnostics classes

## Implemented Today

The current PalmScript surface includes:

- exactly one top-level base `interval <...>` directive per script
- one or more named `source` aliases per executable script
- source-qualified series such as `spot.close` or `hl.1h.close`
- supplemental intervals through `use <alias> <interval>`
- top-level expression-bodied `fn` declarations
- `let`, tuple destructuring, `export`, and `trigger`
- `if / else if / else`, with `else` required
- literal-only series indexing, typed `ma_type.<variant>` enum literals, and deterministic three-valued boolean logic
- a TA-Lib-style builtin surface where some names are executable today and additional reserved names are exposed through diagnostics and IDE metadata

Representative checked-in examples:

- [`examples/strategies/sma_cross.palm`](https://github.com/journeytosilius/palmscript/blob/main/examples/strategies/sma_cross.palm)
- [`examples/strategies/weekly_bias.palm`](https://github.com/journeytosilius/palmscript/blob/main/examples/strategies/weekly_bias.palm)
- [`examples/strategies/macd_tuple.palm`](https://github.com/journeytosilius/palmscript/blob/main/examples/strategies/macd_tuple.palm)
- [`examples/strategies/cross_source_spread.palm`](https://github.com/journeytosilius/palmscript/blob/main/examples/strategies/cross_source_spread.palm)

## Current Boundaries

Important implementation boundaries:

- `interval`, `source`, `use`, `fn`, `export`, and `trigger` are top-level only
- bare market identifiers such as `close` are not valid in executable scripts
- higher source intervals require `use <alias> <interval>`
- only identifiers are callable
- string literals are only valid inside `source` declarations
- series indexing requires a non-negative integer literal
- tuple-valued builtin results must be destructured with `let (...) = ...` before further use
- the TA-Lib catalog is broader than the executable runtime surface; use [Builtins](builtins.md), [Indicators](indicators.md), and [TA-Lib Surface](ta-lib.md) for the exact status

## How To Read It

- start with [Lexical Structure](lexical-structure.md) and [Grammar](grammar.md) for accepted syntax
- use [Declarations and Scope](declarations-and-scope.md) for binding and visibility rules
- use [Evaluation Semantics](evaluation-semantics.md) and [Intervals and Sources](intervals-and-sources.md) for runtime meaning
- use [Builtins](builtins.md), [Indicators](indicators.md), and [Outputs](outputs.md) for callable/output behavior

Examples in this section illustrate the rules, but the normative text is the rule.
