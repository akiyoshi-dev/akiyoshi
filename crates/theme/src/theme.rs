mod appearance;
mod registry;
mod styles;
mod utils;

use std::{
    fmt::{Display, Formatter},
    str::FromStr,
    sync::Arc,
};

pub use crate::{
    registry::ThemeRegistry,
    styles::{ColorTokens, RadiusTokens, SpacingTokens, TypographyTokens},
};
pub use appearance::*;
use dark_light::Mode;
use gpui::{App, BorrowAppContext, Global, SharedString};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use thiserror::Error;

/// 默认主题标识符，指向内置的暗色主题。
pub const DEFAULT_THEME_ID: &str = "akiyoshi_dark";

/// 主题标识符，通常用于注册和查找主题。
pub type ThemeId = SharedString;

/// 主题相关异常
#[derive(Debug, Error)]
pub enum ThemeError {
    /// 主题未找到错误。当请求的主题 ID 在注册表中不存在时返回。
    #[error("theme with id '{0}' not found")]
    ThemeNotFound(ThemeId),
    /// 主题加载错误。当加载主题数据失败时返回。
    #[error("failed to load theme with id '{0}'")]
    ThemeLoadFailed(ThemeId),
    /// 无效的十六进制颜色错误。当解析十六进制颜色字符串失败时返回。
    #[error("invalid hex color format: '{0}'")]
    InvalidHexColor(String),
    /// 主题版本解析错误。当解析版本字符串失败时返回。
    #[error("invalid theme version format: '{0}'")]
    InvalidThemeVersion(String),
}

/// 主题作者信息。这包含了主题作者的名称和联系信息。
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ThemeAuthor {
    /// 主题作者的名称。
    pub name: String,
    /// 主题作者的联系信息，例如电子邮件地址或社交媒体链接。
    pub contact: String,
}

/// 主题版本信息。这包含了主题的版本号，通常采用语义化版本控制（SemVer）的格式。
#[derive(Debug, Clone, PartialEq)]
pub struct ThemeVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

/// 主题样式。这包含了所有与主题相关的设计令牌，如颜色、间距、半径、排版等。
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ThemeStyles {
    /// 颜色令牌。这包含了主题的颜色设计令牌，通常从语义令牌派生而来。
    pub colors: ColorTokens,
    /// 间距令牌。这包含了主题的间距设计令牌，例如小、中、大等不同级别的间距值。
    pub spacing: SpacingTokens,
    /// 圆角令牌。这包含了主题的圆角设计令牌，例如小、中、大等不同级别的圆角值。
    pub radius: RadiusTokens,
    /// 排版令牌。这包含了主题的排版设计令牌，例如小、中、大等不同级别的字体大小值。
    pub typography: TypographyTokens,
}

/// 主题。这包含了主题的标识符、名称、外观模式、设计令牌以及其他相关信息。
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Theme {
    /// 主题标识符，通常用于注册和查找主题。
    pub id: ThemeId,
    /// 主题说明或备注，提供关于主题的额外信息。
    pub name: String,
    /// 主题的外观模式，如 [`Appearance::Light`] 或 [`Appearance::Dark`]。 不能是任何其它值。
    pub appearance: Appearance,
    /// 主题的描述或备注，提供关于主题的额外信息。
    pub describe: String,
    /// 主题的作者信息，包含了主题作者的名称和联系信息。
    pub author: ThemeAuthor,
    /// 主题的版本信息，包含了主题的版本号，通常采用语义化版本控制（SemVer）的格式。
    pub version: ThemeVersion,
    /// 主题的样式令牌。这包含了所有与主题相关的设计令牌，如颜色、间距、半径、排版等。
    pub styles: ThemeStyles,
}

/// 全局主题。这是一个全局可访问的结构，包含当前活动的主题实例。
pub struct GlobalTheme {
    theme: Arc<Theme>,
}

impl GlobalTheme {
    /// 返回当前激活主题。
    pub fn theme(cx: &App) -> &Arc<Theme> {
        &cx.global::<Self>().theme
    }

    /// 更新当前激活主题。
    pub fn set_theme(cx: &mut App, theme: Arc<Theme>) {
        cx.update_global::<Self, _>(|global, _| {
            global.theme = theme;
        });
    }

