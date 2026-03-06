# Cookbook: CSV Strategies

Checked-in CSV-oriented strategies live under `examples/strategies/`.

## SMA Cross

Use this when you want a minimal example of:

- `let`
- `export`
- `trigger`
- indicator comparison on one base interval

```bash
palmscript run csv examples/strategies/sma_cross.palm \
  --bars examples/data/minute_bars.csv
```

## Volume Breakout

Use this when you want an example of:

- breakout-style conditions
- trigger output
- text rendering from the CLI

```bash
palmscript run csv examples/strategies/volume_breakout.palm \
  --bars examples/data/minute_bars.csv \
  --format text
```

## Notes

- CSV mode expects `time,open,high,low,close,volume`
- the raw input feed must be time-ordered
- declared higher intervals must be derivable from full raw buckets

Reference:

- [CSV Mode](../../tooling/csv-mode.md)
- [CLI](../../tooling/cli.md)
