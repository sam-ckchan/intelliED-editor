use std::sync::Arc;
use std::sync::atomic::AtomicUsize;

use gpui::App;
use gpui::AppContext as _;
use gpui::Context;
use gpui::Div;
use gpui::DragMoveEvent;
use gpui::Entity;
use gpui::Focusable;
use gpui::Pixels;
use gpui::WeakEntity;
use project::Project;
use ui::Window;
use ui::prelude::ActiveTheme;
use ui::prelude::FluentBuilder;
use ui::prelude::InteractiveElement;
use ui::prelude::IntoElement;
use ui::prelude::ParentElement;
use ui::prelude::StatefulInteractiveElement;
use ui::prelude::Styled;
use ui::prelude::div;
use ui::prelude::h_flex;
use ui::prelude::px;
use util::ResultExt;

use crate::Event;
use crate::ISLAND_GAP;
use crate::NavHistory;
use crate::Pane;
use crate::PaneRenderContext;
use crate::PaneSearchBarCallbacks;
use crate::SplitDirection;
use crate::SplitMode;
use crate::Workspace;
use crate::WorkspaceMode;
use crate::dock::SIDEBAR_MIN_WIDTH;
use crate::pane;

pub(crate) const GIT_HISTORY_SIDEBAR_WIDTH: Pixels = px(320.);
pub(crate) const GIT_TOOL_WINDOW_HEIGHT: Pixels = px(320.);
const GIT_TOOL_WINDOW_MIN_HEIGHT: Pixels = px(120.);
/// Wider than the hairline it sits on, so it can be grabbed.
const SIDEBAR_RESIZE_HANDLE_WIDTH: Pixels = px(6.);

#[derive(Clone)]
struct DraggedGitHistorySidebar;

#[derive(Clone)]
pub(super) struct DraggedGitToolWindow;

/// Keeps the Git tool window between its minimum and 80% of the area it shares with the editors.
fn clamp_git_tool_window_height(height: Pixels, available: Pixels) -> Pixels {
    height.min(available * 0.8).max(GIT_TOOL_WINDOW_MIN_HEIGHT)
}

/// Keeps the git history sidebar between its minimum and 60% of the window.
fn clamp_git_history_sidebar_width(width: Pixels, available: Pixels) -> Pixels {
    width.min(available * 0.6).max(SIDEBAR_MIN_WIDTH)
}

impl Workspace {
    /// Whether the Git tool window shows under the editors: open, or focused
    /// (Git History mode), and not zoomed, which fills the area instead.
    pub(crate) fn git_tool_window_showing(&self, cx: &App) -> bool {
        match self.mode {
            WorkspaceMode::Editor => self.git_tool_window_open,
            WorkspaceMode::GitHistory => !self.has_zoomed(WorkspaceMode::GitHistory, cx),
            WorkspaceMode::Terminal => false,
        }
    }

    /// The Git tool window as IntelliJ docks it: under the editors and side
    /// tool windows, resized by the gap above it.
    pub(super) fn render_git_tool_window(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("git-tool-window")
            .relative()
            .flex_none()
            .w_full()
            .h(self.git_tool_window_height)
            .pt(ISLAND_GAP)
            .child(self.render_git_history(window, cx))
            .child(
                div()
                    .id("git-tool-window-resize-handle")
                    .absolute()
                    .top_0()
                    .left_0()
                    .w_full()
                    .h(ISLAND_GAP)
                    .cursor_row_resize()
                    .on_click(cx.listener(|workspace, event: &gpui::ClickEvent, _, cx| {
                        if event.click_count() == 2 {
                            workspace.git_tool_window_height = GIT_TOOL_WINDOW_HEIGHT;
                            cx.notify();
                        }
                    }))
                    .on_drag(DraggedGitToolWindow, |_, _, _, cx| cx.new(|_| gpui::Empty)),
            )
    }

    /// Follows a drag of the Git tool window's top edge; `bounds` is the area it shares with the editors.
    pub(super) fn resize_git_tool_window(
        &mut self,
        pointer_y: Pixels,
        bounds: gpui::Bounds<Pixels>,
        cx: &mut Context<Self>,
    ) {
        self.git_tool_window_height = clamp_git_tool_window_height(bounds.bottom() - pointer_y, bounds.size.height);
        cx.notify();
    }

