//! The main toolbar: the window's top row, above every mode, like IntelliJ's.
//! Left to right: room for the macOS traffic lights, the project name with the
//! worktree and branch pickers, Go Back / Go Forward, then, at the right end,
//! Search Everywhere and Find Action. Its empty space moves the window.

use ed_actions::command_palette::Toggle as ToggleCommandPalette;
use gpui::AnyElement;
use gpui::Context;
use gpui::Entity;
use gpui::IntoElement;
use gpui::Pixels;
use gpui::Render;
use gpui::Subscription;
use gpui::TaskExt as _;
use gpui::WeakEntity;
use gpui::Window;
use gpui::div;
use ui::Tooltip;
use ui::prelude::ActiveTheme;
use ui::prelude::ButtonCommon;
use ui::prelude::Clickable;
use ui::prelude::Disableable;
use ui::prelude::IconName;
use ui::prelude::InteractiveElement;
use ui::prelude::ParentElement;
use ui::prelude::Styled;
use ui::prelude::h_flex;
use ui::prelude::px;
use ui::utils::TRAFFIC_LIGHT_PADDING;
use ui::utils::window_drag_area;
use util::ResultExt;

use crate::GoBack;
use crate::GoForward;
use crate::MAIN_TOOLBAR_HEIGHT;
use crate::ToggleFileFinder;
use crate::Workspace;
use crate::pane::top_row_icon_button;

/// Padding at the main toolbar's right end, and at its left end in fullscreen.
const MAIN_TOOLBAR_PADDING: Pixels = px(8.);

/// Where the main toolbar's content starts: after the traffic lights, which
/// fullscreen windows hide.
const fn main_toolbar_start(fullscreen: bool) -> Pixels {
    if fullscreen { MAIN_TOOLBAR_PADDING } else { px(TRAFFIC_LIGHT_PADDING) }
}

