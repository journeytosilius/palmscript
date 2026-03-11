# Repository Architecture

PalmScript is structured as a library-first repository. Tooling layers reuse the library instead of duplicating semantics.

## Main Areas

- `src/`: compiler, runtime, VM, IDE, and shared types
- `cli/`: CLI binary
- `lsp/`: language server binary
- `editors/vscode/`: VS Code extension
- `examples/`: Rust examples, `.ps` strategies, and fixtures
- `tests/`: integration and CLI coverage
- `docs/`: canonical documentation source

## Architectural Principle

Language and runtime behavior belongs in the library. Wrappers such as the CLI and LSP should translate inputs and outputs, not implement separate semantics.

## Exchange Adapter Boundary

Exchange-backed source ingestion also stays inside the library.

Rules for this layer:

- each supported source template is represented by typed Rust enums and structs
- venue request payloads and response payloads should use typed `serde` models rather than ad hoc positional JSON handling
- venue-specific shapes are normalized into the canonical PalmScript bar schema before they reach the runtime
