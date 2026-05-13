use crate::{
    clickable::{BoxedClickHandler, ClickHandlerFn, Clickable},
    disableable::Disableable,
    styled_ext::StyledExt,
};
use gpui::{
    AnyElement, App, Div, ElementId, Hsla, InteractiveElement, IntoElement, ParentElement,
    RenderOnce, StatefulInteractiveElement, StyleRefinement, Styled, Window, div,
    prelude::FluentBuilder, px,
};
use smallvec::SmallVec;
use theme::ActiveTheme;

/// 按钮尺寸，控制文字大小与内边距，适配不同场景（标题栏、工具栏、正文、突出操作）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonSize {
    /// 极小尺寸，适合密集工具栏或行内操作（高度约 15px）。
    Xs,
    /// 小尺寸，适合标题栏、侧边栏等紧凑场景（高度约 22px）。
    Sm,
    /// 默认尺寸，适合大多数正文场景（高度约 26px）。
    #[default]
    Md,
    /// 大尺寸，适合落地页、突出操作等场景（高度约 36px）。
    Lg,
}

impl ButtonSize {
    /// 返回 `(pad_y_px, pad_x_px)`，字号由 [`GlobalTheme::font_size`] 统一提供。
    fn padding(self, theme: &theme::Theme) -> (f32, f32) {
        let s = &theme.styles.spacing;
        match self {
            ButtonSize::Xs => (1.0, s.xs + 2.0),
            ButtonSize::Sm => (3.0, s.sm + 2.0),
            ButtonSize::Md => (s.xs, s.md),
            ButtonSize::Lg => (s.sm, s.lg),
        }
    }
}

pub enum ButtonVariant {
    /// 主按钮，通常用于强调主要操作，具有较高的视觉优先级。
    Primary,
    /// 描边按钮，通常用于次要操作，具有中等视觉优先级。
    Outline,
    /// 幽灵按钮，通常用于次要操作，具有较低视觉优先级。
    Ghost,
    /// 危险按钮，通常用于删除或危险操作，具有高视觉优先级。
    Destructive,
    /// 次按钮，通常用于辅助操作，具有中等视觉优先级。
    Secondary,
    /// 链接按钮，通常用于导航或链接操作，具有最低视觉优先级。
    Link,
}

/// 按钮样式定义了按钮的视觉属性，如背景颜色和边框颜色，通常根据按钮变体和状态（如默认、悬停、激活、禁用）从主题中派生。
pub(crate) struct ButtonVariantStyles {
    /// 按钮背景颜色，通常根据按钮变体和状态（如默认、悬停、激活、禁用）从主题中派生。
    pub background: Hsla,
    /// 按钮边框颜色，通常根据按钮变体和状态（如默认、悬停、激活、禁用）从主题中派生。
    pub border_color: Hsla,
    /// 按钮文本颜色。
    pub foreground: Hsla,
    /// 悬停背景颜色。
    pub hover_background: Hsla,
    /// 按下背景颜色。
    pub active_background: Hsla,
    /// 悬停边框颜色。
    pub hover_border: Hsla,
    /// 按下边框颜色。
    pub active_border: Hsla,
    /// 悬停时是否显示下划线（Link 变体专用）。
    pub hover_underline: bool,
}

#[derive(IntoElement)]
pub struct ButtonLike {
    /// 元素 ID，必须唯一，用于事件处理和样式应用。
    id: ElementId,
    /// 当前元素内容
    content: Div,
    /// 按钮变体，决定按钮的视觉风格和交互行为，例如主按钮、次按钮等。
    variant: ButtonVariant,
    /// 按钮尺寸，决定文字大小和内边距。
    size: ButtonSize,
    /// 用户自定义样式，优先级高于主题默认值。通过 [`Styled`] trait 访问，无需直接操作此字段。
    style: StyleRefinement,
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
            variant: ButtonVariant::Primary,
            size: ButtonSize::default(),
            style: StyleRefinement::default(),
            on_click: None,
            children: SmallVec::new(),
            disabled: false,
        }
    }

    /// 设置按钮的变体，决定按钮的视觉风格和交互行为，例如主按钮、次按钮等。
    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    /// 设置按钮尺寸（Xs / Sm / Md / Lg）。
    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }
}

