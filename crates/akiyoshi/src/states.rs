use gpui::App;
use theme::{GlobalTheme, ThemeId};

/// Akiyoshi 的应用程序状态
#[derive(Clone)]
pub struct AppState {
    /// 当前使用的主题 ID
    pub theme_id: Option<ThemeId>,
}

impl AppState {
    pub fn new(theme_id: Option<ThemeId>) -> Self {
        Self { theme_id }
    }

    /// 在明暗主题之间切换，同时更新 GlobalTheme（会自动通知订阅者）
    pub fn toggle_theme(&mut self, cx: &mut App) {
        let next_id = match self.theme_id.as_deref() {
            Some("akiyoshi_light") => "akiyoshi_dark",
            _ => "akiyoshi_light",
        };

        match GlobalTheme::switch_theme(next_id, cx) {
            Ok(id) => self.theme_id = Some(id),
            Err(e) => eprintln!("切换主题失败: {e}"),
        }
    }
}
