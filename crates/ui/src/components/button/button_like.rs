use crate::clickable::{BoxedClickHandler, ClickHandlerFn, Clickable};
use gpui::{
    div, prelude::FluentBuilder, px, rgb, AnyElement, App, ElementId, Hsla,
    InteractiveElement, IntoElement, ParentElement, Refineable, RenderOnce, StatefulInteractiveElement,
    StyleRefinement, Styled, Window,
};
use smallvec::SmallVec;
use theme::GlobalTheme;

#[derive(IntoElement)]
pub struct ButtonLike {
    /// 元素 ID，必须唯一，用于事件处理和样式应用。
    id: ElementId,
    /// 用户自定义样式，优先级高于主题默认值。
    pub(super) user_style: StyleRefinement,
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
            user_style: StyleRefinement::default(),
            on_click: None,
            children: SmallVec::new(),
            disabled: false,
        }
    }
}

impl RenderOnce for ButtonLike {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = GlobalTheme::theme(cx);
        let primary = theme.styles.colors.primary;

        // 先建立主题默认样式的基础 div
        let mut base = div()
            .id(self.id)
            .flex()
            .items_center()
            .justify_center()
            .bg(rgb(primary.into()))
            .text_color(rgb(theme.styles.colors.text.inverted.into()))
            .text_size(px(theme.styles.typography.md))
            .pl(px(theme.styles.spacing.md))
            .pr(px(theme.styles.spacing.md))
            .rounded(px(theme.styles.radius.md))
            .border_1()
            .border_color(Hsla::from(primary))
            .when(self.disabled, |this| this.cursor_not_allowed().opacity(0.6))
            .when(!self.disabled, |this| {
                // 仅在 hover/active 时轻微压暗。
                this.cursor_pointer()
                    .hover(|style| style.bg(primary.darken(0.08).rgb()))
                    .active(|style| style.bg(primary.darken(0.08).rgb()))
            });

        // 将用户自定义样式覆盖到主题默认值之上
        base.style().refine(&self.user_style);

        // 绑定点击事件
        base.when_some(
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

impl Styled for ButtonLike {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.user_style
    }
}
