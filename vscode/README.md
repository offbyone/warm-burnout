# Warm Burnout for VS Code

The VS Code implementation of [Warm Burnout](../README.md). Two variants: **Warm Burnout Dark** and **Warm Burnout Light**.

For the design philosophy, eye safety audit, comfort scores, and full palette reference, see the [suite README](../README.md).

## Installation

### VS Code Marketplace

TODO -- not yet published.

### From source (development)

```bash
git clone <repository-url>
cd patina-vscode-theme/vscode
```

Open in VS Code, press **F5**, then select **Warm Burnout Dark** or **Warm Burnout Light** with `Cmd+K Cmd+T` (or `Ctrl+K Ctrl+T`).

### From VSIX (local install)

```bash
cd patina-vscode-theme/vscode
npx @vscode/vsce package
code --install-extension warm-burnout-*.vsix
```

## Languages Tested

TypeScript, JavaScript, HTML, JSX, CSS, SCSS, Python, Rust, Go, Java, YAML, JSON, Markdown, Diff.

## License

[MIT](../LICENSE)
