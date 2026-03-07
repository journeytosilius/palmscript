# Diagnostics

PalmScript surfaces diagnostics and errors from four distinct layers.

## 1. Compile Diagnostics

Compile diagnostics are source-level failures with spans.

Diagnostic classes:

- lexical errors
- parse errors
- type and name-resolution errors
- compile-time structural errors

Examples:

- missing or duplicate `interval`
- unsupported `source` template
- unknown source alias
- undeclared `use` interval reference
- lower-than-base interval reference
- duplicate bindings
- invalid function recursion
- invalid builtin arity or argument type

These diagnostics surface through:

- `palmscript check`
- `palmscript run market`
- `palmscript dump-bytecode`
- `palmscript-lsp`
- the VS Code extension

## 2. Market Fetch Errors

After successful compilation, market mode may fail while constructing venue-backed feeds.

Owned by: exchange adapters and feed-fetch assembly.

Examples:

- `--from` is not less than `--to`
- the script has no `source` declarations
- an exchange request fails
- a venue response is malformed
- a required feed returns no data in the requested window
- a Hyperliquid spot symbol cannot be resolved

## 3. Runtime Errors

Runtime errors occur after feed preparation begins or during VM execution.

Owned by: runtime feed validation, VM execution, and output materialization.

Examples:

- feed alignment errors
- unsorted or duplicate prepared interval feeds
- missing or duplicate runtime feeds
- instruction-budget exhaustion
- stack underflow
- type mismatch in the VM
- invalid local or series slot
- history-capacity overflow
- output type mismatch during output collection

## Layer Ownership

The owning layer for a failure is part of the contract:

- syntax and semantic validity belong to compilation
- exchange/network/response validity belong to market fetch
- prepared-feed consistency and bytecode execution belong to runtime

PalmScript fails explicitly instead of silently degrading semantics.

## 4. Backtest Diagnostics

Successful backtests also return a structured diagnostics payload intended for
machine analysis and strategy iteration.

The backtest diagnostics surface includes:

- `order_diagnostics`: per-order snapshots at signal, placement, and fill time
- `trade_diagnostics`: per-trade entry/exit context, MAE, MFE, and exit classification
- `summary`: aggregate order and trade statistics
- `capture_summary`: execution-asset return, time spent flat or in market, and opportunity-cost return while flat
- `export_summaries`: one summary per exported series
- `opportunity_events`: bounded activation and signal-decision events with forward-return context

Export summaries use all named `export` series automatically:

- numeric exports report `min`, `max`, `mean`, `entry_mean`, and `exit_mean`
- bool exports report true/false counts, rising and falling edges, time spent true while flat or in market, and trade stats for entries taken while that export was true

Opportunity events are bounded and deterministic:

- every exported bool series emits an activation event on `false/na -> true`
- consumed backtest signals emit decision events such as queued, ignored same-side, ignored while flat, conflict, or replacement
- each event includes fixed forward-return horizons over `1`, `6`, and `24` execution bars

Backtest diagnostics are not compile failures or runtime errors. They are part
of the successful result surface returned by `palmscript run backtest` and
`run_backtest_with_sources`.
