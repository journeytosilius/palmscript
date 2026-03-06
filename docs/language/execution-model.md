# Execution Model

PalmScript scripts compile once and execute once per fully closed base-interval candle.

## Per-Bar Execution

For each base bar:

1. base market series are loaded
2. referenced higher or equal intervals are advanced up to the current fully closed base boundary
3. bytecode executes
4. bounded series state is updated
5. outputs are emitted for the current bar

## Determinism

PalmScript execution is deterministic:

- no filesystem access
- no network access
- no system time access
- no randomness

The same compiled program and the same input bars produce the same outputs.

## Base Interval Ownership

Every script must declare exactly one base interval:

```palmscript
interval 1m
```

Legacy source-less scripts use unqualified series like `close` and `volume` for the current candle of that declared base interval.

Source-aware scripts declare named markets:

```palmscript
interval 1m
source bn = binance.spot("BTCUSDT")
source hl = hyperliquid.perps("BTC")
plot(bn.close - hl.close)
```

For those scripts:

- the script still has one global execution interval
- the runtime builds the base clock from the union of all declared-source base-interval bar opens
- if one source has no base bar on a clock step, that source contributes `na` for that step
- slower source-qualified intervals such as `hl.1h.close` update only when their candles fully close

## Output Timing

`plot`, `export`, and `trigger` all materialize per-bar outputs after the current instruction stream finishes. Triggers also emit discrete trigger events when their current sample is `true`.
