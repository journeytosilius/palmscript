# Quickstart

## 1. Check a Strategy

```bash
target/debug/palmscript check examples/strategies/sma_cross.palm
```

## 2. Run a Strategy in CSV Mode

```bash
target/debug/palmscript run csv examples/strategies/sma_cross.palm \
  --bars examples/data/minute_bars.csv
```

CSV mode accepts one raw market-data file, infers its source interval, and rolls it up into the strategy's declared `interval` and `use` intervals when possible.

PalmScript also supports exchange-backed source-aware runs:

```palmscript
interval 1m
source bn = binance.spot("BTCUSDT")
plot(bn.close)
```

```bash
target/debug/palmscript run market strategy.palm \
  --from 1704067200000 \
  --to 1704153600000
```

## 3. Inspect Bytecode

```bash
target/debug/palmscript dump-bytecode examples/strategies/sma_cross.palm
```

## 4. Open the Project in VS Code

- install the PalmScript extension
- open a `.palm` file
- diagnostics, completions, hovers, definitions, document symbols, and formatting are provided by `palmscript-lsp`

## 5. Build and Serve the Docs

```bash
python -m venv .venv-docs
source .venv-docs/bin/activate
pip install -r requirements-docs.txt
mkdocs serve
```
