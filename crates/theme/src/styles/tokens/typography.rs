use serde::Deserialize;

/// 圆角标记定义主题中使用的圆角值，例如按钮和卡片的圆角。
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct TypographyTokens {
    pub sm: f32,
    pub md: f32,
    pub lg: f32,
}

impl TypographyTokens {
    pub const fn new(sm: f32, md: f32, lg: f32) -> Self {
        Self { sm, md, lg }
    }
}