use serde::Deserialize;

/// 间距标记定义主题中使用的间距值，例如边距和填充。
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct SpacingTokens {
    pub xs: f32,
    pub sm: f32,
    pub md: f32,
    pub lg: f32,
    pub xl: f32,
}

impl SpacingTokens {
    pub const fn new(xs: f32, sm: f32, md: f32, lg: f32, xl: f32) -> Self {
        Self { xs, sm, md, lg, xl }
    }
}
