---
icon: lucide/code
---

# Editor Integration

!!! note "Coming Soon"

    Editor integration is planned for a future release. This page will be updated with instructions as support is added.

## Planned Integrations

### Language Server Protocol (LSP)

An LSP server for mlt is on the roadmap. Once available, it will provide:

- Real-time diagnostics as you type
- Quick-fix code actions
- Integration with any LSP-compatible editor

### VS Code

A VS Code extension is planned that will wrap the mlt LSP server and provide:

- Inline diagnostic highlighting
- One-click auto-fix via code actions
- Configuration snippets for `.mlt.toml`

### Neovim

Once the LSP server is available, Neovim users can configure it via `nvim-lspconfig` or similar plugins.

### MATLAB Editor

Integration with the built-in MATLAB editor is being explored.

## Current Workaround

Until native editor integration is available, you can use mlt as an external tool:

1. Configure your editor to run `mlt <current-file>` on save
2. Parse the output format (`file:line:column [severity] rule: message`) for diagnostics
3. Many editors support this via "problem matcher" patterns

## Contributing

If you'd like to help build editor integrations, see the [repository](https://github.com/watermarkhu/mlt) for contribution guidelines.
