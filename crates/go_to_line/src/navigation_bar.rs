//! IntelliJ's navigation bar, in the status bar as in OpenIDEA's default:
//! `project › folders › file › symbols at the cursor`. The file reveals itself
//! in the Project tool window; a symbol moves the cursor to it.

use std::time::Duration;

use editor::Anchor;
use editor::Editor;
use editor::EditorEvent;
use editor::SelectionEffects;
use editor::scroll::Autoscroll;
use gpui::Entity;
use gpui::Focusable;
use gpui::SharedString;
use gpui::Subscription;
use gpui::Task;
use gpui::WeakEntity;
use project::Project;
use ui::Button;
use ui::Clickable;
use ui::Color;
use ui::Context;
use ui::IntoElement;
use ui::Label;
use ui::LabelCommon;
use ui::LabelSize;
use ui::ParentElement;
use ui::Render;
use ui::Styled;
use ui::Window;
use ui::h_flex;
use workspace::StatusItemView;
use workspace::Workspace;
use workspace::item::ItemHandle;
use workspace::pane::RevealInProjectPanel;

const UPDATE_DEBOUNCE: Duration = Duration::from_millis(50);
const SEPARATOR: &str = "›";

pub struct NavigationBar {
    /// Project name, then folders, then the file name.
    path: Vec<SharedString>,
    /// Symbols containing the cursor, outermost first, with where each starts.
    symbols: Vec<(SharedString, Anchor)>,
    editor: Option<WeakEntity<Editor>>,
    /// The project, not the workspace: the workspace is mid-update when the active item changes.
    project: WeakEntity<Project>,
    update_symbols: Task<()>,
    _observe_active_editor: Option<Subscription>,
}

impl NavigationBar {
    pub fn new(workspace: &Workspace) -> Self {
        Self {
            path: Vec::new(),
            symbols: Vec::new(),
            editor: None,
            project: workspace.project().downgrade(),
            update_symbols: Task::ready(()),
            _observe_active_editor: None,
        }
    }

    fn update_symbols(&mut self, editor: &Entity<Editor>, debounce: bool, window: &Window, cx: &Context<Self>) {
        let editor = editor.downgrade();
        self.update_symbols = cx.spawn_in(window, async move |bar, cx| {
            if debounce {
                cx.background_executor().timer(UPDATE_DEBOUNCE).await;
            }
            let symbols = editor
                .update(cx, |editor, cx| {
                    let snapshot = editor.buffer().read(cx).snapshot(cx);
                    let cursor = editor.selections.newest_anchor().head();
                    snapshot
                        .symbols_containing(cursor, None)
                        .map(|(_, items)| {
                            items
                                .into_iter()
                                .map(|item| (symbol_name(&item.text, &item.name_ranges), item.selection_range.start))
                                .collect()
                        })
                        .unwrap_or_default()
                })
                .unwrap_or_default();
            bar.update(cx, |bar, cx| {
                bar.symbols = symbols;
                cx.notify();
            })
            .ok();
        });
    }

    fn jump_to(&self, anchor: Anchor, window: &mut Window, cx: &mut Context<Self>) {
        let Some(editor) = self.editor.as_ref().and_then(WeakEntity::upgrade) else {
            return;
        };
        editor.update(cx, |editor, cx| {
            editor.change_selections(SelectionEffects::scroll(Autoscroll::center()), window, cx, |selections| {
                selections.select_ranges(Some(anchor..anchor));
            });
            window.focus(&editor.focus_handle(cx), cx);
        });
    }
}

/// A symbol's name from its outline text ("fn render" → "render"), else the whole text.
fn symbol_name(text: &str, name_ranges: &[std::ops::Range<usize>]) -> SharedString {
    let name: Vec<&str> = name_ranges.iter().filter_map(|range| text.get(range.clone())).collect();
    if name.is_empty() { text.to_string().into() } else { name.join(" ").into() }
}

/// The path part of the bar: the project name, then each component of the
/// file's path inside it.
fn path_segments(project_name: &str, relative_path: &str) -> Vec<SharedString> {
    std::iter::once(project_name)
        .chain(relative_path.split('/').filter(|segment| !segment.is_empty()))
        .map(|segment| SharedString::from(segment.to_string()))
        .collect()
}

impl Render for NavigationBar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let separator = || Label::new(SEPARATOR).size(LabelSize::Small).color(Color::Muted);
        let last_path_ix = self.path.len().saturating_sub(1);
        let mut bar = h_flex().min_w_0().overflow_x_hidden().gap_0p5();
        for (ix, segment) in self.path.iter().enumerate() {
            if ix > 0 {
                bar = bar.child(separator());
            }
            let button = Button::new(("nav-path", ix), segment.clone()).label_size(LabelSize::Small);
            bar = bar.child(if ix == last_path_ix {
                button.on_click(|_, window, cx| {
                    window.dispatch_action(Box::new(RevealInProjectPanel::default()), cx);
                })
            } else {
                button.color(Color::Muted)
            });
        }
        for (ix, (name, anchor)) in self.symbols.iter().enumerate() {
            let anchor = *anchor;
            bar = bar.child(separator()).child(
                Button::new(("nav-symbol", ix), name.clone())
                    .label_size(LabelSize::Small)
                    .color(Color::Muted)
                    .on_click(cx.listener(move |bar, _, window, cx| bar.jump_to(anchor, window, cx))),
            );
        }
        bar
    }
}

impl StatusItemView for NavigationBar {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.path = active_pane_item
            .and_then(|item| {
                let project_path = item.project_path(cx)?;
                let worktree = self.project.upgrade()?.read(cx).worktree_for_id(project_path.worktree_id, cx)?;
                let project_name = worktree.read(cx).root_name().as_std_path().to_string_lossy().into_owned();
                Some(path_segments(&project_name, &project_path.path.as_std_path().to_string_lossy()))
            })
            .unwrap_or_default();
        self.symbols.clear();

        match active_pane_item.and_then(|item| item.act_as::<Editor>(cx)) {
            Some(editor) if !self.path.is_empty() => {
                self.editor = Some(editor.downgrade());
                self._observe_active_editor =
                    Some(cx.subscribe_in(&editor, window, |bar, editor, event, window, cx| {
                        if let EditorEvent::SelectionsChanged { .. } | EditorEvent::Reparsed(_) = event {
                            bar.update_symbols(editor, true, window, cx);
                        }
                    }));
                self.update_symbols(&editor, false, window, cx);
            }
            _ => {
                self.editor = None;
                self._observe_active_editor = None;
            }
        }
        cx.notify();
    }
}

#[cfg(test)]
mod tests {
    use super::path_segments;
    use super::symbol_name;

    #[test]
    fn test_path_segments() {
        assert_eq!(path_segments("intelliED", "crates/ui/src/tab.rs"), ["intelliED", "crates", "ui", "src", "tab.rs"]);
        assert_eq!(path_segments("p", "a.rs"), ["p", "a.rs"]);
        assert_eq!(path_segments("p", ""), ["p"]);
    }

    #[test]
    fn test_symbol_name() {
        assert_eq!(symbol_name("fn render", &[3..9]).as_ref(), "render");
        assert_eq!(symbol_name("impl Render for Tab", &[5..11, 16..19]).as_ref(), "Render Tab");
        assert_eq!(symbol_name("mod tests", &[]).as_ref(), "mod tests");
        assert_eq!(symbol_name("x", &[5..9]).as_ref(), "x");
    }
}
