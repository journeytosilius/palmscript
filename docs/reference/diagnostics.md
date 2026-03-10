# Diagnostics

PalmScript surfaces diagnostics and errors from three public layers.

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

## 2. Market Fetch Errors

After successful compilation, `run market` may fail while preparing the required historical feeds.

Examples:

- `--from` is not less than `--to`
- the script has no `source` declarations
- an exchange request fails
- a venue response is malformed
- a required feed returns no data in the requested window
- a symbol cannot be resolved by the selected venue

## 3. Runtime Errors

Runtime errors occur after feed preparation begins or during execution.

Examples:

- feed alignment errors
- missing or duplicate runtime feeds
- instruction-budget exhaustion
- stack underflow
- type mismatch during execution
- invalid local or series slot
- history-capacity overflow
- output type mismatch during output collection

## Layer Ownership

The owning layer for a failure is part of the contract:

- syntax and semantic validity belong to compilation
- exchange/network/response validity belong to market fetch
- feed consistency and execution validity belong to runtime

PalmScript fails explicitly instead of silently degrading semantics.
