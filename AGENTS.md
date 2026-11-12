# Overview

- this is personal IDE in rust used in MacOS

# Problem to solve

- Been using intellij fallback version (2025.1.7.2) for all daily code editing and its inherit feature like (http
  client, DB GUI etc...), but as a Java-written IDE its consume too much memory in running and slow startup.

# Requirement

- for UI, keep intellij / OpenIDEA layout, config, keymap as possible

- keep minimalist and lightweight, build only the feature i need, avoid lib/dependency possible

# launch

- `./launch.sh [path]`: builds the latest release (`release.sh`), quits a running intelliED, installs it
  (`install.sh`, replacing `/Applications/intelliED.app`) and opens it, optionally on `path`
- `cargo run -- <path>`: debug build straight from the repo, nothing installed
- each release/install overwrites the previous build and app; there is no version history

# code style

- Keep comments minimal and focused on intent or non-obvious behaviour
- Prefer straightforward code over unnecessary encapsulation and abstraction
- Format Rust code with `cargo +nightly fmt --all`
- Fix all warnings reported by `cargo clippy --workspace --all-targets`

# spec

- update `/spec` to reflect what code and design changed
- for spec doc, only keep key design decisions / behaviour / requirements, always goes to code for impl details

# unit test

- add unit tests for every function added or changed

# automation ui test

- follow `spec/automation.md`

# research

- at /research dir, investigate what should deprecate from the remote upstream and what should keep in this project

- upstream forked repo: https://github.com/neowu/editor.git sync everytime before your work, this is mainly where the
  backend (GPUI rendering etc...) inspired from. Drop personal unused/unnecessary, only keep
  performance optimization changes

- OpenIDEA IDE, https://github.com/l0ng-ai/openidea, refer to its UI layout and install language server as you go (
  pop-up), this is the mainly where the intellij UI you get inspired from.
