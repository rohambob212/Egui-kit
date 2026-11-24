//! Basic widgets. Thin, themed, stateless.
use crate::{Theme, Variant};
use egui::{Button, Color32, FontId, Response, RichText, Ui};

/// Small pill label in the primary color.
pub fn badge(ui: &mut Ui, text: String) -> Response {
    let t = Theme::get(ui.ctx());
    let galley = ui.painter().layout_no_wrap(text, FontId::proportional(11.0), t.on_primary);
    let pad = egui::vec2(6.0, 2.0);
    let (rect, resp) = ui.allocate_exact_size(galley.size() + pad * 2.0, egui::Sense::hover());
    ui.painter().rect_filled(rect, t.radius, t.primary);
    ui.painter().galley(rect.min + pad, galley, t.on_primary);
    resp
}

fn variant_fill(t: &Theme, v: Variant) -> Color32 {
    match v {
        Variant::Default => t.surface,
        Variant::Primary => t.primary,
        Variant::Danger => t.danger,
        Variant::Ghost => Color32::TRANSPARENT,
        Variant::Outline => Color32::TRANSPARENT,
    }
}

/// Themed button with one of the shared variants.
pub fn button(ui: &mut Ui, text: &str, v: Variant) -> Response {
    let t = Theme::get(ui.ctx());
    let stroke = match v {
        Variant::Outline => egui::Stroke::new(1.0, t.border),
        _ => egui::Stroke::NONE,
    };
    let fg = match v {
        Variant::Primary | Variant::Danger => t.on_primary,
        _ => t.text,
    };
    let mut b = Button::new(RichText::new(text).color(fg))
        .fill(variant_fill(&t, v))
        .stroke(stroke);
    if matches!(v, Variant::Primary | Variant::Danger) {
        b = b.min_size(egui::vec2(0.0, 30.0));
    }
    ui.add(b)
}
