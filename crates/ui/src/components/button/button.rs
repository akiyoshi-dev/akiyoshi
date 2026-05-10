use crate::{clickable::Clickable, components::button::button_like::ButtonLike};
use gpui::{App, ClickEvent, Context, ElementId, IntoElement, ParentElement, RenderOnce, SharedString, Window, prelude::FluentBuilder, Styled, StyleRefinement};

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

    /// 为按钮添加点击事件处理器。
    ///
    /// 闭包签名为 `|this: &mut V, event: &ClickEvent, window: &mut Window, cx: &mut Context<V>|`，
    /// 无需在调用方手动写 `cx.listener(...)`。
    pub fn on_click<V, F>(mut self, cx: &Context<V>, handler: F) -> Self
    where
        V: 'static,
        F: Fn(&mut V, &ClickEvent, &mut Window, &mut Context<V>) + 'static,
    {
        self.content = self.content.on_click(cx.listener(handler));
        self
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
