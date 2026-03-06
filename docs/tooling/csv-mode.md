# CSV Mode

`palmscript run csv ...` is the file-backed execution mode for source-less strategies.

## Invocation

```bash
palmscript run csv strategy.palm --bars bars.csv
```

## Input Contract

CSV mode accepts one raw market-data file with this exact header:

```text
time,open,high,low,close,volume
```

Rules:

- `time` must be the candle open time in Unix milliseconds UTC
- rows must be strictly increasing in time
- duplicate timestamps are rejected
- all numeric fields must parse as finite numbers

## What CSV Mode Builds

CSV mode uses one raw file to prepare:

- the base feed required by `interval <...>`
- each additional legacy interval declared with `use <...>`

It does not read `source` declarations or fetch exchange data.

## Interval Inference

Before execution, the data-preparation layer infers the raw interval from the timestamps.

Inference requires:

- alignment to a supported PalmScript interval boundary
- strictly increasing timestamps
- consecutive gaps that are whole multiples of the inferred interval
- at least one exact one-candle gap

If inference fails, execution stops before the VM starts.

## Roll-Up Rules

Roll-up is strict.

Rules:

- buckets must be complete
- missing raw bars inside a bucket are fatal
- no partial rolled candle is emitted
- if a declared interval cannot produce at least one full candle, execution fails

Aggregation:

- `open`: first raw bar open
- `high`: maximum high
- `low`: minimum low
- `close`: last raw bar close
- `volume`: sum of raw volume
- `time`: bucket open time

## Common Failure Pattern

If a script declares `interval 1d` and the raw file contains only 8 one-minute bars, CSV mode rejects the run because one full daily candle requires 1440 complete one-minute bars.

That failure belongs to data preparation, not compilation.
