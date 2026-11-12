use gpui::AnyElement;
use gpui::IntoElement;
use gpui::Stateful;
use smallvec::SmallVec;

use crate::prelude::ActiveTheme;
use crate::prelude::App;
use crate::prelude::Div;
use crate::prelude::DynamicSpacing;
use crate::prelude::ElementId;
use crate::prelude::FluentBuilder;
use crate::prelude::InteractiveElement;
use crate::prelude::ParentElement;
use crate::prelude::Pixels;
use crate::prelude::RenderOnce;
use crate::prelude::StatefulInteractiveElement;
use crate::prelude::Styled;
use crate::prelude::Toggleable;
use crate::prelude::Window;
use crate::prelude::div;
use crate::prelude::h_flex;
use crate::prelude::px;

const START_TAB_SLOT_SIZE: Pixels = px(12.);
const END_TAB_SLOT_SIZE: Pixels = px(14.);
/// Corner radius of a tab.
const TAB_RADIUS: Pixels = px(6.);
/// Height of a tab within its 40px row.
const TAB_HEIGHT: Pixels = px(28.);
const TAB_PADDING_X: Pixels = px(10.);

#[derive(IntoElement)]
pub struct Tab {
    div: Stateful<Div>,
    selected: bool,
    floating: bool,
    start_slot: Option<AnyElement>,
    end_slot: Option<AnyElement>,
    children: SmallVec<[AnyElement; 2]>,
    overlays: SmallVec<[AnyElement; 2]>,
}

impl Tab {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            div: div().id(id.into()),
            selected: false,
            floating: false,
            start_slot: None,
            end_slot: None,
            children: SmallVec::new(),
            overlays: SmallVec::new(),
        }
    }

    /// Drawn away from a tab bar (a drag preview): filled, with no hover.
    pub const fn floating(mut self) -> Self {
        self.floating = true;
        self
    }

    pub fn start_slot<E: IntoElement>(mut self, element: impl Into<Option<E>>) -> Self {
        self.start_slot = element.into().map(IntoElement::into_any_element);
        self
    }

    pub fn end_slot<E: IntoElement>(mut self, element: impl Into<Option<E>>) -> Self {
        self.end_slot = element.into().map(IntoElement::into_any_element);
        self
    }

    /// Positioned against the tab's shape (it is `relative`) and painted behind its content,
    /// e.g. a drop indicator.
    pub fn overlay(mut self, overlay: impl IntoElement) -> Self {
        self.overlays.push(overlay.into_any_element());
        self
    }

    /// The gap between neighbouring tabs.
    pub const fn spacing(_cx: &App) -> Pixels {
        px(2.)
    }

    /// Height of the tab row (and of tool window headers).
    pub fn container_height(cx: &App) -> Pixels {
        DynamicSpacing::Base40.px(cx)
    }
}

impl InteractiveElement for Tab {
    fn interactivity(&mut self) -> &mut gpui::Interactivity {
        self.div.interactivity()
    }
}

impl StatefulInteractiveElement for Tab {}

impl Toggleable for Tab {
    fn toggle_state(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
}

impl ParentElement for Tab {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl RenderOnce for Tab {
    #[allow(refining_impl_trait)]
    fn render(self, _: &mut Window, cx: &mut App) -> Stateful<Div> {
        let colors = cx.theme().colors();
        let (selected_bg, selected_border, hover_bg, floating_bg) = (
            colors.tab_active_background,
            colors.border_selected,
            colors.ghost_element_hover,
            colors.elevated_surface_background,
        );
        let start_slot = h_flex().size(START_TAB_SLOT_SIZE).justify_center().children(self.start_slot);
        let end_slot = h_flex().size(END_TAB_SLOT_SIZE).justify_center().children(self.end_slot);

        // Every label is full contrast; the selected tab is a filled, outlined pill.
        self.div.h(Tab::container_height(cx)).flex().items_center().pr(Tab::spacing(cx)).cursor_pointer().child(
            h_flex()
                .group("")
                .relative()
                .h(TAB_HEIGHT)
                .rounded(TAB_RADIUS)
                .px(TAB_PADDING_X)
                .gap(DynamicSpacing::Base06.rems())
                .text_color(colors.text)
                .map(|this| {
                    if self.selected {
                        this.bg(selected_bg).border_1().border_color(selected_border)
                    } else if self.floating {
                        this.bg(floating_bg)
                    } else {
                        this.hover(|style| style.bg(hover_bg))
                    }
                })
                .children(self.overlays)
                .child(start_slot)
                .children(self.children)
                .child(end_slot),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::TAB_HEIGHT;
    use super::Tab;

    #[test]
    fn test_floating() {
        assert!(!Tab::new("tab").floating);
        assert!(Tab::new("tab").floating().floating);
    }

    #[test]
    fn test_tab_fits_row() {
        assert!(TAB_HEIGHT < gpui::px(40.));
    }
}