/// The main toolbar. Redraws only when what it shows changes.
pub(crate) struct WindowHeader {
    workspace: WeakEntity<Workspace>,
    shown: WindowHeaderState,
    _subscription: Subscription,
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
struct WindowHeaderState {
    can_go_back: bool,
    can_go_forward: bool,
}

impl WindowHeaderState {
    fn of(workspace: &Workspace) -> WindowHeaderState {
        WindowHeaderState { can_go_back: workspace.can_go_back(), can_go_forward: workspace.can_go_forward() }
    }
}

impl WindowHeader {
    pub(crate) fn new(workspace: &Entity<Workspace>, cx: &mut Context<WindowHeader>) -> WindowHeader {
        let subscription = cx.observe(workspace, |header: &mut WindowHeader, workspace, cx| {
            if WindowHeaderState::of(workspace.read(cx)) != header.shown {
                cx.notify();
            }
        });
        WindowHeader {
            workspace: workspace.downgrade(),
            shown: WindowHeaderState::default(),
            _subscription: subscription,
        }
    }
}

impl Render for WindowHeader {
    fn render(&mut self, window: &mut Window, cx: &mut Context<WindowHeader>) -> impl IntoElement {
        let Some(workspace) = self.workspace.upgrade() else {
            return div().into_any_element();
        };
        let workspace = workspace.read(cx);
        self.shown = WindowHeaderState::of(workspace);
        let fullscreen = window.is_fullscreen() || window.is_simple_fullscreen();
        let colors = cx.theme().colors();

        window_drag_area(h_flex().id("main-toolbar"))
            .flex_none()
            .w_full()
            .h(MAIN_TOOLBAR_HEIGHT)
            .pl(main_toolbar_start(fullscreen))
            .pr(MAIN_TOOLBAR_PADDING)
            .gap_2()
            .bg(colors.title_bar_background)
            // The hairline under the toolbar.
            .border_b_1()
            .border_color(colors.background)
            .children(workspace.render_titlebar_region())
            .child(controls().child(render_nav_buttons(&self.workspace, self.shown)))
            .child(div().flex_1())
            .child(controls().child(render_search_buttons()))
            .into_any_element()
    }
}

/// A group of toolbar controls: clicks on them must not move the window.
fn controls() -> gpui::Div {
    h_flex().on_mouse_down(gpui::MouseButton::Left, |_, _, cx| cx.stop_propagation())
}

/// Go Back / Go Forward over the workspace's one history, which spans every
/// pane in every mode.
fn render_nav_buttons(workspace: &WeakEntity<Workspace>, state: WindowHeaderState) -> impl IntoElement {
    let back_workspace = workspace.clone();
    let go_back = top_row_icon_button("go-back", IconName::ArrowLeft)
        .disabled(!state.can_go_back)
        .tooltip(|_window, cx| Tooltip::for_action("Back", &GoBack, cx))
        .on_click(move |_, window, cx| {
            back_workspace.update(cx, |workspace, cx| workspace.go_back(window, cx).detach_and_log_err(cx)).log_err();
        });

    let forward_workspace = workspace.clone();
    let go_forward = top_row_icon_button("go-forward", IconName::ArrowRight)
        .disabled(!state.can_go_forward)
        .tooltip(|_window, cx| Tooltip::for_action("Forward", &GoForward, cx))
        .on_click(move |_, window, cx| {
            forward_workspace
                .update(cx, |workspace, cx| workspace.go_forward(window, cx).detach_and_log_err(cx))
                .log_err();
        });

    h_flex().gap_1().child(go_back).child(go_forward)
}

/// Search Everywhere (the file finder) and Find Action (the command palette).
fn render_search_buttons() -> impl IntoElement {
    let search = top_row_icon_button("search-everywhere", IconName::MagnifyingGlass)
        .tooltip(|_window, cx| Tooltip::for_action("Search Everywhere", &ToggleFileFinder::default(), cx))
        .on_click(|_, window, cx| window.dispatch_action(Box::new(ToggleFileFinder::default()), cx));
    let find_action = top_row_icon_button("find-action", IconName::Settings)
        .tooltip(|_window, cx| Tooltip::for_action("Find Action", &ToggleCommandPalette, cx))
        .on_click(|_, window, cx| window.dispatch_action(Box::new(ToggleCommandPalette), cx));
    h_flex().gap_1().child(search).child(find_action)
}

impl Workspace {
    /// The title bar item (project, worktree and branch) inside its focus region.
    /// The region is a toolbar so region navigation (FocusNextPart) can reach
    /// its controls: it lands on the first control, Tab steps through them,
    /// and arrow keys move between them once focus is inside.
    pub(crate) fn render_titlebar_region(&self) -> Option<AnyElement> {
        let item = self.titlebar_item.clone()?;
        let workspace = self.weak_self.clone();
        Some(
            div()
                .id("titlebar-region")
                .track_focus(&self.titlebar_focus_handle)
                .tab_group()
                .on_key_down(move |event: &gpui::KeyDownEvent, window, cx| {
                    if event.keystroke.modifiers.modified() {
                        return;
                    }
                    let forward = match event.keystroke.key.as_str() {
                        "right" => true,
                        "left" => false,
                        _ => return,
                    };
                    workspace
                        .update(cx, |workspace, cx| workspace.move_titlebar_item_focus(forward, window, cx))
                        .log_err();
                    cx.stop_propagation();
                })
                // Clicks on the project controls must not move the window.
                .on_mouse_down(gpui::MouseButton::Left, |_, _, cx| cx.stop_propagation())
                .child(item)
                .into_any_element(),
        )
    }
}

#[cfg(test)]
mod tests {
    use gpui::px;
    use ui::utils::TRAFFIC_LIGHT_PADDING;

    use super::MAIN_TOOLBAR_PADDING;
    use super::main_toolbar_start;
    use crate::MAIN_TOOLBAR_HEIGHT;
    use crate::pane::TOP_ROW_BUTTON_SIZE;

    #[test]
    fn test_main_toolbar_start() {
        assert_eq!(main_toolbar_start(false), px(TRAFFIC_LIGHT_PADDING));
        assert_eq!(main_toolbar_start(true), MAIN_TOOLBAR_PADDING);
    }

    #[test]
    fn test_buttons_fit_main_toolbar() {
        assert!(TOP_ROW_BUTTON_SIZE < MAIN_TOOLBAR_HEIGHT);
    }
}
