# PalmScript Documentation

PalmScript is a deterministic DSL for financial time-series programs. Scripts compile to bytecode and execute inside a bounded-history VM with no filesystem, network, wall-clock, or randomness access during strategy execution.

This site is the canonical documentation source for the repository.

## Documentation Map

- `Learn` teaches how to write and run PalmScript strategies.
- `Reference` is the normative definition of PalmScript syntax and semantics.
- `Tooling` explains the CLI, execution modes, and editor integrations.
- `Internals` documents compiler, bytecode, VM, and runtime architecture.
- `Contributing` covers repository workflow, testing, releases, and docs maintenance.

## Start Here

- New to PalmScript: [Learn Overview](learn/overview.md)
- Want a first runnable strategy: [Quickstart](learn/quickstart.md)
- Need the formal language definition: [Reference Overview](reference/overview.md)
- Running scripts from the CLI: [CLI](tooling/cli.md)
- Understanding the editor workflow: [VS Code Extension](tooling/vscode.md)

## Current Capabilities

PalmScript currently implements:

- a mandatory base `interval <...>` declaration
- legacy global `use <interval>` declarations for source-less scripts
- named exchange-backed `source` declarations
- source-scoped `use <alias> <interval>` declarations
- numeric, boolean, string-in-source-declaration, and `na` literals
- `let`, `export`, and `trigger`
- `if / else if / else`
- arithmetic, comparisons, unary operators, `and`, and `or`
- series indexing with literal offsets
- builtins: `sma`, `ema`, `rsi`, and `plot`
- CSV-backed and exchange-backed execution modes
- a CLI, language server, and first-party VS Code extension

## Reading Strategy

If you want to learn PalmScript, start in `Learn`.

If you need exact rules for what the compiler accepts or what the runtime does, use `Reference`.

If you are changing the implementation, use `Internals` and `Contributing`.
