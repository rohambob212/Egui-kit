//! Basic widgets. Thin, themed, stateless.
use crate::{Theme, Variant};
use egui::{Align2, Button, Color32, CursorIcon, FontId, Response, RichText, Sense, Ui, Vec2};
use egui::{pos2, vec2, CornerRadius, Rect, Stroke, UiBuilder};

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

pub fn rating(ui: &mut Ui, id: &str, value: &mut u8, max: u8) -> Response {
    let theme = Theme::get(ui.ctx());
    ui.push_id(id, |ui| {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            let mut last: Option<Response> = None;
            for i in 1..=max {
                let (rect, mut resp) = ui.allocate_exact_size(vec2(20.0, 20.0), Sense::click());
                if resp.clicked() {
                    *value = i;
                    resp.mark_changed();
                }
                if ui.is_rect_visible(rect) {
                    let color = if i <= *value {
                        theme.primary
                    } else {
                        theme.text_muted
                    };
                    star(ui.painter(), rect.center(), 8.0, color);
                }
                last = Some(resp.on_hover_cursor(CursorIcon::PointingHand));
            }
            match last {
                Some(resp) => resp,
                None => ui.allocate_response(Vec2::ZERO, Sense::hover()),
            }
        })
        .inner
    })
    .inner
}

/// `step` is the zero-based index of the current step; labels are shown 1-based.
pub fn stepper(ui: &mut Ui, step: usize, total: usize) -> Response {
    let theme = Theme::get(ui.ctx());
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 8.0;
        let mut last: Option<Response> = None;
        for i in 0..total {
            let (rect, resp) = ui.allocate_exact_size(vec2(22.0, 22.0), Sense::hover());
            if ui.is_rect_visible(rect) {
                let center = rect.center();
                let painter = ui.painter();
                let text_color = if i < step {
                    painter.circle_filled(center, 11.0, theme.primary);
                    theme.on_primary
                } else if i == step {
                    painter.circle_stroke(center, 10.25, Stroke::new(1.5, theme.primary));
                    theme.text
                } else {
                    painter.circle_filled(center, 11.0, theme.surface);
                    painter.circle_stroke(center, 10.5, Stroke::new(1.0, theme.border));
                    theme.text
                };
                painter.text(
                    center,
                    Align2::CENTER_CENTER,
                    (i + 1).to_string(),
                    FontId::proportional(11.0),
                    text_color,
                );
            }
            last = Some(resp);
        }
        match last {
            Some(resp) => resp,
            None => ui.allocate_response(Vec2::ZERO, Sense::hover()),
        }
    })
    .inner
}

/// `page` is zero-based; buttons are labelled 1-based.
pub fn pagination(ui: &mut Ui, page: &mut usize, total_pages: usize) -> Response {
    let theme = Theme::get(ui.ctx());
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 4.0;

        let current = (*page).min(total_pages.saturating_sub(1));
        let mut target = current;

        let prev = ui.add_enabled(
            current > 0,
            egui::Button::new(RichText::new("Prev").color(theme.text))
                .fill(theme.surface)
                .stroke(Stroke::new(1.0, theme.border))
                .corner_radius(theme.radius),
        );
        if prev.clicked() {
            target = current.saturating_sub(1);
        }

        let start = current.saturating_sub(2);
        let end = (start + 5).min(total_pages);
        let start = end.saturating_sub(5);

        for i in start..end {
            let is_current = i == current;
            let (text_color, fill, stroke) = if is_current {
                (theme.on_primary, theme.primary, Stroke::new(1.0, theme.primary))
            } else {
                (theme.text, theme.surface, Stroke::new(1.0, theme.border))
            };
            let resp = ui.add(
                egui::Button::new(RichText::new((i + 1).to_string()).color(text_color))
                    .fill(fill)
                    .stroke(stroke)
                    .corner_radius(theme.radius)
                    .min_size(vec2(28.0, 0.0)),
            );
            if resp.clicked() {
                target = i;
            }
        }

        let next = ui.add_enabled(
            current + 1 < total_pages,
            egui::Button::new(RichText::new("Next").color(theme.text))
                .fill(theme.surface)
                .stroke(Stroke::new(1.0, theme.border))
                .corner_radius(theme.radius),
        );
        if next.clicked() {
            target = current + 1;
        }

        if target != *page && target < total_pages {
            *page = target;
        }

        next
    })
    .inner
}

