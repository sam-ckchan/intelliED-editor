//! What fills the window above the status bar: one view per
//! [`WorkspaceMode`], with each mode's rendering in its own module.

mod editor;
mod git_history;
mod terminal;

pub(crate) use editor::DockMotion;
pub(crate) use git_history::GIT_HISTORY_SIDEBAR_WIDTH;
pub(crate) use git_history::GIT_TOOL_WINDOW_HEIGHT;
use gpui::AnyElement;
use gpui::App;
use gpui::Context;
use gpui::Entity;
use gpui::IntoElement;
use gpui::Render;
use gpui::Subscription;
use gpui::WeakEntity;
use gpui::Window;
use gpui::div;
pub(crate) use terminal::TerminalLayout;
pub use terminal::TerminalModeDelegate;
pub(crate) use terminal::is_dragged_terminal;
pub use terminal::set_terminal_mode_delegate;
use ui::prelude::Div;
use ui::prelude::ParentElement;
use ui::prelude::Styled;

use crate::ISLAND_GAP;
use crate::Pane;
use crate::Workspace;
use crate::WorkspaceMode;
use crate::dock::Dock;

/// Renders one mode's content, everything above the status bar. The workspace draws only the shown mode's view, cached, so a
/// mode is laid out only while shown and redraws of the shared chrome reuse it.
pub(crate) struct ModeView {
    workspace: WeakEntity<Workspace>,
    mode: WorkspaceMode,
    _subscriptions: Vec<Subscription>,
}

impl ModeView {
    /// A cached view re-renders only when it or a view inside it notifies, so
    /// it also follows the workspace state it reads, and `followed` entities
    /// that aren't always drawn inside it.
    pub(crate) fn new(
        workspace: &Entity<Workspace>,
        mode: WorkspaceMode,
        followed: &[Entity<Dock>],
        cx: &mut Context<ModeView>,
    ) -> ModeView {
        let mut subscriptions = vec![cx.observe(workspace, |mode_view: &mut ModeView, _, cx| {
            mode_view.notify_if_shown(cx);
        })];
        subscriptions.extend(
            followed
                .iter()
                .map(|entity| cx.observe(entity, |mode_view: &mut ModeView, _, cx| mode_view.notify_if_shown(cx))),
        );
        ModeView { workspace: workspace.downgrade(), mode, _subscriptions: subscriptions }
    }

    fn notify_if_shown(&self, cx: &mut Context<ModeView>) {
        if self.workspace.upgrade().is_some_and(|workspace| workspace.read(cx).drawn_mode(cx) == self.mode) {
            cx.notify();
        }
    }
}

impl Render for ModeView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<ModeView>) -> impl IntoElement {
        let mode = self.mode;
        self.workspace
            .update(cx, |workspace: &mut Workspace, cx| workspace.render_mode(mode, window, cx))
            .unwrap_or_else(|_| div().into_any_element())
    }
}

impl Workspace {
    /// What fills the window above the status bar in `mode`, rendered by that
    /// mode's [`ModeView`].
    fn render_mode(&self, mode: WorkspaceMode, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        match mode {
            WorkspaceMode::Editor => self.render_editor_mode(window, cx),
            // Only drawn while a history pane is zoomed (`drawn_mode`): it fills the area alone.
            WorkspaceMode::GitHistory => self.mode_container().child(self.render_git_history(window, cx)),
            WorkspaceMode::Terminal => self.render_terminal_mode(window, cx),
        }
        .into_any_element()
    }

    fn mode_container(&self) -> Div {
        div()
            .relative()
            .size_full()
            .flex()
            .flex_col()
            // The frame shows between the islands and the stripes.
            .px(ISLAND_GAP)
    }

