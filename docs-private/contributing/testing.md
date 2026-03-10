# Testing Expectations

PalmScript is a financial computation engine, so tests are mandatory for non-trivial changes.

## Required Quality Gate

Before completing a change:

```bash
cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
cargo test
mkdocs build --strict
```

## Expected Test Coverage

Depending on the change, add or update:

- lexer and parser tests
- semantic/compiler tests
- VM/runtime tests
- committed golden or oracle fixtures when runtime math must match an external source of truth
- CLI integration tests
- language server and VS Code tests
- documentation links and command examples when docs-facing behavior changes

Regression fixes should include a regression test whenever practical.

## Diagnostic Coverage

Public diagnostics are part of the contract.

When a change adds or changes a user-facing failure in:

- lexing
- parsing
- semantic/type analysis
- CSV data preparation
- exchange-backed market fetching

add or update the catalog-driven regression tests under `tests/diagnostics_*.rs`.

Library-level diagnostic assertions should prefer exact diagnostic kind and exact message text. CLI-layer tests may assert representative framing and selected rendered lines.

## TA-Lib Oracle Fixtures

The TA-Lib port now has an offline oracle path:

- committed fixtures live under `tests/data/ta_lib/`
- parity assertions live in `tests/ta_lib_parity.rs`
- fixture refresh happens through `tools/generate_talib_fixtures.py`

The generator builds the pinned upstream TA-Lib commit in a temporary workspace,
runs the deterministic fixture corpus through the upstream C library, and writes
JSON that CI consumes directly. CI must not build or link TA-Lib live.
