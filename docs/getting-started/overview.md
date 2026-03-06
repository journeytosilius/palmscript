# Overview

PalmScript exists in three layers:

- the Rust library, which owns lexing, parsing, semantic analysis, bytecode generation, VM execution, input preparation, and IDE analysis
- the `palmscript` CLI, which runs scripts directly
- the editor stack, built from `palmscript-lsp` plus the VS Code extension

## Repository Outputs

- `palmscript`: CLI for `check`, `run csv`, `run market`, and `dump-bytecode`
- `palmscript-lsp`: stdio language server used by editors
- `editors/vscode/`: the first-party VS Code extension

## How To Use The Project

- Write `.palm` strategies with an `interval <...>` directive, optional `use <...>` declarations, and optional named `source` declarations.
- Validate them with `palmscript check`.
- Execute file-backed strategies with `palmscript run csv ...` and source-aware strategies with `palmscript run market ...`.
- Inspect compiled output with `palmscript dump-bytecode`.
- Author them interactively with the VS Code extension.

## Required Background

You do not need Rust to use PalmScript from the CLI or VS Code. You only need Rust when building the binaries from source or embedding the library directly.
