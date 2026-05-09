use gpui::WindowAppearance;
use serde::Deserialize;
use serde::{Deserializer, de};

/// 主题外观模式。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Appearance {
    Light,
    Dark,
    Auto,
}

impl<'de> Deserialize<'de> for Appearance {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.to_lowercase().as_str() {
            "light" => Ok(Appearance::Light),
            "dark" => Ok(Appearance::Dark),
            "auto" => Ok(Appearance::Auto),
            _ => Err(de::Error::unknown_variant(&s, &["light", "dark", "auto"])),
        }
    }
}

impl Appearance {
    /// 判断当前外观模式是否为 `Light` 模式
    /// 包括 `Auto` 模式下系统实际为 `Light` 的情况。
    pub fn is_light(&self) -> bool {
        matches!(self.resolve_system(), Appearance::Light)
    }
}

impl From<WindowAppearance> for Appearance {
    fn from(value: WindowAppearance) -> Self {
        match value {
            WindowAppearance::Light | WindowAppearance::VibrantLight => Self::Light,
            WindowAppearance::Dark | WindowAppearance::VibrantDark => Self::Dark,
        }
    }
}

impl Default for Appearance {
    fn default() -> Self {
        Self::Auto
    }
}