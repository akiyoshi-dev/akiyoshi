use crate::{Theme, ThemeError, ThemeId, ThemeLoadMode};
use gpui::{App, Global};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use serde_json;

/// 内置主题的 light JSON 数据。这些数据在编译时包含在二进制文件中，并用于初始化默认的主题注册表。
const LIGHT_THEME_JSON: &str = include_str!("../themes/light.json");
/// 内置主题的 dark JSON 数据。这些数据在编译时包含在二进制文件中，并用于初始化默认的主题注册表。
const DARK_THEME_JSON: &str = include_str!("../themes/dark.json");

#[derive(Default)]
struct GlobalThemeRegistry(Arc<ThemeRegistry>);

impl Global for GlobalThemeRegistry {}

/// 主题注册表的状态。这包含了所有已注册的主题及其相关数据。
#[derive(Default)]
struct ThemeRegistryState {
    themes: HashMap<ThemeId, Theme>,
}

/// 主题注册表。这用于存储可用主题及其相关数据。
#[derive(Default)]
pub struct ThemeRegistry {
    state: RwLock<ThemeRegistryState>,
}

impl ThemeRegistry {
    /// 创建一个新的 [`ThemeRegistry`]
    pub fn new(mode: ThemeLoadMode) -> Self {
        let registry = Self {
            state: RwLock::new(ThemeRegistryState {
                themes: HashMap::default(),
            }),
        };
        match mode {
            ThemeLoadMode::Auto => {
                registry.load_builtin_themes();
            }
            ThemeLoadMode::Last => {
                registry.load_builtin_themes();
            }
            ThemeLoadMode::All => {
                registry.load_builtin_themes();
            }
            ThemeLoadMode::Default(_) => {
                registry.load_builtin_themes();
            }
        }
        registry
    }

    /// 加载内置主题（light 和 dark）到注册表
    fn load_builtin_themes(&self) {
        let _ = self.register_theme_from_json("y", LIGHT_THEME_JSON);
        let _ = self.register_theme_from_json("akiyoshi_dark", DARK_THEME_JSON);
    }

    /// 从 JSON 字符串注册一个主题到注册表
    fn register_theme_from_json(&self, expected_id: &str, json: &str) -> Result<(), ThemeError> {
        let theme: Theme = serde_json::from_str(json)
            .map_err(|_| ThemeError::ThemeLoadFailed(expected_id.into()))?;

        let mut state = self.state.write().unwrap();
        state.themes.insert(theme.id.clone(), theme);
        Ok(())
    }

    /// 设置全局 [`ThemeRegistry`]
    pub(crate) fn set_global(registry: Option<ThemeRegistry>, cx: &mut App) {
        cx.set_global(GlobalThemeRegistry(Arc::new(
            registry.unwrap_or_else(|| ThemeRegistry::default()),
        )));
    }

    /// 返回全局 [`ThemeRegistry`]。
    ///
    /// 如果全局 [`ThemeRegistry`] 不存在则插入一个默认的实例并返回。
    pub fn default_global(cx: &mut App) -> Arc<Self> {
        cx.default_global::<GlobalThemeRegistry>().0.clone()
    }

    /// 通过 [`ThemeId`] 返回一个主题实例的引用计数指针。
    ///
    /// 如果请求的主题 ID 不存在于注册表中，则返回 [`ThemeError::ThemeNotFound`]。
    pub fn get(&self, theme_id: impl Into<ThemeId>) -> Result<Arc<Theme>, ThemeError> {
        let state = self.state.read().unwrap();
        let theme_id = theme_id.into();
        state
            .themes
            .get(&theme_id)
            .cloned()
            .map(Arc::new)
            .ok_or(ThemeError::ThemeNotFound(theme_id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builtin_themes_load_correctly() {
        let registry = ThemeRegistry::new(ThemeLoadMode::All);

        // Verify light theme is loaded
        match registry.get("akiyoshi_light") {
            Ok(_) => {}
            Err(e) => panic!("Light theme load failed: {:?}", e),
        }

        // Verify dark theme is loaded
        match registry.get("akiyoshi_dark") {
            Ok(_) => {}
            Err(e) => panic!("Dark theme load failed: {:?}", e),
        }
    }

    #[test]
    fn theme_not_found_returns_error() {
        let registry = ThemeRegistry::new(ThemeLoadMode::All);

        // Verify requesting non-existent theme returns error
        assert!(
            registry.get("nonexistent_theme").is_err(),
            "Non-existent theme should return error"
        );
    }
}
