use gpui::{
    App, BorderStyle, Bounds, ContentMask, Context, Corners, CursorStyle, DispatchPhase, Edges,
    Element, ElementId, Entity, GlobalElementId, Hitbox, HitboxBehavior, Hsla, IntoElement,
    LayoutId, MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent, Pixels, Point, Position,
    Render, ScrollHandle, Style, Window, px, quad, relative, size,
};
use theme::ActiveTheme;

/// 滚动条宽度（px）
const SCROLLBAR_WIDTH: Pixels = px(6.);
/// 滚动条与容器边缘的间距（px）
const SCROLLBAR_PADDING: Pixels = px(3.);
/// thumb 最小长度（px）
const MIN_THUMB_SIZE: Pixels = px(24.);

// ── 轴向 ──────────────────────────────────────────────────────────────────────

/// 滚动条轴向。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollAxis {
    /// 竖向（沿 Y 轴）
    Vertical,
    /// 横向（沿 X 轴）
    Horizontal,
}

// ── 状态 ─────────────────────────────────────────────────────────────────────

/// 滚动条持久化状态，需通过 [`Entity`] 创建以跨帧保持。
///
/// ```ignore
/// let state = cx.new(|_| ScrollbarState::new());
/// ```
#[derive(Default)]
pub struct ScrollbarState {
    /// 拖拽偏移量（thumb 内鼠标位置）；`None` 表示未拖拽。
    drag_offset: Option<Pixels>,
    /// 鼠标是否悬停在 thumb 上。
    thumb_hover: bool,
}

impl ScrollbarState {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn is_dragging(&self) -> bool {
        self.drag_offset.is_some()
    }
}

impl Render for ScrollbarState {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        gpui::Empty
    }
}

// ── 元素 ─────────────────────────────────────────────────────────────────────

/// 滚动条组件，支持竖向与横向两种轴向。
///
/// 通过 GPUI 低层 [`Element`] API 实现，使用 [`Hitbox`] 精准拦截鼠标事件。
///
/// # 创建方式
///
/// ```ignore
/// // 竖向（常用）
/// Scrollbar::vertical("id", handle, state)
/// // 横向
/// Scrollbar::horizontal("id", handle, state)
/// ```
///
/// 推荐通过 [`ScrollbarExt`](crate::ScrollbarExt) 的声明式 API 自动管理：
/// ```ignore
/// div().id("c").overflow_y_scrollbar(window, cx).children(items)
/// ```
pub struct Scrollbar {
    id: ElementId,
    scroll_handle: ScrollHandle,
    state: Entity<ScrollbarState>,
    axis: ScrollAxis,
}

impl Scrollbar {
    /// 通用构造函数。
    pub fn new(
        id: impl Into<ElementId>,
        scroll_handle: ScrollHandle,
        state: Entity<ScrollbarState>,
        axis: ScrollAxis,
    ) -> Self {
        Self {
            id: id.into(),
            scroll_handle,
            state,
            axis,
        }
    }

    /// 创建竖向滚动条。
    pub fn vertical(
        id: impl Into<ElementId>,
        scroll_handle: ScrollHandle,
        state: Entity<ScrollbarState>,
    ) -> Self {
        Self::new(id, scroll_handle, state, ScrollAxis::Vertical)
    }

    /// 创建横向滚动条。
    pub fn horizontal(
        id: impl Into<ElementId>,
        scroll_handle: ScrollHandle,
        state: Entity<ScrollbarState>,
    ) -> Self {
        Self::new(id, scroll_handle, state, ScrollAxis::Horizontal)
    }

    /// 计算 thumb 的像素范围，内容未溢出时返回 `None`。
    fn compute_thumb(
        track: Bounds<Pixels>,
        handle: &ScrollHandle,
        axis: ScrollAxis,
    ) -> Option<Bounds<Pixels>> {
        let (max, viewport_size, track_size) = match axis {
            ScrollAxis::Vertical => (
                handle.max_offset().y,
                handle.bounds().size.height,
                track.size.height,
            ),
            ScrollAxis::Horizontal => (
                handle.max_offset().x,
                handle.bounds().size.width,
                track.size.width,
            ),
        };

        if max <= Pixels::ZERO || viewport_size <= Pixels::ZERO {
            return None;
        }

        let thumb_size = (viewport_size / (viewport_size + max) * track_size).max(MIN_THUMB_SIZE);
        let available = track_size - thumb_size;

        if available <= Pixels::ZERO {
            return None;
        }

        let scroll = match axis {
            ScrollAxis::Vertical => handle.offset().y.abs().min(max),
            ScrollAxis::Horizontal => handle.offset().x.abs().min(max),
        };
        let offset = available * (scroll / max);

        Some(match axis {
            ScrollAxis::Vertical => Bounds::new(
                Point::new(track.origin.x, track.origin.y + offset),
                gpui::Size {
                    width: track.size.width,
                    height: thumb_size,
                },
            ),
            ScrollAxis::Horizontal => Bounds::new(
                Point::new(track.origin.x + offset, track.origin.y),
                gpui::Size {
                    width: thumb_size,
                    height: track.size.height,
                },
            ),
        })
    }
}

