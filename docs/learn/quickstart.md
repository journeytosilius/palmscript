# Quickstart

## 1. Validate A Script

```bash
palmscript check strategy.palm
```

## 2. Run A Market-Backed Script

```bash
palmscript run market strategy.palm \
  --from 1704067200000 \
  --to 1704153600000
```

## 3. Run Another Exchange-Backed Script

```bash
palmscript run market spread_strategy.palm \
  --from 1704067200000 \
  --to 1704153600000
```

## 4. Inspect Compiled Output

```bash
palmscript dump-bytecode strategy.palm
```

Next: [First Strategy](first-strategy.md)
