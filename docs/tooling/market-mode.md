# Market Mode

`palmscript run market ...` executes source-aware scripts by fetching historical candles directly from supported exchange REST APIs.

## Invocation

```bash
palmscript run market strategy.palm --from 1704067200000 --to 1704153600000
```

Rules:

- the script must declare at least one `source`
- `--from` and `--to` are Unix milliseconds UTC
- `--from` must be strictly less than `--to`

## Supported Source Templates

- `binance.spot("<symbol>")`
- `binance.usdm("<symbol>")`
- `hyperliquid.spot("<symbol>")`
- `hyperliquid.perps("<symbol>")`

Example:

```palmscript
interval 1m
source bn = binance.spot("BTCUSDT")
source hl = hyperliquid.perps("BTC")
use hl 1h

plot(bn.close - hl.close)
plot(hl.1h.close)
```

## Fetch Model

Market mode:

- reads declared `source` directives from the script
- determines the required `(source, interval)` feeds from the compiled program
- fetches each required feed directly from the venue
- converts venue responses into the canonical bar schema `time,open,high,low,close,volume`
- runs the VM over the resulting source-aware runtime configuration

## Base Clock

Source-aware scripts still execute on one declared base interval.

The runtime builds that execution clock from the union of all declared-source base-interval bar open times.

On a step where one source has no base candle:

- that source's base fields become `na`
- the other sources still contribute their own current samples
- slower declared intervals keep their last fully closed values until their next close boundary

## Failure Cases

Market mode fails deterministically for:

- scripts with no `source` declarations
- invalid time windows
- request failures
- malformed venue responses
- unresolved Hyperliquid spot symbols
- empty historical windows for a required feed