    /// The pane zoomed in `mode`. Editor mode can zoom a dock panel instead,
    /// which [`Workspace::has_zoomed`] also counts.
    pub(crate) fn zoomed_pane(&self, mode: WorkspaceMode, cx: &App) -> Option<Entity<Pane>> {
        match mode {
            WorkspaceMode::Editor => self.zoomed.as_ref().and_then(|view| view.upgrade()?.downcast::<Pane>().ok()),
            WorkspaceMode::GitHistory => {
                self.git_history_center.panes().into_iter().find(|pane| pane.read(cx).is_zoomed()).cloned()
            }
            WorkspaceMode::Terminal => {
                self.terminal_center.panes().into_iter().find(|pane| pane.read(cx).is_zoomed()).cloned()
            }
        }
    }

    /// Whether `mode` has a pane or dock panel zoomed.
    pub(crate) fn has_zoomed(&self, mode: WorkspaceMode, cx: &App) -> bool {
        match mode {
            WorkspaceMode::Editor => self.zoomed.is_some(),
            WorkspaceMode::GitHistory | WorkspaceMode::Terminal => self.zoomed_pane(mode, cx).is_some(),
        }
    }

    pub(crate) const fn mode_view(&self, mode: WorkspaceMode) -> &Entity<ModeView> {
        match mode {
            WorkspaceMode::Editor => &self.editor_view,
            WorkspaceMode::GitHistory => &self.git_history_view,
            WorkspaceMode::Terminal => &self.terminal_mode_view,
        }
    }

    /// Redraws after a change to `mode`'s content. A hidden mode isn't drawn,
    /// and re-renders when it is shown again, so its changes redraw nothing.
    pub(crate) fn notify_mode(&self, mode: WorkspaceMode, cx: &mut Context<Self>) {
        if mode_is_drawn(mode, self.mode, self.git_tool_window_open) {
            cx.notify();
        }
    }

    /// The mode whose view fills the window. The Git tool window is drawn by
    /// Editor mode's view, under the editors, so Git History mode has a view
    /// of its own only while one of its panes is zoomed.
    pub(crate) fn drawn_mode(&self, cx: &App) -> WorkspaceMode {
        match self.mode {
            WorkspaceMode::GitHistory if !self.has_zoomed(WorkspaceMode::GitHistory, cx) => WorkspaceMode::Editor,
            mode => mode,
        }
    }
}

/// Whether a change to `mode`'s content is on screen while `current` is the
/// workspace's mode: its own mode, or Editor and Git History for each other
/// while the Git tool window shows under the editors.
const fn mode_is_drawn(mode: WorkspaceMode, current: WorkspaceMode, git_tool_window_open: bool) -> bool {
    match (mode, current) {
        (WorkspaceMode::Editor, WorkspaceMode::GitHistory) => true,
        (WorkspaceMode::GitHistory, WorkspaceMode::Editor) => git_tool_window_open,
        (WorkspaceMode::Editor, WorkspaceMode::Editor)
        | (WorkspaceMode::GitHistory, WorkspaceMode::GitHistory)
        | (WorkspaceMode::Terminal, WorkspaceMode::Terminal) => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::mode_is_drawn;
    use crate::WorkspaceMode;

    #[test]
    fn test_mode_is_drawn() {
        assert!(mode_is_drawn(WorkspaceMode::Editor, WorkspaceMode::Editor, false));
        assert!(mode_is_drawn(WorkspaceMode::Editor, WorkspaceMode::GitHistory, true));
        assert!(mode_is_drawn(WorkspaceMode::GitHistory, WorkspaceMode::Editor, true));
        assert!(!mode_is_drawn(WorkspaceMode::GitHistory, WorkspaceMode::Editor, false));
        assert!(mode_is_drawn(WorkspaceMode::GitHistory, WorkspaceMode::GitHistory, true));
        assert!(!mode_is_drawn(WorkspaceMode::Terminal, WorkspaceMode::Editor, true));
        assert!(!mode_is_drawn(WorkspaceMode::GitHistory, WorkspaceMode::Terminal, true));
        assert!(!mode_is_drawn(WorkspaceMode::Editor, WorkspaceMode::Terminal, false));
    }
}
