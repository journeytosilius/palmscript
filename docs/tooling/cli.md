# CLI

The official CLI binary is `palmscript`.

## Commands

- `palmscript run csv <script.palm> --bars <bars.csv>`
- `palmscript run market <script.palm> --from <unix_ms> --to <unix_ms>`
- `palmscript check <script.palm>`
- `palmscript dump-bytecode <script.palm>`

## `run csv`

Executes a strategy in CSV mode:

```bash
palmscript run csv examples/strategies/sma_cross.palm \
  --bars examples/data/minute_bars.csv
```

Options:

- `--format json|text`
- `--max-instructions-per-bar <N>`
- `--max-history-capacity <N>`

The command:

1. loads source
2. compiles it
3. loads the raw CSV bars
4. infers the raw interval
5. rolls bars into the declared `interval` and `use` intervals
6. runs the existing runtime
7. prints structured outputs

## `run market`

Executes a source-aware strategy by fetching historical candles from supported exchange REST APIs:

```bash
palmscript run market strategy.palm \
  --from 1704067200000 \
  --to 1704153600000
```

Options:

- `--format json|text`
- `--max-instructions-per-bar <N>`
- `--max-history-capacity <N>`

The command:

1. loads source
2. compiles it
3. reads declared `source` directives
4. fetches each required `(source, interval)` directly from the venue
5. builds a source-aware runtime config
6. runs the runtime on the union base clock
7. prints structured outputs

## `check`

Validates source without running it:

```bash
palmscript check strategy.palm
```

## `dump-bytecode`

Compiles and renders the compiled program:

```bash
palmscript dump-bytecode strategy.palm
palmscript dump-bytecode strategy.palm --format json
```
