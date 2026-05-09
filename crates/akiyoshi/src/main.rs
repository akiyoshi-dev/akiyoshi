use gpui::{
    App, AppContext, Bounds, Context, IntoElement, ParentElement, Render, Styled, Window,
    WindowBounds, WindowOptions, div, px, rgb, size,
};
use gpui_platform::application;
use theme::{ThemeKind, active_theme, active_theme_kind, set_active_theme_kind};
use ui::clickable::Clickable;
use ui::Button;

struct HelloWorld {
    theme_kind: ThemeKind,
}

impl HelloWorld {
    fn next_theme_kind(&self) -> ThemeKind {
        match self.theme_kind {
            ThemeKind::Auto => ThemeKind::Light,
            ThemeKind::Light => ThemeKind::Dark,
            ThemeKind::Dark => ThemeKind::Auto,
        }
    }

    fn mode_label(kind: ThemeKind) -> &'static str {
        match kind {
            ThemeKind::Auto => "Auto",
            ThemeKind::Light => "Light",
            ThemeKind::Dark => "Dark",
        }
    }
}

impl Render for HelloWorld {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let next = self.next_theme_kind();
        let effective = self.theme_kind.resolve_system();
        let palette = active_theme();
        let status = format!(
            "当前: {} (生效: {})",
            Self::mode_label(self.theme_kind),
            Self::mode_label(effective)
        );
        let label = format!(
            "切换主题: {} -> {}",
            Self::mode_label(self.theme_kind),
            Self::mode_label(next)
        );

        div()
            .size(px(500.))
            .bg(rgb(palette.colors.background))
            .text_color(rgb(palette.colors.text_primary))
            .child(status)
            .child(
                Button::new("toggle-theme")
                    .label(label)
                    .on_click(cx.listener(|this, _event, _window, cx| {
                        this.theme_kind = this.next_theme_kind();
                        set_active_theme_kind(this.theme_kind);
                        cx.notify();
                    })),
            )
    }
}

fn main() {
    set_active_theme_kind(ThemeKind::Auto);

    application().run(|cx: &mut App| {
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
                    theme_kind: active_theme_kind(),
                })
            },
        )
        .unwrap();
        cx.activate(true);
    });
}
