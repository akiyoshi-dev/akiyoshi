use crate::states::AppState;
use gpui::{Context, Entity, IntoElement, ParentElement, Render, SharedString, Window, div};
use theme::GlobalTheme;
use ui::Button;
use ui::clickable::Clickable;

pub struct Akiyoshi {
    state: Entity<AppState>,
}

impl Akiyoshi {
    pub fn new(state: Entity<AppState>) -> Self {
        Self { state }
    }
}

impl Render for Akiyoshi {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = GlobalTheme::theme(cx);
        let state = self.state.read(cx);
        let theme_id = state.theme_id.clone();
        println!("当前主题 ID: {:?}", theme_id);
        println!(
            "当前主题字体颜色: {}",
            theme.styles.colors.text.primary.as_hex_string()
        );
        div().child(
            Button::new("change_theme_button")
                .label(
                    theme_id
                        .unwrap_or_else(|| SharedString::from("默认主题"))
                        .as_str(),
                )
                .on_click(cx.listener(|this, _event, _, cx| {
                    this.state.update(cx, |this, cx| {
                        if let Some(theme_id) = this.theme_id.clone() {
                            if theme_id == "akiyoshi_light" {
                                this.theme_id = Some("akiyoshi_dark".into());
                            } else {
                                this.theme_id = Some("akiyoshi_light".into());
                            }
                        } else {
                            this.theme_id = Some("akiyoshi_dark".into());
                        }
                        if let Some(theme_id) = this.theme_id.clone() {
                            // todo 重新设计设置主题逻辑
                            theme::init(Some(theme_id), cx).unwrap();
                        }
                    });
                    cx.notify();
                })),
        )
    }
}
