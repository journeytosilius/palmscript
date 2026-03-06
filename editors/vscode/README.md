# TradeLang VS Code Extension

This extension provides first-party VS Code support for TradeLang strategy
files (`.trl`).

It launches the Rust `tradelang-lsp` binary from this repository and exposes:

- compiler-backed diagnostics
- hover information
- completions
- go-to-definition
- document symbols
- document formatting

## Development

From the repository root, build the language server first:

```bash
cargo build --bin tradelang-lsp
```

Then install extension dependencies and compile the extension:

```bash
cd editors/vscode
npm install
npm run compile
```

If you are developing inside this repository, the extension will fall back to
`../../target/debug/tradelang-lsp` or `../../target/release/tradelang-lsp`
automatically.

You can also point the extension at a specific binary with:

- `tradelang.server.path`

## Workspace Config

Composed strategies can declare editor-only compile environments through a
workspace `.tradelang.json` file:

```json
{
  "version": 1,
  "documents": {
    "strategies/consumer.trl": {
      "compile_environment": {
        "external_inputs": [
          {
            "name": "trend",
            "ty": "SeriesBool",
            "kind": "ExportSeries"
          }
        ]
      }
    }
  }
}
```

The extension watches that file and reloads diagnostics when it changes.

## Packaging

Release builds should place prebuilt `tradelang-lsp` binaries under:

```text
server/<platform>-<arch>/tradelang-lsp
server/<platform>-<arch>/tradelang-lsp.exe
```

The extension prefers those bundled binaries, then falls back to the local repo
build output during development.

To stage a locally built binary into the extension tree:

```bash
node ./scripts/install-server-bundle.mjs --platform linux-x64 --binary ../../target/release/tradelang-lsp
```

The extension package step validates that all required bundled binaries are
present:

```bash
npm run verify:server
npm run package:vsix
```

## Publishing

This repository includes a GitHub Actions workflow at
`.github/workflows/publish-vscode-extension.yml`.

It:

1. builds `tradelang-lsp` for the supported release platforms
2. stages the binaries under `server/<platform>-<arch>/`
3. packages the extension as a `.vsix`
4. publishes it on tags matching `v*`

Required secret:

- `VSCE_PAT`: Visual Studio Marketplace personal access token with Marketplace
  manage scope
