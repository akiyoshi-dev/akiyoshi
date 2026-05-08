use gpui::{px, size, App, AppContext, Bounds, Context, IntoElement, Render, Window, WindowBounds, WindowOptions};
use gpui_platform::application;

struct HelloWorld {
}

impl Render for HelloWorld {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        ui::Button::new("Hello")
    }
}

fn main() {
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
                })
            },
        )
        .unwrap();
        cx.activate(true);
    });
}
