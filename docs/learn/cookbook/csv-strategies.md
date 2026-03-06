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

## Signal Helpers

Use this when you want a checked-in example of:

- `above`, `crossover`, `roc`, `highest`, `lowest`
- directional helpers such as `rising`
- combining helper builtins with `export` and `trigger`

```bash
palmscript run csv examples/strategies/signal_helpers.palm \
  --bars examples/data/minute_bars.csv
```

## Event Memory

Use this when you want a checked-in example of:

- `barssince`
- `valuewhen`
- event-style breakout tracking without user-defined helper functions

```bash
palmscript run csv examples/strategies/event_memory.palm \
  --bars examples/data/minute_bars.csv
```

## Notes

- CSV mode expects `time,open,high,low,close,volume`
- the raw input feed must be time-ordered
- declared higher intervals must be derivable from full raw buckets

Reference:

- [CSV Mode](../../tooling/csv-mode.md)
- [CLI](../../tooling/cli.md)
