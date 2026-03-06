# CLI Command Reference

## `palmscript check`

```bash
palmscript check <script.palm> [--env <compile-env.json>]
```

Compiles and validates a strategy without running it.

## `palmscript run csv`

```bash
palmscript run csv <script.palm> --bars <bars.csv> \
  [--format json|text] \
  [--max-instructions-per-bar N] \
  [--max-history-capacity N]
```

Runs a strategy in CSV mode. The input file is treated as the raw source feed and rolled up to declared intervals if possible.

## `palmscript dump-bytecode`

```bash
palmscript dump-bytecode <script.palm> \
  [--env <compile-env.json>] \
  [--format text|json]
```

Compiles a strategy and prints the program in text or JSON form.
