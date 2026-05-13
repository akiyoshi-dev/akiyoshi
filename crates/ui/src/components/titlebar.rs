use gpui::{
    AnyElement, InteractiveElement, IntoElement, ParentElement, Pixels, RenderOnce,
    StyleRefinement, Styled, Window, div, prelude::FluentBuilder, px,
};
use smallvec::SmallVec;
use theme::ActiveTheme;

use crate::styled_ext::StyledExt;

/// 标题栏高度（px）
const TITLEBAR_HEIGHT: Pixels = px(40.);

/// 标题栏 ID
const TITLEBAR_ID: &str = "titlebar";

/// macOS 非全屏时，左侧为红绿灯按钮预留的内边距。
///
/// 数值 80px = 9px 左边距 + 3 个 12px 按钮 + 两个 8px 间距 + 若干余量。
#[cfg(target_os = "macos")]
const TITLEBAR_LEFT_PADDING: Pixels = px(80.);

/// 非 macOS 或全屏时的左侧内边距，与主题 `spacing.md` 对齐。
#[cfg(not(target_os = "macos"))]
const TITLEBAR_LEFT_PADDING: Pixels = px(12.);

/// 标题栏组件。
///
/// 渲染在窗口内容区顶部，作为应用的视觉顶栏，可承载主题切换、
/// 排版调整等全局快捷操作。
///
/// **macOS 行为：**
/// - 非全屏：左侧自动预留 `TITLEBAR_LEFT_PADDING`（80px）空间，避免内容与红绿灯重叠。
/// - 全屏：左侧使用主题 `spacing.md`（红绿灯在全屏模式下不可见）。
///
/// # 用法
///
/// ```ignore
/// Titlebar::new()
///     .child(Button::new("btn_theme").label("暗色主题").on_click(...))
///     .child(Button::new("btn_font").label("字号：中").on_click(...))
/// ```
#[derive(IntoElement)]
pub struct Titlebar {
    /// 用户自定义样式，优先级高于主题默认值。
    style: StyleRefinement,
    /// 子元素列表。
    children: SmallVec<[AnyElement; 4]>,
}

impl Titlebar {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            children: SmallVec::new(),
        }
    }
}

impl RenderOnce for Titlebar {
    fn render(self, window: &mut Window, cx: &mut gpui::App) -> impl IntoElement {
        let colors = cx.theme().styles.colors;
        let spacing = cx.theme().styles.spacing;
        let fullscreen = window.is_fullscreen();

        div()
            .id(TITLEBAR_ID)
            .h_flex()
            .w_full()
            .h(TITLEBAR_HEIGHT)
            .flex_shrink_0()
            // macOS 非全屏：左侧为红绿灯留空；全屏：红绿灯不显示，正常缩进
            .when(!fullscreen, |this| this.pl(TITLEBAR_LEFT_PADDING))
            .when(fullscreen, |this| this.pl(px(spacing.md)))
            .pr(px(spacing.md))
            .gap(px(spacing.xs))
            .bg(colors.surface.elevated.rgb())
            .border_b_1()
            .border_color(colors.border.default.rgb())
            .refine_style(&self.style)
            .children(self.children)
    }
}

impl ParentElement for Titlebar {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for Titlebar {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
