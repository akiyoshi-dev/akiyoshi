use crate::components::button::button_like::ButtonLike;
use crate::clickable::Clickable;
use gpui::{App, ClickEvent, ElementId, IntoElement, ParentElement, RenderOnce, SharedString, Window};
use gpui::prelude::FluentBuilder;
use theme::{Theme, ThemeKind};

/// 按钮组件。
#[derive(IntoElement)]
pub struct Button {
    content: ButtonLike,
    label: Option<SharedString>,
    theme_kind: Option<ThemeKind>,
}

impl Button {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            content: ButtonLike::new(id.into()),
            label: None,
            theme_kind: None,
        }
    }

    /// 设置按钮的标签。
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// 覆盖当前按钮使用的主题模式（默认跟随全局主题）。
    pub fn theme_kind(mut self, kind: ThemeKind) -> Self {
        self.theme_kind = Some(kind);
        self
    }
}

impl RenderOnce for Button {
    fn render(mut self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        if let Some(kind) = self.theme_kind {
            self.content = self.content.theme(Theme::from_kind(kind));
        }

        self.content
            .when_some(self.label, |this, label| {
                this.child(label)
            })
    }
}

impl Clickable for Button {
    fn on_click(mut self, handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        self.content = self.content.on_click(handler);
        self
    }
}
