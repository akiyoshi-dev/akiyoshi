use serde::Deserialize;

/// 行高标记，定义不同字体大小下对应的默认行高值。
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct LineHeightTokens {
    /// 小字号对应行高（px）
    pub sm: f32,
    /// 中字号对应行高（px）
    pub md: f32,
    /// 大字号对应行高（px）
    pub lg: f32,
}

impl LineHeightTokens {
    pub const fn new(sm: f32, md: f32, lg: f32) -> Self {
        Self { sm, md, lg }
    }
}

/// 排版标记，定义主题中使用的字体大小与行高值。
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct TypographyTokens {
    /// 小字号（px）
    pub sm: f32,
    /// 中字号（px）
    pub md: f32,
    /// 大字号（px）
    pub lg: f32,
    /// 各字号对应的默认行高
    pub line_height: LineHeightTokens,
}

impl TypographyTokens {
    pub const fn new(sm: f32, md: f32, lg: f32, line_height: LineHeightTokens) -> Self {
        Self {
            sm,
            md,
            lg,
            line_height,
        }
    }
}
