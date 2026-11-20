//! Theme engine and `gpui-kit` preset palettes mapped to `egui::Visuals`.

use egui::{Color32, Context, Rounding, Stroke, Visuals};

/// Light or dark polarity of a `ThemeTokens` preset.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ThemeMode {
    Light,
    Dark,
}

/// Curated `gpui-kit` theme presets included out of the box.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ThemeId {
    #[default]
    AuroraLight,
    TokyoNight,
    TokyoMoon,
    TokyoStorm,
    CatppuccinMocha,
    CatppuccinLatte,
    CatppuccinFrappe,
    CatppuccinMacchiato,
    GruvboxDark,
    GruvboxLight,
    SolarizedDark,
    SolarizedLight,
    AyuDark,
    AyuLight,
    EverforestDark,
    EverforestLight,
    FlexokiDark,
    FlexokiLight,
    MacOsClassicDark,
    MacOsClassicLight,
}

impl ThemeId {
    /// Human-readable display name.
    pub const fn label(self) -> &'static str {
        match self {
            ThemeId::AuroraLight => "Aurora Light",
            ThemeId::TokyoNight => "Tokyo Night",
            ThemeId::TokyoMoon => "Tokyo Moon",
            ThemeId::TokyoStorm => "Tokyo Storm",
            ThemeId::CatppuccinMocha => "Catppuccin Mocha",
            ThemeId::CatppuccinLatte => "Catppuccin Latte",
            ThemeId::CatppuccinFrappe => "Catppuccin Frappe",
            ThemeId::CatppuccinMacchiato => "Catppuccin Macchiato",
            ThemeId::GruvboxDark => "Gruvbox Dark",
            ThemeId::GruvboxLight => "Gruvbox Light",
            ThemeId::SolarizedDark => "Solarized Dark",
            ThemeId::SolarizedLight => "Solarized Light",
            ThemeId::AyuDark => "Ayu Dark",
            ThemeId::AyuLight => "Ayu Light",
            ThemeId::EverforestDark => "Everforest Dark",
            ThemeId::EverforestLight => "Everforest Light",
            ThemeId::FlexokiDark => "Flexoki Dark",
            ThemeId::FlexokiLight => "Flexoki Light",
            ThemeId::MacOsClassicDark => "macOS Classic Dark",
            ThemeId::MacOsClassicLight => "macOS Classic Light",
        }
    }

    /// All built-in presets in display order.
    pub const fn all() -> &'static [ThemeId] {
        &[
            ThemeId::AuroraLight,
            ThemeId::TokyoNight,
            ThemeId::TokyoMoon,
            ThemeId::TokyoStorm,
            ThemeId::CatppuccinMocha,
            ThemeId::CatppuccinLatte,
            ThemeId::CatppuccinFrappe,
            ThemeId::CatppuccinMacchiato,
            ThemeId::GruvboxDark,
            ThemeId::GruvboxLight,
            ThemeId::SolarizedDark,
            ThemeId::SolarizedLight,
            ThemeId::AyuDark,
            ThemeId::AyuLight,
            ThemeId::EverforestDark,
            ThemeId::EverforestLight,
            ThemeId::FlexokiDark,
            ThemeId::FlexokiLight,
            ThemeId::MacOsClassicDark,
            ThemeId::MacOsClassicLight,
        ]
    }
}

/// Complete semantic color palette shared by all `egui-kit` widgets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThemeTokens {
    pub id: ThemeId,
    pub mode: ThemeMode,
    pub background: Color32,
    pub foreground: Color32,
    pub card: Color32,
    pub card_foreground: Color32,
    pub primary: Color32,
    pub primary_foreground: Color32,
    pub secondary: Color32,
    pub secondary_foreground: Color32,
    pub muted: Color32,
    pub muted_foreground: Color32,
    pub accent: Color32,
    pub accent_foreground: Color32,
    pub border: Color32,
    pub input: Color32,
    pub ring: Color32,
    pub success: Color32,
    pub warning: Color32,
    pub destructive: Color32,
}

