use gpui::{App, Styled};
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
}

impl<E: Styled> StyledExt for E {}