// ── Prepaint 产物 ─────────────────────────────────────────────────────────────

/// [`Scrollbar`] 的 prepaint 产物，存储当帧的布局信息与点击盒。
pub struct ScrollbarPrepaint {
    track_bounds: Bounds<Pixels>,
    thumb_bounds: Option<Bounds<Pixels>>,
    thumb_hitbox: Option<Hitbox>,
    #[allow(dead_code)]
    track_hitbox: Hitbox,
}

// ── Element 实现 ──────────────────────────────────────────────────────────────

impl Element for Scrollbar {
    type RequestLayoutState = ();
    type PrepaintState = ScrollbarPrepaint;

    fn id(&self) -> Option<ElementId> {
        Some(self.id.clone())
    }

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, ()) {
        let style = Style {
            position: Position::Absolute,
            size: size(relative(1.), relative(1.)).map(Into::into),
            ..Default::default()
        };
        (window.request_layout(style, None, cx), ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut (),
        window: &mut Window,
        _cx: &mut App,
    ) -> ScrollbarPrepaint {
        // 轨道区域：根据轴向定位到右侧（竖）或底部（横）
        let track_bounds = match self.axis {
            ScrollAxis::Vertical => Bounds::new(
                Point::new(
                    bounds.origin.x + bounds.size.width - SCROLLBAR_WIDTH - SCROLLBAR_PADDING,
                    bounds.origin.y + SCROLLBAR_PADDING,
                ),
                gpui::Size {
                    width: SCROLLBAR_WIDTH,
                    height: (bounds.size.height - SCROLLBAR_PADDING * 2.).max(Pixels::ZERO),
                },
            ),
            ScrollAxis::Horizontal => Bounds::new(
                Point::new(
                    bounds.origin.x + SCROLLBAR_PADDING,
                    bounds.origin.y + bounds.size.height - SCROLLBAR_WIDTH - SCROLLBAR_PADDING,
                ),
                gpui::Size {
                    width: (bounds.size.width - SCROLLBAR_PADDING * 2.).max(Pixels::ZERO),
                    height: SCROLLBAR_WIDTH,
                },
            ),
        };

        let thumb_bounds = Self::compute_thumb(track_bounds, &self.scroll_handle, self.axis);
        let thumb_hitbox =
            thumb_bounds.map(|tb| window.insert_hitbox(tb, HitboxBehavior::BlockMouseExceptScroll));
        let track_hitbox =
            window.insert_hitbox(track_bounds, HitboxBehavior::BlockMouseExceptScroll);

        ScrollbarPrepaint {
            track_bounds,
            thumb_bounds,
            thumb_hitbox,
            track_hitbox,
        }
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut (),
        prepaint: &mut ScrollbarPrepaint,
        window: &mut Window,
        cx: &mut App,
    ) {
        let Some(thumb_bounds) = prepaint.thumb_bounds else {
            return;
        };
        let track_bounds = prepaint.track_bounds;
        let axis = self.axis;
        let is_dragging = self.state.read(cx).is_dragging();

        // 光标样式
        if is_dragging {
            window.set_window_cursor_style(CursorStyle::Arrow);
        } else if let Some(ref hb) = prepaint.thumb_hitbox {
            window.set_cursor_style(CursorStyle::Arrow, hb);
        }

        window.with_content_mask(Some(ContentMask { bounds }), |window| {
            let thumb_hover = self.state.read(cx).thumb_hover;
            let colors = cx.theme().styles.colors;

            let thumb_color: Hsla = if is_dragging {
                let mut c: Hsla = colors.text.primary.into();
                c.a = 0.85;
                c
            } else if thumb_hover {
                let mut c: Hsla = colors.text.secondary.into();
                c.a = 0.70;
                c
            } else {
                let mut c: Hsla = colors.text.muted.into();
                c.a = 0.45;
                c
            };

            // 绘制 thumb（圆角胶囊）
            window.paint_quad(quad(
                thumb_bounds,
                Corners::all(SCROLLBAR_WIDTH / 2.),
                thumb_color,
                Edges::default(),
                Hsla::transparent_black(),
                BorderStyle::default(),
            ));

            // ── 事件处理 ──────────────────────────────────────────────────────
            let state = self.state.clone();
            let scroll_handle = self.scroll_handle.clone();

            // MouseDown
            window.on_mouse_event({
                let state = state.clone();
                let scroll_handle = scroll_handle.clone();
                move |event: &MouseDownEvent, phase, _w, cx| {
                    if phase != DispatchPhase::Bubble || event.button != MouseButton::Left {
                        return;
                    }
                    if thumb_bounds.contains(&event.position) {
                        let offset = match axis {
                            ScrollAxis::Vertical => event.position.y - thumb_bounds.origin.y,
                            ScrollAxis::Horizontal => event.position.x - thumb_bounds.origin.x,
                        };
                        state.update(cx, |s, cx| {
                            s.drag_offset = Some(offset);
                            cx.notify();
                        });
                        cx.stop_propagation();
                    } else if track_bounds.contains(&event.position) {
                        let (max, available, click, origin, thumb_sz) = match axis {
                            ScrollAxis::Vertical => (
                                scroll_handle.max_offset().y,
                                track_bounds.size.height - thumb_bounds.size.height,
                                event.position.y,
                                track_bounds.origin.y,
                                thumb_bounds.size.height,
                            ),
                            ScrollAxis::Horizontal => (
                                scroll_handle.max_offset().x,
                                track_bounds.size.width - thumb_bounds.size.width,
                                event.position.x,
                                track_bounds.origin.x,
                                thumb_bounds.size.width,
                            ),
                        };
                        if available > Pixels::ZERO {
                            let top = (click - origin - thumb_sz / 2.)
                                .max(Pixels::ZERO)
                                .min(available);
                            let ratio = top / available;
                            let cur = scroll_handle.offset();
                            let new_offset = match axis {
                                ScrollAxis::Vertical => Point::new(cur.x, -(max * ratio)),
                                ScrollAxis::Horizontal => Point::new(-(max * ratio), cur.y),
                            };
                            scroll_handle.set_offset(new_offset);
                            state.update(cx, |_, cx| cx.notify());
                        }
                        cx.stop_propagation();
                    }
                }
            });

            // MouseMove
            window.on_mouse_event({
                let state = state.clone();
                let scroll_handle = scroll_handle.clone();
                move |event: &MouseMoveEvent, phase, _w, cx| {
                    if phase != DispatchPhase::Capture {
                        return;
                    }
                    let drag_offset = state.read(cx).drag_offset;
                    if let Some(offset) = drag_offset {
                        let (max, available, pos, origin) = match axis {
                            ScrollAxis::Vertical => (
                                scroll_handle.max_offset().y,
                                track_bounds.size.height - thumb_bounds.size.height,
                                event.position.y,
                                track_bounds.origin.y,
                            ),
                            ScrollAxis::Horizontal => (
                                scroll_handle.max_offset().x,
                                track_bounds.size.width - thumb_bounds.size.width,
                                event.position.x,
                                track_bounds.origin.x,
                            ),
                        };
                        if available > Pixels::ZERO {
                            let top = (pos - origin - offset).max(Pixels::ZERO).min(available);
                            let ratio = top / available;
                            let cur = scroll_handle.offset();
                            let new_offset = match axis {
                                ScrollAxis::Vertical => Point::new(cur.x, -(max * ratio)),
                                ScrollAxis::Horizontal => Point::new(-(max * ratio), cur.y),
                            };
                            scroll_handle.set_offset(new_offset);
                            state.update(cx, |_, cx| cx.notify());
                        }
                        cx.stop_propagation();
                    } else {
                        let hovering = thumb_bounds.contains(&event.position);
                        state.update(cx, |s, cx| {
                            if s.thumb_hover != hovering {
                                s.thumb_hover = hovering;
                                cx.notify();
                            }
                        });
                    }
                }
            });

            // MouseUp
            window.on_mouse_event({
                let state = state.clone();
                move |_: &MouseUpEvent, phase, _w, cx| {
                    if phase != DispatchPhase::Capture {
                        return;
                    }
                    state.update(cx, |s, cx| {
                        if s.drag_offset.is_some() {
                            s.drag_offset = None;
                            cx.notify();
                        }
                    });
                }
            });
        });
    }
}

impl IntoElement for Scrollbar {
    type Element = Self;
    fn into_element(self) -> Self::Element {
        self
    }
}
