mod states;
mod views;

use gpui::{App, AppContext, Bounds, WindowBounds, WindowOptions, px, size};
use gpui_platform::application;

use crate::views::Akiyoshi;
use states::AppState;

fn main() {
    // 创建应用
    let app = application();

    // 创建应用状态实例
    let mut state = AppState::new(None);

    app.run(|cx: &mut App| {
        cx.activate(true);

        // 初始化主题
        let theme_id = theme::init(state.theme_id.clone(), cx).unwrap();

        // 更新主题
        state.theme_id = Some(theme_id);

        // 创建应用状态
        let state = cx.new(|_| state);

        let primary_display = cx.primary_display();
        let window_size = size(px(500.), px(500.));
        let bounds = Bounds::centered(primary_display.as_ref().map(|d| d.id()), window_size, cx);

        cx.open_window(
            WindowOptions {
                display_id: primary_display.map(|d| d.id()),
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            // 传入 cx，以便 Akiyoshi::new 可以注册 observe_global
            |_, cx| cx.new(|cx| Akiyoshi::new(state, cx)),
        )
        .unwrap();
    });
}
