# UI CLI

A CLI tool to add **rust-ui** components to your Leptos project.

## Installation

```bash
cargo install ui-cli --force
```

## Commands

```bash
ui starters                  # clone a starter project
ui init                      # set up an existing project
ui add                       # interactive component picker
ui add button card           # add components directly
ui add button --dry-run      # preview without installing
ui add button -y             # skip overwrite prompt
ui list                      # browse all components
ui search <query>            # search components by name
ui view <name>               # view a component's source
ui diff                      # compare installed vs registry
ui update                    # check for outdated components
ui mcp                       # start the MCP server (for AI editors)
ui mcp init --client claude  # write editor config for MCP
```

## MCP (AI Editor Integration)

Run `ui mcp init --client <claude|cursor|vscode|opencode>` once in your project.
Your editor will auto-connect and can then browse, search, and add components for you.

## Links

- Docs & components: [rust-ui.com](https://www.rust-ui.com)
- Changelog: [CHANGELOG.md](./CHANGELOG.md)

## License

MIT — see [LICENSE](./LICENSE).
