# PalmScript VS Code Extension

This file is a short repository-local development note for the VS Code extension.

The public PalmScript documentation site is language-focused and does not publish editor-stack internals. Private repo-only notes now live under:

- `../../docs-private/tooling/vscode.md`
- `../../docs-private/tooling/language-server.md`
- `../../docs-private/contributing/releases.md`

## Development

From the repository root:

```bash
cargo build --bin palmscript-lsp
cd editors/vscode
npm install
npm run compile
```

The extension resolves the language server in this order:

1. `palmscript.server.path`
2. bundled binary in `server/<platform>-<arch>/`
3. local repo fallback in `../../target/debug/` or `../../target/release/`

## Packaging

```bash
npm run verify:server
npm run package:vsix
```
