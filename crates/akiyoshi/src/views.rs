use crate::states::AppState;
use gpui::{
    Context, Entity, IntoElement, ParentElement, Render, Styled, Window, div,
    prelude::FluentBuilder, px,
};
use theme::{ActiveTheme, ActiveThemeMut, ThemeId};
use ui::{Button, ButtonSize, ButtonVariant, Titlebar, clickable::Clickable};

pub struct Akiyoshi {
    state: Entity<AppState>,
}

impl Akiyoshi {
    pub fn new(state: Entity<AppState>) -> Self {
        Self { state }
    }
}

impl Render for Akiyoshi {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // ── 读取所有需要的数据（短暂借用，立即 Copy/Clone，后续不再持有 cx 的引用）──
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
            // ── 工具栏 ────────────────────────────────────────────────
            .child(
                Titlebar::new()
                    // 主题切换
                    .child(
                        Button::new("btn_theme")
                            .size(ButtonSize::Sm)
                            .variant(ButtonVariant::Primary)
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
                    // 字号切换
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
                    // 行距切换
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
                    )
                    .when(window.is_fullscreen(), |e| e.pl(px(spacing.lg)))
                    .pr(px(spacing.lg)),
            )
            // ── 主内容区 ──────────────────────────────────────────────
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .gap(px(spacing.md))
                    .p(px(spacing.lg))
                    // 排版预览卡片
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
                            // 卡片标题（始终使用 lg 字号，不受全局档位影响）
                            .child(
                                div()
                                    .text_size(px(typo.lg))
                                    .line_height(px(typo.line_height.lg))
                                    .text_color(colors.text.primary.rgb())
                                    .child("排版预览"),
                            )
                            // 示例正文（跟随全局字号和行距档位）
                            .child(
                                div()
                                    .text_size(px(font_size))
                                    .line_height(px(line_height))
                                    .text_color(colors.text.primary.rgb())
                                    .child(
                                        "这是一段用于展示排版效果的示例文本。\
                                        点击工具栏按钮可切换主题、字号与行距，\
                                        所有 UI 组件均实时响应全局排版设置。",
                                    ),
                            )
                            // 当前参数（次要色，始终用 sm 字号）
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
                    ),
            )
    }
}
