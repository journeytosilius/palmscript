# CLI Strategies

Checked-in `.palm` strategies live under `examples/strategies/`.

## `sma_cross.palm`

- base interval: `1m`
- demonstrates `let`, `export`, `trigger`, and indicator comparison

```bash
palmscript run csv examples/strategies/sma_cross.palm \
  --bars examples/data/minute_bars.csv
```

## `volume_breakout.palm`

- base interval: `1m`
- demonstrates breakout logic plus trigger output

```bash
palmscript run csv examples/strategies/volume_breakout.palm \
  --bars examples/data/minute_bars.csv \
  --format text
```

## `weekly_bias.palm`

- base interval: `1d`
- declared supplemental interval: `1w`
- demonstrates higher-timeframe basis logic

```bash
palmscript run csv examples/strategies/weekly_bias.palm \
  --bars /path/to/daily_bars.csv
```

That example requires enough daily data to roll into full weekly candles.
