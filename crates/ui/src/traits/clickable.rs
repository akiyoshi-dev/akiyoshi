use gpui::{App, ClickEvent, Window};

/// 可单击的 UI 元素的特征。
pub trait Clickable {
    /// 为该元素添加一个点击事件处理器。
    fn on_click(self, handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self;
}