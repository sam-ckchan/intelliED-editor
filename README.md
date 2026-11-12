# intelliED

A lightweight IntelliJ-style IDE for macOS, written in Rust: IntelliJ's layout,
Dark theme and macOS keymap on a GPUI editor core. See `AGENTS.md` and `spec/`.

```sh
cargo run -- /path/to/project   # run a debug build
./release.sh && ./install.sh    # build and install /Applications/intelliED.app
```

## Install LSPs manually, or let npx/uvx run them

The editor also offers to install missing servers from a pop-up (rust-analyzer
and anything brew-installable).

```sh
# json, css, html
npm install -g vscode-langservers-extracted
# typescript / javascript: TypeScript 7 (`tsc --lsp`), from the project's node_modules
npm install -D typescript@^7
# lint (js/ts/json/css) and formatting
npm install -D @biomejs/biome

# python
uv tool install ruff
uv tool install ty

# terraform
sudo apt install terraform-ls
```
