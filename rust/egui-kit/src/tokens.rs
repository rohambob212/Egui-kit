//! Shared sizing, variant, and geometry tokens for `egui-kit`.

use egui::{Color32, Rounding};

/// Standard component scale matching `gpui-kit` sizing steps.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Size {
    Xs,
    Sm,
    #[default]
    Md,
    Lg,
}

impl Size {
    /// Standard control height in logical pixels.
    pub const fn height(self) -> f32 {
        match self {
            Size::Xs => 24.0,
            Size::Sm => 28.0,
            Size::Md => 34.0,
            Size::Lg => 40.0,
        }
    }

    /// Horizontal inner padding in logical pixels.
    pub const fn padding_x(self) -> f32 {
        match self {
            Size::Xs => 8.0,
            Size::Sm => 10.0,
            Size::Md => 14.0,
            Size::Lg => 18.0,
        }
    }

    /// Vertical inner padding in logical pixels.
    pub const fn padding_y(self) -> f32 {
        match self {
            Size::Xs => 3.0,
            Size::Sm => 5.0,
            Size::Md => 7.0,
            Size::Lg => 9.0,
        }
    }

    /// Base font size in points.
    pub const fn font_size(self) -> f32 {
        match self {
            Size::Xs => 11.0,
            Size::Sm => 12.0,
            Size::Md => 13.5,
            Size::Lg => 15.0,
        }
    }

    /// Icon edge size in logical pixels.
    pub const fn icon_size(self) -> f32 {
        match self {
            Size::Xs => 12.0,
            Size::Sm => 14.0,
            Size::Md => 16.0,
            Size::Lg => 18.0,
        }
    }

    /// Corner rounding scaled proportionally to component size.
    pub fn rounding(self) -> Rounding {
        match self {
            Size::Xs => Rounding::same(4.0),
            Size::Sm => Rounding::same(5.0),
            Size::Md => Rounding::same(6.0),
            Size::Lg => Rounding::same(8.0),
        }
    }
}

/// Shared semantic surface variant across interactive primitives.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Variant {
    #[default]
    Primary,
    Secondary,
    Outline,
    Ghost,
    Destructive,
    Success,
    Warning,
}

/// Shared alignment option for inline groups and labels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Alignment {
    #[default]
    Start,
    Center,
    End,
}

/// Layout orientation for compound controls.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Orientation {
    #[default]
    Horizontal,
    Vertical,
}

/// Cubic ease-out curve for crisp immediate-mode motion.
#[inline]
pub fn ease_out_cubic(t: f32) -> f32 {
    let inv = 1.0 - t.clamp(0.0, 1.0);
    1.0 - inv * inv * inv
}

/// Smoothly interpolates two RGBA colors component-wise.
#[inline]
pub fn lerp_color(a: Color32, b: Color32, t: f32) -> Color32 {
    let t = t.clamp(0.0, 1.0);
    Color32::from_rgba_unmultiplied(
        egui::lerp(a.r() as f32..=b.r() as f32, t) as u8,
        egui::lerp(a.g() as f32..=b.g() as f32, t) as u8,
        egui::lerp(a.b() as f32..=b.b() as f32, t) as u8,
        egui::lerp(a.a() as f32..=b.a() as f32, t) as u8,
    )
}
