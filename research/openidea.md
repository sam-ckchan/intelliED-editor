# OpenIDEA (UI reference)

- OpenIDEA IDE, https://github.com/l0ng-ai/openidea, refer to its UI layout and install language server as you go (
  pop-up), this is the mainly where the intellij UI you get inspired from.

It is a Tauri app (React/CSS frontend), so nothing is merged; designs are
re-implemented in GPUI.

## Last referenced

- Commit `de9c8db` (2026-11-12, 15:49).
- Next pass: diff from that commit to the latest and port only what changed
  in the files below:

  ```sh
  git clone https://github.com/l0ng-ai/openidea.git /tmp/openidea
  cd /tmp/openidea
  git diff de9c8db..HEAD -- src/styles/tokens.css src/ui/Tabs.module.css \
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
- `src/workbench/projectColor.ts` and `--project-color-N` tokens: project
  colour slot (Java string hash of the name mod 9), the initials badge and the
  toolbar wash (`.framed` in `Workbench.module.css`).
- `ToolWindows.module.css` `.stripeWithNames`: tool window names on the
  stripes (here a View toggle instead of dragging the stripe edge).
- `--bg-tab-selected-inactive` / `--border-tab-selected-inactive`: the gray
  selected tab of an unfocused pane.
- `Workbench.tsx` navigation bar (default position `statusBar`): breadcrumbs
  at the start of the status bar.
- `[data-theme="light"]` and `[data-theme="dark-hc"]`: Islands Light and the
  high-contrast dark theme, chosen at launch from the macOS appearance.
- `src/features/tools/missing.ts`: the "<server> is not installed — Install"
  notification.

## Not taken

- Undocking tool windows, distraction-free mode, LightEdit, the welcome
  frame, and everything outside the UI shell (plugins, debugger, database,
  HTTP client, settings UI).
