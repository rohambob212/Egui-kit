//! Semantic theme tokens + one `apply` call. Light and dark, no per-widget styling.
use egui::{Color32, Context, CornerRadius, Stroke, Visuals};

const C: fn(u8, u8, u8) -> Color32 = Color32::from_rgb;

#[derive(Clone, Copy, Debug)]
pub struct Theme {
    pub is_dark: bool,
    pub bg: Color32,
    pub panel: Color32,
    pub surface: Color32,
    pub surface_hover: Color32,
    pub border: Color32,
    pub text: Color32,
    pub text_muted: Color32,
    pub primary: Color32,
    pub primary_hover: Color32,
    pub on_primary: Color32,
    pub danger: Color32,
    pub warning: Color32,
    pub success: Color32,
    pub radius: f32,
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}

impl Theme {
    pub fn light() -> Self {
        Self {
            is_dark: false,
            bg: C(255, 255, 255),
            panel: C(250, 250, 251),
            surface: C(244, 244, 246),
            surface_hover: C(236, 236, 239),
            border: C(224, 224, 229),
            text: C(24, 24, 27),
            text_muted: C(113, 113, 122),
            primary: C(37, 99, 235),
            primary_hover: C(29, 78, 216),
            on_primary: C(255, 255, 255),
            danger: C(220, 38, 38),
            warning: C(217, 119, 6),
            success: C(22, 163, 74),
            radius: 6.0,
        }
    }

    pub fn dark() -> Self {
        Self {
            is_dark: true,
            bg: C(18, 18, 21),
            panel: C(24, 24, 28),
            surface: C(32, 32, 37),
            surface_hover: C(44, 44, 50),
            border: C(56, 56, 64),
            text: C(244, 244, 245),
            text_muted: C(161, 161, 170),
            primary: C(59, 130, 246),
            primary_hover: C(96, 165, 250),
            on_primary: C(255, 255, 255),
            danger: C(248, 113, 113),
            warning: C(251, 191, 36),
            success: C(74, 222, 128),
            radius: 6.0,
        }
    }

    pub fn visuals(&self) -> Visuals {
        let mut v = if self.is_dark { Visuals::dark() } else { Visuals::light() };
        let r = CornerRadius::same(self.radius as u8);
        v.override_text_color = Some(self.text);
        v.panel_fill = self.panel;
        v.window_fill = self.panel;
        v.extreme_bg_color = self.bg;
        v.faint_bg_color = self.surface;
        v.window_stroke = Stroke::new(1.0, self.border);
        v.selection.bg_fill = self.primary.gamma_multiply(0.35);
        v.hyperlink_color = self.primary;
        v.warn_fg_color = self.warning;
        v.error_fg_color = self.danger;

        let w = &mut v.widgets;
        for (set, bg, fg, stroke) in [
            (&mut w.noninteractive, self.panel, self.text_muted, self.border),
            (&mut w.inactive, self.surface, self.text, self.border),
            (&mut w.hovered, self.surface_hover, self.text, self.border),
            (&mut w.active, self.surface_hover, self.text, self.primary),
            (&mut w.open, self.surface, self.text, self.border),
        ] {
            set.bg_fill = bg;
            set.weak_bg_fill = bg;
            set.bg_stroke = Stroke::new(1.0, stroke);
            set.fg_stroke = Stroke::new(1.0, fg);
            set.corner_radius = r;
        }
        v
    }

    /// Apply to a context: visuals, spacing, text sizes. Call once per frame or on change.
    pub fn apply(&self, ctx: &Context) {
        let et = if self.is_dark { egui::Theme::Dark } else { egui::Theme::Light };
        ctx.set_theme(et);

        let mut style = (*ctx.style_of(et)).clone();
        style.spacing.item_spacing = egui::vec2(8.0, 8.0);
        style.spacing.button_padding = egui::vec2(10.0, 5.0);
        style.text_styles = [
            (egui::TextStyle::Heading, egui::FontId::proportional(20.0)),
            (egui::TextStyle::Body, egui::FontId::proportional(14.0)),
            (egui::TextStyle::Button, egui::FontId::proportional(14.0)),
            (egui::TextStyle::Small, egui::FontId::proportional(12.0)),
            (egui::TextStyle::Monospace, egui::FontId::monospace(13.0)),
        ]
        .into();

        ctx.set_style_of(et, style);
        ctx.set_visuals_of(et, self.visuals());
        ctx.data_mut(|d| d.insert_temp(Id::theme(), *self));
    }

    /// The theme last applied to this context.
    pub fn get(ctx: &Context) -> Theme {
        ctx.data(|d| d.get_temp::<Theme>(Id::theme())).unwrap_or_default()
    }
}

struct Id;

impl Id {
    fn theme() -> egui::Id {
        egui::Id::unique("egui_kit.theme")
    }
}
