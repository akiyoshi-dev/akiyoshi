use gpui::*;
use gpui_platform::application;

struct HelloWorld {
    text: SharedString,
}

impl Render for HelloWorld {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        ui::Button::new(&self.text)
    }
}

fn main() {
    application().run(|cx: &mut App| {
        let display_id = DisplayId::new(3);
        let bounds = Bounds::centered(None, size(px(500.), px(500.)), cx);
        cx.open_window(
            WindowOptions {
                display_id: Some(display_id),
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| {
                cx.new(|_cx| HelloWorld {
                    text: "Akiyoshi(秋吉)".into(),
                })
            },
        )
        .unwrap();
        cx.activate(true);
    });
}
