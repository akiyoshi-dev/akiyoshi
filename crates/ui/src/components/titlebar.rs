use gpui::{
    AnyElement, InteractiveElement, IntoElement, ParentElement, Pixels, RenderOnce,
    StyleRefinement, Styled, TitlebarOptions, div, point, prelude::FluentBuilder, px,
};
use smallvec::SmallVec;
use theme::ActiveTheme;

use crate::styled_ext::StyledExt;

/// 标题栏组件 ID
const TITLEBAR_ID: &str = "titlebar";
/// 标题栏高度
const TITLEBAR_HEIGHT: Pixels = px(34.);

/// 标题栏内文字大小（比正文 md/15px 略小，营造层级感）
const TITLEBAR_TEXT_SIZE: Pixels = px(13.);

/// 在 macOS 上，标题栏左侧的内边距
#[cfg(target_os = "macos")]
const TITLEBAR_PADDING_LEFT: Pixels = px(80.);

/// 在 Windows 上，标题栏左侧的内边距
#[cfg(not(target_os = "macos"))]
const TITLEBAR_PADDING_LEFT: Pixels = px(12.);

/// 在 macOS 上，标题栏左侧的内边距
const MACOS_TRAFFIC_LIGHT_POSITION: gpui::Point<Pixels> = point(px(9.), px(9.));

#[derive(IntoElement)]
pub struct Titlebar {
    /// 自定义样式
    style: StyleRefinement,
    /// 子元素
    children: SmallVec<[AnyElement; 2]>,
}

impl Titlebar {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            children: SmallVec::new(),
        }
    }
}

impl Into<TitlebarOptions> for Titlebar {
    fn into(self) -> TitlebarOptions {
        TitlebarOptions {
            title: None,
            appears_transparent: true,
            traffic_light_position: Some(MACOS_TRAFFIC_LIGHT_POSITION),
        }
    }
}

impl RenderOnce for Titlebar {
    fn render(self, window: &mut gpui::Window, cx: &mut gpui::App) -> impl IntoElement {
        let theme = cx.theme();

        // 获取窗口是否处于全屏模式
        let fullscreen = window.is_fullscreen();

        div().flex_shrink_0().child(
            div()
                .id(TITLEBAR_ID)
                .h_flex()
                .items_center()
                .h(TITLEBAR_HEIGHT)
                .when(!fullscreen, |this| this.pl(TITLEBAR_PADDING_LEFT))
                .when(fullscreen, |this| this.pl(px(theme.styles.spacing.md)))
                .border_b_1()
                .border_color(theme.styles.colors.border.default)
                .bg(theme.styles.colors.surface.elevated.rgb())
                // 标题栏文字比内容区小一档，颜色用 text.secondary（略 muted），
                // 让主内容区的 text.primary 在视觉层级上更突出。
                .text_color(theme.styles.colors.text.secondary.rgb())
                .text_size(TITLEBAR_TEXT_SIZE)
                .refine_style(&self.style)
                .children(self.children),
        )
    }
}

impl ParentElement for Titlebar {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for Titlebar {
    #[doc = " Returns a reference to the style memory of this element."]
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
