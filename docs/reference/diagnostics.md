# Diagnostics and Error Classes

PalmScript surfaces several classes of user-visible errors.

## Compile Errors

Compile-time diagnostics include:

- lexer and parser errors
- invalid `interval` / `use` declarations
- type errors
- invalid identifiers
- illegal function usage

These surface through:

- `palmscript check`
- `palmscript run csv` before execution
- `palmscript run market` before execution
- `palmscript dump-bytecode`
- `palmscript-lsp`
- the VS Code extension

## CSV Mode Data Preparation Errors

The data-preparation layer can fail before runtime with errors such as:

- `CannotInferInputInterval`
- `MissingBaseIntervalDeclaration`
- `RawIntervalTooCoarse`
- `UnsupportedRollupPath`
- `InsufficientDataForInterval`
- `IncompleteRollupBucket`
- `UnsortedInputBars`
- `DuplicateInputBarTime`

These happen after successful compilation but before VM execution.

## Market Mode Fetch Errors

Exchange-backed runs can fail before VM execution with errors such as:

- invalid `--from` / `--to` windows
- unsupported source templates or intervals
- malformed exchange responses
- no returned candles for a required source feed
- unknown Hyperliquid spot symbols

## Runtime Errors

Runtime errors include:

- feed compatibility problems
- history-cap violations
- execution-limit violations

The runtime fails deterministically rather than silently degrading semantics.
