# neowu/editor (upstream)

- upstream forked repo: https://github.com/neowu/editor.git sync everytime before your work, this is mainly where the
  backend (GPUI rendering etc...) inspired from. Drop personal unused/unnecessary, only keep
  performance optimization changes

## How to sync

- `git fetch upstream`, then fast-forward `neo` to `upstream/neo` (`neo` is a read-only mirror) and merge `neo` into
  `intellied`. Push to `origin` only; never push to or open a PR against upstream.
- Branch rules, porting steps and the last reviewed upstream commit: `spec/upstream.md`.

## Keep / drop

- Keep: GPUI rendering and performance work, editor core, LSP and git correctness fixes.
- Drop: upstream's personal look and workflow (theme, layout, keymap, modes) where intelliED follows IntelliJ /
  OpenIDEA instead, and features this project doesn't use.
