use theme::ThemeId;

/// Akiyoshi 的应用程序状态。
/// 排版档位（字号/行距）由 [`theme::GlobalTheme`] 统一管理，无需在此存储。
#[derive(Clone)]
pub struct AppState {
    /// 当前使用的主题 ID，用于持久化和启动时恢复。
    pub theme_id: Option<ThemeId>,
}

impl AppState {
    pub fn new(theme_id: Option<ThemeId>) -> Self {
        Self { theme_id }
    }
}