impl RenderOnce for ButtonLike {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let variant_styles = self.variant.styles(cx);
        let (pad_y, pad_x) = self.size.padding(theme);
        let text_size = cx.font_size();
        let line_height = cx.line_height();

        // 先建立主题默认样式的基础 div，最后用 refine_style 合并用户样式（用户优先）
        self.content
            .h_flex()
            .id(self.id)
            .items_center()
            .justify_center()
            .bg(variant_styles.background)
            .text_color(variant_styles.foreground)
            .text_size(px(text_size))
            .line_height(px(line_height))
            .pl(px(pad_x))
            .pr(px(pad_x))
            .pt(px(pad_y))
            .pb(px(pad_y))
            .rounded(px(theme.styles.radius.md))
            .border_1()
            .border_color(variant_styles.border_color)
            .when(self.disabled, |this| this.cursor_not_allowed().opacity(0.6))
            .when(!self.disabled, |this| {
                this.cursor_pointer()
                    .hover(|style| {
                        let style = style
                            .bg(variant_styles.hover_background)
                            .border_color(variant_styles.hover_border);
                        if variant_styles.hover_underline {
                            style.underline()
                        } else {
                            style
                        }
                    })
                    .active(|style| {
                        style
                            .bg(variant_styles.active_background)
                            .border_color(variant_styles.active_border)
                    })
            })
            .refine_style(&self.style)
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

impl Styled for ButtonLike {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl Disableable for ButtonLike {
    fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl ButtonVariant {
    fn styles(&self, cx: &App) -> ButtonVariantStyles {
        let c = &cx.theme().styles.colors;
        let transparent = Hsla {
            h: 0.,
            s: 0.,
            l: 0.,
            a: 0.,
        };

        match self {
            ButtonVariant::Primary => Self::solid(c.primary.into(), c.text.inverted.into()),
            ButtonVariant::Secondary => Self::solid(c.secondary.into(), c.text.primary.into()),
            ButtonVariant::Destructive => Self::solid(c.danger.into(), c.text.inverted.into()),
            ButtonVariant::Outline => Self::surface_hover(
                transparent,
                c.border.default.into(),
                c.text.primary.into(),
                c.surface.hover.into(),
                c.surface.active.into(),
            ),
            ButtonVariant::Ghost => Self::surface_hover(
                transparent,
                transparent,
                c.text.primary.into(),
                c.surface.hover.into(),
                c.surface.active.into(),
            ),
            ButtonVariant::Link => ButtonVariantStyles {
                background: transparent,
                border_color: transparent,
                foreground: c.primary.into(),
                hover_background: transparent,
                active_background: transparent,
                hover_border: transparent,
                active_border: transparent,
                hover_underline: true,
            },
        }
    }

    /// 实色背景变体（Primary / Secondary / Destructive）：
    /// hover/active 通过降低 alpha 制造蒙层压暗效果。
    fn solid(bg: Hsla, fg: Hsla) -> ButtonVariantStyles {
        let with_alpha = |mut c: Hsla, a: f32| -> Hsla {
            c.a = a;
            c
        };
        ButtonVariantStyles {
            background: bg,
            border_color: bg,
            foreground: fg,
            hover_background: with_alpha(bg, 0.90),
            active_background: with_alpha(bg, 0.80),
            hover_border: with_alpha(bg, 0.90),
            active_border: with_alpha(bg, 0.80),
            hover_underline: false,
        }
    }

    /// 透明/描边变体（Outline / Ghost）：
    /// hover/active 使用 surface token，边框保持不变。
    fn surface_hover(
        background: Hsla,
        border: Hsla,
        fg: Hsla,
        hover_bg: Hsla,
        active_bg: Hsla,
    ) -> ButtonVariantStyles {
        ButtonVariantStyles {
            background,
            border_color: border,
            foreground: fg,
            hover_background: hover_bg,
            active_background: active_bg,
            hover_border: border,
            active_border: border,
            hover_underline: false,
        }
    }
}
