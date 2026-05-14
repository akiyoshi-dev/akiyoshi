use crate::states::AppState;
use gpui::{
    Context, Entity, Hsla, InteractiveElement, IntoElement, ParentElement, Render, Styled, Window,
    div, px,
};
use theme::{ActiveTheme, ActiveThemeMut, ThemeId};
use ui::clickable::Clickable;
use ui::{Button, ButtonSize, ButtonVariant, ScrollbarExt, Titlebar};

pub struct Akiyoshi {
    state: Entity<AppState>,
}

impl Akiyoshi {
    pub fn new(state: Entity<AppState>, _cx: &mut Context<Self>) -> Self {
        Self { state }
    }
}

impl Render for Akiyoshi {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // ── 读取所有需要的数据 ─────────────────────────────────────────────
        let theme_id = cx.theme().id.clone();
        let font_scale = cx.font_size_scale();
        let lh_scale = cx.line_height_scale();
        let font_size = cx.font_size();
        let line_height = cx.line_height();

        let colors = cx.theme().styles.colors;
        let spacing = cx.theme().styles.spacing;
        let radius = cx.theme().styles.radius;
        let typo = cx.theme().styles.typography;

        div()
            .flex()
            .flex_col()
            .w_full()
            .h_full()
            .bg(colors.background.rgb())
            // ── 标题栏 ─────────────────────────────────────────────────────
            .child(
                Titlebar::new()
                    .child(
                        Button::new("btn_theme")
                            .size(ButtonSize::Sm)
                            .variant(ButtonVariant::Ghost)
                            .label(theme_id.as_str())
                            .on_click(cx.listener(|this, _ev, _, cx| {
                                let next: ThemeId = match cx.theme().id.as_ref() {
                                    "akiyoshi_light" => "akiyoshi_dark".into(),
                                    _ => "akiyoshi_light".into(),
                                };
                                if let Ok(id) = cx.switch_theme(next) {
                                    this.state.update(cx, |s, _| s.theme_id = Some(id));
                                }
                                cx.notify();
                            })),
                    )
                    .child(
                        Button::new("btn_font_size")
                            .size(ButtonSize::Sm)
                            .variant(ButtonVariant::Ghost)
                            .label(font_scale.label())
                            .on_click(cx.listener(|_, _ev, _, cx| {
                                let next = cx.font_size_scale().next();
                                cx.set_font_size_scale(next);
                                cx.notify();
                            })),
                    )
                    .child(
                        Button::new("btn_line_height")
                            .size(ButtonSize::Sm)
                            .variant(ButtonVariant::Ghost)
                            .label(lh_scale.label())
                            .on_click(cx.listener(|_, _ev, _, cx| {
                                let next = cx.line_height_scale().next();
                                cx.set_line_height_scale(next);
                                cx.notify();
                            })),
                    ),
            )
            // ── 主内容区：使用新声明式 API ────────────────────────────────
            //
            //   div().id("x").overflow_y_scrollbar(window, cx)
            //       ↑ 自动创建并持久化 ScrollHandle + ScrollbarState
            //
            .child(
                div()
                    .id("main-content")
                    .flex_1()
                    .flex()
                    .flex_col()
                    .gap(px(spacing.md))
                    .p(px(spacing.lg))
                    // 卡片 1：排版预览
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(spacing.sm))
                            .p(px(spacing.md))
                            .rounded(px(radius.md))
                            .bg(colors.surface.default.rgb())
                            .border_1()
                            .border_color(colors.border.default.rgb())
                            .child(
                                div()
                                    .text_size(px(typo.lg))
                                    .line_height(px(typo.line_height.lg))
                                    .text_color(colors.text.primary.rgb())
                                    .child("排版预览"),
                            )
                            .child(
                                div()
                                    .text_size(px(font_size))
                                    .line_height(px(line_height))
                                    .text_color(colors.text.primary.rgb())
                                    .child(
                                        "这是一段用于展示排版效果的示例文本。\
                                        点击标题栏按钮可切换主题、字号与行距，\
                                        所有 UI 组件均实时响应全局排版设置。",
                                    ),
                            )
                            .child(
                                div()
                                    .text_size(px(typo.sm))
                                    .line_height(px(typo.line_height.sm))
                                    .text_color(colors.text.muted.rgb())
                                    .child(format!(
                                        "字号：{:.0}px  ·  行高：{:.1}px",
                                        font_size, line_height
                                    )),
                            ),
                    )
                    // 卡片 2：按钮变体
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(spacing.sm))
                            .p(px(spacing.md))
                            .rounded(px(radius.md))
                            .bg(colors.surface.default.rgb())
                            .border_1()
                            .border_color(colors.border.default.rgb())
                            .child(
                                div()
                                    .text_size(px(typo.lg))
                                    .line_height(px(typo.line_height.lg))
                                    .text_color(colors.text.primary.rgb())
                                    .child("按钮变体"),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .flex_wrap()
                                    .gap(px(spacing.sm))
                                    .child(Button::new("d-primary").label("Primary"))
                                    .child(
                                        Button::new("d-secondary")
                                            .label("Secondary")
                                            .variant(ButtonVariant::Secondary),
                                    )
                                    .child(
                                        Button::new("d-outline")
                                            .label("Outline")
                                            .variant(ButtonVariant::Outline),
                                    )
                                    .child(
                                        Button::new("d-ghost")
                                            .label("Ghost")
                                            .variant(ButtonVariant::Ghost),
                                    )
                                    .child(
                                        Button::new("d-destructive")
                                            .label("Destructive")
                                            .variant(ButtonVariant::Destructive),
                                    )
                                    .child(
                                        Button::new("d-link")
                                            .label("Link")
                                            .variant(ButtonVariant::Link),
                                    ),
                            ),
                    )
                    // 卡片 3：颜色令牌
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(spacing.sm))
                            .p(px(spacing.md))
                            .rounded(px(radius.md))
                            .bg(colors.surface.default.rgb())
                            .border_1()
                            .border_color(colors.border.default.rgb())
                            .child(
                                div()
                                    .text_size(px(typo.lg))
                                    .line_height(px(typo.line_height.lg))
                                    .text_color(colors.text.primary.rgb())
                                    .child("颜色令牌"),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .gap(px(spacing.sm))
                                    .child(color_swatch(
                                        colors.primary.rgb(),
                                        "primary",
                                        radius.sm,
                                        colors.text.primary.rgb(),
                                    ))
                                    .child(color_swatch(
                                        colors.secondary.rgb(),
                                        "secondary",
                                        radius.sm,
                                        colors.text.primary.rgb(),
                                    ))
                                    .child(color_swatch(
                                        colors.danger.rgb(),
                                        "danger",
                                        radius.sm,
                                        colors.text.primary.rgb(),
                                    ))
                                    .child(color_swatch(
                                        colors.success.rgb(),
                                        "success",
                                        radius.sm,
                                        colors.text.primary.rgb(),
                                    ))
                                    .child(color_swatch(
                                        colors.warning.rgb(),
                                        "warning",
                                        radius.sm,
                                        colors.text.primary.rgb(),
                                    ))
                                    .child(color_swatch(
                                        colors.info.rgb(),
                                        "info",
                                        radius.sm,
                                        colors.text.primary.rgb(),
                                    )),
                            ),
                    )
                    // 卡片 4：间距令牌
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(spacing.sm))
                            .p(px(spacing.md))
                            .rounded(px(radius.md))
                            .bg(colors.surface.default.rgb())
                            .border_1()
                            .border_color(colors.border.default.rgb())
                            .child(
                                div()
                                    .text_size(px(typo.lg))
                                    .line_height(px(typo.line_height.lg))
                                    .text_color(colors.text.primary.rgb())
                                    .child("间距令牌"),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_end()
                                    .gap(px(spacing.md))
                                    .child(spacing_block(
                                        spacing.xs,
                                        "xs",
                                        colors.primary.rgb(),
                                        radius.sm,
                                        colors.text.primary.rgb(),
                                    ))
                                    .child(spacing_block(
                                        spacing.sm,
                                        "sm",
                                        colors.primary.rgb(),
                                        radius.sm,
                                        colors.text.primary.rgb(),
                                    ))
                                    .child(spacing_block(
                                        spacing.md,
                                        "md",
                                        colors.primary.rgb(),
                                        radius.sm,
                                        colors.text.primary.rgb(),
                                    ))
                                    .child(spacing_block(
                                        spacing.lg,
                                        "lg",
                                        colors.primary.rgb(),
                                        radius.sm,
                                        colors.text.primary.rgb(),
                                    ))
                                    .child(spacing_block(
                                        spacing.xl,
                                        "xl",
                                        colors.primary.rgb(),
                                        radius.sm,
                                        colors.text.primary.rgb(),
                                    )),
                            ),
                    )
                    // 卡片 5：圆角令牌
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(spacing.sm))
                            .p(px(spacing.md))
                            .rounded(px(radius.md))
                            .bg(colors.surface.default.rgb())
                            .border_1()
                            .border_color(colors.border.default.rgb())
                            .child(
                                div()
                                    .text_size(px(typo.lg))
                                    .line_height(px(typo.line_height.lg))
                                    .text_color(colors.text.primary.rgb())
                                    .child("圆角令牌"),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .gap(px(spacing.md))
                                    .child(radius_block(
                                        radius.sm,
                                        "sm",
                                        colors.primary.rgb(),
                                        colors.text.primary,
                                    ))
                                    .child(radius_block(
                                        radius.md,
                                        "md",
                                        colors.primary.rgb(),
                                        colors.text.primary,
                                    ))
                                    .child(radius_block(
                                        radius.lg,
                                        "lg",
                                        colors.primary.rgb(),
                                        colors.text.primary,
                                    )),
                            ),
                    )
                    .overflow_y_scrollbar(window, cx), // ← 最后渲染，在内容之上
            )
    }
}

// ── 辅助渲染函数 ──────────────────────────────────────────────────────────────

fn color_swatch(
    bg: gpui::Rgba,
    label: &'static str,
    radius: f32,
    text_color: impl Into<Hsla>,
) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .items_center()
        .text_color(text_color)
        .gap(px(4.))
        .child(div().w(px(48.)).h(px(28.)).rounded(px(radius)).bg(bg))
        .child(div().text_size(px(11.)).child(label))
}

fn spacing_block(
    size: f32,
    label: &'static str,
    color: gpui::Rgba,
    radius: f32,
    text_color: impl Into<Hsla>,
) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .items_center()
        .gap(px(4.))
        .text_color(text_color)
        .child(div().w(px(size)).h(px(size)).rounded(px(radius)).bg(color))
        .child(div().text_size(px(11.)).child(label))
}

fn radius_block(
    r: f32,
    label: &'static str,
    color: gpui::Rgba,
    text_color: impl Into<Hsla>,
) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .items_center()
        .gap(px(4.))
        .text_color(text_color)
        .child(div().w(px(48.)).h(px(28.)).rounded(px(r)).bg(color))
        .child(div().text_size(px(11.)).child(format!("{r}px")))
        .child(div().text_size(px(11.)).child(label))
}
