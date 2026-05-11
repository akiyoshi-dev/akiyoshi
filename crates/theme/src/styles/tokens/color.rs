use crate::ThemeError;
use gpui::{Hsla, hsla, Rgba, rgba, rgb};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{
    fmt::{Display, Formatter},
    str::FromStr
};

/// 颜色标记定义主题中使用的颜色值，通常以十六进制格式表示，例如 `#RRGGBB` 或 `#AARRGGBB`。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HexColor(pub u32);

/// 颜色设计令牌。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub struct ColorTokens {
    /// 应用背景色。
    pub background: HexColor,
    /// 默认前景色，通常用于默认文本颜色
    pub foreground: HexColor,
    /// 表面层颜色
    pub surface: SurfaceTokens,
    /// 卡片颜色
    pub card: CardTokens,
    /// 弹出层颜色
    pub popover: PopoverTokens,
    /// 主色调 用于主要按钮 强调操作等
    pub primary: HexColor,
    /// 次要色调 用于次要按钮 辅助操作等
    pub secondary: HexColor,
    /// 强调色调 用于强调文本 链接等
    pub accent: HexColor,
    /// 弱化色调 用于禁用状态 占位文本等
    pub muted: HexColor,
    /// 危险色调 用于错误提示 危险操作等
    pub danger: HexColor,
    /// 成功提示颜色 用于成功提示 等
    pub success: HexColor,
    /// 警告色调 用于警告提示 等
    pub warning: HexColor,
    /// 信息提示颜色 用于信息提示 等
    pub info: HexColor,
    /// 文本颜色
    pub text: TextTokens,
    /// 边框颜色
    pub border: BorderTokens,
    /// 输入框颜色
    pub input: InputTokens,
    /// 聚焦状态外圈颜色
    pub ring: HexColor,
    /// 遮罩层颜色 例如 `Modal` 背景遮罩
    pub overlay: HexColor,
}

/// 表面层颜色，通常用于卡片、弹出层、容器等元素的背景色。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub struct SurfaceTokens {
    /// 默认表面背景颜色
    pub default: HexColor,
    /// 鼠标悬停时的表面颜色
    pub hover: HexColor,
    /// 激活状态下的表面颜色
    pub active: HexColor,
    /// 提升层级后的表面颜色, 例如弹窗
    pub elevated: HexColor,
}

/// 卡片组件颜色
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub struct CardTokens {
    /// 卡片背景颜色
    pub background: HexColor,
    /// 卡片前景颜色，通常用于文本和图标
    pub foreground: HexColor,
    /// 卡片边框颜色
    pub border: HexColor,
}

/// 弹出层颜色
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub struct PopoverTokens {
    /// 弹出层背景颜色
    pub background: HexColor,
    /// 弹出层前景颜色，通常用于文本和图标
    pub foreground: HexColor,
    /// 弹出层边框颜色
    pub border: HexColor,
}

/// 文本颜色
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub struct TextTokens {
    /// 主要文本颜色
    pub primary: HexColor,
    /// 次要文本颜色
    pub secondary: HexColor,
    /// 弱化文本颜色
    pub muted: HexColor,
    /// 禁用文本颜色
    pub disabled: HexColor,
    /// 反色文本颜色，通常用于暗色背景上的文本
    pub inverted: HexColor,
}

/// 边框颜色设计令牌，定义了主题中各种 UI 元素的边框颜色。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub struct BorderTokens {
    /// 默认边框颜色
    pub default: HexColor,
    /// 弱化边框颜色，通常用于禁用状态的元素
    pub muted: HexColor,
    /// 强调边框颜色，通常用于强调状态的元素
    pub strong: HexColor,
}

/// 输入框颜色设计令牌，定义了主题中输入框相关的颜色。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub struct InputTokens {
    /// 输入框背景颜色
    pub background: HexColor,
    /// 输入框前景颜色，通常用于文本和占位符
    pub foreground: HexColor,
    /// 输入框边框颜色
    pub border: HexColor,
    /// 占位符文本颜色
    pub placeholder: HexColor,
}

impl FromStr for HexColor {
    type Err = ThemeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hex = s.trim_start_matches('#');
        match hex.len() {
            // #RRGGBB
            6 => u32::from_str_radix(hex, 16)
                .map(HexColor)
                .map_err(|e| ThemeError::InvalidHexColor(e.to_string())),
            // #AARRGGBB
            8 => u32::from_str_radix(hex, 16)
                .map(HexColor)
                .map_err(|e| ThemeError::InvalidHexColor(e.to_string())),
            _ => Err(ThemeError::InvalidHexColor(format!("length {}", hex.len()))),
        }
    }
}

impl HexColor {
    pub fn r(self) -> u8 {
        ((self.0 >> 16) & 0xff) as u8
    }

    pub fn g(self) -> u8 {
        ((self.0 >> 8) & 0xff) as u8
    }

    pub fn b(self) -> u8 {
        (self.0 & 0xff) as u8
    }

    pub fn a(self) -> u8 {
        ((self.0 >> 24) & 0xff) as u8
    }

    pub fn as_hex_string(self) -> String {
        if self.0 <= 0xffffff {
            format!("#{:06x}", self.0)
        } else {
            format!("#{:08x}", self.0)
        }
    }

    pub fn rgb(self) -> Rgba {
        rgb(self.into())
    }

    pub fn rgba(self) -> Rgba {
        rgba(self.into())
    }
}

impl Display for HexColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // RGB
        if self.0 <= 0xffffff {
            write!(f, "#{:06x}", self.0)
        }
        // RGBA
        else {
            write!(f, "#{:08x}", self.0)
        }
    }
}

impl Serialize for HexColor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for HexColor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

impl From<HexColor> for Hsla {
    fn from(value: HexColor) -> Self {
        let r = value.r() as f32 / 255.0;
        let g = value.g() as f32 / 255.0;
        let b = value.b() as f32 / 255.0;
        let a = value.a() as f32 / 255.0;

        // Convert RGB to HSL
        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let mut h = 0.0;
        let s;
        let l = (max + min) / 2.0;

        if max == min {
            s = 0.0; // achromatic
        } else {
            s = if l > 0.5 {
                (max - min) / (2.0 - max - min)
            } else {
                (max - min) / (max + min)
            };

            h = if max == r {
                (g - b) / (max - min) + if g < b { 6.0 } else { 0.0 }
            } else if max == g {
                (b - r) / (max - min) + 2.0
            } else {
                (r - g) / (max - min) + 4.0
            };

            h /= 6.0;
        }

        hsla(h * 360.0, s, l, a)
    }
}

impl Into<u32> for HexColor {
    fn into(self) -> u32 {
        self.0
    }
}

impl Into<Rgba> for HexColor {
    fn into(self) -> Rgba {
        rgba(self.into())
    }
}