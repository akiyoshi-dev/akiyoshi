use gpui::{App, Refineable, StyleRefinement, Styled};
use theme::ActiveTheme;

/// 使用 Zed 特有的样式方法扩展 [`gpui::Styled`]。
///
/// 参考自 [Zed 官方实现](https://github.com/zed-industries/zed/blob/main/crates/ui/src/traits/styled_ext.rs)
#[cfg_attr(
    all(debug_assertions, not(rust_analyzer)),
    gpui_macros::derive_inspector_reflection
)]
pub trait StyledExt: Styled + Sized {
    /// 水平居中的 flex 布局。
    ///
    /// 设置了 `flex()`, `flex_row()`, `items_center()`。
    fn h_flex(self) -> Self {
        self.flex().flex_row().items_center()
    }

    /// 垂直居中的 flex 布局。
    ///
    /// 设置了 `flex()`, `flex_col()`, `items_center()`。
    fn v_flex(self) -> Self {
        self.flex().flex_col().items_center()
    }

    /// 设置主题的边框颜色为 [`theme::BorderTokens`] 的 `default`。
    fn border_default(self, cx: &App) -> Self {
        self.border_1()
            .border_color(cx.theme().styles.colors.border.default.rgb())
    }

    /// 设置主题的边框颜色为 [`theme::BorderTokens`] 的 `muted`。
    fn border_muted(self, cx: &App) -> Self {
        self.border_1()
            .border_color(cx.theme().styles.colors.border.muted.rgb())
    }

    /// 设置主题的边框颜色为 [`theme::BorderTokens`] 的 `strong`。
    fn border_strong(self, cx: &App) -> Self {
        self.border_1()
            .border_color(cx.theme().styles.colors.border.strong.rgb())
    }
    /// 将一个 [`StyleRefinement`] 合并到当前样式之上并返回 `Self`，支持链路调用。
    ///
    /// 常用于组件内部将用户自定义样式覆盖到主题默认样式之上：
    /// ```ignore
    /// div()
    ///     .bg(theme_color)   // 主题默认样式
    ///     .refine_style(&user_style)  // 用户样式优先，继续链路
    ///     .child(...)
    /// ```
    fn refine_style(mut self, refinement: &StyleRefinement) -> Self {
        self.style().refine(refinement);
        self
    }
}

impl<E: Styled> StyledExt for E {}
