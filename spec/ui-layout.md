# UI layout

## Decision

The window looks and is laid out like IntelliJ IDEA's New UI (2025.1, the
"Dark" theme; OpenIDEA is the second reference): a main toolbar across the
top, tool window stripes at the left and right edges, flat tool windows and
editors divided by hairlines, underlined editor tabs, and a status bar. It is
always on, with no setting. The earlier look (Nord, floating rounded cards on
one chrome colour, no title bar row) is gone.

Where this document says "card", it means a pane's flat content area below its
tab bar.

## Theme

- The theme is constant: IntelliJ Dark, built in Rust in `theme::built_in`, so
  there is no theme JSON, no light/dark default palette, no refinement or
  fallback derivation, and no `theme_settings` crate. To change colours, edit
  `theme::built_in` and rebuild.
- `theme::built_in` is the single origin of every colour, in three layers:
  the palette (the only raw hex values: IntelliJ's New UI grays and blues, the
  "Dark" editor colour scheme, the terminal's ANSI colours), role constants
  that name what a palette colour is used for (`shade` scales RGB channels,
  alpha via `Hsla::alpha`), and theme fields that reference roles only. No
  theme field holds a colour from outside the palette (except the macOS
  traffic lights).
- Surfaces: tool windows, the main toolbar, the stripes, the status bar and
  popups are the chrome (`#2B2D30`); editors, their tab bars and terminals are
  the editor colour (`#1E1F22`), which is also the window background, so it
  shows through the 1px gaps between panels as the hairlines. The accent is
  blue: `#3574F0` for fills (the active tab's underline, focus borders, a
  showing tool window's stripe button), the lighter `#548AF7` for text, icons
  and links. Selected list rows are `#2E436E`.
- Syntax follows IntelliJ's scheme, which colours declarations and literals,
  not uses: keywords orange, strings green, numbers cyan, functions blue,
  constants and fields purple, metadata olive, comments gray (doc comments
  green); types, variables, operators and punctuation stay plain text.
- File status colours follow IntelliJ: added green, modified and renamed blue,
  ignored olive, deleted and conflicting red.
- The theme is one lazily built `static`: `built_in::theme()` and `cx.theme()`
  return `&'static Theme`. There is no theme global, no `theme::init`, and no
  `Arc` around the theme, the syntax theme or the accents (a fixed array);
  holders keep a `&'static SyntaxTheme`. Tests cannot install another theme.
- Terminal bright / dim ANSI colours are the normal colour shaded +20% / -30%.
- `StatusColors` only has the fields the UI reads; unused `_background` /
  `_border` variants and `predictive` are gone.
- There are no Vim/Helix theme colours (`vim_*`).
- There is no theme registry, theme picker, light/dark mode toggle, `Settings`
  app-menu submenu, or theme settings (`theme`, `markdown_preview.theme`,
  `theme_overrides`, `experimental.theme_overrides`).
- The theme never changes at runtime, so nothing observes theme changes and the
  OS light/dark appearance is not tracked. Native window chrome is forced to the
  theme's appearance.

## Fonts

- Fonts are constants in `ui::font`, not settings: UI font `.SystemUIFont`
  16px with ligatures off, buffer font `JetBrainsMono Nerd Font` 15px, both
  weight 400. Mermaid diagrams use the buffer font family. To change them, edit
  `ui::font` and rebuild.
- Font families never change at runtime; only sizes change, through in-memory
  zoom that resets on restart. `ed::{Increase,Decrease,Reset}UiFontSize`,
  `ed::{Increase,Decrease,Reset}BufferFontSize`, and `ed::ResetAllZoom` take
  no arguments and never write settings. Sizes clamp to 6–100px.
- The markdown preview (default: UI font size) and the git commit editor
  (default: 12px) zoom independently of the buffer when focused.
- `ui` reads fonts directly; `theme` provides no font settings.

## Behaviour

- The window is, top to bottom: the main toolbar, then the left stripe, the
  shown mode's content and the right stripe side by side, then the status bar.
  The toolbar, stripes and status bar (plus notifications, toasts and modals)
  are drawn outside the modes, so they stay put when the mode changes.
- Main toolbar (`workspace::window_header`): 40px high, full width, chrome
  coloured, over a hairline. Left to right: room for the three macOS traffic
  lights (on the toolbar's centre line, at (13, 13)), the project name (the
  active folder, see `git.md`), the worktree and branch pickers as
  `worktree / branch` (only for a project in a git repository), Go Back / Go
  Forward; at the right end, Search Everywhere (the file finder) and Find
  Action (the command palette). Its empty space is the titlebar: dragging
  moves the window and a double click zooms or minimises it as macOS is set
  up to. Tab bars are not titlebar space. Fullscreen hides the traffic lights
  and the toolbar's content starts at its left padding.
  Every header text button (project, worktree, branch) shares one style
  (`title_bar::worktree_branch::header_text_button`). The project name is
  full contrast; worktree and branch are muted, so the row reads as a
  breadcrumb. The branch icon keeps its change-status colour. Toolbar and tab
  bar icon buttons are a 16px icon in a 28px square
  (`workspace::pane::top_row_icon_button`).
- Tool window stripes (`workspace::stripe`): 40px wide icon columns, chrome
  coloured. The left stripe holds the left dock's tool windows (Project,
  Commit) at its top and, at its bottom, the bottom dock's (Terminal) and Git
  (Git History mode, only with a repository); the right stripe holds the right
  dock's (Structure). A button is a 16px icon in a 28px square; while its tool
  window is showing, it is filled with the accent and its icon is full
  contrast. A click shows and focuses the tool window, or hides it when it is
  already showing (`Workspace::toggle_tool_window`). Docks belong to Editor
  mode, so a click in another mode switches to Editor mode first and always
  shows the tool window. The Git button toggles between Git History and
  Editor mode. The dock buttons moved here from the status bar.
- Panels are flat and divided by 1px hairlines (`PANEL_GAP`): the window
  background showing through the gap between a dock and the editors, between
  split panes, and between the stripes and the content. Nothing is rounded,
  inset or masked; there are no floating cards.
- Tabs are flat (`TabStyle::Flat`): they fill the 36px tab bar, which is the
  editor colour over a hairline and runs edge to edge. The selected tab has a
  3px accent underline; other tabs show the hover fill under the mouse. A
  dragged tab is filled and has no underline. An empty pane keeps a blank
  strip of tab-bar height, so its content does not jump when the first tab
  opens.
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
  and is styled like a left dock (flat, resizable by a 6px handle on its
  hairline). `cmd-9` again returns to Editor mode.
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
  - Panes use the terminal panel's pill tabs inside their content area.
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
- Docks have no border and blend into the chrome. Panes inside them (the
  terminal) draw their own cards; a non-pane panel docked at the bottom just
  sits on the chrome.
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
  focus is in the window. The Go Back / Go Forward buttons sit
  only in the main toolbar (see above), not on tab bars.
  The `tab_bar.show_nav_history_buttons` and `reveal_if_open`
  settings are removed. The terminal panel keeps its own panes and history.
- Every pane always shows its tab bar (pill tabs inside the card, otherwise on
  the chrome above it). Tab bars always carry their buttons and keep pinned
  tabs in the same row as the others. There are no `tab_bar` settings and no
  Toggle Tab Bar action.
- Tabs are hardcoded too; there are no `tabs` settings. Every tab shows its
  file icon and git status colour, the close button sits on the right and
  appears on hover, closing the active tab activates the previously active one
  (else its left neighbour), and tabs carry no diagnostic badges. The tab
  switcher and file headers follow the same icon and git colouring.
- Split panes are `PANEL_GAP` apart, which reads as a divider line. The
  resize handle straddles the gap.
  Finding the neighbouring pane (pane navigation, "open to the side") looks
  across the gap, so it reuses an existing pane rather than splitting again.
- Inactive split panes are dimmed (70% overlay opacity) over the content
  only, never the tab bar. The active pane has no border.
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
  - Status bar: shown only in Editor mode (and not while zoomed), with the language and
    cursor position buttons; the encoding button only for non-UTF-8 or
    UTF-8-with-BOM buffers.
  - Preview tabs are always on (`TogglePreviewTab` always works). A single
    click in the project panel, opening from a multibuffer, and single-file
    code navigation open preview tabs, and code navigation replaces the
    current preview rather than keeping it. The file finder and multibuffer
    results of code navigation (references, multiple definitions, selections)
    open permanent tabs.
- A zoomed view (editor or any dock) covers the whole area between the
  stripes, overlaying the others, so every zoomed view has the same size.
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
- The status bar is chrome coloured over a hairline, with 2px vertical
  padding.
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
- The terminal panel's panes use pill tabs instead: rounded buttons at the
  top of the pane's content area.
  Pills have no fill: the active pill gets full-contrast text, inactive ones are muted, and
  any pill shows `ghost_element_hover` on hover. A plain terminal icon is
  accented only on the active pill of the focused pane, and the active label
  goes muted when the pane loses focus, so the terminal draws attention only
  while you are typing in it. Task status icons keep their own colours. A
  filled active pill drew too much attention, especially with a single
  terminal, where there is nothing to tell it apart from. Split drop zones
  cover only the content below the pills, and inactive-pane dimming covers the
  whole card.
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
  sit flat inside the card. Every toolbar-like row (pane toolbars, the Git
  History header and its search row) shares one set of metrics so they line up
  when switching tabs or modes: 6px vertical and 8px horizontal padding around
  a 32px row, square icon buttons with 16px (`IconSize::Medium`) icons, and
  14px (default label size) control text; counts and stats stay small. Right-side toolbar items (e.g. image viewer
  controls) stay right-aligned even when nothing is on the left. Editor panes have no breadcrumbs or quick action
  bar (see editor.md).
- An empty pane keeps its card and shows centred keybinding hints (New File,
  Open File, Go to File, Command Palette), styled like the project search
  landing page.
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
