# CLI コマンドリファレンス

PalmScript がオープンソース化されたため、このページは再び公開されました。完全な翻訳は後続の更新で追加します。それまでは、この言語版のサイトでも同じ公開 CLI / ツール内容を参照できるよう、下に英語の正規版を掲載します。

## English Canonical Content


# CLI Command Reference

This page is the compact public command reference for the `palmscript` CLI. For workflows and examples, see [CLI](../tooling/cli.md).

## `palmscript check`

```bash
palmscript check <script.ps>
```

Compiles and validates a script without executing it.

Arguments:

- `<script.ps>`: path to the PalmScript source file

## `palmscript docs`

```bash
palmscript docs [<topic>] [--list|--all]
```

Reads the embedded public English docs snapshot shipped inside the CLI binary.

Arguments and flags:

- `<topic>`: exact embedded docs topic path, for example `tooling/cli` or `reference/intervals-and-sources`
- `--list`: print every embedded topic with its title and relative docs path
- `--all`: print the full embedded English docs set in one terminal-friendly stream

Notes:

- if neither `<topic>` nor a flag is passed, the command prints usage plus the topic list
- `--list` is the best discovery mode before calling a specific topic
- the embedded docs are generated from `web/docs/docs/` at build time

## `palmscript run market`

```bash
palmscript run market <script.ps> --from <unix_ms> --to <unix_ms> \
  [--format json|text] \
  [--max-instructions-per-bar <N>] \
  [--max-history-capacity <N>]
```

Arguments and flags:

- `<script.ps>`: path to the PalmScript source file
- `--from <unix_ms>`: inclusive lower time bound in Unix milliseconds UTC
- `--to <unix_ms>`: exclusive upper time bound in Unix milliseconds UTC
- `--format json|text`: output rendering format, default `json`
- `--max-instructions-per-bar <N>`: VM instruction budget per step, default `10000`
- `--max-history-capacity <N>`: maximum retained history per series slot, default `1024`

Requirements:

- the script must declare at least one `source`
- `--from` must be strictly less than `--to`

## `palmscript run optimize`

```bash
palmscript run optimize <script.ps> --from <unix_ms> --to <unix_ms> \
  [--runner walk-forward|backtest] \
  [--train-bars <N>] \
  [--test-bars <N>] \
  [--step-bars <N>] \
  [--holdout-bars <N>] \
  [--no-holdout] \
  [--param int:name=low:high[:step]] \
  [--param float:name=low:high[:step]] \
  [--param choice:name=v1,v2,v3] \
  [--objective robust-return|total-return|ending-equity|return-over-drawdown] \
  [--trials <N>] \
  [--startup-trials <N>] \
  [--seed <N>] \
  [--workers <N>] \
  [--top <N>] \
  [--preset-out <path>] \
  [--format json|text]
```

Arguments and flags:

- `<script.ps>`: path to the PalmScript source file
- `--from <unix_ms>`: inclusive lower time bound in Unix milliseconds UTC
- `--to <unix_ms>`: exclusive upper time bound in Unix milliseconds UTC
- `--runner`: optimize evaluation mode; defaults to `walk-forward`
- `--train-bars <N>`: in-sample bars per walk-forward segment
- `--test-bars <N>`: out-of-sample bars per walk-forward segment
- `--step-bars <N>`: segment advance size; defaults to `test-bars`
- `--holdout-bars <N>`: reserve the final `N` execution bars as a final untouched holdout
- `--no-holdout`: explicitly disable the default untouched holdout reservation
- `--param ...`: search-space declaration; repeat for multiple tuned inputs, with optional integer/float step support
- `--objective ...`: ranking objective; defaults to `robust-return`
- `--trials <N>`: total bounded trial budget
- `--startup-trials <N>`: initial random trial count before the TPE search phase
- `--seed <N>`: deterministic optimizer seed
- `--workers <N>`: bounded parallel worker count
- `--top <N>`: number of top candidates to retain
- `--preset-out <path>`: write the best preset and top candidates to disk
- `--format json|text`: output rendering format; default `json`

Default safety behavior:

- `walk-forward` is the default optimizer runner
- when `walk-forward` is used, the CLI reserves a final untouched holdout automatically
- the default holdout size matches `test-bars`
- if `--param` is omitted, PalmScript first looks for preset parameter space and then infers search space from `input ... optimize(...)` metadata inside the script

## `palmscript dump-bytecode`

```bash
palmscript dump-bytecode <script.ps> [--format text|json]
```

Arguments and flags:

- `<script.ps>`: path to the PalmScript source file
- `--format text|json`: bytecode output format, default `text`

## Latest Diagnostics Additions

PalmScript now exposes richer machine-readable backtest diagnostics in every public locale build:

- `run backtest`, `run walk-forward`, and `run optimize` accept `--diagnostics summary|full-trace`
- summary mode keeps cohort, drawdown-path, source-alignment, holdout-drift, robustness, and hint data
- full-trace mode adds one typed per-bar decision trace per execution bar
- optimize output now includes top-candidate holdout checks plus parameter stability summaries

