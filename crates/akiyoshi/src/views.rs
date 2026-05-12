use crate::states::AppState;
use gpui::{div, px, Context, Entity, IntoElement, ParentElement, Render, Styled, Window};
use theme::{ActiveTheme, GlobalTheme};
use ui::clickable::Clickable;
use ui::disableable::Disableable;
use ui::{Button, ButtonVariant};

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
                    .flex()
                    .flex_row()
                    .items_center()
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
                    .flex()
                    .flex_row()
                    .items_center()
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
    }
}