    /// Hides the Git tool window; focus returns to the editors if it was inside.
    pub fn hide_git_tool_window(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.git_tool_window_open = false;
        self.zoom_git_history_pane(None, cx);
        if self.mode == WorkspaceMode::GitHistory {
            self.set_mode(WorkspaceMode::Editor, window, cx);
        }
        cx.notify();
    }

    /// `mode::Editor`: leaves Terminal mode, or hides the Git tool window.
    pub(crate) fn leave_mode(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.mode == WorkspaceMode::Terminal {
            self.set_mode(WorkspaceMode::Editor, window, cx);
        } else {
            self.hide_git_tool_window(window, cx);
        }
    }

    /// One tool window at the bottom, as in IntelliJ: the bottom dock opening
    /// hides the Git tool window, and the Git tool window opening closes it.
    pub(crate) fn bottom_dock_changed(&mut self, cx: &mut Context<Self>) {
        let open = self.bottom_dock.read(cx).is_open();
        let opened = open && !std::mem::replace(&mut self.bottom_dock_was_open, open);
        if opened && self.git_tool_window_open {
            self.git_tool_window_open = false;
            self.switch_mode(WorkspaceMode::Editor, cx);
            cx.notify();
        }
    }

    fn close_bottom_dock_for_git(&self, window: &mut Window, cx: &mut Context<Self>) {
        if self.bottom_dock.read(cx).is_open() {
            self.bottom_dock.update(cx, |dock, cx| dock.set_open(false, window, cx));
        }
    }

    pub(super) fn render_git_history(&self, window: &mut Window, cx: &mut Context<Self>) -> Div {
        let pane = self.active_git_history_pane.clone();
        let zoomed_pane = self.zoomed_pane(WorkspaceMode::GitHistory, cx);
        let side_card = self.git_history_side_card(cx);
        let panes = match zoomed_pane {
            Some(zoomed_pane) => self.git_history_center.render_zoomed(&zoomed_pane, cx).into_any_element(),
            None => {
                let weak_handle = self.weak_handle();
                let render_cx = PaneRenderContext { active_pane: &pane, workspace: &weak_handle };
                self.git_history_center.render(None, None, &render_cx, window, cx).into_any_element()
            }
        };
        let content = h_flex()
            .size_full()
            .on_drag_move(cx.listener(|workspace, e: &DragMoveEvent<DraggedGitHistorySidebar>, _, cx| {
                workspace.git_history_sidebar_width =
                    clamp_git_history_sidebar_width(e.bounds.right() - e.event.position.x, e.bounds.size.width);
                cx.notify();
            }))
            .child(div().id("git-history-panes").flex().flex_1().min_w_0().h_full().child(panes))
            .when_some(side_card, |this, side_card| {
                this.child(
                    div()
                        .id("git-history-sidebar")
                        .relative()
                        .flex_none()
                        .h_full()
                        .w(self.git_history_sidebar_width)
                        .pl(ISLAND_GAP)
                        .flex()
                        .flex_col()
                        .child(
                            div()
                                .flex_1()
                                .min_h_0()
                                .w_full()
                                .overflow_hidden()
                                .bg(cx.theme().colors().panel_background)
                                .child(side_card),
                        )
                        .child(
                            div()
                                .id("git-history-sidebar-resize-handle")
                                .absolute()
                                .top_0()
                                .left_0()
                                .h_full()
                                .w(SIDEBAR_RESIZE_HANDLE_WIDTH)
                                .cursor_col_resize()
                                .on_click(cx.listener(|workspace, event: &gpui::ClickEvent, _, cx| {
                                    if event.click_count() == 2 {
                                        workspace.git_history_sidebar_width = GIT_HISTORY_SIDEBAR_WIDTH;
                                        cx.notify();
                                    }
                                }))
                                .on_drag(DraggedGitHistorySidebar, |_, _, _, cx| cx.new(|_| gpui::Empty)),
                        ),
                )
            });
        match cx.try_global::<PaneSearchBarCallbacks>() {
            Some(callbacks) => (callbacks.wrap_div_with_search_actions)(content, pane),
            None => content,
        }
    }

