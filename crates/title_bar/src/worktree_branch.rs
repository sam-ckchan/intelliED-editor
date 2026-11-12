use ed_actions::git::Branch;
use ed_actions::git::Worktree;
use git_ui_core::worktree_picker::WorktreePicker;
use gpui::AnyElement;
use gpui::AppContext as _;
use gpui::Context;
use gpui::Entity;
use gpui::IntoElement;
use gpui::ParentElement;
use project::git_store::Repository;
use ui::PopoverMenu;
use ui::TintColor;
use ui::Tooltip;
use ui::prelude::Button;
use ui::prelude::ButtonCommon;
use ui::prelude::ButtonSize;
use ui::prelude::ButtonStyle;
use ui::prelude::Color;
use ui::prelude::FluentBuilder;
use ui::prelude::Icon;
use ui::prelude::IconName;
use ui::prelude::IconSize;
use ui::prelude::Label;
use ui::prelude::LabelCommon;
use ui::prelude::SelectableButton;
use ui::prelude::SharedString;
use ui::prelude::h_flex;

use crate::TitleBar;

const MAX_BRANCH_NAME_LENGTH: usize = 40;
const MAX_SHORT_SHA_LENGTH: usize = 8;

/// The worktree picker's label: the worktree being created or loaded while
/// that is in progress, else the linked worktree's name, or "main".
fn worktree_display_label(
    linked_worktree_name: Option<SharedString>,
    creation: Option<&SharedString>,
    is_switch: bool,
) -> SharedString {
    match creation {
        Some(name) if is_switch => format!("Loading {name}…").into(),
        Some(name) => format!("Creating {name}…").into(),
        None => linked_worktree_name.unwrap_or_else(|| "main".into()),
    }
}

/// Whether the worktree picker shows before the branch: only in a linked
/// worktree, or while one is being created or loaded. In the main worktree the
/// toolbar shows just the branch, as IntelliJ does; `git::Worktree` still opens the picker.
const fn shows_worktree_picker(in_linked_worktree: bool, creating: bool) -> bool {
    in_linked_worktree || creating
}

/// A text button in the window header: the top-row height, the tab label's
/// size, and a muted 14px leading icon, so the project name, worktree and
/// branch read as one row with the header's icon buttons.
pub(crate) fn header_text_button(id: &'static str, label: impl Into<SharedString>) -> Button {
    Button::new(id, label)
        .size(ButtonSize::Medium)
        .selected_style(ButtonStyle::Tinted(TintColor::Accent))
        .tab_index(0isize)
}

impl TitleBar {
    /// The worktree and branch pickers, after the project name, as
    /// `worktree / branch`. `None` when the workspace is gone.
    pub(crate) fn render_worktree_and_branch(
        &self,
        repository: Entity<Repository>,
        linked_worktree_name: Option<SharedString>,
        cx: &Context<TitleBar>,
    ) -> Option<AnyElement> {
        let workspace = self.workspace.upgrade()?;

        let (branch_name, is_detached_head) = {
            let repo = repository.read(cx);
            let branch_name = repo
                .branch
                .as_ref()
                .map(|branch| util::truncate_and_trailoff(branch.name(), MAX_BRANCH_NAME_LENGTH))
                .or_else(|| {
                    repo.head_commit
                        .as_ref()
                        .map(|commit| commit.sha.chars().take(MAX_SHORT_SHA_LENGTH).collect::<String>())
                });
            (branch_name, repo.branch.is_none())
        };

        let show_worktree = shows_worktree_picker(
            linked_worktree_name.is_some(),
            workspace.read(cx).active_worktree_creation().label.is_some(),
        );
        let worktree_button = {
            let creation = workspace.read(cx).active_worktree_creation();
            let is_creating = creation.label.is_some();
            let worktree_label = linked_worktree_name.clone().unwrap_or_else(|| "main".into());
            let label = worktree_display_label(linked_worktree_name, creation.label.as_ref(), creation.is_switch);
            let project = self.project.clone();
            let workspace_handle = workspace.downgrade();
            PopoverMenu::new("worktree-picker-menu")
                .menu(move |window, cx| {
                    Some(cx.new(|cx| WorktreePicker::new(project.clone(), workspace_handle.clone(), window, cx)))
                })
                .trigger_with_tooltip(
                    header_text_button("worktree_picker_trigger", label)
                        .color(Color::Muted)
                        .loading(is_creating)
                        .start_icon(Icon::new(IconName::GitWorktree).size(IconSize::Small).color(Color::Muted)),
                    move |_window, cx| {
                        Tooltip::with_meta(
                            "Worktree",
                            Some(&Worktree),
                            format!("Currently In Use: {worktree_label}"),
                            cx,
                        )
                    },
                )
                .anchor(gpui::Anchor::TopLeft)
        };

        let branch_picker = branch_name.map(|branch_name| {
            let tooltip_meta = if is_detached_head {
                format!("Detached HEAD: {branch_name}")
            } else {
                format!("Currently Checked Out: {branch_name}")
            };
            let trigger = if is_detached_head {
                header_text_button("project_branch_trigger", "Create Branch")
                    .start_icon(Icon::new(IconName::GitBranchPlus).size(IconSize::Small).color(Color::Muted))
            } else {
                header_text_button("project_branch_trigger", branch_name)
                    .start_icon(Icon::new(IconName::GitBranch).size(IconSize::Small).color(Color::Muted))
                    .end_icon(Icon::new(IconName::ChevronDown).size(IconSize::XSmall).color(Color::Muted))
            };
            PopoverMenu::new("branch-menu")
                .menu(move |window, cx| {
                    git_ui_core::build_branch_picker(workspace.downgrade(), Some(repository.clone()), window, cx)
                })
                .trigger_with_tooltip(trigger, move |_window, cx| {
                    Tooltip::with_meta("Branch & Stash", Some(&Branch), tooltip_meta.clone(), cx)
                })
                .anchor(gpui::Anchor::TopLeft)
        });

        Some(
            h_flex()
                .when(show_worktree, |this| {
                    this.child(worktree_button).child(Label::new("/").color(Color::Muted).alpha(0.4))
                })
                .children(branch_picker)
                .into_any_element(),
        )
    }
}

#[cfg(test)]
mod tests {
    use gpui::SharedString;

    use super::worktree_display_label;

    #[test]
    fn test_shows_worktree_picker() {
        assert!(!super::shows_worktree_picker(false, false));
        assert!(super::shows_worktree_picker(true, false));
        assert!(super::shows_worktree_picker(false, true));
    }

    #[test]
    fn test_worktree_display_label() {
        assert_eq!(worktree_display_label(None, None, false), SharedString::from("main"));
        assert_eq!(worktree_display_label(Some("feature".into()), None, false), SharedString::from("feature"));
        let name = SharedString::from("next");
        assert_eq!(worktree_display_label(None, Some(&name), false), SharedString::from("Creating next…"));
        assert_eq!(
            worktree_display_label(Some("feature".into()), Some(&name), true),
            SharedString::from("Loading next…")
        );
    }
}
