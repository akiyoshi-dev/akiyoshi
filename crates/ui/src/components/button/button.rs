use crate::components::button::button_like::ButtonLike;
use gpui::{App, ElementId, IntoElement, ParentElement, RenderOnce, SharedString, Window};
use gpui::prelude::FluentBuilder;

#[derive(IntoElement)]
pub struct Button {
    content: ButtonLike,
    label: Option<SharedString>,
}

impl Button {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            content: ButtonLike::new(id.into()),
            label: None,
        }
    }
}

impl RenderOnce for Button {
    #[allow(refining_impl_trait)]
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        self.content
            .when_some(self.label, |this, label| {
                this.child(label)
            })
    }
}