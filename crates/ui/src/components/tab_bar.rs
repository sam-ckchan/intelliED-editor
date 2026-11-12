use gpui::AnyElement;
use gpui::ScrollHandle;
use smallvec::SmallVec;

use crate::Tab;
use crate::TabStyle;
use crate::prelude::ActiveTheme;
use crate::prelude::App;
use crate::prelude::DynamicSpacing;
use crate::prelude::ElementId;
use crate::prelude::FluentBuilder;
use crate::prelude::InteractiveElement;
use crate::prelude::IntoElement;
use crate::prelude::ParentElement;
use crate::prelude::RenderOnce;
use crate::prelude::StatefulInteractiveElement;
use crate::prelude::Styled;
use crate::prelude::Window;
use crate::prelude::div;
use crate::prelude::h_flex;

#[derive(IntoElement)]
pub struct TabBar {
    id: ElementId,
    start_children: SmallVec<[AnyElement; 2]>,
    children: SmallVec<[AnyElement; 2]>,
    end_children: SmallVec<[AnyElement; 2]>,
    scroll_handle: Option<ScrollHandle>,
    style: TabStyle,
}

impl TabBar {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            start_children: SmallVec::new(),
            children: SmallVec::new(),
            end_children: SmallVec::new(),
            scroll_handle: None,
            style: TabStyle::Flat,
        }
    }

    pub const fn style(mut self, style: TabStyle) -> Self {
        self.style = style;
        self
    }

    pub fn track_scroll(mut self, scroll_handle: &ScrollHandle) -> Self {
        self.scroll_handle = Some(scroll_handle.clone());
        self
    }

    pub const fn start_children_mut(&mut self) -> &mut SmallVec<[AnyElement; 2]> {
        &mut self.start_children
    }

    pub fn start_children(mut self, start_children: impl IntoIterator<Item = impl IntoElement>) -> Self
    where
        Self: Sized,
    {
        self.start_children_mut().extend(start_children.into_iter().map(|child| child.into_any_element()));
        self
    }

    pub const fn end_children_mut(&mut self) -> &mut SmallVec<[AnyElement; 2]> {
        &mut self.end_children
    }

    pub fn end_children(mut self, end_children: impl IntoIterator<Item = impl IntoElement>) -> Self
    where
        Self: Sized,
    {
        self.end_children_mut().extend(end_children.into_iter().map(|child| child.into_any_element()));
        self
    }
}

impl ParentElement for TabBar {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl RenderOnce for TabBar {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
            .id(self.id)
            .group("tab_bar")
            .flex()
            .flex_none()
            .w_full()
            .h(Tab::container_height(cx))
            .map(|this| match self.style {
                // Flat tabs run edge to edge over a hairline, on the editor's colour.
                TabStyle::Flat => this
                    .bg(cx.theme().colors().tab_bar_background)
                    .border_b_1()
                    .border_color(cx.theme().colors().border),
                TabStyle::Pill => this.pl(DynamicSpacing::Base04.px(cx)),
            })
            .when(!self.start_children.is_empty(), |this| {
                this.child(
                    h_flex()
                        .flex_none()
                        .gap(DynamicSpacing::Base04.rems())
                        .px(DynamicSpacing::Base06.rems())
                        .children(self.start_children),
                )
            })
            .child(
                div().relative().flex_1().h_full().overflow_x_hidden().child(
                    h_flex()
                        .id("tabs")
                        .flex_grow_1()
                        .overflow_x_scroll()
                        .when_some(self.scroll_handle, |cx, scroll_handle| cx.track_scroll(&scroll_handle))
                        .children(self.children),
                ),
            )
            .when(!self.end_children.is_empty(), |this| {
                this.child(
                    h_flex()
                        .flex_none()
                        .gap(DynamicSpacing::Base04.rems())
                        .px(DynamicSpacing::Base06.rems())
                        .children(self.end_children),
                )
            })
    }
}