impl ThemeTokens {
    /// Builds a [`ThemeTokens`] instance for a built-in [`ThemeId`].
    pub fn from_preset(id: ThemeId) -> Self {
        match id {
            ThemeId::AuroraLight => Self {
                id,
                mode: ThemeMode::Light,
                background: hex("#FFFFFF"),
                foreground: hex("#020617"),
                card: hex("#FFFFFF"),
                card_foreground: hex("#020617"),
                primary: hex("#0F172A"),
                primary_foreground: hex("#F8FAFC"),
                secondary: hex("#E2E8F0"),
                secondary_foreground: hex("#0F172A"),
                muted: hex("#F1F5F9"),
                muted_foreground: hex("#64748B"),
                accent: hex("#F1F5F9"),
                accent_foreground: hex("#0F172A"),
                border: hex("#E2E8F0"),
                input: hex("#CBD5E1"),
                ring: hex("#0F172A"),
                success: hex("#16A34A"),
                warning: hex("#CA8A04"),
                destructive: hex("#DC2626"),
            },
            ThemeId::TokyoNight | ThemeId::TokyoMoon | ThemeId::TokyoStorm => Self {
                id,
                mode: ThemeMode::Dark,
                background: hex(if id == ThemeId::TokyoMoon { "#222436" } else if id == ThemeId::TokyoStorm { "#24283B" } else { "#1A1B26" }),
                foreground: hex("#C0CAF5"),
                card: hex("#1F2335"),
                card_foreground: hex("#C0CAF5"),
                primary: hex("#7AA2F7"),
                primary_foreground: hex("#15161E"),
                secondary: hex("#292E42"),
                secondary_foreground: hex("#C0CAF5"),
                muted: hex("#24283B"),
                muted_foreground: hex("#737AA2"),
                accent: hex("#3B4261"),
                accent_foreground: hex("#C0CAF5"),
                border: hex("#292E42"),
                input: hex("#3B4261"),
                ring: hex("#7AA2F7"),
                success: hex("#9ECE6A"),
                warning: hex("#E0AF68"),
                destructive: hex("#F7768E"),
            },
            ThemeId::CatppuccinMocha | ThemeId::CatppuccinFrappe | ThemeId::CatppuccinMacchiato => Self {
                id,
                mode: ThemeMode::Dark,
                background: hex(if id == ThemeId::CatppuccinFrappe { "#303446" } else if id == ThemeId::CatppuccinMacchiato { "#24273A" } else { "#1E1E2E" }),
                foreground: hex("#CDD6F4"),
                card: hex("#181825"),
                card_foreground: hex("#CDD6F4"),
                primary: hex("#89B4FA"),
                primary_foreground: hex("#11111B"),
                secondary: hex("#313244"),
                secondary_foreground: hex("#CDD6F4"),
                muted: hex("#313244"),
                muted_foreground: hex("#A6ADC8"),
                accent: hex("#45475A"),
                accent_foreground: hex("#CDD6F4"),
                border: hex("#313244"),
                input: hex("#45475A"),
                ring: hex("#89B4FA"),
                success: hex("#A6E3A1"),
                warning: hex("#F9E2AF"),
                destructive: hex("#F38BA8"),
            },
            ThemeId::CatppuccinLatte => Self {
                id,
                mode: ThemeMode::Light,
                background: hex("#EFF1F5"),
                foreground: hex("#4C4F69"),
                card: hex("#E6E9EF"),
                card_foreground: hex("#4C4F69"),
                primary: hex("#1E66F5"),
                primary_foreground: hex("#EFF1F5"),
                secondary: hex("#CCD0DA"),
                secondary_foreground: hex("#4C4F69"),
                muted: hex("#DCE0E8"),
                muted_foreground: hex("#6C6F85"),
                accent: hex("#BCC0CC"),
                accent_foreground: hex("#4C4F69"),
                border: hex("#CCD0DA"),
                input: hex("#BCC0CC"),
                ring: hex("#1E66F5"),
                success: hex("#40A02B"),
                warning: hex("#DF8E1D"),
                destructive: hex("#D20F39"),
            },
            ThemeId::GruvboxDark => Self {
                id,
                mode: ThemeMode::Dark,
                background: hex("#282828"),
                foreground: hex("#EBDBB2"),
                card: hex("#1D2021"),
                card_foreground: hex("#EBDBB2"),
                primary: hex("#FE8019"),
                primary_foreground: hex("#282828"),
                secondary: hex("#3C3836"),
                secondary_foreground: hex("#EBDBB2"),
                muted: hex("#32302F"),
                muted_foreground: hex("#A89984"),
                accent: hex("#504945"),
                accent_foreground: hex("#EBDBB2"),
                border: hex("#3C3836"),
                input: hex("#504945"),
                ring: hex("#FE8019"),
                success: hex("#B8BB26"),
                warning: hex("#FABD2F"),
                destructive: hex("#FB4934"),
            },
            ThemeId::GruvboxLight => Self {
                id,
                mode: ThemeMode::Light,
                background: hex("#FBF1C7"),
                foreground: hex("#3C3836"),
                card: hex("#F2E5BC"),
                card_foreground: hex("#3C3836"),
                primary: hex("#D65D0E"),
                primary_foreground: hex("#FBF1C7"),
                secondary: hex("#EBDBB2"),
                secondary_foreground: hex("#3C3836"),
                muted: hex("#EBDBB2"),
                muted_foreground: hex("#7C6F64"),
                accent: hex("#D5C4A1"),
                accent_foreground: hex("#3C3836"),
                border: hex("#D5C4A1"),
                input: hex("#BDAE93"),
                ring: hex("#D65D0E"),
                success: hex("#79740E"),
                warning: hex("#B57614"),
                destructive: hex("#CC241D"),
            },
            ThemeId::SolarizedDark => Self {
                id,
                mode: ThemeMode::Dark,
                background: hex("#002B36"),
                foreground: hex("#839496"),
                card: hex("#073642"),
                card_foreground: hex("#93A1A1"),
                primary: hex("#268BD2"),
                primary_foreground: hex("#FDF6E3"),
                secondary: hex("#073642"),
                secondary_foreground: hex("#93A1A1"),
                muted: hex("#073642"),
                muted_foreground: hex("#586E75"),
                accent: hex("#094352"),
                accent_foreground: hex("#FDF6E3"),
                border: hex("#0B4B5A"),
                input: hex("#586E75"),
                ring: hex("#268BD2"),
                success: hex("#859900"),
                warning: hex("#B58900"),
                destructive: hex("#DC322F"),
            },
            ThemeId::SolarizedLight => Self {
                id,
                mode: ThemeMode::Light,
                background: hex("#FDF6E3"),
                foreground: hex("#657B83"),
                card: hex("#EEE8D5"),
                card_foreground: hex("#586E75"),
                primary: hex("#268BD2"),
                primary_foreground: hex("#FDF6E3"),
                secondary: hex("#EEE8D5"),
                secondary_foreground: hex("#586E75"),
                muted: hex("#EEE8D5"),
                muted_foreground: hex("#93A1A1"),
                accent: hex("#E4DEC9"),
                accent_foreground: hex("#073642"),
                border: hex("#DDD6C1"),
                input: hex("#93A1A1"),
                ring: hex("#268BD2"),
                success: hex("#859900"),
                warning: hex("#B58900"),
                destructive: hex("#DC322F"),
            },
            ThemeId::AyuDark | ThemeId::EverforestDark | ThemeId::FlexokiDark | ThemeId::MacOsClassicDark => Self {
                id,
                mode: ThemeMode::Dark,
                background: hex("#0D1017"),
                foreground: hex("#BFBDB6"),
                card: hex("#131721"),
                card_foreground: hex("#BFBDB6"),
                primary: hex("#E6B450"),
                primary_foreground: hex("#0D1017"),
                secondary: hex("#1B202C"),
                secondary_foreground: hex("#BFBDB6"),
                muted: hex("#161B24"),
                muted_foreground: hex("#6C7380"),
                accent: hex("#232A38"),
                accent_foreground: hex("#BFBDB6"),
                border: hex("#222733"),
                input: hex("#323949"),
                ring: hex("#E6B450"),
                success: hex("#7FD962"),
                warning: hex("#FFB454"),
                destructive: hex("#F07178"),
            },
            ThemeId::AyuLight | ThemeId::EverforestLight | ThemeId::FlexokiLight | ThemeId::MacOsClassicLight => Self {
                id,
                mode: ThemeMode::Light,
                background: hex("#FAFAFA"),
                foreground: hex("#575F66"),
                card: hex("#FFFFFF"),
                card_foreground: hex("#575F66"),
                primary: hex("#FF9940"),
                primary_foreground: hex("#FFFFFF"),
                secondary: hex("#F0F0F0"),
                secondary_foreground: hex("#575F66"),
                muted: hex("#F3F4F5"),
                muted_foreground: hex("#8A9199"),
                accent: hex("#E7E9EB"),
                accent_foreground: hex("#24292F"),
                border: hex("#E1E4E8"),
                input: hex("#D0D5DC"),
                ring: hex("#FF9940"),
                success: hex("#86B300"),
                warning: hex("#F2AE49"),
                destructive: hex("#F07171"),
            },
        }
    }

