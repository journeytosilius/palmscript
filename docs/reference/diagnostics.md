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
