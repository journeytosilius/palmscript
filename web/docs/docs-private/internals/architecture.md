# Repository Architecture

PalmScript is structured as a library-first repository. Tooling layers reuse the library instead of duplicating semantics.

## Main Areas

- `crates/palmscript/`: compiler, runtime, VM, IDE analysis, exchange adapters, and shared types
- `apps/cli/`: CLI binary crate
- `apps/lsp/`: language server binary crate
- `apps/ide-server/`: hosted browser IDE HTTP server
- `web/ide/`: React, TypeScript, Vite, and Monaco frontend bundle
- `web/docs/`: MkDocs source plus repo-private documentation
- `editors/vscode/`: VS Code extension
- `infra/`: Dockerfiles, nginx config, and web build scripts

## Architectural Principle

Language and runtime behavior belongs in the library. Wrappers such as the CLI and LSP should translate inputs and outputs, not implement separate semantics.

## Exchange Adapter Boundary

Exchange-backed source ingestion also stays inside the library.

Rules for this layer:

- each supported source template is represented by typed Rust enums and structs
- venue request payloads and response payloads should use typed `serde` models rather than ad hoc positional JSON handling
- venue-specific shapes are normalized into the canonical PalmScript bar schema before they reach the runtime
