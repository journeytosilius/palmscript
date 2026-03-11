# PalmScript

<p align="center">
  <img src="editors/vscode/images/palmscript.png" alt="PalmScript logo" width="220">
</p>

PalmScript is a language for financial time-series strategies.

The public documentation is focused on:

- language syntax and semantics
- builtins and indicators
- examples and learning guides
- the basic CLI flow for checking and running scripts

Documentation:

- published site: <https://palmscript.dev/docs/>
- hosted IDE: <https://palmscript.dev/app/>
- local source: [docs/index.md](docs/index.md)

Start here:

- [Learn](docs/learn/overview.md)
- [Language Reference](docs/reference/overview.md)
- [Indicators Reference](docs/reference/indicators.md)

Repo-local tooling docs:

- [Browser IDE](docs-private/tooling/browser-ide.md)

## Common Commands

```bash
cargo build --bin palmscript
cargo build --bin palmscript-ide-server
npm --prefix ide-web run build
target/debug/palmscript check examples/strategies/sma_cross.ps
target/debug/palmscript run market examples/strategies/sma_cross.ps --from 1704067200000 --to 1704153600000
target/debug/palmscript run market examples/strategies/cross_source_spread.ps --from 1704067200000 --to 1704153600000
target/debug/palmscript dump-bytecode examples/strategies/sma_cross.ps
mkdocs build --strict
```

## Browser IDE Container

```bash
bash scripts/build_ide_web.sh
docker build -f Dockerfile.ide -t palmscript-ide .
docker run --rm -p 8080:8080 palmscript-ide
```

The browser IDE shell now ships as a Vite-built React and TypeScript frontend
using Monaco Editor, embedded directly by the `palmscript-ide-server` binary.
Refresh the checked-in browser bundle with `bash scripts/build_ide_web.sh` when
you change the frontend under `ide-web/`.

The web shell keeps the same blue-grey and accent-blue visual language as the
published docs at <https://palmscript.dev/docs/>.

The public demo keeps the chrome intentionally minimal: one editor buffer, a
calendar date-range picker over the curated BTCUSDT dataset, Monaco editing,
compile diagnostics, and backtest output panels. The toolbar keeps the
PalmScript logo inside the header instead of a text title.

The hosted reverse-proxy entrypoint is `/app/`. `https://palmscript.dev/app`
redirects there.
