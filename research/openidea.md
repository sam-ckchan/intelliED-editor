# OpenIDEA (UI reference)

Repo: https://github.com/l0ng-ai/openidea. It's the reference for intelliED's
IntelliJ look: layout, theme, tool windows, keymap habits. It is a Tauri app
(React/CSS frontend), so nothing is merged; designs are re-implemented in
GPUI.

## Last referenced

- Commit `e1f41093ec8e9777639a2b78377a443088a373f4` (2026-11-12).
- Next pass: diff from that commit to the latest and port only what changed
  in the files below:

  ```sh
  git clone https://github.com/l0ng-ai/openidea.git /tmp/openidea
  cd /tmp/openidea
  git diff e1f4109..HEAD -- src/styles/tokens.css src/ui/Tabs.module.css \
      src/workbench/toolwindows/ToolWindows.module.css \
      src/workbench/TitleBar.module.css src/workbench/StatusBar.module.css
  ```

  Afterwards, update the commit above.
- Don't keep a long-lived clone. Re-clone into `/tmp`, or read single files
  from GitHub, so the reference is always the latest.

## What was taken, and from where

- `src/styles/tokens.css` (`:root`, Islands Dark): every colour in
  `crates/theme/src/built_in.rs`. That covers the frame and island surfaces,
  text, accent and link, the selected tab and tree row, file status colours,
  the editor scheme and the terminal ANSI palette. It also gave the metrics:
  `--island-gap` 6px, `--island-radius` 10px, `--h-titlebar` 40px,
  `--h-header` 40px, `--h-tab` 28px, `--radius-tab` 6px,
  `--stripe-icon-button` 30px, `--stripe-outer-padding` 4px and
  `--h-statusbar` 30px.
- `src/ui/Tabs.module.css`: pill tabs in the editor island's top row. The
  selected tab has a fill plus a 1px inset outline. There is no underline and
  no separator.
- `src/workbench/toolwindows/ToolWindows.module.css`:
  - tool windows are islands on the frame, separated by gaps;
  - stripes sit on the frame, transparent;
  - stripe buttons have 6px corners and a white wash when active;
  - each tool window has a 40px title header.
- `src/workbench/TitleBar.module.css`: the main toolbar sits on the frame with
  no divider and leaves room on the left for the traffic lights.
- `src/workbench/StatusBar.module.css`: the status bar sits on the frame with
  no divider.

## Not taken yet

Candidates if wanted:

- the project colour wash in the top-left corner (`.framed` in
  `Workbench.module.css`);
- stripe button labels (`.stripeWithNames`);
- the navigation bar / breadcrumbs;
- the inactive-pane selected tab colours (`--bg-tab-selected-inactive`);
- Islands Light and the high-contrast themes;
- the language server install pop-up.
