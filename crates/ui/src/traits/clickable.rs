use gpui::{App, ClickEvent, Window};

/// 可单击的 UI 元素的特征。
pub trait Clickable {
    /// 为该元素添加一个点击事件处理器。
    fn on_click<H>(self, handler: H) -> Self
    where
        H: ClickHandlerFn;
}

/// 点击事件处理器类型。
pub type ClickHandler = dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static;

/// 盒装点击事件处理器，适合存储在组件字段中。
pub type BoxedClickHandler = Box<ClickHandler>;

/// 统一约束所有可作为点击处理器的闭包类型。
pub trait ClickHandlerFn: Fn(&ClickEvent, &mut Window, &mut App) + 'static {}

/// 自动为所有符合条件的闭包类型实现 `ClickHandlerFn`，简化使用。
impl<T> ClickHandlerFn for T where T: Fn(&ClickEvent, &mut Window, &mut App) + 'static {}
