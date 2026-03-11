# Release Workflows

## VS Code Extension Publishing

The repository includes `.github/workflows/publish-vscode-extension.yml`.

That workflow:

1. builds `palmscript-lsp` for the supported platforms
2. stages the binaries under `editors/vscode/server/<platform>-<arch>/`
3. packages the extension as a `.vsix`, including marketplace metadata from `editors/vscode/package.json` such as the icon asset
4. publishes it to the Visual Studio Marketplace on tags matching `v*`

Required secret:

- `VSCE_PAT`

## Documentation Publishing

The repository also publishes the MkDocs site to GitHub Pages from `main`.

Documentation deploys are separate from extension publishing so docs updates can ship continuously without bundling a VS Code release.
