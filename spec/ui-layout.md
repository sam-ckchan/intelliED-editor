# UI layout

## Decision

The window follows OpenIDEA (https://github.com/l0ng-ai/openidea), which
copies IntelliJ / PyCharm 2026.1's "Islands" UI: a main toolbar across the
top, tool window stripes at the left and right edges, and the editors and tool
windows as rounded islands separated by gaps on a darker frame, with pill tabs
inside the editor island and a status bar. Colours and metrics come from
OpenIDEA's design tokens (`src/styles/tokens.css`). It is always on, with no
setting. A flat IntelliJ New UI layout (hairlines, underlined tabs) was tried
first and dropped: it read like the old fork rather than OpenIDEA.

## Theme

- Three built-in themes, OpenIDEA's: Islands Dark (`:root` in its
  `tokens.css`), Islands Light (`[data-theme="light"]`) and High Contrast
  (`[data-theme="dark-hc"]`), built in Rust in `theme::built_in`. There is no
  theme JSON, no refinement or fallback derivation, and no `theme_settings`
  crate. To change colours, edit `theme::built_in` and rebuild.
- The variant is chosen once at launch from macOS, as there is no settings UI:
  light appearance → Islands Light; dark with Accessibility "Increase
  contrast" on → High Contrast; otherwise Islands Dark. `ED_THEME=light|dark|
  dark-hc` overrides the choice (for testing; an unknown value is logged and
  ignored). The theme does not follow appearance or contrast changes while
  running; a restart picks them up. Native window chrome is forced to the
  chosen theme's appearance.
- `theme::built_in` is the single origin of every colour: one `Palette` per
  variant (the only raw hex values, fields named after OpenIDEA's tokens:
  surfaces, text, accents, file status, the editor scheme, the terminal's ANSI
  colours) and one set of builders (`theme_colors(&Palette)`, …) that map
  palette fields to theme fields, so the variants cannot drift in which role
  a colour plays. `shade` scales RGB channels; alpha comes from `Hsla::alpha`
  or `0xRRGGBBAA` palette values. No theme field holds a colour from outside
  the palette (except the macOS traffic lights and three accent hues).
- Surfaces: the frame is the window background, the main toolbar, the
  stripes and the status bar; islands are editors with their tab rows, tool
  windows and terminals (dark: frame `#26282C`, islands `#191A1C`; light:
  `#E9EAEE` / white; high contrast: `#1A1B1E` / black). Dark islands sit
  below the frame's lightness, light ones above. The accent fill is `--accent`
  (fills, focus borders), accent text is `--link`. A showing tool window's
  stripe button is `--bg-stripe-selected`, a translucent white (dark) or black
  (light) wash.
- Syntax follows IntelliJ's schemes, which colour declarations and literals,
  not uses (types, variables, operators and punctuation stay plain text). Each
  variant has its own scheme from the token block (`--fg-editor`, `--syn-*`);
  Islands Light italicises comments (`--syn-comment-style`).
- File status colours follow IntelliJ: added green, modified and renamed blue,
  ignored orange, deleted gray, conflicting red (per-variant shades).
