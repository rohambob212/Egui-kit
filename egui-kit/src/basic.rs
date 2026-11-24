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

pub fn tag(ui: &mut Ui, text: String) -> Response {
    let theme = Theme::get(ui.ctx());
    let padding = egui::vec2(8.0, 3.0);
    let galley = ui
        .painter()
        .layout_no_wrap(text, FontId::proportional(12.0), theme.text);
    let (rect, response) =
        ui.allocate_exact_size(galley.size() + padding * 2.0, egui::Sense::hover());
    if ui.is_rect_visible(rect) {
        let radius = egui::CornerRadius::same(255);
        let painter = ui.painter();
        painter.rect_filled(rect, radius, theme.surface);
        painter.rect_stroke(
            rect,
            radius,
            egui::Stroke::new(1.0, theme.border),
            egui::StrokeKind::Inside,
        );
        painter.galley(rect.min + padding, galley, theme.text);
    }
    response
}

pub fn kbd(ui: &mut Ui, keys: &str) -> Response {
    let theme = Theme::get(ui.ctx());
    let padding = egui::vec2(5.0, 2.0);
    let galley = ui
        .painter()
        .layout_no_wrap(keys.to_owned(), FontId::monospace(11.0), theme.text);
    let (rect, response) =
        ui.allocate_exact_size(galley.size() + padding * 2.0, egui::Sense::hover());
    if ui.is_rect_visible(rect) {
        let radius = egui::CornerRadius::same(4);
        let painter = ui.painter();
        painter.rect_filled(rect, radius, theme.surface);
        painter.rect_stroke(
            rect,
            radius,
            egui::Stroke::new(1.0, theme.border),
            egui::StrokeKind::Inside,
        );
        painter.galley(rect.min + padding, galley, theme.text);
    }
    response
}

pub fn divider(ui: &mut Ui) {
    let theme = Theme::get(ui.ctx());
    let (rect, _response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), 1.0),
        egui::Sense::hover(),
    );
    if ui.is_rect_visible(rect) {
        ui.painter().line_segment(
            [rect.left_center(), rect.right_center()],
            egui::Stroke::new(1.0, theme.border),
        );
    }
}

pub fn progress(ui: &mut Ui, value: f32) -> Response {
    let theme = Theme::get(ui.ctx());
    let fraction = if value.is_nan() {
        0.0
    } else {
        value.clamp(0.0, 1.0)
    };
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), 6.0),
        egui::Sense::hover(),
    );
    if ui.is_rect_visible(rect) {
        let radius = egui::CornerRadius::same(3);
        let painter = ui.painter();
        painter.rect_filled(rect, radius, theme.surface);
        if fraction > 0.0 {
            let fill = egui::Rect::from_min_size(
                rect.min,
                egui::vec2(rect.width() * fraction, rect.height()),
            );
            painter.rect_filled(fill, radius, theme.primary);
        }
    }
    response
}

pub fn avatar(ui: &mut Ui, name: &str, size: f32) -> Response {
    const COLORS: [Color32; 6] = [
        Color32::from_rgb(99, 102, 241),
        Color32::from_rgb(14, 165, 233),
        Color32::from_rgb(16, 185, 129),
        Color32::from_rgb(245, 158, 11),
        Color32::from_rgb(239, 68, 68),
        Color32::from_rgb(168, 85, 247),
    ];
    let theme = Theme::get(ui.ctx());
    let hash = name
        .bytes()
        .fold(5381u32, |h, b| h.wrapping_mul(33) ^ b as u32);
    let color = COLORS[hash as usize % COLORS.len()];
    let initials: String = name
        .split_whitespace()
        .take(2)
        .filter_map(|word| word.chars().next())
        .flat_map(char::to_uppercase)
        .collect();
    let (rect, response) = ui.allocate_exact_size(egui::vec2(size, size), egui::Sense::hover());
    if ui.is_rect_visible(rect) {
        let painter = ui.painter();
        painter.circle_filled(rect.center(), size * 0.5, color);
        painter.text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            initials,
            FontId::proportional(size * 0.4),
            theme.on_primary,
        );
    }
    response.on_hover_text(name)
}

pub fn alert(ui: &mut Ui, text: &str, variant: Variant) -> Response {
    let theme = Theme::get(ui.ctx());
    let (accent, glyph) = match variant {
        Variant::Danger => (theme.danger, "!"),
        Variant::Primary => (theme.primary, "i"),
        _ => (theme.text_muted, "i"),
    };
    let pad = egui::vec2(10.0, 8.0);
    let (bar, glyph_w) = (3.0, 16.0);
    let font = FontId::proportional(14.0);
    let width = ui.available_width();
    let wrap = (width - bar - pad.x * 2.0 - glyph_w).max(0.0);
    let galley = ui
        .painter()
        .layout(text.to_owned(), font.clone(), theme.text, wrap);
    let size = egui::vec2(width, galley.size().y + pad.y * 2.0);
    let (rect, response) = ui.allocate_exact_size(size, egui::Sense::hover());
    if ui.is_rect_visible(rect) {
        let radius: egui::CornerRadius = theme.radius.into();
        let body_radius = egui::CornerRadius {
            nw: 0,
            sw: 0,
            ..radius
        };
        let body = rect.with_min_x(rect.left() + bar);
        let painter = ui.painter();
        painter.rect_filled(rect, radius, accent);
        painter.rect_filled(body, body_radius, theme.surface);
        painter.rect_filled(body, body_radius, accent.gamma_multiply(0.15));
        let origin = body.min + pad;
        painter.text(origin, egui::Align2::LEFT_TOP, glyph, font, accent);
        painter.galley(origin + egui::vec2(glyph_w, 0.0), galley, theme.text);
    }
    response
}

pub fn skeleton(ui: &mut Ui, width: f32, height: f32) -> Response {
    let theme = Theme::get(ui.ctx());
    let (rect, response) =
        ui.allocate_exact_size(egui::vec2(width, height), egui::Sense::hover());
    if ui.is_rect_visible(rect) {
        ui.ctx().request_repaint();
        let wave = (ui.input(|i| i.time) * 3.0).sin() as f32;
        let alpha = 0.7 + 0.3 * wave;
        ui.painter()
            .rect_filled(rect, theme.radius, theme.surface.gamma_multiply(alpha));
    }
    response
}
