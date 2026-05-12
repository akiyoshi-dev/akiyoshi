use crate::states::AppState;
use gpui::{Context, Entity, IntoElement, ParentElement, Render, Styled, Window, div, px};
use theme::{ActiveTheme, GlobalTheme};
use ui::{
    Button, ButtonSize, ButtonVariant, Titlebar, clickable::Clickable, disableable::Disableable,
    styled_ext::StyledExt,
};

pub struct Akiyoshi {
    state: Entity<AppState>,
}

impl Akiyoshi {
    pub fn new(state: Entity<AppState>, cx: &mut Context<Self>) -> Self {
        cx.observe_global::<GlobalTheme>(|_, cx| {
            cx.notify();
        })
        .detach();
        Self { state }
    }
}

impl Render for Akiyoshi {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let bg = theme.styles.colors.background.rgb();
        let text = theme.styles.colors.text.primary.rgb();

        // 切换主题的 on_click 闭包
        let toggle = {
            let state = self.state.clone();
            move |_event: &_, _window: &mut _, cx: &mut _| {
                state.update(cx, |s, cx| s.toggle_theme(cx));
            }
        };

        div()
            .w_full()
            .h_full()
            .child(
                Titlebar::new()
                    .gap(px(theme.styles.spacing.lg))
                    .child("Title Button🍂")
                    .child(
                        Button::new("create-item")
                            .label("Title Button")
                            .size(ButtonSize::Sm)
                            .variant(ButtonVariant::Link),
                    ),
            )
            .child(
                div()
                    .size_full()
                    .bg(bg)
                    .flex()
                    .flex_col()
                    .items_center()
                    .justify_center()
                    .gap(px(32.))
                    .child(
                        // ── 标题 ──
                        div()
                            .text_color(text)
                            .text_size(px(theme.styles.typography.lg))
                            .child("Button Showcase"),
                    )
                    .child(
                        // ── 切换主题 ──
                        Button::new("toggle-theme")
                            .label("切换主题")
                            .on_click(toggle.clone()),
                    )
                    .child(
                        // ── 所有变体一行展示 ──
                        div()
                            .h_flex()
                            .gap(px(12.))
                            .child(
                                Button::new("btn-primary")
                                    .label("Primary")
                                    .on_click(toggle.clone()),
                            )
                            .child(
                                Button::new("btn-secondary")
                                    .label("Secondary")
                                    .variant(ButtonVariant::Secondary)
                                    .on_click(toggle.clone()),
                            )
                            .child(
                                Button::new("btn-outline")
                                    .label("Outline")
                                    .variant(ButtonVariant::Outline)
                                    .on_click(toggle.clone()),
                            )
                            .child(
                                Button::new("btn-ghost")
                                    .label("Ghost")
                                    .variant(ButtonVariant::Ghost)
                                    .on_click(toggle.clone()),
                            )
                            .child(
                                Button::new("btn-destructive")
                                    .label("Destructive")
                                    .variant(ButtonVariant::Destructive)
                                    .on_click(toggle.clone()),
                            )
                            .child(
                                Button::new("btn-link")
                                    .label("Link")
                                    .variant(ButtonVariant::Link)
                                    .on_click(toggle.clone()),
                            ),
                    )
                    .child(
                        // ── 禁用态展示 ──
                        div()
                            .h_flex()
                            .gap(px(12.))
                            .child(
                                Button::new("btn-primary-disabled")
                                    .label("Primary (禁用)")
                                    .disabled(true),
                            )
                            .child(
                                Button::new("btn-outline-disabled")
                                    .label("Outline (禁用)")
                                    .variant(ButtonVariant::Outline)
                                    .disabled(true),
                            )
                            .child(
                                Button::new("btn-destructive-disabled")
                                    .label("Destructive (禁用)")
                                    .variant(ButtonVariant::Destructive)
                                    .disabled(true),
                            ),
                    )
                    .child(
                        // ── 尺寸对比展示 ──
                        div()
                            .h_flex()
                            .items_center()
                            .gap(px(12.))
                            .child(
                                Button::new("btn-xs")
                                    .label("Xs")
                                    .size(ButtonSize::Xs)
                                    .variant(ButtonVariant::Outline),
                            )
                            .child(
                                Button::new("btn-sm")
                                    .label("Sm")
                                    .size(ButtonSize::Sm)
                                    .variant(ButtonVariant::Outline),
                            )
                            .child(
                                Button::new("btn-md")
                                    .label("Md (默认)")
                                    .size(ButtonSize::Md)
                                    .variant(ButtonVariant::Outline),
                            )
                            .child(
                                Button::new("btn-lg")
                                    .label("Lg")
                                    .size(ButtonSize::Lg)
                                    .variant(ButtonVariant::Outline),
                            ),
                    )
                    .child(
                        // ── 全宽按钮演示（给一个固定宽父容器）──
                        div()
                            .w(px(400.))
                            .v_flex()
                            .gap(px(8.))
                            .child(
                                Button::new("btn-full-primary")
                                    .label("全宽 Primary")
                                    .full_width(),
                            )
                            .child(
                                Button::new("btn-full-outline")
                                    .label("全宽 Outline")
                                    .variant(ButtonVariant::Outline)
                                    .full_width(),
                            ),
                    ),
            )
    }
}