    /// Synchronizes `egui::Visuals` with these theme tokens so built-in `egui`
    /// controls match `egui-kit` components seamlessly.
    pub fn apply_to_egui(&self, ctx: &Context) {
        let mut visuals = match self.mode {
            ThemeMode::Dark => Visuals::dark(),
            ThemeMode::Light => Visuals::light(),
        };

        visuals.panel_fill = self.background;
        visuals.window_fill = self.card;
        visuals.extreme_bg_color = self.muted;
        visuals.faint_bg_color = self.muted;
        visuals.selection.bg_fill = self.primary;
        visuals.selection.stroke = Stroke::new(1.0, self.primary_foreground);

        visuals.widgets.noninteractive.bg_fill = self.card;
        visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, self.foreground);
        visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, self.border);
        visuals.widgets.noninteractive.rounding = Rounding::same(6.0);

        visuals.widgets.inactive.bg_fill = self.secondary;
        visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, self.secondary_foreground);
        visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, self.border);
        visuals.widgets.inactive.rounding = Rounding::same(6.0);

        visuals.widgets.hovered.bg_fill = self.accent;
        visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, self.accent_foreground);
        visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, self.ring);
        visuals.widgets.hovered.rounding = Rounding::same(6.0);

        visuals.widgets.active.bg_fill = self.primary;
        visuals.widgets.active.fg_stroke = Stroke::new(1.0, self.primary_foreground);
        visuals.widgets.active.bg_stroke = Stroke::new(1.0, self.ring);
        visuals.widgets.active.rounding = Rounding::same(6.0);

        visuals.window_stroke = Stroke::new(1.0, self.border);
        visuals.window_rounding = Rounding::same(8.0);
        ctx.set_visuals(visuals);
    }
}

