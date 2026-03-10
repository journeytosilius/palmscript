# Reference Overview

This section is the normative definition of PalmScript as documented publicly.

If a guide page and a reference page ever differ, the reference page is authoritative.

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
- the public CLI command surface

## Implemented Today

The current PalmScript surface includes:

- exactly one top-level base `interval <...>` directive per script
- one or more named `source` aliases per executable script
- source-qualified series such as `spot.close` or `hl.1h.close`
- supplemental intervals through `use <alias> <interval>`
- top-level expression-bodied `fn` declarations
- `let`, `const`, `input`, tuple destructuring, `export`, `trigger`, first-class `entry` / `exit`, and `order`
- literal-only series indexing, typed `ma_type.<variant>`, `tif.<variant>`, `trigger_ref.<variant>`, `position_side.<variant>`, and `exit_kind.<variant>` enum literals, and deterministic three-valued boolean logic
- a TA-Lib-style builtin surface where some names are executable today and additional reserved names are exposed through diagnostics

## Current Boundaries

- `interval`, `source`, `use`, `fn`, `const`, `input`, `export`, `trigger`, `entry`, `exit`, and `order` are top-level only
- bare market identifiers such as `close` are not valid in executable scripts
- higher source intervals require `use <alias> <interval>`
- only identifiers are callable
- string literals are only valid inside `source` declarations
- series indexing requires a non-negative integer literal
- tuple-valued builtin results must be destructured with `let (...) = ...` before further use

## How To Read It

- start with [Lexical Structure](lexical-structure.md) and [Grammar](grammar.md) for accepted syntax
- use [Declarations and Scope](declarations-and-scope.md) for binding and visibility rules
- use [Evaluation Semantics](evaluation-semantics.md) and [Intervals and Sources](intervals-and-sources.md) for language meaning
- use [Builtins](builtins.md), [Indicators](indicators.md), and [Outputs](outputs.md) for callable and output behavior
