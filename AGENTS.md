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
- run `python3 test/ui_layout.py` before committing any UI change, and click through the changed controls; a
  screenshot of the default state is not a test
- test in the background: `edctl launch` hides the window by default. Use `--foreground` only for checks that need an
  active window (focus, cursor blink) or perf / memory measurements, and say so first
- drive the app on a scratch project under `/tmp`, never on this repository (typed text is autosaved into real files)
- don't run `./launch.sh` unless asked: it quits the running intelliED and reopens it in front

# git

- `neo` is a read-only mirror of `upstream/neo`: only fast-forward it, never commit to it
- all work goes on `intellied`; to take upstream changes, fast-forward `neo`, then merge it into `intellied`
- push to `origin` only; never push to `upstream` or open a pull request against it

# research

- at /research dir, investigate what should deprecate from the remote upstream (https://github.com/neowu/editor.git), or review open source repo that could utilize in this project

- upstream forked repo: https://github.com/neowu/editor.git sync everytime before your work, this is mainly where the
  backend (GPUI rendering etc...) inspired from. Drop personal unused/unnecessary, only keep
  performance optimization changes

- OpenIDEA IDE, https://github.com/l0ng-ai/openidea, refer to its UI layout and install language server as you go (
  pop-up), this is the mainly where the intellij UI you get inspired from.
