# VS Code Extension

The first-party VS Code extension provides editor support for PalmScript source files.

- source extension: `.ps`

Marketplace identity:

- display name: `PalmScript`
- publisher: `journeytosilius`
- extension id: `journeytosilius.palmscript-vscode`
- marketplace icon: `editors/vscode/images/palmscript.png`

## Capabilities

- syntax highlighting
- snippets
- diagnostics
- hover
- completions
- callable completion snippets
- definitions
- document symbols
- formatting

The extension is intentionally thin. Language semantics, diagnostics,
completion data, callable completion snippets, and formatting come from
`palmscript-lsp` rather than a second parser or analyzer inside the extension.

## Language Server Resolution

The extension resolves `palmscript-lsp` in this order:

1. `palmscript.server.path`
2. bundled platform binary inside the extension
3. local development fallback in the repository `target/` directory

## Settings

- `palmscript.server.path`
- `palmscript.trace.server`

## Packaging

Release builds bundle platform-specific `palmscript-lsp` binaries under:

```text
server/<platform>-<arch>/palmscript-lsp
server/<platform>-<arch>/palmscript-lsp.exe
```

See [Release Workflows](../contributing/releases.md) for the publishing pipeline.
