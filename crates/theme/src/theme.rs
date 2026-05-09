use std::sync::{LazyLock, RwLock};

use dark_light::Mode;
use serde::Deserialize;

const LIGHT_THEME_JSON: &str = include_str!("../themes/light.json");
const DARK_THEME_JSON: &str = include_str!("../themes/dark.json");

static ACTIVE_THEME_KIND: LazyLock<RwLock<ThemeKind>> =
    LazyLock::new(|| RwLock::new(ThemeKind::Auto));

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeKind {
    Light,
    Dark,
    Auto,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Theme {
    pub kind: ThemeKind,
    pub name: String,
    pub note: String,
    pub semantic: SemanticTokens,
    pub colors: ColorTokens,
    pub spacing: SpacingTokens,
    pub radius: RadiusTokens,
    pub typography: TypographyTokens,
    pub styles: StyleTokens,
    pub button: ButtonTokens,
}

impl Theme {
    pub fn from_kind(kind: ThemeKind) -> Self {
        Self::try_from_kind(kind).unwrap_or_else(|_| Self::fallback_dark())
    }

    pub fn try_from_kind(kind: ThemeKind) -> Result<Self, ThemeLoadError> {
        match kind.resolve_system() {
            ThemeKind::Light => parse_theme_json(ThemeKind::Light, LIGHT_THEME_JSON),
            ThemeKind::Dark | ThemeKind::Auto => parse_theme_json(ThemeKind::Dark, DARK_THEME_JSON),
        }
    }

    pub fn light() -> Self {
        Self::from_kind(ThemeKind::Light)
    }

    pub fn dark() -> Self {
        Self::from_kind(ThemeKind::Dark)
    }

    fn fallback_dark() -> Self {
        let semantic = SemanticTokens {
            base: BaseSemanticTokens {
                background: 0x020817,
                foreground: 0xf8fafc,
                card: 0x020817,
                card_foreground: 0xf8fafc,
                popover: 0x020817,
                popover_foreground: 0xf8fafc,
                border: 0x1e293b,
                input: 0x1e293b,
                ring: 0x334155,
            },
            intent: IntentSemanticTokens {
                primary: InteractiveColorTokens {
                    default: 0xf8fafc,
                    foreground: 0x0f172a,
                    hover: 0xe2e8f0,
                    active: 0xcbd5e1,
                    disabled: 0x334155,
                    disabled_foreground: 0x64748b,
                },
                secondary: InteractiveColorTokens {
                    default: 0x1e293b,
                    foreground: 0xf8fafc,
                    hover: 0x334155,
                    active: 0x475569,
                    disabled: 0x1f2937,
                    disabled_foreground: 0x64748b,
                },
                destructive: InteractiveColorTokens {
                    default: 0x7f1d1d,
                    foreground: 0xfef2f2,
                    hover: 0x991b1b,
                    active: 0xb91c1c,
                    disabled: 0x450a0a,
                    disabled_foreground: 0xfecaca,
                },
                muted: InteractiveColorTokens {
                    default: 0x1e293b,
                    foreground: 0x94a3b8,
                    hover: 0x334155,
                    active: 0x475569,
                    disabled: 0x1f2937,
                    disabled_foreground: 0x64748b,
                },
                accent: InteractiveColorTokens {
                    default: 0x1e293b,
                    foreground: 0xf8fafc,
                    hover: 0x334155,
                    active: 0x475569,
                    disabled: 0x1f2937,
                    disabled_foreground: 0x64748b,
                },
            },
            state: StateSemanticTokens {
                success: FeedbackColorTokens {
                    default: 0x22c55e,
                    foreground: 0x052e16,
                    subtle: 0x14532d,
                },
                warning: FeedbackColorTokens {
                    default: 0xf59e0b,
                    foreground: 0x451a03,
                    subtle: 0x78350f,
                },
                info: FeedbackColorTokens {
                    default: 0x3b82f6,
                    foreground: 0x172554,
                    subtle: 0x1e3a8a,
                },
            },
            interactive: InteractiveSurfaceTokens {
                surface_hover: 0x111827,
                surface_active: 0x1f2937,
                overlay: 0x000000,
                selection: 0x1d4ed8,
            },
        };

        Self {
            kind: ThemeKind::Dark,
            name: "dark".to_owned(),
            note: "Fallback dark theme when JSON parsing fails.".to_owned(),
            semantic,
            colors: ColorTokens::from_semantic(&semantic),
            spacing: SpacingTokens::new(4., 8., 12., 16., 24.),
            radius: RadiusTokens::new(4., 8., 12.),
            typography: TypographyTokens::new(12., 14., 16.),
            styles: StyleTokens::from_semantic(&semantic),
            button: ButtonTokens {
                background: semantic.intent.primary.default,
                text: semantic.intent.primary.foreground,
                border: semantic.base.border,
                hover_background: semantic.intent.primary.hover,
                active_background: semantic.intent.primary.active,
                disabled_background: semantic.intent.primary.disabled,
                disabled_text: semantic.intent.primary.disabled_foreground,
                size: 120.,
            },
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        active_theme()
    }
}

impl ThemeKind {
    pub fn resolve_system(self) -> Self {
        match self {
            ThemeKind::Auto => match dark_light::detect() {
                Ok(Mode::Light) => ThemeKind::Light,
                Ok(Mode::Dark) | Ok(Mode::Unspecified) | Err(_) => ThemeKind::Dark,
            },
            _ => self,
        }
    }
}

pub fn set_active_theme_kind(kind: ThemeKind) {
    if let Ok(mut current) = ACTIVE_THEME_KIND.write() {
        *current = kind;
    }
}

pub fn active_theme_kind() -> ThemeKind {
    ACTIVE_THEME_KIND
        .read()
        .map(|kind| *kind)
        .unwrap_or(ThemeKind::Dark)
}

pub fn active_theme() -> Theme {
    Theme::from_kind(active_theme_kind())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColorTokens {
    /// 应用背景色。
    pub background: u32,
    /// 卡片或容器背景色。
    pub surface: u32,
    /// 主文字颜色。
    pub text_primary: u32,
    /// 次文字颜色。
    pub text_secondary: u32,
    /// 强调色。
    pub accent: u32,
    /// 边框颜色。
    pub border: u32,
}

impl ColorTokens {
    fn from_semantic(semantic: &SemanticTokens) -> Self {
        Self {
            background: semantic.base.background,
            surface: semantic.base.card,
            text_primary: semantic.base.foreground,
            text_secondary: semantic.intent.muted.foreground,
            accent: semantic.intent.primary.default,
            border: semantic.base.border,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SemanticTokens {
    pub base: BaseSemanticTokens,
    pub intent: IntentSemanticTokens,
    pub state: StateSemanticTokens,
    pub interactive: InteractiveSurfaceTokens,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BaseSemanticTokens {
    pub background: u32,
    pub foreground: u32,
    pub card: u32,
    pub card_foreground: u32,
    pub popover: u32,
    pub popover_foreground: u32,
    pub border: u32,
    pub input: u32,
    pub ring: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntentSemanticTokens {
    pub primary: InteractiveColorTokens,
    pub secondary: InteractiveColorTokens,
    pub destructive: InteractiveColorTokens,
    pub muted: InteractiveColorTokens,
    pub accent: InteractiveColorTokens,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InteractiveColorTokens {
    pub default: u32,
    pub foreground: u32,
    pub hover: u32,
    pub active: u32,
    pub disabled: u32,
    pub disabled_foreground: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StateSemanticTokens {
    pub success: FeedbackColorTokens,
    pub warning: FeedbackColorTokens,
    pub info: FeedbackColorTokens,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FeedbackColorTokens {
    pub default: u32,
    pub foreground: u32,
    pub subtle: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InteractiveSurfaceTokens {
    pub surface_hover: u32,
    pub surface_active: u32,
    pub overlay: u32,
    pub selection: u32,
}

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

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct RadiusTokens {
    pub sm: f32,
    pub md: f32,
    pub lg: f32,
}

impl RadiusTokens {
    pub const fn new(sm: f32, md: f32, lg: f32) -> Self {
        Self { sm, md, lg }
    }
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StyleTokens {
    pub elevated_surface: u32,
    pub muted_surface: u32,
    pub focus_ring: u32,
}

impl StyleTokens {
    fn from_semantic(semantic: &SemanticTokens) -> Self {
        Self {
            elevated_surface: semantic.base.popover,
            muted_surface: semantic.intent.muted.default,
            focus_ring: semantic.base.ring,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ButtonTokens {
    pub background: u32,
    pub text: u32,
    pub border: u32,
    pub hover_background: u32,
    pub active_background: u32,
    pub disabled_background: u32,
    pub disabled_text: u32,
    pub size: f32,
}

#[derive(Debug)]
pub enum ThemeLoadError {
    InvalidJson,
    InvalidColor,
}

#[derive(Deserialize)]
struct ThemeJson {
    name: String,
    note: String,
    semantic: SemanticTokensJson,
    spacing: SpacingTokens,
    radius: RadiusTokens,
    typography: TypographyTokens,
    button: ButtonTokensJson,
}

#[derive(Deserialize)]
struct SemanticTokensJson {
    base: BaseSemanticTokensJson,
    intent: IntentSemanticTokensJson,
    state: StateSemanticTokensJson,
    interactive: InteractiveSurfaceTokensJson,
}

#[derive(Deserialize)]
struct BaseSemanticTokensJson {
    background: String,
    foreground: String,
    card: String,
    card_foreground: String,
    popover: String,
    popover_foreground: String,
    border: String,
    input: String,
    ring: String,
}

#[derive(Deserialize)]
struct IntentSemanticTokensJson {
    primary: InteractiveColorTokensJson,
    secondary: InteractiveColorTokensJson,
    destructive: InteractiveColorTokensJson,
    muted: InteractiveColorTokensJson,
    accent: InteractiveColorTokensJson,
}

#[derive(Deserialize)]
struct InteractiveColorTokensJson {
    #[serde(rename = "default")]
    default_color: String,
    foreground: String,
    hover: String,
    active: String,
    disabled: String,
    disabled_foreground: String,
}

#[derive(Deserialize)]
struct StateSemanticTokensJson {
    success: FeedbackColorTokensJson,
    warning: FeedbackColorTokensJson,
    info: FeedbackColorTokensJson,
}

#[derive(Deserialize)]
struct FeedbackColorTokensJson {
    #[serde(rename = "default")]
    default_color: String,
    foreground: String,
    subtle: String,
}

#[derive(Deserialize)]
struct InteractiveSurfaceTokensJson {
    surface_hover: String,
    surface_active: String,
    overlay: String,
    selection: String,
}

#[derive(Deserialize)]
struct ButtonTokensJson {
    size: f32,
    variants: ButtonVariantsJson,
}

#[derive(Deserialize)]
struct ButtonVariantsJson {
    primary: ButtonVariantJson,
    secondary: ButtonVariantJson,
    destructive: ButtonVariantJson,
    outline: ButtonVariantJson,
    ghost: ButtonVariantJson,
    link: ButtonVariantJson,
}

#[derive(Deserialize)]
struct ButtonVariantJson {
    background: String,
    foreground: String,
    border: String,
    hover_background: String,
    active_background: String,
    disabled_background: String,
    disabled_foreground: String,
}

fn parse_theme_json(kind: ThemeKind, json: &str) -> Result<Theme, ThemeLoadError> {
    let raw: ThemeJson = serde_json::from_str(json).map_err(|_| ThemeLoadError::InvalidJson)?;
    let semantic = parse_semantic_tokens(raw.semantic)?;
    let _other_variants = (
        &raw.button.variants.secondary,
        &raw.button.variants.destructive,
        &raw.button.variants.outline,
        &raw.button.variants.ghost,
        &raw.button.variants.link,
    );

    Ok(Theme {
        kind,
        name: raw.name,
        note: raw.note,
        colors: ColorTokens::from_semantic(&semantic),
        spacing: raw.spacing,
        radius: raw.radius,
        typography: raw.typography,
        styles: StyleTokens::from_semantic(&semantic),
        semantic,
        button: ButtonTokens {
            background: parse_hex_color(&raw.button.variants.primary.background)?,
            text: parse_hex_color(&raw.button.variants.primary.foreground)?,
            border: parse_hex_color(&raw.button.variants.primary.border)?,
            hover_background: parse_hex_color(&raw.button.variants.primary.hover_background)?,
            active_background: parse_hex_color(&raw.button.variants.primary.active_background)?,
            disabled_background: parse_hex_color(&raw.button.variants.primary.disabled_background)?,
            disabled_text: parse_hex_color(&raw.button.variants.primary.disabled_foreground)?,
            size: raw.button.size,
        },
    })
}

fn parse_semantic_tokens(raw: SemanticTokensJson) -> Result<SemanticTokens, ThemeLoadError> {
    Ok(SemanticTokens {
        base: BaseSemanticTokens {
            background: parse_hex_color(&raw.base.background)?,
            foreground: parse_hex_color(&raw.base.foreground)?,
            card: parse_hex_color(&raw.base.card)?,
            card_foreground: parse_hex_color(&raw.base.card_foreground)?,
            popover: parse_hex_color(&raw.base.popover)?,
            popover_foreground: parse_hex_color(&raw.base.popover_foreground)?,
            border: parse_hex_color(&raw.base.border)?,
            input: parse_hex_color(&raw.base.input)?,
            ring: parse_hex_color(&raw.base.ring)?,
        },
        intent: IntentSemanticTokens {
            primary: parse_interactive_color_tokens(raw.intent.primary)?,
            secondary: parse_interactive_color_tokens(raw.intent.secondary)?,
            destructive: parse_interactive_color_tokens(raw.intent.destructive)?,
            muted: parse_interactive_color_tokens(raw.intent.muted)?,
            accent: parse_interactive_color_tokens(raw.intent.accent)?,
        },
        state: StateSemanticTokens {
            success: parse_feedback_color_tokens(raw.state.success)?,
            warning: parse_feedback_color_tokens(raw.state.warning)?,
            info: parse_feedback_color_tokens(raw.state.info)?,
        },
        interactive: InteractiveSurfaceTokens {
            surface_hover: parse_hex_color(&raw.interactive.surface_hover)?,
            surface_active: parse_hex_color(&raw.interactive.surface_active)?,
            overlay: parse_hex_color(&raw.interactive.overlay)?,
            selection: parse_hex_color(&raw.interactive.selection)?,
        },
    })
}

fn parse_interactive_color_tokens(
    raw: InteractiveColorTokensJson,
) -> Result<InteractiveColorTokens, ThemeLoadError> {
    Ok(InteractiveColorTokens {
        default: parse_hex_color(&raw.default_color)?,
        foreground: parse_hex_color(&raw.foreground)?,
        hover: parse_hex_color(&raw.hover)?,
        active: parse_hex_color(&raw.active)?,
        disabled: parse_hex_color(&raw.disabled)?,
        disabled_foreground: parse_hex_color(&raw.disabled_foreground)?,
    })
}

fn parse_feedback_color_tokens(raw: FeedbackColorTokensJson) -> Result<FeedbackColorTokens, ThemeLoadError> {
    Ok(FeedbackColorTokens {
        default: parse_hex_color(&raw.default_color)?,
        foreground: parse_hex_color(&raw.foreground)?,
        subtle: parse_hex_color(&raw.subtle)?,
    })
}

fn parse_hex_color(input: &str) -> Result<u32, ThemeLoadError> {
    let trimmed = input.trim();
    let hex = trimmed.strip_prefix('#').unwrap_or(trimmed);
    if hex.len() != 6 {
        return Err(ThemeLoadError::InvalidColor);
    }

    u32::from_str_radix(hex, 16).map_err(|_| ThemeLoadError::InvalidColor)
}

#[cfg(test)]
mod tests {
    use super::{active_theme, set_active_theme_kind, Theme, ThemeKind};

    #[test]
    fn creates_light_and_dark_themes_from_json() {
        let light = Theme::from_kind(ThemeKind::Light);
        let dark = Theme::from_kind(ThemeKind::Dark);

        assert_ne!(light.colors.background, dark.colors.background);
        assert!(light.button.size > 0.);
        assert_ne!(light.semantic.intent.primary.default, dark.semantic.intent.primary.default);
        assert!(dark.button.size > 0.);
        assert_eq!(light.name, "light");
        assert_eq!(dark.name, "dark");
    }

    #[test]
    fn auto_theme_can_be_activated() {
        set_active_theme_kind(ThemeKind::Auto);
        let theme = active_theme();

        assert!(theme.button.size > 0.);
    }
}

