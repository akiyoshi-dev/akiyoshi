use crate::states::AppState;
use gpui::{Context, Entity, IntoElement, ParentElement, Render, SharedString, Window, div};
use theme::GlobalTheme;
use ui::Button;
use ui::clickable::Clickable;

pub struct Akiyoshi {
    state: Entity<AppState>,
}

impl Akiyoshi {
    pub fn new(state: Entity<AppState>, cx: &mut Context<Self>) -> Self {
        // 订阅全局主题变化，GlobalTheme 被任何人更新时自动重渲染本视图
        cx.observe_global::<GlobalTheme>(|_, cx| {
            cx.notify();
        })
        .detach();

        Self { state }
    }
}

impl Render for Akiyoshi {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme_id = self.state.read(cx).theme_id.clone();
        div().child(
            Button::new("change_theme_button")
                .label(
                    theme_id
                        .unwrap_or_else(|| SharedString::from("默认主题"))
                        .as_str(),
                )
                .on_click({
                    // 直接 clone Entity（cheap，只是引用计数），不需要 cx.listener
                    let state = self.state.clone();
                    move |_event, _window, cx| {
                        state.update(cx, |state, cx| {
                            state.toggle_theme(cx);
                        });
                    }
                }),
        )
    }
}