const THEME_STORAGE_KEY: &str = "egui_kit_active_theme";

/// Applies a [`ThemeId`] preset to the `egui::Context` and stores it in temp memory.
pub fn set_theme(ctx: &Context, id: ThemeId) {
    let tokens = ThemeTokens::from_preset(id);
    tokens.apply_to_egui(ctx);
    ctx.data_mut(|d| d.insert_temp(egui::Id::new(THEME_STORAGE_KEY), tokens));
}

/// Retrieves the currently active [`ThemeTokens`] from `egui::Context`.
pub fn active_theme(ctx: &Context) -> ThemeTokens {
    ctx.data(|d| d.get_temp::<ThemeTokens>(egui::Id::new(THEME_STORAGE_KEY)))
        .unwrap_or_else(|| ThemeTokens::from_preset(ThemeId::AuroraLight))
}

/// Parses a `#RRGGBB` hex string into `egui::Color32` at runtime.
pub fn hex(code: &str) -> Color32 {
    let trimmed = code.trim_start_matches('#');
    if trimmed.len() == 6 {
        let r = u8::from_str_radix(&trimmed[0..2], 16).unwrap_or(0);
        let g = u8::from_str_radix(&trimmed[2..4], 16).unwrap_or(0);
        let b = u8::from_str_radix(&trimmed[4..6], 16).unwrap_or(0);
        Color32::from_rgb(r, g, b)
    } else {
        Color32::GRAY
    }
}
