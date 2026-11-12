//! The tool window stripes: icon columns at the window's left and right
//! edges, outside the modes, like IntelliJ's. The left stripe holds the left
//! dock's tool windows at its top and the bottom dock's, plus Git (the Git
//! History mode), at its bottom; the right stripe holds the right dock's.

use ed_actions::mode::Editor;
use ed_actions::mode::GitHistory;
use gpui::Action;
use gpui::App;
use gpui::IntoElement;
use gpui::Pixels;
use gpui::px;
use ui::Tooltip;
use ui::prelude::ButtonCommon;
use ui::prelude::Clickable;
use ui::prelude::IconName;
use ui::prelude::ParentElement;
use ui::prelude::Styled;
use ui::prelude::v_flex;

use crate::Workspace;
use crate::WorkspaceMode;
use crate::dock::DockPosition;
use crate::dock::stripe_button;
use crate::dock::stripe_button_fill;
use crate::dock::stripe_button_size;

/// Outer padding of a tool window stripe; the island gap on its inner side comes from the content.
const STRIPE_PADDING: Pixels = px(4.);

/// Width of a tool window stripe: a button and the outer padding.
pub(crate) const fn stripe_width(show_names: bool) -> Pixels {
    let (button_width, _) = stripe_button_size(show_names);
    px(button_width.as_f32() + STRIPE_PADDING.as_f32())
}

/// Whether a click on a tool window's stripe button hides it: only when it is
/// already showing in the mode being shown. A click that had to leave another
/// mode always shows the tool window.
pub(crate) const fn stripe_click_hides(showing: bool, switched_mode: bool) -> bool {
    showing && !switched_mode
}

/// The action behind the Git stripe button: Git History, or back to the
/// editor while it is shown.
fn git_stripe_action(mode: WorkspaceMode) -> Box<dyn Action> {
    if mode == WorkspaceMode::GitHistory { Box::new(Editor) } else { Box::new(GitHistory) }
}

impl Workspace {
    /// A stripe button: shows and focuses panel `panel_ix` of the dock at
    /// `position`, or hides the dock when that panel is already showing.
    /// Docks belong to Editor mode, so it switches there first.
    pub fn toggle_tool_window(
        &mut self,
        position: DockPosition,
        panel_ix: usize,
        window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) {
        let switched_mode = self.switch_mode(WorkspaceMode::Editor, cx);
        let dock = self.dock_at_position(position).clone();
        let visible = self.is_dock_visible(position, cx);
        let showing = visible && dock.read(cx).active_panel_index() == Some(panel_ix);
        if stripe_click_hides(showing, switched_mode) {
            self.toggle_dock(position, window, cx);
            return;
        }
        dock.update(cx, |dock, cx| dock.activate_panel(panel_ix, window, cx));
        if !visible {
            self.toggle_dock(position, window, cx);
            return;
        }
        if let Some(panel) = dock.read(cx).active_panel() {
            window.focus(&panel.activation_focus_handle(cx), cx);
        }
        cx.notify();
        self.serialize_workspace(window, cx);
    }

    /// Shows or hides the tool window names on the stripes, for this session.
    pub fn toggle_tool_window_names(&mut self, cx: &mut gpui::Context<Self>) {
        self.show_tool_window_names = !self.show_tool_window_names;
        for buttons in [&self.left_dock_buttons, &self.bottom_dock_buttons, &self.right_dock_buttons] {
            buttons.update(cx, |_, cx| cx.notify());
        }
        cx.notify();
    }

    pub(crate) fn render_left_stripe(&self, cx: &App) -> impl IntoElement {
        let has_repository = self.project.read(cx).active_repository(cx).is_some();
        let git = has_repository.then(|| {
            let action = git_stripe_action(self.mode);
            let tooltip_action = action.boxed_clone();
            let showing = self.mode == WorkspaceMode::GitHistory;
            stripe_button_fill(showing, cx).child(
                stripe_button("stripe-git", IconName::GitGraph, "Git".into(), showing, self.show_tool_window_names)
                    .tooltip(move |_window, cx| Tooltip::for_action("Git", tooltip_action.as_ref(), cx))
                    .on_click(move |_, window, cx| window.dispatch_action(action.boxed_clone(), cx)),
            )
        });
        stripe(self.show_tool_window_names)
            .child(self.left_dock_buttons.clone())
            .child(v_flex().items_center().gap_1().child(self.bottom_dock_buttons.clone()).children(git))
    }

    pub(crate) fn render_right_stripe(&self) -> impl IntoElement {
        stripe(self.show_tool_window_names).child(self.right_dock_buttons.clone())
    }
}

fn stripe(show_names: bool) -> gpui::Div {
    v_flex().flex_none().w(stripe_width(show_names)).h_full().pt_1().pb(px(6.)).items_center().justify_between()
}

#[cfg(test)]
mod tests {
    use ed_actions::mode::Editor;
    use ed_actions::mode::GitHistory;
    use gpui::px;

    use super::git_stripe_action;
    use super::stripe_click_hides;
    use super::stripe_width;
    use crate::WorkspaceMode;
    use crate::dock::STRIPE_BUTTON_SIZE;
    use crate::dock::STRIPE_NAMED_BUTTON_WIDTH;

    #[test]
    fn test_stripe_click_hides() {
        assert!(stripe_click_hides(true, false));
        assert!(!stripe_click_hides(true, true));
        assert!(!stripe_click_hides(false, false));
        assert!(!stripe_click_hides(false, true));
    }

    #[test]
    fn test_git_stripe_action() {
        assert!(git_stripe_action(WorkspaceMode::Editor).partial_eq(&GitHistory));
        assert!(git_stripe_action(WorkspaceMode::Terminal).partial_eq(&GitHistory));
        assert!(git_stripe_action(WorkspaceMode::GitHistory).partial_eq(&Editor));
    }

    #[test]
    fn test_stripe_width_fits_buttons() {
        assert_eq!(stripe_width(false), STRIPE_BUTTON_SIZE + px(4.));
        assert_eq!(stripe_width(true), STRIPE_NAMED_BUTTON_WIDTH + px(4.));
    }
}