    /// 通过 ThemeId 切换主题。
    /// 从全局注册表中查找主题，更新 GlobalTheme，触发所有订阅者刷新。
    pub fn switch_theme(theme_id: impl Into<ThemeId>, cx: &mut App) -> Result<ThemeId, ThemeError> {
        let theme_id: ThemeId = theme_id.into();
        let registry = ThemeRegistry::default_global(cx);
        let theme = registry.get(theme_id.clone())?;
        cx.update_global::<Self, _>(|global, _| {
            global.theme = theme;
        });
        Ok(theme_id)
    }
}

pub trait ActiveTheme {
    fn theme(&self) -> &Arc<Theme>;
}

impl ActiveTheme for App {
    fn theme(&self) -> &Arc<Theme> {
        GlobalTheme::theme(self)
    }
}

/// 主题加载模式，定义了在应用启动时如何加载主题。
pub enum ThemeLoadMode {
    /// 自动 优先使用 [`ThemeLoadMode::Last`] 加载
    /// 如果上一次不存在则使用 [`ThemeLoadMode::Default`] 加载
    Auto,
    /// 上一次使用加载 如果上一次不存在则使用 [`ThemeLoadMode::Default`] 加载
    Last,
    /// 全部加载 加载所有可用的主题 但不设置全局主题 需要用户手动设置
    All,
    /// 默认加载模式 根据传入 [`ThemeId`] 加载主题
    /// 如果 [`ThemeId`] 不存在则使用 [`ThemeLoadMode::All`] 加载
    Default(ThemeId),
}

/// 主题加载模式的默认实现，默认为自动模式。
impl Default for ThemeLoadMode {
    fn default() -> Self {
        Self::Auto
    }
}

impl Global for GlobalTheme {}

impl Appearance {
    pub fn resolve_system(self) -> Self {
        match self {
            Appearance::Auto => match dark_light::detect() {
                Ok(Mode::Light) => Appearance::Light,
                Ok(Mode::Dark) | Ok(Mode::Unspecified) | Err(_) => Appearance::Dark,
            },
            _ => self,
        }
    }
}

impl ThemeVersion {
    /// 将版本信息格式化为字符串，通常采用 "major.minor.patch" 的格式。
    pub fn as_string(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl Display for ThemeVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl FromStr for ThemeVersion {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('.').collect();

        if parts.len() != 3 {
            return Err("invalid version format".into());
        }

        Ok(Self {
            major: parts[0].parse().map_err(|_| "invalid major version")?,

            minor: parts[1].parse().map_err(|_| "invalid minor version")?,

            patch: parts[2].parse().map_err(|_| "invalid patch version")?,
        })
    }
}

impl Serialize for ThemeVersion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for ThemeVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        s.parse().map_err(serde::de::Error::custom)
    }
}

/// 根据给定 [`ThemeId`] 初始化主题系统
///
/// 注册表只在首次调用时创建，后续调用只切换当前主题。
/// 如果提供的主题 ID 无效，则回退到默认主题 [`DEFAULT_THEME_ID`]。
/// 返回当前使用的主题 ID。
pub fn init(theme_id: Option<ThemeId>, cx: &mut App) -> Result<ThemeId, ThemeError> {
    // 若注册表尚未初始化则创建；default_global 只在不存在时插入默认值
    // 但默认值是空的，所以只在首次（GlobalTheme 还不存在）时显式 set_global
    if cx.try_global::<GlobalTheme>().is_none() {
        let theme_registry = ThemeRegistry::new(ThemeLoadMode::default());
        ThemeRegistry::set_global(Some(theme_registry), cx);
    }

    let registry = ThemeRegistry::default_global(cx);
    let theme_id = theme_id.unwrap_or_else(|| DEFAULT_THEME_ID.into());
    let theme = registry.get(theme_id.clone())?;

    // 首次：直接 set_global；后续切换通过 switch_theme，这里统一处理
    if cx.try_global::<GlobalTheme>().is_some() {
        cx.update_global::<GlobalTheme, _>(|global, _| {
            global.theme = theme;
        });
    } else {
        cx.set_global(GlobalTheme { theme });
    }

    Ok(theme_id)
}
