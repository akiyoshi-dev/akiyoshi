use crate::clickable::{BoxedClickHandler, ClickHandlerFn, Clickable};
use gpui::{
    prelude::FluentBuilder,
    div,
    px,
    rgb,
    AnyElement,
    App,
    Div,
    ElementId,
    Hsla,
    InteractiveElement,
    IntoElement,
    ParentElement,
    RenderOnce,
    StatefulInteractiveElement,
    Styled,
    Window
};
use smallvec::SmallVec;
use theme::GlobalTheme;

#[derive(IntoElement)]
pub struct ButtonLike {
    /// 元素 ID，必须唯一，用于事件处理和样式应用。
    id: ElementId,
    /// 按钮的内容容器，允许添加文本或其他子元素。
    pub(super) content: Div,
    /// 可选的点击事件处理器，当按钮被点击时触发。
    on_click: Option<BoxedClickHandler>,
    /// 按钮的子元素列表，可以包含文本、图标或其他 UI 组件。
    children: SmallVec<[AnyElement; 2]>,
    /// 是否禁用按钮，如果为 `true`，按钮将不可点击并显示为禁用状态。
    pub disabled: bool,
}

impl ButtonLike {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            content: div(),
            on_click: None,
            children: SmallVec::new(),
            disabled: false,
        }
    }
}

impl RenderOnce for ButtonLike {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = GlobalTheme::theme(cx);

        self.content
            .id(self.id)
            .flex()
            .items_center()
            .justify_center()
            .bg(rgb(theme.styles.colors.primary.into()))
            .text_color(rgb(theme.styles.colors.text.inverted.into()))
            .text_size(px(theme.styles.typography.md))
            .px(px(theme.styles.spacing.xl * 1.5))
            .py(px(theme.styles.spacing.md))
            .rounded(px(theme.styles.radius.md))
            .border_1()
            .border_color(Hsla::from(theme.styles.colors.primary))
            // Subtle visual edge so it reads like a clickable control.
            .text_bg(rgb(theme.styles.colors.primary.into()))
            .when_some(
                self.on_click.filter(|_| !self.disabled),
                |this, on_click| {
                    this.on_click(move |event, window, cx| {
                        cx.stop_propagation();
                        on_click(event, window, cx);
                    })
                },
            )
            .children(self.children)
    }
}

impl Clickable for ButtonLike {
    fn on_click<H>(mut self, handler: H) -> Self
    where
        H: ClickHandlerFn,
    {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl ParentElement for ButtonLike {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}
