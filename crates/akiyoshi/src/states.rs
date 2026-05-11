use theme::ThemeId;

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
}
