# PalmScript

<p align="center">
  <img src="editors/vscode/images/palmscript.png" alt="PalmScript logo" width="220">
</p>

PalmScript is a deterministic DSL and bytecode VM for financial time-series strategies.

The language now includes indicator, signal-helper, event-memory, and early TA-Lib-style builtins such as `crossover`, `highest`, `barssince`, `valuewhen`, `ma`, `apo`, `ppo`, `macd`, `wma`, `avgdev`, `stddev`, `linearreg`, `beta`, `correl`, `aroon`, `aroonosc`, `bop`, `cci`, `cmo`, `mom`, `roc`, `willr`, and `minmax` in addition to the core OHLCV series model.

The repository currently ships:

- the Rust library crate
- the `palmscript` CLI
- the `palmscript-lsp` language server
- the first-party VS Code extension
- the MkDocs documentation site

## Current Language Surface

PalmScript currently implements:

- exactly one top-level base `interval <...>` directive per script
- source-less scripts that use bare OHLCV series such as `close`
- source-aware scripts that use named `source` declarations and source-qualified series such as `bn.close` or `hl.1h.close`
- legacy `use <interval>` declarations for source-less supplemental intervals
- source-scoped `use <alias> <interval>` declarations for source-aware supplemental intervals
- top-level expression-bodied `fn` declarations, `let`, tuple destructuring, `export`, and `trigger`
- deterministic three-valued boolean logic, bounded-history indexing, and typed `ma_type.<variant>` enum literals
- a partially executable TA-Lib-style builtin surface, with additional reserved catalog names exposed through diagnostics and IDE metadata

Checked-in strategy examples live under [`examples/strategies/`](examples/strategies/).

## Documentation

The canonical documentation source is the MkDocs site under `docs/`.

- local source: [docs/index.md](docs/index.md)
- published site: <https://palmscript.dev/docs/>
- GitHub Pages mirror: <https://journeytosilius.github.io/palmscript/>

Start here:

- [Learn](docs/learn/overview.md)
- [Language Reference](docs/reference/overview.md)
- [CLI](docs/tooling/cli.md)
- [VS Code Extension](docs/tooling/vscode.md)

## Common Commands

```bash
cargo build --bin palmscript --bin palmscript-lsp
target/debug/palmscript check examples/strategies/sma_cross.palm
target/debug/palmscript run csv examples/strategies/sma_cross.palm --bars examples/data/minute_bars.csv
target/debug/palmscript run csv examples/strategies/macd_tuple.palm --bars examples/data/minute_bars.csv
target/debug/palmscript run market examples/strategies/cross_source_spread.palm --from 1704067200000 --to 1704153600000
mkdocs build --strict
docker build -f Dockerfile.docs -t palmscript-docs .
```
