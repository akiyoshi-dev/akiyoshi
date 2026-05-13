use std::sync::Arc;

use crate::Theme;

/// 全局字体大小档位。
///
/// 作为 [`crate::GlobalTheme`] 的一部分存储，
/// 所有 UI 组件通过 [`crate::GlobalTheme::font_size`] 统一读取当前字号，
/// 无需在各组件内部硬编码。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FontSizeScale {
    /// 小字号 → `theme.styles.typography.sm`
    Small,
    /// 中字号 → `theme.styles.typography.md`（默认）
    #[default]
    Medium,
    /// 大字号 → `theme.styles.typography.lg`
    Large,
}

impl FontSizeScale {
    /// 返回当前全局字体大小（px），由 [`FontSizeScale`] 和主题 token 共同决定。
    pub fn font_size(&self, theme: &Arc<Theme>) -> f32 {
        match self {
            FontSizeScale::Small => theme.styles.typography.sm,
            FontSizeScale::Medium => theme.styles.typography.md,
            FontSizeScale::Large => theme.styles.typography.lg,
        }
    }
}

impl FontSizeScale {
    /// 循环切换到下一个档位：`Small` → `Medium` → `Large` → `Small`
    pub fn next(self) -> Self {
        match self {
            FontSizeScale::Small => FontSizeScale::Medium,
            FontSizeScale::Medium => FontSizeScale::Large,
            FontSizeScale::Large => FontSizeScale::Small,
        }
    }

    /// 返回适合显示在按钮/菜单上的中文标签
    pub fn label(self) -> &'static str {
        match self {
            FontSizeScale::Small => "字号：小",
            FontSizeScale::Medium => "字号：中",
            FontSizeScale::Large => "字号：大",
        }
    }
}

/// 全局行间距档位。
///
/// 与 [`FontSizeScale`] 一同存储于 [`crate::GlobalTheme`]，
/// 所有 UI 组件通过 [`crate::GlobalTheme::line_height`] 统一读取当前行高。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LineHeightScale {
    /// 紧凑行距：字号 × 1.2
    Compact,
    /// 正常行距：使用主题 `line_height` token（默认）
    #[default]
    Normal,
    /// 宽松行距：字号 × 1.9
    Relaxed,
}

impl LineHeightScale {
    /// 循环切换到下一个档位：`Compact` → `Normal` → `Relaxed` → `Compact`
    pub fn next(&self) -> Self {
        match self {
            LineHeightScale::Compact => LineHeightScale::Normal,
            LineHeightScale::Normal => LineHeightScale::Relaxed,
            LineHeightScale::Relaxed => LineHeightScale::Compact,
        }
    }

    /// 返回适合显示在按钮/菜单上的中文标签
    pub fn label(&self) -> &'static str {
        match self {
            LineHeightScale::Compact => "行距：紧凑",
            LineHeightScale::Normal => "行距：正常",
            LineHeightScale::Relaxed => "行距：宽松",
        }
    }

    /// 返回当前全局行高（px），由 [`LineHeightScale`] 和当前 [`FontSizeScale`] 共同决定。
    pub fn line_height(&self, theme: &Arc<Theme>, font_size_scale: &FontSizeScale) -> f32 {
        let typography_tokens = &theme.styles.typography;
        let font_size = match font_size_scale {
            FontSizeScale::Small => typography_tokens.sm,
            FontSizeScale::Medium => typography_tokens.md,
            FontSizeScale::Large => typography_tokens.lg,
        };
        match self {
            LineHeightScale::Compact => font_size * 1.2,
            LineHeightScale::Normal => match font_size_scale {
                FontSizeScale::Small => typography_tokens.line_height.sm,
                FontSizeScale::Medium => typography_tokens.line_height.md,
                FontSizeScale::Large => typography_tokens.line_height.lg,
            },
            LineHeightScale::Relaxed => font_size * 1.9,
        }
    }
}
