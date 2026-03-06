# Market Mode

`palmscript run market ...` executes source-aware scripts by fetching historical candles directly from supported exchange REST APIs.

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

Run it with:

```bash
palmscript run market strategy.palm --from 1704067200000 --to 1704153600000
```

## Fetch Model

Market mode:

- reads declared `source` directives from the script
- fetches each required `(source, interval)` directly from the source venue
- converts each venue response into the canonical bar schema `time,open,high,low,close,volume`
- executes the existing VM over a source-aware runtime configuration

The CLI flags `--from` and `--to` define the historical window in Unix milliseconds UTC.

## Base Clock

Source-aware scripts still run on one declared `interval`.

The runtime builds that execution clock from the union of all declared-source base-interval bar opens. On a step where one source has no base candle:

- that source's base fields become `na`
- other sources still advance normally
- slower declared intervals keep their last fully closed value until their next close boundary

## Failure Cases

Market mode fails deterministically for:

- scripts with no `source` declarations
- unsupported source templates
- source intervals referenced without matching `use <alias> <interval>`
- malformed venue responses
- empty historical windows
- no returned candles for a required source feed
