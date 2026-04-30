use crate::components::button::button_like::ButtonLike;
use gpui::{App, IntoElement, ParentElement, RenderOnce, SharedString, Window};

#[derive(IntoElement)]
pub struct Button {
    content: ButtonLike,
    label: SharedString,
}

impl Button {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            content: ButtonLike::new(),
            label: label.into(),
        }
    }
}

impl RenderOnce for Button {
    #[allow(refining_impl_trait)]
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        self.content
            .child(self.label)
    }
}