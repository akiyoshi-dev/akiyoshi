use std::panic::Location;

use gpui::{
    App, AppContext, Div, ElementId, ParentElement, ScrollHandle, Stateful,
    StatefulInteractiveElement, Window,
};

use crate::components::scrollbar::{Scrollbar, ScrollbarState};

// ── 内部持久化状态 ────────────────────────────────────────────────────────────

/// 与可滚动容器 ID 绑定的持久化状态。
///
/// 通过 [`Window::use_keyed_state`] 以调用点位置为 key 自动管理，
/// 无需在业务组件中手动持有 [`ScrollHandle`] 或 [`ScrollbarState`]。
pub(crate) struct ScrollableState {
    /// 共享的滚动句柄，同时用于 X、Y 两个轴向的滚动条
    pub scroll_handle: ScrollHandle,
    /// 竖向滚动条的持久化状态
    pub scrollbar_y: gpui::Entity<ScrollbarState>,
    /// 横向滚动条的持久化状态
    pub scrollbar_x: gpui::Entity<ScrollbarState>,
}

// ── 扩展 Trait ────────────────────────────────────────────────────────────────

/// 为 [`gpui::Div`]（`Stateful<Div>`）提供声明式滚动条支持。
///
/// 内部通过 [`Window::use_keyed_state`] 以 `#[track_caller]` 返回的调用点位置为 key，
/// 自动创建并跨帧持久化 [`ScrollHandle`] 与 [`ScrollbarState`]，
/// 无需在组件结构体中手动持有任何滚动相关状态。
///
/// # 用法
///
/// ```ignore
/// // 需要先调用 .id() 使元素变为 Stateful<Div>
/// div()
///     .id("container")
///     .size_full()
///     .overflow_y_scrollbar(window, cx)   // 仅竖向
///     .children(items)
///
/// div()
///     .id("wide-list")
///     .overflow_x_scrollbar(window, cx)   // 仅横向
///     .children(items)
///
/// div()
///     .id("grid")
///     .overflow_scrollbar(window, cx)     // 双向
///     .children(items)
/// ```
///
/// > **为什么需要 `window` 和 `cx`？**
/// > GPUI 的跨帧状态需通过 [`Window::use_keyed_state`] 或 Entity 持久化。
/// > 在 `render()` 阶段可直接传入方法参数中的 `window` 和 `cx`。
pub trait ScrollbarExt: Sized {
    /// 添加**竖向**滚动条，内容超出时自动显示。
    fn overflow_y_scrollbar(self, window: &mut Window, cx: &mut App) -> Self;

    /// 添加**横向**滚动条，内容超出时自动显示。
    fn overflow_x_scrollbar(self, window: &mut Window, cx: &mut App) -> Self;

    /// 同时添加**双向**滚动条，内容超出时自动显示。
    fn overflow_scrollbar(self, window: &mut Window, cx: &mut App) -> Self;
}

impl ScrollbarExt for Stateful<Div> {
    #[track_caller]
    fn overflow_y_scrollbar(self, window: &mut Window, cx: &mut App) -> Self {
        let caller = Location::caller();
        // use_keyed_state 以 caller 位置为 key，保证同一调用点在不同帧间复用同一份状态
        let state = window.use_keyed_state(caller, cx, |_, cx| {
            let sb_y = cx.new(|_| ScrollbarState::new());
            let sb_x = cx.new(|_| ScrollbarState::new());
            // 观察链：ScrollbarState 变化 → 通知 ScrollableState → 通知视图重绘
            cx.observe(&sb_y, |_, _, cx| cx.notify()).detach();
            cx.observe(&sb_x, |_, _, cx| cx.notify()).detach();
            ScrollableState {
                scroll_handle: ScrollHandle::new(),
                scrollbar_y: sb_y,
                scrollbar_x: sb_x,
            }
        });

        let handle = state.read(cx).scroll_handle.clone();
        let sb_y = state.read(cx).scrollbar_y.clone();
        let sb_id = ElementId::from((ElementId::from(caller), "sb-y"));

        self.overflow_y_scroll()
            .track_scroll(&handle)
            .child(Scrollbar::vertical(sb_id, handle, sb_y))
    }

    #[track_caller]
    fn overflow_x_scrollbar(self, window: &mut Window, cx: &mut App) -> Self {
        let caller = Location::caller();
        let state = window.use_keyed_state(caller, cx, |_, cx| {
            let sb_y = cx.new(|_| ScrollbarState::new());
            let sb_x = cx.new(|_| ScrollbarState::new());
            // 观察链：ScrollbarState 变化 → 通知 ScrollableState → 通知视图重绘
            cx.observe(&sb_y, |_, _, cx| cx.notify()).detach();
            cx.observe(&sb_x, |_, _, cx| cx.notify()).detach();
            ScrollableState {
                scroll_handle: ScrollHandle::new(),
                scrollbar_y: sb_y,
                scrollbar_x: sb_x,
            }
        });

        let handle = state.read(cx).scroll_handle.clone();
        let sb_x = state.read(cx).scrollbar_x.clone();
        let sb_id = ElementId::from((ElementId::from(caller), "sb-x"));

        self.overflow_x_scroll()
            .track_scroll(&handle)
            .child(Scrollbar::horizontal(sb_id, handle, sb_x))
    }

    #[track_caller]
    fn overflow_scrollbar(self, window: &mut Window, cx: &mut App) -> Self {
        let caller = Location::caller();
        let state = window.use_keyed_state(caller, cx, |_, cx| {
            let sb_y = cx.new(|_| ScrollbarState::new());
            let sb_x = cx.new(|_| ScrollbarState::new());
            // 观察链：ScrollbarState 变化 → 通知 ScrollableState → 通知视图重绘
            cx.observe(&sb_y, |_, _, cx| cx.notify()).detach();
            cx.observe(&sb_x, |_, _, cx| cx.notify()).detach();
            ScrollableState {
                scroll_handle: ScrollHandle::new(),
                scrollbar_y: sb_y,
                scrollbar_x: sb_x,
            }
        });

        let handle = state.read(cx).scroll_handle.clone();
        let sb_y = state.read(cx).scrollbar_y.clone();
        let sb_x = state.read(cx).scrollbar_x.clone();
        let sb_y_id = ElementId::from((ElementId::from(caller), "sb-y"));
        let sb_x_id = ElementId::from((ElementId::from(caller), "sb-x"));

        self.overflow_scroll()
            .track_scroll(&handle)
            .child(Scrollbar::vertical(sb_y_id, handle.clone(), sb_y))
            .child(Scrollbar::horizontal(sb_x_id, handle, sb_x))
    }
}
