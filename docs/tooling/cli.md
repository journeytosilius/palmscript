# CLI

The first-party command-line entrypoint is `palmscript`.

Use this page for workflows and examples. Use [CLI Command Reference](../reference/cli.md) for the exact command and flag surface.

## Common Workflow

Typical development flow:

1. validate a strategy with `palmscript check`
2. run it in `market` mode
3. inspect outputs in `json` or `text`
4. inspect the compiled form with `palmscript dump-bytecode` when debugging semantics

## Validate Without Running

```bash
palmscript check examples/strategies/sma_cross.palm
```

This compiles the script and reports source diagnostics without executing it.

## Run In Market Mode

```bash
palmscript run market strategy.palm \
  --from 1704067200000 \
  --to 1704153600000
```

Use market mode when:

- the script declares one or more `source` directives
- you want PalmScript to fetch each required base or supplemental feed directly from supported exchanges

Market mode compiles the script, resolves the required source-qualified feeds, validates venue-specific guardrails, fetches candles for each required `(source, interval)`, constructs the source-aware runtime inputs, runs the VM on the union of base timestamps, and prints outputs.

See [Market Mode](market-mode.md) for supported templates and fetch behavior.

## Output Formats

Market mode supports:

- `--format json`
- `--format text`

`json` is the default.

## Execution Limits

Market mode supports:

- `--max-instructions-per-bar`
- `--max-history-capacity`

Use these when testing pathological scripts or when tightening deterministic operational bounds.

## Inspect Bytecode

```bash
palmscript dump-bytecode examples/strategies/sma_cross.palm
palmscript dump-bytecode examples/strategies/sma_cross.palm --format json
```

This prints the compiled program rather than executing it.
