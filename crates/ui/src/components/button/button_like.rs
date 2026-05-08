use gpui::{div, px, rgb, AnyElement, App, Div, ElementId, InteractiveElement, IntoElement, ParentElement, RenderOnce, Styled, Window};
use smallvec::SmallVec;

#[derive(IntoElement)]
pub struct ButtonLike {
    id: ElementId,
    pub(super) content: Div,
    children: SmallVec<[AnyElement; 2]>,
}

impl ButtonLike {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            content: div(),
            children: SmallVec::new(),
        }
    }
}

impl RenderOnce for ButtonLike {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        self.content
            .id(self.id)
            .bg(rgb(0x505050))
            .text_color(rgb(0xffffff))
            .size(px(120.))
            .children(self.children)
    }
}

impl ParentElement for ButtonLike {
    fn extend(&mut self, elements: impl IntoIterator<Item=AnyElement>) {
        self.children.extend(elements);
    }
}