    /// What the history sidebar shows: the active history item's side card.
    /// A zoomed history pane fills the area alone, without the sidebar, as
    /// zooming hides docks and other panes in Editor mode.
    pub(crate) fn git_history_side_card(&self, cx: &App) -> Option<gpui::AnyView> {
        if self.has_zoomed(WorkspaceMode::GitHistory, cx) {
            return None;
        }
        self.active_git_history_pane.read(cx).active_item().and_then(|item| item.side_card(cx))
    }

    /// The active history pane, where the graph and commit views open.
    pub const fn git_history_pane(&self) -> &Entity<Pane> {
        &self.active_git_history_pane
    }

    pub fn git_history_panes(&self) -> Vec<&Entity<Pane>> {
        self.git_history_center.panes()
    }

    /// Whether no history pane has any tabs.
    pub fn git_history_is_empty(&self, cx: &App) -> bool {
        self.git_history_panes().iter().all(|pane| pane.read(cx).items_len() == 0)
    }

    pub(crate) fn new_git_history_pane(
        workspace: WeakEntity<Self>,
        project: Entity<Project>,
        pane_history_timestamp: Arc<AtomicUsize>,
        nav_history: NavHistory,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Entity<Pane> {
        let pane = cx.new(|cx| {
            let mut pane =
                Pane::new(workspace, project, pane_history_timestamp, None, Box::new(gpui::NoAction), window, cx);
            pane.set_can_split(Some(Arc::new(|_, _, _, _| true)));
            pane.set_read_only_tabs(true);
            pane.set_nav_history(nav_history);
            pane
        });
        cx.subscribe_in(&pane, window, Self::handle_git_history_pane_event).detach();
        cx.emit(Event::PaneAdded(pane.clone()));
        pane
    }

    pub(crate) fn add_git_history_pane(&self, window: &mut Window, cx: &mut Context<Self>) -> Entity<Pane> {
        Self::new_git_history_pane(
            self.weak_handle(),
            self.project.clone(),
            self.pane_history_timestamp.clone(),
            self.nav_history.clone(),
            window,
            cx,
        )
    }

    fn split_git_history_pane(
        &mut self,
        pane: Entity<Pane>,
        direction: SplitDirection,
        mode: SplitMode,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // The new pane would be hidden behind a zoomed one.
        self.zoom_git_history_pane(None, cx);
        match mode {
            SplitMode::EmptyPane => {
                self.split_git_history_pane_empty(&pane, direction, window, cx);
            }
            SplitMode::MovePane => {
                // A lone tab stays, with an empty pane on the other side (as in the editor).
                if pane.read(cx).items_len() < 2 {
                    let new_pane = self.add_git_history_pane(window, cx);
                    self.git_history_center.split(&pane, &new_pane, direction.opposite(), cx);
                    self.notify_mode(WorkspaceMode::GitHistory, cx);
                    return;
                }
                let Some(item) = pane.update(cx, |pane, cx| pane.take_active_item(window, cx)) else {
                    return;
                };
                let new_pane = self.add_git_history_pane(window, cx);
                new_pane.update(cx, |pane, cx| pane.add_item(item, true, true, None, window, cx));
                self.git_history_center.split(&pane, &new_pane, direction, cx);
                self.set_active_git_history_pane(&new_pane, window, cx);
            }
        }
        self.notify_mode(WorkspaceMode::GitHistory, cx);
    }

    fn split_git_history_pane_empty(
        &mut self,
        pane: &Entity<Pane>,
        direction: SplitDirection,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Entity<Pane> {
        let new_pane = self.add_git_history_pane(window, cx);
        self.git_history_center.split(pane, &new_pane, direction, cx);
        self.set_active_git_history_pane(&new_pane, window, cx);
        window.focus(&new_pane.focus_handle(cx), cx);
        new_pane
    }

    /// Drops an emptied history pane, unless it is the last one.
    fn remove_git_history_pane(&mut self, pane: &Entity<Pane>, window: &mut Window, cx: &mut Context<Self>) {
        if !self.git_history_center.remove(pane, cx).log_err().unwrap_or(false) {
            return;
        }
        if self.active_git_history_pane == *pane {
            let next_pane = self.git_history_center.first_pane();
            self.set_active_git_history_pane(&next_pane, window, cx);
            if self.mode == WorkspaceMode::GitHistory {
                window.focus(&next_pane.focus_handle(cx), cx);
            }
        }
        self.notify_mode(WorkspaceMode::GitHistory, cx);
    }

    pub fn set_active_git_history_pane(&mut self, pane: &Entity<Pane>, window: &mut Window, cx: &mut Context<Self>) {
        if self.active_git_history_pane == *pane {
            return;
        }
        self.active_git_history_pane = pane.clone();
        if self.mode == WorkspaceMode::GitHistory {
            self.status_bar.update(cx, |status_bar, cx| status_bar.set_active_pane(pane, window, cx));
        }
        self.notify_mode(WorkspaceMode::GitHistory, cx);
    }

    /// Zooms `pane` (unzooming any other history pane), or unzooms all for `None`.
    pub(crate) fn zoom_git_history_pane(&self, pane: Option<&Entity<Pane>>, cx: &mut Context<Self>) {
        for history_pane in self.git_history_center.panes() {
            let zoomed = pane == Some(history_pane);
            if history_pane.read(cx).is_zoomed() != zoomed {
                history_pane.update(cx, |history_pane, cx| history_pane.set_zoomed(zoomed, cx));
            }
        }
        self.notify_mode(WorkspaceMode::GitHistory, cx);
    }

    fn handle_git_history_pane_event(
        &mut self,
        pane: &Entity<Pane>,
        event: &pane::Event,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            pane::Event::AddItem { item } => {
                item.added_to_pane(self, pane.clone(), window, cx);
                cx.emit(Event::GitHistoryItemAdded { item: item.boxed_clone() });
            }
            pane::Event::Split { direction, mode } => {
                self.split_git_history_pane(pane.clone(), *direction, *mode, window, cx);
            }
            pane::Event::Remove { .. } => self.remove_git_history_pane(pane, window, cx),
            pane::Event::RemovedItem { .. } if self.git_history_is_empty(cx) => {
                self.hide_git_tool_window(window, cx);
            }
            pane::Event::ZoomIn => self.zoom_git_history_pane(Some(pane), cx),
            pane::Event::ZoomOut => self.zoom_git_history_pane(None, cx),
            pane::Event::Focus => {
                self.content_pane_focused(pane, window, cx);
                self.set_active_git_history_pane(pane, window, cx);
                self.switch_mode(WorkspaceMode::GitHistory, cx);
                self.close_bottom_dock_for_git(window, cx);
            }
            pane::Event::ChangeItemTitle | pane::Event::ActivateItem { .. } | pane::Event::RemovedItem { .. } => {
                self.notify_mode(WorkspaceMode::GitHistory, cx)
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use gpui::px;

    use super::GIT_TOOL_WINDOW_MIN_HEIGHT;
    use super::clamp_git_history_sidebar_width;
    use super::clamp_git_tool_window_height;
    use crate::dock::SIDEBAR_MIN_WIDTH;

    #[test]
    fn test_clamp_git_tool_window_height() {
        assert_eq!(clamp_git_tool_window_height(px(300.), px(1000.)), px(300.));
        assert_eq!(clamp_git_tool_window_height(px(950.), px(1000.)), px(800.));
        assert_eq!(clamp_git_tool_window_height(px(10.), px(1000.)), GIT_TOOL_WINDOW_MIN_HEIGHT);
    }

    #[test]
    fn test_clamp_git_history_sidebar_width() {
        assert_eq!(clamp_git_history_sidebar_width(px(400.), px(1000.)), px(400.));
        assert_eq!(clamp_git_history_sidebar_width(px(900.), px(1000.)), px(600.));
        assert_eq!(clamp_git_history_sidebar_width(px(100.), px(1000.)), SIDEBAR_MIN_WIDTH);
        assert_eq!(clamp_git_history_sidebar_width(px(300.), px(200.)), SIDEBAR_MIN_WIDTH);
    }
}
