use gpui::*;
use gpui_platform::application;

struct HelloWorld {
    text: SharedString,
}

impl Render for HelloWorld {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .bg(rgb(0xffc0cb))
            .size_full()
            .justify_center()
            .items_center()
            .text_xl()
            // 使用ffc0pb颜色，类似于粉色，来突出显示文本
            .text_color(rgb(0xffffff))
            .child(format!("Hello, {}!", &self.text))
    }
}

fn main() {
    application().run(|cx: &mut App| {
        cx.open_window(WindowOptions::default(), |_, cx| {
            cx.new(|_cx| HelloWorld {
                text: "Akiyoshi(秋吉)".into(),
            })
        })
        .unwrap();
    });
}
