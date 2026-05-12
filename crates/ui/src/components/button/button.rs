use crate::{clickable::Clickable, components::button::button_like::{ButtonLike, ButtonSize, ButtonVariant}, disableable::Disableable};
use gpui::{
    App, ElementId, IntoElement, ParentElement, RenderOnce, SharedString, StyleRefinement, Styled,
    Window, prelude::FluentBuilder,
};

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

    /// 设置按钮变体（Primary / Secondary / Outline / Ghost / Destructive / Link）。
    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.content = self.content.variant(variant);
        self
    }

    /// 设置按钮尺寸（Xs / Sm / Md / Lg）。
    pub fn size(mut self, size: ButtonSize) -> Self {
        self.content = self.content.size(size);
        self
    }

    /// 让按钮宽度撑满父容器。
    /// 等价于 `.w_full()`，只是语义更明确。
    pub fn full_width(self) -> Self {
        self.w_full()
    }
}

impl RenderOnce for Button {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        self.content
            .when_some(self.label, |this, label| this.child(label))
    }
}

impl Styled for Button {
    fn style(&mut self) -> &mut StyleRefinement {
        self.content.style()
    }
}

impl Clickable for Button {
    fn on_click<H>(mut self, handler: H) -> Self
    where
        H: crate::clickable::ClickHandlerFn,
    {
        self.content = self.content.on_click(handler);
        self
    }
}

impl Disableable for Button {
    fn disabled(mut self, disabled: bool) -> Self {
        self.content = self.content.disabled(disabled);
        self
    }
}
