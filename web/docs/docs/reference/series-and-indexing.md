# Series and Indexing

Series values represent time-indexed samples with bounded history.

## Market Series Forms

PalmScript exposes market series through source-qualified forms only:

```palmscript
bn.close
bb.1h.close
```

Rules:

- `<alias>.<field>` refers to that source on the script base interval
- `<alias>.<interval>.<field>` refers to that source on the named interval
- bare market identifiers such as `close` are rejected
- higher source interval references require a matching `use <alias> <interval>` declaration

## Current-Sample Semantics

When a series is used without indexing, the expression evaluates to the current sample of that series on the current execution step.

## Indexing

Indexing has the form:

```palmscript
x[n]
```

Rules:

- `n` must be a non-negative integer literal
- dynamic indexing is rejected
- only series values may be indexed
- `x[0]` refers to the current sample
- `x[1]` refers to the previous sample
- `x[n]` refers to the sample `n` updates ago on that series' own update clock

If insufficient history exists, the indexed expression evaluates to `na`.

## Update Clock Ownership

Every series advances on its own update clock.

Examples:

- `bn.close[1]` follows the base interval
- `bb.1h.close[1]` follows source `bb` on the one-hour clock

Derived series inherit the update clocks of their inputs. A slower series is not re-counted on faster clocks when it has not advanced.

## Missing Samples

Series may produce `na` for the current sample when:

- there is insufficient history
- the source feed is missing on a base-clock step from the union of declared-source base timestamps
- the series is a higher-interval feed that has not yet closed once
- an indicator is still warming up

## Time Series

`time` is a numeric series whose sample is the candle open time in Unix milliseconds UTC.

Rules:

- base `time` exposes the base-interval candle open time
- higher-interval `time` exposes that higher-interval candle open time
- source-qualified `time` follows the same source and interval selection rules as the price and volume fields
