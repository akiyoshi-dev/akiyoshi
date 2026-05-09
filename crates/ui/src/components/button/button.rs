use crate::{
    clickable::{ClickHandlerFn, Clickable},
    components::button::button_like::ButtonLike,
};
use gpui::{App, ElementId, IntoElement, ParentElement, RenderOnce, SharedString, Window, prelude::FluentBuilder, Styled, StyleRefinement};

/// 按钮组件。
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

    /// 设置按钮的标签。
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }
}

impl RenderOnce for Button {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        self.content
            .when_some(self.label, |this, label| this.child(label))
    }
}

impl Clickable for Button {
    fn on_click<H>(mut self, handler: H) -> Self
    where
        H: ClickHandlerFn,
    {
        self.content = self.content.on_click(handler);
        self
    }
}

impl Styled for Button {
    fn style(&mut self) -> &mut StyleRefinement {
        self.content.content.style()
    }
}
