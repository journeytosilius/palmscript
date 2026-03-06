# Quickstart

## 1. Build The Binaries

```bash
cargo build --bin palmscript --bin palmscript-lsp
```

## 2. Validate A Strategy

```bash
target/debug/palmscript check examples/strategies/sma_cross.palm
```

## 3. Run A CSV-Backed Strategy

```bash
target/debug/palmscript run csv examples/strategies/sma_cross.palm \
  --bars examples/data/minute_bars.csv
```

CSV mode treats the file as a raw OHLCV feed with the schema:

```text
time,open,high,low,close,volume
```

See [CSV Mode](../tooling/csv-mode.md) for the full operational contract.

## 4. Run An Exchange-Backed Strategy

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

See [Market Mode](../tooling/market-mode.md) for supported source templates and fetch behavior.

## 5. Inspect Compiled Output

```bash
target/debug/palmscript dump-bytecode examples/strategies/sma_cross.palm
```

## 6. Use The Editor Tooling

- install or build the PalmScript VS Code extension
- open a `.palm` file
- use diagnostics, formatting, hover, completion, definitions, and document symbols from `palmscript-lsp`

Next: [First Strategy](first-strategy.md)
