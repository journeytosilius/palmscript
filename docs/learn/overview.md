# Learn PalmScript

PalmScript has three user-facing layers:

- the language for writing strategies
- the `palmscript` CLI for checking and running them
- the editor stack built from `palmscript-lsp` and the VS Code extension

## What You Do With PalmScript

Typical workflow:

1. write a `.palm` strategy
2. declare a base `interval`
3. use either CSV-backed data or exchange-backed `source` declarations
4. validate with `palmscript check`
5. run with `palmscript run csv ...` or `palmscript run market ...`

## What To Read Next

- First runnable flow: [Quickstart](quickstart.md)
- First complete strategy walkthrough: [First Strategy](first-strategy.md)
- Big-picture language tour: [Language Overview](language-overview.md)
- Exact rules and semantics: [Reference Overview](../reference/overview.md)

## Documentation Roles

- `Learn` explains how to use PalmScript effectively.
- `Reference` defines what PalmScript means.
- `Tooling` explains commands, modes, and editor behavior.
- `Internals` explains how the implementation is built.
