mod states;
mod views;

use gpui::{
    App, AppContext, Bounds, TitlebarOptions, WindowBounds, WindowOptions, point, px, size,
};
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
                // 透明原生标题栏 + 红绿灯居中于 40px 内容标题栏（y = (40 - 12) / 2 = 14）
                titlebar: Some(TitlebarOptions {
                    title: None,
                    appears_transparent: true,
                    traffic_light_position: Some(point(px(9.), px(14.))),
                }),
                window_min_size: Some(window_size),
                ..Default::default()
            },
            |_, cx| cx.new(|_cx| Akiyoshi::new(state)),
        )
        .unwrap();
    });
}