pub fn tabs(ui: &mut Ui, labels: &[&str], selected: &mut usize) -> Response {
    let theme = Theme::get(ui.ctx());
    let padding = vec2(10.0, 6.0);

    let inner = ui.horizontal(|ui| {
        let mut last: Option<Response> = None;
        let mut items = Vec::with_capacity(labels.len());

        for (i, label) in labels.iter().enumerate() {
            let galley = ui.painter().layout_no_wrap(
                (*label).to_owned(),
                FontId::proportional(14.0),
                Color32::PLACEHOLDER,
            );
            let (rect, mut response) =
                ui.allocate_exact_size(galley.size() + 2.0 * padding, Sense::click());

            if response.clicked() && *selected != i {
                *selected = i;
                response.mark_changed();
            }

            items.push((rect, galley));
            last = Some(response);
        }

        for (i, (rect, galley)) in items.into_iter().enumerate() {
            if !ui.is_rect_visible(rect) {
                continue;
            }

            let active = *selected == i;
            let color = if active { theme.text } else { theme.text_muted };
            ui.painter().galley(rect.min + padding, galley, color);

            if active {
                let underline = Rect::from_min_max(
                    pos2(rect.left(), rect.bottom() - 2.0),
                    rect.right_bottom(),
                );
                ui.painter()
                    .rect_filled(underline, CornerRadius::ZERO, theme.primary);
            }
        }

        last
    });

    inner.inner.unwrap_or(inner.response)
}

pub fn accordion(
    ui: &mut Ui,
    title: &str,
    open: &mut bool,
    body: impl FnOnce(&mut Ui),
) -> Response {
    let theme = Theme::get(ui.ctx());
    let pad_x = 10.0;

    let (rect, mut response) =
        ui.allocate_exact_size(vec2(ui.available_width(), 30.0), Sense::click());

    if response.clicked() {
        *open = !*open;
        response.mark_changed();
    }

    if ui.is_rect_visible(rect) {
        let painter = ui.painter();

        painter.rect_filled(rect, CornerRadius::same(theme.radius as u8), theme.surface);

        let title_galley =
            painter.layout_no_wrap(title.to_owned(), FontId::proportional(14.0), theme.text);
        let title_pos = pos2(
            rect.left() + pad_x,
            rect.center().y - title_galley.size().y / 2.0,
        );
        painter.galley(title_pos, title_galley, theme.text);

        let glyph = if *open { "-" } else { "+" };
        let glyph_galley = painter.layout_no_wrap(
            glyph.to_owned(),
            FontId::proportional(14.0),
            theme.text_muted,
        );
        let glyph_pos = pos2(
            rect.right() - pad_x - glyph_galley.size().x,
            rect.center().y - glyph_galley.size().y / 2.0,
        );
        painter.galley(glyph_pos, glyph_galley, theme.text_muted);
    }

    if *open {
        let mut body_rect = ui.available_rect_before_wrap();
        body_rect.min.x += 12.0;
        ui.scope_builder(UiBuilder::new().max_rect(body_rect), |ui| body(ui));
    }

    response
}

/// Five triangles make a star; no icon font needed.
fn star(painter: &egui::Painter, center: egui::Pos2, r: f32, color: Color32) {
    use std::f32::consts::PI;
    let inner = r * 0.42;
    for k in 0..5 {
        let a = -PI / 2.0 + k as f32 * 2.0 * PI / 5.0;
        let p = |ang: f32, rad: f32| center + rad * egui::vec2(ang.cos(), ang.sin());
        painter.add(egui::Shape::convex_polygon(
            vec![p(a, r), p(a - PI / 5.0, inner), p(a + PI / 5.0, inner)],
            color,
            egui::Stroke::NONE,
        ));
    }
}