- Project colours (`--project-color-N`) are the same in every variant.
- The theme is built once into a `OnceLock`: `built_in::init(variant)` (called
  first thing in `ed`'s `app.run`, before anything reads the theme) sets it,
  and `built_in::theme()` / `cx.theme()` return `&'static Theme`, defaulting to
  Islands Dark if `init` wasn't called (tests). The first of the two wins.
  There is no theme global and no `Arc` around the theme, the syntax theme or
  the accents (a fixed array); holders keep a `&'static SyntaxTheme`.
- Terminal dim ANSI colours are the normal colour shaded -30%; bright ones are
  the palette's own.
- `StatusColors` only has the fields the UI reads; unused `_background` /
  `_border` variants and `predictive` are gone.
- There are no Vim/Helix theme colours (`vim_*`).
- There is no theme registry, theme picker, light/dark mode toggle, `Settings`
  app-menu submenu, or theme settings (`theme`, `markdown_preview.theme`,
  `theme_overrides`, `experimental.theme_overrides`).
- The theme never changes at runtime, so nothing observes theme changes.

## Fonts

- Fonts are constants in `ui::font`, not settings: UI font `.SystemUIFont`
  16px with ligatures off, buffer font `JetBrainsMono Nerd Font` 15px, both
  weight 400. Mermaid diagrams use the buffer font family. To change them, edit
  `ui::font` and rebuild.
- Font families never change at runtime; only sizes change, through in-memory
  zoom that resets on restart. `intellied::{Increase,Decrease,Reset}UiFontSize`,
  `intellied::{Increase,Decrease,Reset}BufferFontSize`, and `intellied::ResetAllZoom` take
  no arguments and never write settings. Sizes clamp to 6–100px.
- The markdown preview (default: UI font size) and the git commit editor
  (default: 12px) zoom independently of the buffer when focused.
- `ui` reads fonts directly; `theme` provides no font settings.

## Behaviour

- The window is, top to bottom: the main toolbar, then the left stripe, the
  shown mode's content and the right stripe side by side, then the status bar.
  The toolbar, stripes and status bar (plus notifications, toasts and modals)
  are drawn outside the modes, so they stay put when the mode changes.
- Main toolbar (`workspace::window_header`): 40px high, full width,
  on the frame, with no divider. Left to right: room for the three macOS traffic
  lights (on the toolbar's centre line, at (13, 13)), the project badge (the
  project's initials on its colour gradient, IntelliJ's `AvatarUtils.initials`),
  the project name (the active folder, see `git.md`) and the branch picker
  (only for a project in a git repository), each with a dropdown arrow; at
  the right end, Search Everywhere (the file finder) and Find
  Action (the command palette). Its empty space is the titlebar: dragging
  moves the window and a double click zooms or minimises it as macOS is set
  up to. Tab bars are not titlebar space. Fullscreen hides the traffic lights
  and the toolbar's content starts at its left padding.
  The project colour (one of IntelliJ's nine, chosen by a Java string hash of
  the first folder's name, `theme::built_in::project_color_index`) also washes
  the toolbar from its left edge: 22% strength fading out over 600px, so each
  project's window is recognisable.
  The worktree picker shows before the branch (`worktree / branch`) only in a
  linked worktree or while one is being created; in the main worktree it is
  reached through `git::Worktree`. The branch has a plain branch icon. There
  are no Go Back / Go Forward buttons (`cmd-[` / `cmd-]`). Toolbar and tab
  bar icon buttons are a 16px icon in a 28px square
  (`workspace::pane::top_row_icon_button`).
- Tool window stripes (`workspace::stripe`): icon columns on the frame, with
  each tool window's name under its icon by default, as in OpenIDEA: 72×48px
  buttons with a 20px icon, in a 76px stripe (a 4px outer padding; the island
  gap is on the inner side). `workspace::ToggleToolWindowNames` (View › Tool
  Window Names) switches to icon-only 30px buttons in a 34px stripe, for the
  session. The left stripe holds the left dock's tool windows at its top
  (Project, Commit, Structure) and, at its bottom, Git (Git History mode,
  only with a repository), the bottom dock's Terminal, and Problems (opens
  project diagnostics). A right stripe shows only when a tool window sits in
  the right dock; none does now. A button is muted with 6px corners; while
  its tool window is showing it gets a light wash and a full-contrast icon.
  A click shows and focuses the tool window, or hides it when it is already
  showing (`Workspace::toggle_tool_window`). Docks belong to Editor mode, so
  a click in another mode switches to Editor mode first and always shows the
  tool window. The Git button toggles between Git History and Editor mode.
  The dock buttons moved here from the status bar.
- Islands (`ISLAND_GAP` 6px, `ISLAND_RADIUS` 10px, in `ui::utils`): every
  editor pane, the side tool windows and each terminal pane is a rounded
  island in the island colour, 6px from its neighbours, the stripes and the
  window's bottom edge (or the status bar). GPUI clips to rectangles only, so
  an island insets its content by `ISLAND_INSET` (3px) and paints an
  island-coloured ring over it whose inner edge is concentric with the
  island; anything painted into a content corner is trimmed to the curve by
  that one mask. If the radius grows, the inset must grow so that
  inset >= radius × (1 − 1/√2).
- A side tool window island starts with a 34px header: its title (Project,
  Commit, Structure: the panel's `icon_tooltip`) and a Hide button at the end. The bottom dock
  holds panes, which draw their own islands.
- Tabs are pills inside the editor island's 40px top row: 28px high, 6px
  corners, 10px padding, 2px apart, every label at full contrast (or its git
  status colour). The selected tab is filled and outlined in blue in the
  focused pane, and gray (the frame colour, outlined in the strong border
  gray) in other panes; others
  show the hover wash under the mouse. A dragged tab is filled with the popup
  colour. An empty pane's island shows no tab row.
- All three traffic lights are shown, spaced as AppKit lays them out.
- The status bar has no worktree or branch pickers; they are in the main
  toolbar.
- Modes are strictly isolated for rendering. Only the main toolbar, the stripes and the status bar
  (plus notifications, toasts and modals) are drawn outside the modes; everything between them
  belongs to one mode, rendered by that mode's own cached view, and only the
  shown mode's view is drawn. Changes in a hidden mode redraw nothing (it
  re-renders when shown again), and redraws of the shared chrome reuse the
  shown mode's layout. A new mode adds its own view and module rather than
  branching inside another mode's layout.
- In Git History mode (`git.md`, `cmd-9`, the Git stripe button) the history
  sidebar and panes fill the area between the stripes. The status bar is
  hidden there. The sidebar shows the active history item's `Item::side_card`
  and is styled like a left dock island (resizable by a 6px handle in the gap). `cmd-9` again returns to Editor mode.
- Terminal mode (`alt-shift-f12`, pressed again to leave) fills the area of the docks and center panes with
  terminal panes, split like editor panes; the status bar is hidden, as in Git
  History. The workspace owns these panes (like Git History's), apart from
  the terminal panel's; the terminal crate plugs in through
  `TerminalModeDelegate` (pane setup, opening shells, reading a terminal's
  directory), since it depends on `workspace`.
  - The two terminal sets have separate jobs: terminal mode for work in
    shells, the bottom dock (`alt-f12`) for a quick shell and task output beside
    the code. Tasks always run in the dock.
  - Showing the mode with no terminals starts a shell, except that a repeated
    press of its key while it is shown leaves the mode instead. Closing the last terminal keeps one
    empty pane offering "New Terminal" (`EmptyPaneHints::Terminal`) instead of
    leaving the mode. The first time the mode is shown in a window it restores
    the saved layout instead (`configuration.md`).
  - Tab bars are the exception to
    "no split buttons": `+` (new shell in the project's default directory),
    Split Right (`cmd-d`), Split Down (`cmd-shift-d`), then Zoom. A split
    (button, `pane::Split*`) opens
    a new pane with a shell in the active terminal's working directory,
    focused, on the side asked for; only a move split of a pane with another
    tab left behind moves the tab instead. Dragging a tab to an edge moves it,
    as elsewhere.
  - `workspace::NewTerminal` opens in terminal mode's active pane while that
    mode is shown, else in the terminal panel. "Open in Terminal"
    (`workspace::OpenTerminal`, from the project panel, outline panel, tab and
    file header menus) opens a shell in that directory in terminal mode and
    switches to it.
  - Pane navigation (`ActivatePane*`, next/previous pane) stays among the
    terminal panes; Zoom expands one terminal pane over the others.
- There are no terminals in editor panes: the `NewCenterTerminal` action is
  gone, and editor panes refuse dropped terminal tabs, so a terminal never
  lands among files.
- Docks have no border. Side tool windows are islands; panes in the bottom
  dock (the terminal) draw their own islands.
- Split panes are sections of one set of tabs, not separate editors: a tab
  (a file, a commit, a revision) lives in exactly one pane, and the same file is
  never open in two panes. Opening something already open reveals and focuses
  its tab wherever it is; the pane passed by the caller (or the active pane)
  only decides where new tabs go. "Open to the side" on an open file reveals it
  rather than splitting. Splitting (`pane::Split*`, dragging
  a tab to an edge) moves the active tab into the new pane; nothing is cloned,
  and there is no clone split mode (`SplitMode` is only `MovePane`, the
  default, and `EmptyPane`). A pane's only tab can't leave it, so splitting it
  opens an empty pane on the other side instead, leaving the tab, still
  focused, on the side asked for. Dragging a pane's only tab over that
  same pane offers no split zones (every pane, terminal included), since the
  drop would just move the pane.
- One back/forward history covers every center and Git History pane. Going back
  or forward reveals the entry's tab in whichever pane holds it, switching mode
  if needed; a closed file reopens in the last active center pane. Moving focus
  from one pane to another records where the user was in the pane they left,
  so going back returns there. `workspace::GoBack` / `workspace::GoForward`
  are workspace actions bound only to `cmd-[` / `cmd-]`, so they work wherever
  focus is in the window. There are no Go Back / Go Forward buttons.
  The `tab_bar.show_nav_history_buttons` and `reveal_if_open`
  settings are removed. The terminal panel keeps its own panes and history.
- Every pane with tabs shows its tab row inside its island. Tab bars always carry their buttons and keep pinned
  tabs in the same row as the others. There are no `tab_bar` settings and no
  Toggle Tab Bar action.
- Tabs are hardcoded too; there are no `tabs` settings. Every tab shows its
  file icon and git status colour, the close button sits on the right and
  appears on hover, closing the active tab activates the previously active one
  (else its left neighbour), and tabs carry no diagnostic badges. The tab
  switcher and file headers follow the same icon and git colouring.
- Split panes are separate islands `ISLAND_GAP` apart. The resize handle
  covers the gap.
  Finding the neighbouring pane (pane navigation, "open to the side") looks
  across the gap, so it reuses an existing pane rather than splitting again.
- Inactive split panes are dimmed (70% overlay opacity) over the whole island. The active pane has no border.
- The workspace has no settings (`WorkspaceSettingsContent`,
  `StatusBarSettings` and `PreviewTabsSettings` are gone). Fixed behaviour:
  - Layout: the bottom dock sits between the left and right docks. Splitting
    goes right or down. Split drop targets are 20% of the pane's smaller
    side. Resizing the left dock resizes all its panels; other docks resize
    only the active panel. No max tab count.
  - Closing the active item in an empty pane closes the window. Files deleted
    on disk keep their tab. A buffer without unsaved edits follows its file's
    deletion and becomes empty (undo brings the text back as unsaved edits),
    so git diffs show the file as deleted; unsaved edits are kept.
  - Focus never follows the mouse.
  - Window: native macOS fullscreen. Text uses the platform's default
    rendering mode.
  - Status bar: shown only in Editor mode (and not while zoomed). The navigation
    bar is its only left item; every widget is on the right (progress, search,
    language servers, problems, cursor position, language); the encoding button only for non-UTF-8 or
    UTF-8-with-BOM buffers.
  - Preview tabs are always on (`TogglePreviewTab` always works). A single
    click in the project panel, opening from a multibuffer, and single-file
    code navigation open preview tabs, and code navigation replaces the
    current preview rather than keeping it. The file finder and multibuffer
    results of code navigation (references, multiple definitions, selections)
    open permanent tabs.
- A zoomed view (editor or any dock) covers the whole area between the
  stripes, inset by the island gap, overlaying the others, so every zoomed view has the same size.
  The `zoomed_padding` setting is removed, as is centered layout
  (`workspace::ToggleCenteredLayout` and the `centered_layout` setting). The status bar is hidden while
  something is zoomed, so the zoomed view reaches down to the window's bottom
  edge.
- A zoomed dock (e.g. an expanded terminal) belongs to Editor mode: Git
  History mode hides it, and switching back to
  Editor mode focuses it again, so it is still open and zoomed rather than
  closed.
- Actions act only in the mode they belong to; in other modes they are ignored
  rather than switching modes. Switching is explicit: the mode keys (`cmd-9`,
  `alt-shift-f12`) and the stripe buttons. Docks belong to Editor mode:
  toggling, opening, closing, focusing or resizing docks and panels by key
  does nothing in the other modes.
- Tool windows have IntelliJ's keys: `cmd-1` Project, `cmd-0` and `cmd-k`
  Commit, `cmd-7` Structure, `cmd-6` Problems (project diagnostics), `cmd-9`
  Git, `alt-f12` Terminal (`terminal_panel::ToggleShell`: with a shell in
  front of a visible panel it hides the panel; otherwise it shows and focuses
  a shell, creating one if needed). `shift-escape` hides the focused tool
  window. `workspace::CycleLeftDock` / `CycleRightDock` remain as unbound
  actions.
- Zoom (`workspace::ToggleZoom`, `` cmd-` ``, also `cmd-shift-f12`) expands the
  focused pane. In the terminal dock that is the focused terminal pane alone,
  not every split, as zooming an editor pane hides the other panes.
- `cmd-1`…`cmd-9` don't activate panes (`ctrl-1`…`ctrl-9` do), so the tool
  window keys work everywhere.
- The status bar is 34px high on the frame, with no divider. It starts with
  the navigation bar (`go_to_line::navigation_bar`, OpenIDEA's default
  position): `project › folders › file › symbols containing the cursor`. The
  file reveals itself in the Project tool window; a symbol moves the cursor
  to its name. Symbols update 50ms after the cursor moves or the file reparses.
- Status bar items have no "Hide Button" right-click entry and can't be hidden. Items without other menu entries (search, cursor
  position, language, diagnostics, …) have no right-click menu at all; dock
  panel buttons have none either.
- Status bar buttons that open a pane item (project search, project
  diagnostics, Git History's graph) turn accent while that item is the active
  tab (`status_item_color`). Diagnostics keeps its error/warning icon colours
  and its counts stay default (the icons already carry colour); only its check
  icon takes the accent.
- Docks always have a fixed size in pixels (width for left/right, height for
  bottom). There is no proportional "flex width" mode and no
  `terminal.flexible` setting. A side dock may shrink when space is tight but
  springs back to its stored width.
- The project, git (commit) and outline panels, and the Git History sidebar,
  share a 240px minimum width (`dock::SIDEBAR_MIN_WIDTH`): dragging or the
  dock-size actions stop there instead of storing a smaller width.
- A tab's close button (editor and terminal tabs alike) is quiet: no fill on
  hover or press and no tooltip; only the muted × brightens to full contrast
  on hover. A filled hover chip plus a "Close" tooltip outweighed the tab
  title.
- The terminal panel's and terminal mode's panes use the same pill tabs.
  Pills have no fill: the active pill gets full-contrast text, inactive ones are muted, and
  any pill shows `ghost_element_hover` on hover. A plain terminal icon is
  accented only on the active pill of the focused pane, and the active label
  goes muted when the pane loses focus, so the terminal draws attention only
  while you are typing in it. Task status icons keep their own colours. A
  filled active pill drew too much attention, especially with a single
  terminal, where there is nothing to tell it apart from. Split drop zones
  cover only the content below the pills, and inactive-pane dimming covers the
  whole island.
- Drag-and-drop targets everywhere (panes, project panel) take their colours
  from the theme's `text_accent`: `ThemeColors::drop_target_border()` is the
  accent and `drop_target_background()` a faint wash of it. Themes cannot set
  them separately; the old `drop_target.background` / `drop_target.border`
  theme keys are gone, since the bundled themes set them to a flat grey. A position in the tab bar (onto a tab, or the
  empty strip after the tabs or after the pinned tabs) shows a thin rounded
  accent bar in the gap where the tab will land: after the target when a tab
  moves right within its pane, before it otherwise, nothing when it would not
  move. A content split zone is an area, so it shows a box with a 1px accent outline and a faint accent fill. The project
  panel draws the same box around a dragged-over folder and its visible rows
  (`project-panel.md`).
- Pane tab bars (center panes, Git History and the terminal panel) have no
  split buttons or split menu: panes are split by dragging a tab to a pane
  edge, or with the `pane::Split*` actions (keys, app menu). The
  `SplitRight` / `SplitDown` icons stay in the icon set. The remaining
  buttons come from one shared `pane::render_tab_bar_pane_buttons`, so every
  tab bar behaves the same. There is no search button: search is opened with
  `cmd-f`, the menu or the action. Left of Zoom, an eye button opens the Markdown, SVG or
  tabular data preview of the active file (Option-click opens it in a split),
  shown only for files that have one. Center (editor) panes and the terminal
  panel put a "+" before them: `workspace::NewFile` (an untitled file) and
  `NewTerminal`.
- Previews go through `workspace::preview`: each preview crate registers a
  `PreviewProvider` (a tooltip for items it can preview, and how to open
  one); the first that matches an item wins. One pair of actions,
  `preview::OpenPreview` (`cmd-shift-v` in any editor) and
  `preview::OpenPreviewToTheSide`, serves every file type, as do the tab bar
  button and the "Open Preview" entries in the editor and tab context menus; a
  new preview type only registers a provider. A pane handles the actions only
  while its active item has a preview, so the shortcut falls through elsewhere
  and menus just check whether the action is available. An editor's language
  change updates its tab, so the button appears once a new file's language is
  detected. Following previews (`markdown::OpenFollowingPreview`,
  `svg::OpenFollowingPreview`) stay type-specific.
  Zoom stays rightmost. Multibuffers have no collapse/expand all files button:
  that is `editor::ToggleFoldAll` (cmd-shift-enter in search bars) or View >
  Fold All / Unfold All, so a multibuffer with nothing else to show (e.g. find
  references) gets no toolbar row. While a buffer or project search input has
  focus, these fold actions go to the searched editor or results, not the input.
  Only the terminal panel has a `+`: a plain button that opens a terminal in
  that pane. Center and Git History panes have no `+` or new-item menu; new
  files, project/symbol search and terminals come from their keybindings and
  the command palette. Every terminal
  pane gets these buttons in `new_terminal_pane`, so panes created by moving
  a tab into a split get them too.
- Pane toolbars (search bars, diff controls) have no bottom line either; they
  sit flat inside the island. Every toolbar-like row (pane toolbars, the Git
  History header and its search row) shares one set of metrics so they line up
  when switching tabs or modes: 6px vertical and 8px horizontal padding around
  a 32px row, square icon buttons with 16px (`IconSize::Medium`) icons, and
  14px (default label size) control text; counts and stats stay small. Right-side toolbar items (e.g. image viewer
  controls) stay right-aligned even when nothing is on the left. Editor panes have no breadcrumbs or quick action
  bar (see editor.md).
- An empty pane keeps its island and shows IntelliJ's centred hints: action
  names right-aligned against their shortcuts in the accent (Go to File, Find
  Action, Find in Files, New File), then "Drop files here to open them".
  Bindings are read from the keymap. This applies to every empty pane, whether
  or not a project is open (there is no welcome page).
- Symbols in the outline panel render in the UI
  font at the project panel's label size, not the buffer font. Syntax highlight
  colours are kept, but theme font weight and style are dropped, so every entry
  (e.g. bold markdown headings) uses the same weight and style.
- In multi-buffer views (search results, diffs) the outline panel always shows
  excerpts, symbols and search matches under each file. The upstream
  `ToggleSymbols` action, its filter-bar button and the
  `outline_panel.multi_buffer_hide_symbols` setting are removed.
- There is a single UI density. The upstream `unstable.ui_density` setting
  (compact/comfortable) is removed, and `DynamicSpacing` is a plain enum whose
  `BaseXX` variant is XX px at a 16px rem; `px()` scales it with the UI font
  size.
