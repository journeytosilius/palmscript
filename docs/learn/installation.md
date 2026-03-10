# Installation

PalmScript is used through the `palmscript` CLI.

Before following the Learn examples, make sure:

- `palmscript` is installed
- the binary is available on your `PATH`

Verify it:

```bash
palmscript --help
```

The examples in this documentation assume commands are run as:

```bash
palmscript check strategy.palm
palmscript run market strategy.palm --from 1704067200000 --to 1704153600000
```

If you are working from a local development checkout, use the build and packaging workflow provided by your environment to make the `palmscript` binary available before continuing.
