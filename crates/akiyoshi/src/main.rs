use gpui::{
    App, AppContext, Bounds, Context, IntoElement, ParentElement, Render, Styled, Window,
    WindowBounds, WindowOptions, div, px, rgb, size,
};
use gpui_platform::application;
use theme::{GlobalTheme, ThemeId};
use ui::{Button, clickable::Clickable};

struct HelloWorld {
    theme_id: ThemeId,
}

impl HelloWorld {
    fn next_theme_id(&self) -> ThemeId {
        if self.theme_id.as_ref() == "akiyoshi_dark" {
            "akiyoshi_light".into()
        } else {
            "akiyoshi_dark".into()
        }
    }
}

impl Render for HelloWorld {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = GlobalTheme::theme(cx);
        let next_theme = self.next_theme_id();
        let button_label = if next_theme.as_ref() == "akiyoshi_light" {
            "切换到 Light"
        } else {
            "切换到 Dark"
        };

        div()
            .bg(rgb(theme.styles.colors.background.into()))
            .text_color(rgb(theme.styles.colors.text.primary.into()))
            .items_center()
            .justify_center()
            .child(
                div()
                    .flex()
                    .justify_center()
                    .items_center()
                    .child(
                        Button::new("theme-button")
                            .label(button_label)
                            .on_click(cx.listener(|this, _event, _window, cx| {
                                this.theme_id = this.next_theme_id();
                                let _ = theme::init(Some(this.theme_id.clone()), cx);
                                cx.notify();
                            }))
                    )
                    .h(px(120.))
                    .px(px(125.0))
                ,
            )
    }
}

fn main() {
    application().run(|cx: &mut App| {
        // 初始化主题
        theme::init(None, cx).unwrap();

        let primary_display = cx.primary_display();
        let window_size = size(px(500.), px(500.));
        let bounds = Bounds::centered(primary_display.as_ref().map(|d| d.id()), window_size, cx);
        cx.open_window(
            WindowOptions {
                display_id: primary_display.map(|d| d.id()),
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| {
                cx.new(|_cx| HelloWorld {
                    theme_id: theme::DEFAULT_THEME_ID.into(),
                })
            },
        )
        .unwrap();
        cx.activate(true);
    });
}
