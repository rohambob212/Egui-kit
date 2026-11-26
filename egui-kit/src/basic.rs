//! Basic widgets. Thin, themed, stateless.
use crate::{Theme, Variant};
use egui::{Align2, Button, Color32, CursorIcon, FontId, Response, RichText, Sense, Ui, Vec2};
use egui::{pos2, vec2, CornerRadius, Rect, Stroke, StrokeKind, UiBuilder};

/// Small pill label in the primary color.
pub fn badge(ui: &mut Ui, text: String) -> Response {
    let t = Theme::get(ui.ctx());
    let fg = readable_on(t.primary, &t);
    let galley = ui.painter().layout_no_wrap(text, FontId::proportional(11.0), fg);
    let pad = egui::vec2(6.0, 2.0);
    let (rect, resp) = ui.allocate_exact_size(galley.size() + pad * 2.0, egui::Sense::hover());
    ui.painter().rect_filled(rect, t.radius, t.primary);
    ui.painter().galley(rect.min + pad, galley, fg);
    resp
}

/// Text colour that stays readable on `fill`: the theme's text, or the page
/// background colour when the fill is light (WCAG-ish, whichever wins).
pub fn readable_on(fill: Color32, t: &Theme) -> Color32 {
    let ratio_text = contrast_ratio(t.text, fill);
    let ratio_bg = contrast_ratio(t.bg, fill);
    if ratio_bg > ratio_text {
        t.bg
    } else {
        t.text
    }
}

/// WCAG contrast ratio, 1.0..21.0.
pub fn contrast_ratio(a: Color32, b: Color32) -> f32 {
    let lum = |c: Color32| {
        let f = |v: u8| {
            let s = v as f32 / 255.0;
            if s <= 0.03928 {
                s / 12.92
            } else {
                ((s + 0.055) / 1.055).powf(2.4)
            }
        };
        0.2126 * f(c.r()) + 0.7152 * f(c.g()) + 0.0722 * f(c.b())
    };
    let (x, y) = (lum(a), lum(b));
    let (hi, lo) = if x > y { (x, y) } else { (y, x) };
    (hi + 0.05) / (lo + 0.05)
}

fn variant_fill(t: &Theme, v: Variant) -> Color32 {
    match v {
        Variant::Default => t.surface,
        Variant::Primary => t.primary,
        Variant::Danger => t.danger,
        Variant::Warning => t.warning,
        Variant::Success => t.success,
        Variant::Ghost => Color32::TRANSPARENT,
        Variant::Outline => Color32::TRANSPARENT,
    }
}

/// Themed button with one of the shared variants.
pub fn button(ui: &mut Ui, text: &str, v: Variant) -> Response {
    let t = Theme::get(ui.ctx());
    let fill = variant_fill(&t, v);
    let fg = if matches!(v, Variant::Ghost | Variant::Outline) {
        t.text
    } else {
        readable_on(fill, &t)
    };
    if v == Variant::Ghost {
        // transparent at rest, but keeps egui's hover/press feedback
        return ui
            .scope(|ui| {
                ui.visuals_mut().widgets.inactive.weak_bg_fill = Color32::TRANSPARENT;
                ui.visuals_mut().widgets.inactive.bg_stroke = egui::Stroke::NONE;
                ui.add(Button::new(RichText::new(text).color(fg)).min_size(egui::vec2(0.0, 30.0)))
            })
            .inner;
    }
    let stroke = match v {
        Variant::Outline => egui::Stroke::new(1.0, t.border),
        _ => egui::Stroke::NONE,
    };
    ui.add(
        Button::new(RichText::new(text).color(fg))
            .fill(fill)
            .stroke(stroke)
            .min_size(egui::vec2(0.0, 30.0)),
    )
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
        let fraction = ui
            .ctx()
            .animate_value_with_time(response.id.with("v"), fraction, 0.25);
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
        painter.circle_stroke(
            rect.center(),
            size * 0.5 - 0.5,
            egui::Stroke::new(1.0, theme.bg.gamma_multiply(0.35)),
        );
        painter.text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            initials,
            FontId::proportional(size * 0.4),
            readable_on(color, &theme),
        );
    }
    response.on_hover_text(name)
}

pub fn alert(ui: &mut Ui, text: &str, variant: Variant) -> Response {
    let theme = Theme::get(ui.ctx());
    let (accent, glyph) = match variant {
        Variant::Danger => (theme.danger, "!"),
        Variant::Warning => (theme.warning, "!"),
        Variant::Success => (theme.success, "i"),
        Variant::Primary => (theme.primary, "i"),
        _ => (theme.text_muted, "i"),
    };
    let pad = egui::vec2(12.0, 10.0);
    let badge = 18.0;
    let font = FontId::proportional(13.0);
    let width = ui.available_width();
    let text_left = pad.x + badge + 10.0;
    let galley = ui.painter().layout(
        text.to_owned(),
        font,
        theme.text,
        (width - text_left - pad.x).max(1.0),
    );
    let size = egui::vec2(width, galley.size().y.max(badge) + pad.y * 2.0);
    let (rect, response) = ui.allocate_exact_size(size, Sense::hover());
    if ui.is_rect_visible(rect) {
        let radius = CornerRadius::same(theme.radius as u8);
        let painter = ui.painter();
        painter.rect_filled(rect, radius, accent.gamma_multiply(0.14));
        painter.rect_stroke(rect, radius, Stroke::new(1.0, accent.gamma_multiply(0.45)), StrokeKind::Inside);
        let center = egui::pos2(rect.left() + pad.x + badge * 0.5, rect.center().y);
        painter.circle_filled(center, badge * 0.5, accent);
        painter.text(center, Align2::CENTER_CENTER, glyph, FontId::proportional(12.0), theme.on_primary);
        painter.galley(
            egui::pos2(rect.left() + text_left, rect.center().y - galley.size().y / 2.0),
            galley,
            theme.text,
        );
    }
    response
}

pub fn skeleton(ui: &mut Ui, width: f32, height: f32) -> Response {
    let theme = Theme::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(egui::vec2(width, height), Sense::hover());
    if ui.is_rect_visible(rect) {
        ui.ctx().request_repaint();
        let radius = CornerRadius::same((height * 0.35).clamp(2.0, 8.0) as u8);
        let painter = ui.painter();
        painter.rect_filled(rect, radius, theme.surface_hover);
        let phase = (ui.input(|i| i.time) * 0.6) as f32 % 1.0;
        let band_w = (rect.width() * 0.35).max(24.0);
        let x = rect.left() - band_w + phase * (rect.width() + band_w);
        let band = egui::Rect::from_min_size(
            egui::pos2(x, rect.top()),
            egui::vec2(band_w, rect.height()),
        )
        .intersect(rect);
        if band.width() > 0.0 {
            painter.rect_filled(band, radius, theme.border.gamma_multiply(0.7));
        }
    }
    response
}

/// Labeled bar with the value on the right: `ui.progress_labeled(0.4, "upload")`.
pub fn progress_labeled(ui: &mut Ui, value: f32, label: &str) -> Response {
    let theme = Theme::get(ui.ctx());
    let mut resp = ui
        .horizontal(|ui| {
            ui.label(RichText::new(label).small().color(theme.text_muted));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(
                    RichText::new(format!("{:.0}%", value.clamp(0.0, 1.0) * 100.0))
                        .small()
                        .color(theme.text_muted),
                );
            });
        })
        .response;
    resp |= progress(ui, value);
    resp
}

pub fn rating(ui: &mut Ui, id: &str, value: &mut u8, max: u8) -> Response {
    let theme = Theme::get(ui.ctx());
    ui.push_id(id, |ui| {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            let mut last: Option<Response> = None;
            let mut hovered: Option<u8> = None;
            for i in 1..=max {
                let (rect, mut resp) = ui.allocate_exact_size(vec2(20.0, 20.0), Sense::click());
                if resp.clicked() {
                    *value = i;
                    resp.mark_changed();
                }
                if resp.hovered() {
                    hovered = Some(i);
                    resp = resp.on_hover_cursor(egui::CursorIcon::PointingHand);
                }
                let shown = hovered.unwrap_or(*value);
                if ui.is_rect_visible(rect) {
                    let p = ui.painter();
                    let growth = ui.ctx().animate_bool(resp.id, resp.hovered());
                    let r = 4.6 + 0.6 * growth;
                    let center = rect.center();
                    let filled = i <= shown;
                    let color = if filled { theme.warning } else { theme.border };
                    // five-point star: outline when empty, solid when set
                    let pts: Vec<egui::Pos2> = (0..10)
                        .map(|k| {
                            let a = -std::f32::consts::FRAC_PI_2
                                + k as f32 * std::f32::consts::PI / 5.0;
                            let rad = if k % 2 == 0 { r } else { r * 0.45 };
                            center + rad * egui::vec2(a.cos(), a.sin())
                        })
                        .collect();
                    if filled {
                        p.add(egui::Shape::convex_polygon(pts, color, Stroke::NONE));
                    } else {
                        p.add(egui::Shape::closed_line(pts, Stroke::new(1.2, color)));
                    }
                }
                last = Some(resp);
            }
            last
        })
        .inner
        .unwrap_or_else(|| ui.allocate_response(egui::Vec2::ZERO, Sense::hover()))
    })
    .inner
}

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
                // done fills, active gets a primary ring, upcoming stays neutral
                let done = ui.ctx().animate_bool(resp.id.with("done"), i < step);
                let active = ui.ctx().animate_bool(resp.id.with("active"), i == step);
                painter.circle_filled(center, 11.0, mix(theme.surface, theme.primary, done));
                let ring = mix(theme.border, theme.primary, done.max(active));
                painter.circle_stroke(
                    center,
                    10.5,
                    Stroke::new(1.0 + active, ring),
                );
                let text_color = mix(theme.text, theme.on_primary, done);
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
            let sel = ui
                .ctx()
                .animate_bool(egui::Id::unique("kit-page").with(i), is_current);
            let (text_color, fill) = (
                mix(theme.text, theme.on_primary, sel),
                mix(theme.surface, theme.primary, sel),
            );
            let stroke = Stroke::new(1.0, mix(theme.border, theme.primary, sel));
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
    let anim_id = egui::Id::unique("kit-tabs-underline");
    let sel_f = ui.ctx().animate_value_with_time(anim_id, *selected as f32, 0.18);

    let inner = ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 2.0;
        let mut last: Option<Response> = None;
        let mut rects = Vec::with_capacity(labels.len());
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
            rects.push(rect);
            last = Some(response.on_hover_cursor(egui::CursorIcon::PointingHand));
        }

        for (i, rect) in rects.iter().enumerate() {
            if !ui.is_rect_visible(*rect) {
                continue;
            }
            let color = if *selected == i { theme.text } else { theme.text_muted };
            let galley = ui.painter().layout_no_wrap(
                labels[i].to_owned(),
                FontId::proportional(14.0),
                color,
            );
            ui.painter().galley(rect.min + padding, galley, color);
        }

        if !rects.is_empty() {
            let baseline_y = rects[0].bottom() - 0.5;
            ui.painter().hline(
                rects[0].left()..=rects[rects.len() - 1].right(),
                baseline_y,
                Stroke::new(1.0, theme.border),
            );
            let lo = (sel_f.floor() as usize).min(rects.len() - 1);
            let hi = (lo + 1).min(rects.len() - 1);
            let t = (sel_f - lo as f32).clamp(0.0, 1.0);
            let x0 = egui::lerp(rects[lo].left()..=rects[hi].left(), t);
            let x1 = egui::lerp(rects[lo].right()..=rects[hi].right(), t);
            let y = rects[lo].bottom() - 2.0;
            ui.painter().rect_filled(
                egui::Rect::from_min_max(egui::pos2(x0, y), egui::pos2(x1, y + 2.0)),
                CornerRadius::ZERO,
                theme.primary,
            );
        }

        last
    });

    inner.inner.unwrap_or(inner.response)
}

pub fn accordion(ui: &mut Ui, title: &str, open: &mut bool, body: impl FnOnce(&mut Ui)) -> Response {
    let theme = Theme::get(ui.ctx());
    let id = ui.id().with(title);
    let (header_rect, response) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), 34.0), egui::Sense::click());
    let t = ui.ctx().animate_bool(id, *open);

    if response.hovered() {
        ui.painter()
            .rect_filled(header_rect, theme.radius, theme.surface);
    }
    ui.painter().text(
        egui::pos2(header_rect.left(), header_rect.center().y),
        egui::Align2::LEFT_CENTER,
        title,
        FontId::proportional(14.0),
        theme.text,
    );

    // Chevron drawn by hand so its angle can animate: right (closed) -> down (open).
    let c = egui::pos2(header_rect.right() - 12.0, header_rect.center().y);
    let h = 4.0_f32;
    let angle = t * std::f32::consts::FRAC_PI_2;
    let d = egui::vec2(angle.cos(), angle.sin());
    let tip = c + d * h;
    let rot = |v: egui::Vec2, a: f32| {
        egui::vec2(v.x * a.cos() - v.y * a.sin(), v.x * a.sin() + v.y * a.cos())
    };
    let arm1 = tip + rot(d, 135_f32.to_radians()) * h;
    let arm2 = tip + rot(d, -135_f32.to_radians()) * h;
    let chev = egui::Stroke::new(1.5, theme.text_muted);
    ui.painter().line_segment([tip, arm1], chev);
    ui.painter().line_segment([tip, arm2], chev);

    if response.clicked() {
        *open = !*open;
    }

    if t > 0.0 {
        let mut body_rect = ui.available_rect_before_wrap();
        body_rect.min.x += 12.0;
        body_rect.min.y = header_rect.bottom();
        body_rect.min.y += (1.0 - t) * 8.0;
        ui.multiply_opacity(t);
        ui.scope_builder(egui::UiBuilder::new().max_rect(body_rect), |ui| body(ui));
    }
    response
}

pub fn otp_input(ui: &mut Ui, code: &mut String, len: usize) -> Response {
    const BOX_W: f32 = 34.0;
    const BOX_H: f32 = 40.0;
    const GAP: f32 = 6.0;

    let theme = Theme::get(ui.ctx());
    let before = code.clone();

    let total_w = len as f32 * BOX_W + len.saturating_sub(1) as f32 * GAP;
    let (rect, mut response) =
        ui.allocate_exact_size(egui::vec2(total_w, BOX_H), Sense::click());

    if response.clicked() {
        response.request_focus();
    }

    if response.has_focus() {
        let events = ui.input(|i| i.events.clone());
        for event in events {
            match event {
                egui::Event::Text(text) => {
                    for ch in text.chars().filter(|c| c.is_ascii_digit()) {
                        if code.chars().count() < len {
                            code.push(ch);
                        }
                    }
                }
                egui::Event::Key {
                    key: egui::Key::Backspace,
                    pressed: true,
                    ..
                } => {
                    code.pop();
                }
                _ => {}
            }
        }
    }

    if code.chars().count() > len {
        *code = code.chars().take(len).collect();
    }

    if *code != before {
        response.mark_changed();
    }

    if ui.is_rect_visible(rect) {
        let painter = ui.painter();
        let radius = CornerRadius::same(theme.radius as u8);
        let active = code.chars().count();
        let chars: Vec<char> = code.chars().collect();

        for i in 0..len {
            let min = egui::pos2(rect.min.x + i as f32 * (BOX_W + GAP), rect.min.y);
            let box_rect = egui::Rect::from_min_size(min, egui::vec2(BOX_W, BOX_H));

            let stroke_color = if i == active {
                theme.primary
            } else {
                theme.border.gamma_multiply(1.6)
            };

            painter.rect_filled(box_rect, radius, theme.surface_hover.gamma_multiply(0.6));
            painter.rect_stroke(
                box_rect,
                radius,
                Stroke::new(1.0, stroke_color),
                StrokeKind::Inside,
            );

            if let Some(ch) = chars.get(i) {
                painter.text(
                    box_rect.center(),
                    Align2::CENTER_CENTER,
                    ch,
                    FontId::monospace(18.0),
                    theme.text,
                );
            }
        }
    }

    response
}

pub fn time_field(ui: &mut Ui, hh: &mut u8, mm: &mut u8) -> Response {
    let theme = Theme::get(ui.ctx());

    let frame = egui::Frame::new()
        .fill(theme.surface)
        .stroke(egui::Stroke::new(1.0, theme.border))
        .corner_radius(egui::CornerRadius::same(theme.radius as u8))
        .inner_margin(6.0);

    ui.horizontal(|ui| {
        let hours = frame
            .show(ui, |ui| {
                ui.add(
                    egui::DragValue::new(hh)
                        .range(0..=23)
                        .custom_formatter(|n, _| format!("{:02}", n as i64)),
                )
            })
            .inner;

        ui.label(egui::RichText::new(":").monospace().color(theme.text));

        frame.show(ui, |ui| {
            ui.add(
                egui::DragValue::new(mm)
                    .range(0..=59)
                    .custom_formatter(|n, _| format!("{:02}", n as i64)),
            )
        });

        hours
    })
    .inner
}

/// Native egui dropdown, themed. `ui.select(&items, &mut selected)`.
pub fn select(ui: &mut Ui, items: &[&str], selected: &mut usize) -> Response {
    egui::ComboBox::from_id_salt("kit-select")
        .selected_text(items.get(*selected).copied().unwrap_or(""))
        .width(180.0)
        .show_ui(ui, |ui| {
            for (i, item) in items.iter().enumerate() {
                if ui.selectable_label(i == *selected, *item).clicked() {
                    *selected = i;
                }
            }
        })
        .response
}

/// Filterable dropdown: type in the popup to narrow the list. `ui.combobox(&items, &mut query, &mut selected)`.
pub fn combobox(ui: &mut Ui, items: &[&str], query: &mut String, selected: &mut usize) -> Response {
    let needle = query.to_lowercase();
    egui::ComboBox::from_id_salt("kit-combobox")
        .selected_text(items.get(*selected).copied().unwrap_or(""))
        .width(200.0)
        .show_ui(ui, |ui| {
            ui.add(egui::TextEdit::singleline(query).hint_text("Search...").desired_width(f32::INFINITY));
            let mut hits = 0;
            for (i, item) in items.iter().enumerate() {
                if !needle.is_empty() && !item.to_lowercase().contains(&needle) {
                    continue;
                }
                hits += 1;
                if ui.selectable_label(i == *selected, *item).clicked() {
                    *selected = i;
                    query.clear();
                    ui.close();
                }
            }
            if hits == 0 {
                ui.weak("no match");
            }
        })
        .response
}

/// Animated on/off switch, 40x22.
pub fn switch(ui: &mut Ui, id: &str, on: &mut bool) -> Response {
    let theme = Theme::get(ui.ctx());
    let (rect, mut resp) = ui.allocate_exact_size(egui::vec2(40.0, 22.0), Sense::click());
    if resp.clicked() {
        *on = !*on;
        resp.mark_changed();
    }
    let frac = ui.ctx().animate_value_with_time(resp.id.with(id), if *on { 1.0 } else { 0.0 }, 0.12);
    if ui.is_rect_visible(rect) {
        let bg = mix(theme.surface, theme.primary, frac);
        ui.painter().rect_filled(rect, CornerRadius::same(11), bg);
        let knob_x = egui::lerp((rect.left() + 11.0)..=(rect.right() - 11.0), frac);
        ui.painter().circle_filled(egui::pos2(knob_x, rect.center().y), 9.0, theme.on_primary);
    }
    resp
}

fn mix(a: Color32, b: Color32, t: f32) -> Color32 {
    Color32::from_rgb(
        egui::lerp(a.r() as f32..=b.r() as f32, t) as u8,
        egui::lerp(a.g() as f32..=b.g() as f32, t) as u8,
        egui::lerp(a.b() as f32..=b.b() as f32, t) as u8,
    )
}

/// Native slider, themed.
pub fn slider(ui: &mut Ui, value: &mut f32, range: std::ops::RangeInclusive<f32>) -> Response {
    // egui sizes the rail from `spacing.slider_width`, and colours it from
    // `widgets.inactive.bg_fill` — both need setting or the rail vanishes
    let theme = Theme::get(ui.ctx());
    ui.scope(|ui| {
        ui.spacing_mut().slider_width = (ui.available_width() - 96.0).max(120.0);
        ui.visuals_mut().widgets.inactive.bg_fill = theme.border;
        ui.visuals_mut().widgets.inactive.corner_radius = egui::CornerRadius::same(4);
        ui.add(egui::Slider::new(value, range).trailing_fill(true))
    })
    .inner
}

/// Single-line text input in a themed frame.
pub fn text_input(ui: &mut Ui, text: &mut String, placeholder: &str) -> Response {
    let theme = Theme::get(ui.ctx());
    frame(theme).show(ui, |ui| {
        ui.add(egui::TextEdit::singleline(text).hint_text(placeholder).frame(egui::Frame::NONE))
            .on_hover_text(placeholder)
    }).inner
}

/// Multi-line text input in a themed frame.
pub fn textarea(ui: &mut Ui, text: &mut String, rows: usize) -> Response {
    let theme = Theme::get(ui.ctx());
    frame(theme).show(ui, |ui| {
        ui.add(egui::TextEdit::multiline(text).desired_rows(rows).frame(egui::Frame::NONE))
    }).inner
}

/// Numeric input with drag-to-change, in a themed frame.
pub fn number_input(ui: &mut Ui, value: &mut f64, range: std::ops::RangeInclusive<f64>) -> Response {
    let theme = Theme::get(ui.ctx());
    frame(theme).show(ui, |ui| {
        ui.add(egui::DragValue::new(value).range(range).speed(0.1))
    }).inner
}

fn frame(theme: Theme) -> egui::Frame {
    egui::Frame::new()
        .fill(theme.surface)
        .stroke(Stroke::new(1.0, theme.border))
        .corner_radius(CornerRadius::same(theme.radius as u8))
        .inner_margin(6.0)
        .outer_margin(0.0)
}

/// Hover text on whatever the closure renders: `ui.tooltip("Saved", |ui| ui.button("x"))`.
pub fn tooltip(ui: &mut Ui, text: &str, add: impl FnOnce(&mut Ui) -> Response) -> Response {
    add(ui).on_hover_text(text)
}

/// Dropdown menu button; returns the clicked item index.
pub fn menu(ui: &mut Ui, label: &str, items: &[&str]) -> Option<usize> {
    let mut chosen = None;
    ui.menu_button(label, |ui| {
        for (i, item) in items.iter().enumerate() {
            if ui.button(*item).clicked() {
                chosen = Some(i);
                ui.close();
            }
        }
    });
    chosen
}

/// Bordered section with a title: `ui.group_box("Details", |ui| ...)`.
pub fn group_box(ui: &mut Ui, title: &str, body: impl FnOnce(&mut Ui)) -> Response {
    let theme = Theme::get(ui.ctx());
    egui::Frame::new()
        .fill(theme.panel)
        .stroke(Stroke::new(1.0, theme.border))
        .corner_radius(CornerRadius::same(theme.radius as u8))
        .inner_margin(10.0)
        .show(ui, |ui| {
            ui.label(RichText::new(title).strong());
            ui.add_space(4.0);
            body(ui);
        })
        .response
}

/// Key/value table: `ui.description_list(&[("Plan", "Infinite")])`.
pub fn description_list(ui: &mut Ui, rows: &[(&str, &str)]) -> Response {
    let theme = Theme::get(ui.ctx());
    let id = ui.next_auto_id();
    egui::Grid::new(id).num_columns(2).spacing([20.0, 6.0]).show(ui, |ui| {
        for (k, v) in rows {
            ui.label(RichText::new(*k).color(theme.text_muted));
            ui.label(*v);
            ui.end_row();
        }
    })
    .response
}

/// Centred placeholder: `ui.empty("No results", "Try another search")`.
pub fn empty(ui: &mut Ui, title: &str, hint: &str) -> Response {
    let theme = Theme::get(ui.ctx());
    ui.vertical_centered(|ui| {
        ui.add_space(18.0);
        let (rect, resp) = ui.allocate_exact_size(egui::vec2(40.0, 40.0), Sense::hover());
        if ui.is_rect_visible(rect) {
            ui.painter().circle_stroke(rect.center(), 18.0, Stroke::new(2.0, theme.border));
            ui.painter().line_segment(
                [rect.center() + egui::vec2(-7.0, 7.0), rect.center() + egui::vec2(7.0, -7.0)],
                Stroke::new(2.0, theme.border),
            );
        }
        ui.label(RichText::new(title).strong());
        ui.label(RichText::new(hint).color(theme.text_muted));
        ui.add_space(18.0);
        resp
    })
    .inner
}

/// File chip with a remove button: `ui.attachment("report.pdf", &mut removed)`.
pub fn attachment(ui: &mut Ui, name: &str, removed: &mut bool) -> Response {
    let theme = Theme::get(ui.ctx());
    let size = theme.radius;
    ui.horizontal(|ui| {
        egui::Frame::new()
            .fill(theme.surface)
            .stroke(Stroke::new(1.0, theme.border))
            .corner_radius(CornerRadius::same(size as u8))
            .inner_margin(6.0)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new(name).color(theme.text));
                    if ui.small_button("x").clicked() {
                        *removed = true;
                    }
                });
            });
    })
    .response
}

/// Vertical scroll region with themed scrollbars: `ui.scrollable(200.0, |ui| ...)`.
pub fn scrollable<R>(ui: &mut Ui, height: f32, add: impl FnOnce(&mut Ui) -> R) -> R {
    egui::ScrollArea::vertical().max_height(height).show(ui, add).inner
}

/// Form row: label above a field, 12px gap to the next row.
pub fn form_row(ui: &mut Ui, label: &str, field: impl FnOnce(&mut Ui)) -> Response {
    let theme = Theme::get(ui.ctx());
    let resp = ui
        .vertical(|ui| {
            ui.label(RichText::new(label).color(theme.text_muted));
            field(ui);
            ui.add_space(12.0);
        })
        .response;
    resp
}

/// Rich hover card: `ui.hover_card(&resp, |ui| ui.label("details"))`.
pub fn hover_card(ui: &mut Ui, content: impl FnOnce(&mut Ui), add: impl FnOnce(&mut Ui) -> Response) -> Response {
    let theme = Theme::get(ui.ctx());
    let fill = theme.panel;
    let border = theme.border;
    add(ui).on_hover_ui(|ui| {
        egui::Frame::new()
            .fill(fill)
            .stroke(Stroke::new(1.0, border))
            .corner_radius(CornerRadius::same(theme.radius as u8))
            .inner_margin(8.0)
            .show(ui, content);
    })
}

/// Selectable row for lists; returns true when picked.
pub fn list_item(ui: &mut Ui, selected: bool, label: &str) -> bool {
    let width = ui.available_width().max(40.0);
    ui.add_sized([width, 24.0], egui::Button::selectable(selected, label)).clicked()
}

const MONTHS: [&str; 12] = [
    "January", "February", "March", "April", "May", "June",
    "July", "August", "September", "October", "November", "December",
];
const WEEKDAYS: [&str; 7] = ["Mo", "Tu", "We", "Th", "Fr", "Sa", "Su"];

fn days_in_month(y: i32, m: u32) -> u32 {
    match m {
        4 | 6 | 9 | 11 => 30,
        2 if (y % 4 == 0 && y % 100 != 0) || y % 400 == 0 => 29,
        2 => 28,
        _ => 31,
    }
}

/// Sakamoto's algorithm, shifted so that 0 = Monday .. 6 = Sunday.
fn weekday(y: i32, m: u32, d: u32) -> u32 {
    const T: [i32; 12] = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let m = m.clamp(1, 12);
    let y = if m < 3 { y - 1 } else { y };
    let sunday0 = (y + y.div_euclid(4) - y.div_euclid(100) + y.div_euclid(400)
        + T[(m - 1) as usize]
        + d as i32)
        .rem_euclid(7);
    ((sunday0 + 6) % 7) as u32
}

pub fn calendar(ui: &mut Ui, year: &mut i32, month: &mut u32, selected_day: &mut Option<u32>) -> Response {
    let t = Theme::get(ui.ctx());
    *month = (*month).clamp(1, 12);
    let radius = CornerRadius::same(t.radius as u8);
    let frame = egui::Frame::new()
        .fill(t.surface)
        .stroke(Stroke::new(1.0, t.border))
        .corner_radius(radius)
        .inner_margin(egui::Margin::same(6));
    let inner = frame.show(ui, |ui| {
        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing = vec2(0.0, 2.0);
            let mut resp = ui
                .horizontal(|ui| {
                    if ui.add_sized([30.0, 24.0], egui::Button::new("<")).clicked() {
                        if *month == 1 {
                            *month = 12;
                            *year -= 1;
                        } else {
                            *month -= 1;
                        }
                    }
                    let (rect, _) = ui.allocate_exact_size(vec2(150.0, 24.0), Sense::hover());
                    if ui.add_sized([30.0, 24.0], egui::Button::new(">")).clicked() {
                        if *month == 12 {
                            *month = 1;
                            *year += 1;
                        } else {
                            *month += 1;
                        }
                    }
                    let title = format!("{} {}", MONTHS[(*month - 1) as usize], *year);
                    ui.painter().text(rect.center(), Align2::CENTER_CENTER, title, FontId::proportional(13.0), t.text);
                })
                .response;
            let dim = days_in_month(*year, *month) as i32;
            if let Some(d) = selected_day {
                *d = (*d).min(dim as u32);
            }
            ui.horizontal(|ui| {
                for name in WEEKDAYS {
                    let (rect, _) = ui.allocate_exact_size(vec2(30.0, 18.0), Sense::hover());
                    ui.painter().text(rect.center(), Align2::CENTER_CENTER, name, FontId::proportional(11.0), t.text_muted);
                }
            });
            let first = weekday(*year, *month, 1) as i32;
            for week in 0..6 {
                ui.horizontal(|ui| {
                    for dow in 0..7 {
                        let n = week * 7 + dow - first + 1;
                        let in_month = (1..=dim).contains(&n);
                        let sense = if in_month { Sense::click() } else { Sense::hover() };
                        let (rect, mut r) = ui.allocate_exact_size(vec2(30.0, 28.0), sense);
                        if in_month {
                            let day = n as u32;
                            if r.clicked() && *selected_day != Some(day) {
                                *selected_day = Some(day);
                                r.mark_changed();
                            }
                            let selected = *selected_day == Some(day);
                            let fill = if selected {
                                t.primary
                            } else if r.hovered() {
                                t.surface_hover
                            } else {
                                Color32::TRANSPARENT
                            };
                            let color = if selected { t.on_primary } else { t.text };
                            ui.painter().rect_filled(rect.shrink(1.0), radius, fill);
                            ui.painter().text(rect.center(), Align2::CENTER_CENTER, day.to_string(), FontId::proportional(13.0), color);
                        }
                        resp = r;
                    }
                });
            }
            resp
        })
        .inner
    });
    inner.inner
}

pub fn date_picker(ui: &mut Ui, id: &str, year: &mut i32, month: &mut u32, day: &mut Option<u32>) -> Response {
    let t = Theme::get(ui.ctx());
    let open_id = egui::Id::unique(id).with("date_picker_open");
    let mut open = ui.ctx().data(|d| d.get_temp::<bool>(open_id)).unwrap_or(false);
    let (label, color) = match *day {
        Some(d) => (format!("{:04}-{:02}-{:02}", *year, *month, d), t.text),
        None => ("Pick a date".to_owned(), t.text_muted),
    };
    ui.vertical(|ui| {
        let button = egui::Button::new(RichText::new(label).color(color))
            .fill(t.surface)
            .stroke(Stroke::new(1.0, t.border))
            .corner_radius(CornerRadius::same(t.radius as u8));
        let resp = ui.add(button);
        if resp.clicked() {
            open = !open;
            ui.ctx().data_mut(|d| d.insert_temp(open_id, open));
        }
        if open {
            ui.add_space(4.0);
            calendar(ui, year, month, day);
        }
        resp
    })
    .inner
}

pub struct Node<'a> {
    pub label: &'a str,
    pub children: Vec<Node<'a>>,
}

impl<'a> Node<'a> {
    pub fn leaf(label: &'a str) -> Self {
        Self { label, children: Vec::new() }
    }

    pub fn branch(label: &'a str, children: Vec<Node<'a>>) -> Self {
        Self { label, children }
    }
}

pub fn tree(ui: &mut Ui, id: &str, nodes: &[Node<'_>], selected: &mut Option<Vec<usize>>) -> Response {
    fn rows(
        ui: &mut Ui,
        theme: &Theme,
        nodes: &[Node<'_>],
        path: &mut Vec<usize>,
        expanded: &mut Vec<Vec<usize>>,
        selected: &mut Option<Vec<usize>>,
        changed: &mut bool,
    ) {
        for (i, node) in nodes.iter().enumerate() {
            path.push(i);
            let has_children = !node.children.is_empty();
            let (rect, resp) =
                ui.allocate_exact_size(egui::vec2(ui.available_width(), 30.0), egui::Sense::click());
            let resp = resp.on_hover_cursor(egui::CursorIcon::PointingHand);
            let x = rect.left() + 8.0 + (path.len() - 1) as f32 * 14.0;

            if resp.clicked() {
                let on_glyph =
                    has_children && resp.interact_pointer_pos().is_some_and(|p| p.x < x + 16.0);
                if on_glyph {
                    if let Some(at) = expanded.iter().position(|p| *p == *path) {
                        expanded.swap_remove(at);
                    } else {
                        expanded.push(path.clone());
                    }
                } else {
                    *selected = Some(path.clone());
                    *changed = true;
                }
            }

            let is_selected = selected.as_deref() == Some(path.as_slice());
            let is_open = has_children && expanded.contains(&*path);

            if ui.is_rect_visible(rect) {
                let painter = ui.painter().with_clip_rect(rect.intersect(ui.clip_rect()));
                let radius = egui::CornerRadius::same(theme.radius as u8);
                if is_selected {
                    painter.rect_filled(rect, radius, theme.primary.gamma_multiply(0.18));
                } else if resp.hovered() {
                    painter.rect_filled(rect, radius, theme.surface_hover);
                }
                if has_children {
                    painter.text(
                        egui::pos2(x, rect.center().y),
                        egui::Align2::LEFT_CENTER,
                        if is_open { "-" } else { "+" },
                        egui::FontId::monospace(13.0),
                        theme.text_muted,
                    );
                }
                painter.text(
                    egui::pos2(x + 16.0, rect.center().y),
                    egui::Align2::LEFT_CENTER,
                    node.label,
                    egui::FontId::proportional(14.0),
                    theme.text,
                );
            }

            if is_open {
                rows(ui, theme, &node.children, path, expanded, selected, changed);
            }
            path.pop();
        }
    }

    let theme = Theme::get(ui.ctx());
    // egui has no `Id::unique`; `Id::new` is the hashing constructor.
    let state_id = egui::Id::unique(id).with("tree_expanded");
    let mut expanded = ui
        .ctx()
        .data(|d| d.get_temp::<Vec<Vec<usize>>>(state_id))
        .unwrap_or_default();
    let mut changed = false;
    let before = expanded.clone();

    let mut response = ui
        .vertical(|ui| {
            ui.spacing_mut().item_spacing.y = 2.0;
            rows(ui, &theme, nodes, &mut Vec::new(), &mut expanded, selected, &mut changed);
        })
        .response;

    if expanded != before {
        ui.ctx().data_mut(|d| d.insert_temp(state_id, expanded));
    }
    if changed {
        response.mark_changed();
    }
    response
}

pub fn diff(ui: &mut Ui, patch: &str) -> Response {
    let theme = Theme::get(ui.ctx());
    let panel = ui.visuals().panel_fill;
    let font = egui::FontId::monospace(12.0);

    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing.y = 0.0;
        for line in patch.lines() {
            let (bg, fg) = if line.starts_with("@@") {
                (theme.surface, theme.primary)
            } else if line.starts_with('+') {
                (theme.success.gamma_multiply(0.18), theme.success)
            } else if line.starts_with('-') {
                (theme.danger.gamma_multiply(0.18), theme.danger)
            } else {
                (panel, theme.text_muted)
            };

            let (rect, _) =
                ui.allocate_exact_size(egui::vec2(ui.available_width(), 16.0), egui::Sense::hover());
            if !ui.is_rect_visible(rect) {
                continue;
            }

            let painter = ui.painter().with_clip_rect(rect.intersect(ui.clip_rect()));
            painter.rect_filled(rect, egui::CornerRadius::ZERO, bg);
            let galley = painter.layout_no_wrap(line.to_owned(), font.clone(), fg);
            let pos = egui::pos2(rect.left() + 8.0, rect.center().y - galley.size().y / 2.0);
            painter.galley(pos, galley, fg);
        }
    })
    .response
}

/// Command palette in a modal: type to filter, arrows move, Enter picks.
pub fn palette(ui: &mut Ui, id: &str, open: &mut bool, query: &mut String, items: &[&str]) -> Option<usize> {
    if !*open {
        return None;
    }
    let theme = Theme::get(ui.ctx());
    let needle = query.to_lowercase();
    let hits: Vec<usize> = items
        .iter()
        .enumerate()
        .filter(|(_, s)| needle.is_empty() || s.to_lowercase().contains(&needle))
        .map(|(i, _)| i)
        .take(8)
        .collect();
    let hl_id = egui::Id::unique(id).with("highlight");
    let mut hl = ui
        .ctx()
        .data(|d| d.get_temp::<usize>(hl_id))
        .unwrap_or(0)
        .min(hits.len().saturating_sub(1));
    let last = hits.len().saturating_sub(1);
    let mut picked = None;
    let anim = egui::Id::unique(id).with("anim");
    let t = ui.ctx().animate_bool(anim, true);
    let modal = egui::Modal::new(egui::Id::unique(id)).show(ui.ctx(), |ui| {
        ui.multiply_opacity(t);
        ui.add_space((1.0 - t) * 8.0);
        ui.set_width(440.0);
        let search = ui.add(
            egui::TextEdit::singleline(query)
                .hint_text("Type a command...")
                .frame(egui::Frame::NONE)
                .desired_width(f32::INFINITY),
        );
        if !search.has_focus() {
            search.request_focus();
        }
        ui.input(|i| {
            if i.key_pressed(egui::Key::ArrowDown) {
                hl = (hl + 1).min(last);
            }
            if i.key_pressed(egui::Key::ArrowUp) {
                hl = hl.saturating_sub(1);
            }
            if i.key_pressed(egui::Key::Enter) {
                picked = hits.get(hl).copied();
            }
        });
        ui.add_space(6.0);
        for (row, &idx) in hits.iter().enumerate() {
            let on = row == hl;
            let text = RichText::new(items[idx]).color(if on { theme.on_primary } else { theme.text });
            let button = egui::Button::new(text)
                .fill(if on { theme.primary } else { Color32::TRANSPARENT })
                .min_size(egui::vec2(ui.available_width(), 26.0));
            if ui.add(button).clicked() {
                picked = Some(idx);
            }
        }
        if hits.is_empty() {
            ui.weak("no match");
        }
    });
    if picked.is_some() || modal.should_close() {
        *open = false;
        query.clear();
    }
    ui.ctx().data_mut(|d| d.insert_temp(hl_id, hl));
    picked
}

/// Modal dialog with a title and a close button.
pub fn dialog(ui: &mut Ui, id: &str, open: &mut bool, title: &str, body: impl FnOnce(&mut Ui)) -> Response {
    if !*open {
        return ui.allocate_response(egui::Vec2::ZERO, Sense::hover());
    }
    let anim = egui::Id::unique(id).with("anim");
    let t = ui.ctx().animate_bool(anim, true);
    let modal = egui::Modal::new(egui::Id::unique(id)).show(ui.ctx(), |ui| {
        ui.multiply_opacity(t);
        ui.add_space((1.0 - t) * 8.0); // rises into place
        ui.set_width(360.0);
        ui.horizontal(|ui| {
            ui.heading(title);
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.small_button("x").clicked() {
                    ui.close();
                }
            });
        });
        ui.add_space(8.0);
        body(ui);
    });
    if modal.should_close() {
        *open = false;
    }
    modal.response
}

/// Floating card anchored under a widget: `ui.popover("menu", &resp, &mut open, |ui| ...)`.
pub fn popover(ui: &mut Ui, id: &str, anchor: &Response, open: &mut bool, body: impl FnOnce(&mut Ui)) {
    if !*open {
        return;
    }
    let theme = Theme::get(ui.ctx());
    let t = ui.ctx().animate_bool(egui::Id::unique(id).with("anim"), true);
    let area = egui::Area::new(egui::Id::unique(id))
        .fixed_pos(anchor.rect.left_bottom() + egui::vec2(0.0, 6.0 + (1.0 - t) * 6.0))
        .order(egui::Order::Foreground)
        .show(ui.ctx(), |ui| {
            ui.multiply_opacity(t);
            egui::Frame::new()
                .fill(theme.panel)
                .stroke(Stroke::new(1.0, theme.border))
                .corner_radius(CornerRadius::same(theme.radius as u8))
                .inner_margin(10.0)
                .show(ui, body);
        });
    let clicked_outside = ui.input(|i| i.pointer.any_click())
        && ui
            .input(|i| i.pointer.interact_pos())
            .is_some_and(|p| !area.response.rect.contains(p));
    if clicked_outside {
        *open = false;
    }
}

/// Right-hand slide-in panel (no animation): `ui.sheet("help", &mut open, |ui| ...)`.
pub fn sheet(ui: &mut Ui, id: &str, open: &mut bool, body: impl FnOnce(&mut Ui)) {
    if !*open {
        return;
    }
    let theme = Theme::get(ui.ctx());
    let mut close = false;
    let t = ui.ctx().animate_bool(egui::Id::unique(id).with("anim"), true);
    egui::Area::new(egui::Id::unique(id))
        .anchor(egui::Align2::RIGHT_TOP, [-10.0 + (1.0 - t) * 48.0, 10.0])
        .order(egui::Order::Foreground)
        .show(ui.ctx(), |ui| {
            ui.multiply_opacity(t);
            egui::Frame::new()
                .fill(theme.panel)
                .stroke(Stroke::new(1.0, theme.border))
                .corner_radius(CornerRadius::same(theme.radius as u8))
                .inner_margin(12.0)
                .show(ui, |ui| {
                    ui.set_width(280.0);
                    ui.horizontal(|ui| {
                        ui.strong("Details");
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.small_button("x").clicked() {
                                close = true;
                            }
                        });
                    });
                    ui.add_space(8.0);
                    body(ui);
                });
        });
    if close {
        *open = false;
    }
}

/// Slide carousel: arrows, dots, and the current index.
pub fn carousel(ui: &mut Ui, index: &mut usize, count: usize) -> Response {
    let theme = Theme::get(ui.ctx());
    let count = count.max(1);
    // the index animates, so slides and dots glide instead of snapping
    let t = ui
        .ctx()
        .animate_value_with_time(egui::Id::unique("kit-carousel"), *index as f32, 0.22);
    let drift = (t - *index as f32) * 22.0; // residual distance while sliding
    let alpha = (1.0 - drift.abs() / 44.0).clamp(0.25, 1.0);
    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            if ui.button("<").clicked() {
                *index = if *index == 0 { count - 1 } else { *index - 1 };
            }
            let (rect, _) = ui.allocate_exact_size(egui::vec2(300.0, 160.0), Sense::hover());
            if ui.is_rect_visible(rect) {
                let radius = CornerRadius::same(theme.radius as u8);
                let painter = ui.painter();
                painter.rect_filled(rect, radius, theme.panel);
                painter.rect_stroke(rect, radius, Stroke::new(1.0, theme.border), StrokeKind::Inside);
                painter.text(
                    rect.center() + egui::vec2(drift, 0.0),
                    Align2::CENTER_CENTER,
                    format!("{} / {}", *index + 1, count),
                    FontId::proportional(15.0),
                    theme.text.gamma_multiply(alpha),
                );
            }
            if ui.button(">").clicked() {
                *index = (*index + 1) % count;
            }
        });
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 6.0;
            for i in 0..count {
                let (rect, resp) = ui.allocate_exact_size(egui::vec2(14.0, 12.0), Sense::click());
                // how "selected" this dot is, animated between 0 and 1
                let sel = (1.0 - (t - i as f32).abs()).clamp(0.0, 1.0);
                let size = 4.0 + 3.0 * sel;
                let color = if sel > 0.5 { theme.primary } else { theme.border };
                ui.painter().circle_filled(rect.center(), size, color);
                if resp.clicked() {
                    *index = i;
                }
            }
        });
    })
    .response
}

/// Clickable link-styled text.
pub fn link(ui: &mut Ui, text: &str) -> Response {
    let theme = Theme::get(ui.ctx());
    ui.add(egui::Label::new(RichText::new(text).color(theme.primary).underline()).sense(Sense::click()))
        .on_hover_cursor(egui::CursorIcon::PointingHand)
}

/// Split button: primary action plus a caret that opens `items`. Returns (main, picked).
pub fn dropdown_button(ui: &mut Ui, id: &str, label: &str, items: &[&str]) -> (Response, Option<usize>) {
    let theme = Theme::get(ui.ctx());
    let open_id = egui::Id::unique(id).with("dropdown_open");
    let mut open = ui.ctx().data(|d| d.get_temp::<bool>(open_id)).unwrap_or(false);
    let main = ui.add(
        Button::new(RichText::new(label).color(theme.on_primary))
            .fill(theme.primary)
            .min_size(egui::vec2(0.0, 30.0)),
    );
    let caret = ui.button("v");
    if caret.clicked() {
        open = !open;
    }
    let mut picked = None;
    if open {
        let area = egui::Area::new(egui::Id::unique(id).with("dropdown_area"))
            .fixed_pos(caret.rect.left_bottom() + egui::vec2(0.0, 4.0))
            .order(egui::Order::Foreground)
            .show(ui.ctx(), |ui| {
                egui::Frame::new()
                    .fill(theme.panel)
                    .stroke(Stroke::new(1.0, theme.border))
                    .corner_radius(CornerRadius::same(theme.radius as u8))
                    .inner_margin(4.0)
                    .show(ui, |ui| {
                        for (i, item) in items.iter().enumerate() {
                            if ui.add(egui::Button::new(*item).min_size(egui::vec2(140.0, 26.0))).clicked() {
                                picked = Some(i);
                            }
                        }
                    });
            });
        let clicked_outside = ui.input(|i| i.pointer.any_click())
            && ui.input(|i| i.pointer.interact_pos()).is_some_and(|p| !area.response.rect.contains(p));
        if picked.is_some() || clicked_outside {
            open = false;
        }
    }
    ui.ctx().data_mut(|d| d.insert_temp(open_id, open));
    (main, picked)
}

/// Top toolbar panel inside a `Ui`.
pub fn toolbar<R>(ui: &mut Ui, id: &str, add: impl FnOnce(&mut Ui) -> R) -> R {
    let theme = Theme::get(ui.ctx());
    egui::Panel::top(id)
        .frame(
            egui::Frame::new()
                .fill(theme.panel)
                .inner_margin(egui::Margin::symmetric(8, 4)),
        )
        .show(ui, add)
        .inner
}

/// Bottom status bar with a left and a right region.
pub fn status_bar(ui: &mut Ui, id: &str, left: &str, right: &str) -> Response {
    let theme = Theme::get(ui.ctx());
    egui::Panel::bottom(id)
        .frame(
            egui::Frame::new()
                .fill(theme.panel)
                .inner_margin(egui::Margin::symmetric(8, 3)),
        )
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new(left).small().color(theme.text_muted));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(RichText::new(right).small().color(theme.text_muted));
                });
            });
        })
        .response
}

/// Auto-expiring toast stack, drawn bottom-right.
#[derive(Default)]
pub struct Toasts {
    items: Vec<(String, Variant, f64, f64)>, // text, variant, born, until
}

impl Toasts {
    /// Show a toast for `ttl` seconds.
    pub fn push(&mut self, text: &str, variant: Variant, now: f64, ttl: f64) {
        self.items.push((text.to_owned(), variant, now, now + ttl));
    }

    pub fn show(&mut self, ui: &mut Ui) {
        let theme = Theme::get(ui.ctx());
        let now = ui.input(|i| i.time);
        self.items.retain(|(_, _, _, until)| *until > now);
        if self.items.is_empty() {
            return;
        }
        egui::Area::new(egui::Id::unique("egui_kit_toasts"))
            .anchor(egui::Align2::RIGHT_BOTTOM, [-16.0, -16.0])
            .order(egui::Order::Foreground)
            .show(ui.ctx(), |ui| {
                for (text, variant, born, until) in &self.items {
                    let accent = match variant {
                        Variant::Primary => theme.primary,
                        Variant::Danger => theme.danger,
                        _ => theme.border,
                    };
                    // slide in over 180ms, fade out over the last 400ms
                    let enter = ((now - born) / 0.18).clamp(0.0, 1.0) as f32;
                    let exit = ((until - now) / 0.4).clamp(0.0, 1.0) as f32;
                    let t = enter.min(exit);
                    ui.scope(|ui| {
                        ui.multiply_opacity(t);
                        ui.add_space((1.0 - t) * 6.0);
                        egui::Frame::new()
                            .fill(theme.panel)
                            .stroke(Stroke::new(1.0, accent.gamma_multiply(0.6)))
                            .corner_radius(CornerRadius::same(theme.radius as u8))
                            .inner_margin(egui::Margin::symmetric(10, 8))
                            .shadow(egui::epaint::Shadow {
                                offset: [0, 4],
                                blur: 12,
                                spread: 0,
                                color: egui::Color32::from_black_alpha(80),
                            })
                            .show(ui, |ui| {
                                ui.label(RichText::new(text).color(theme.text));
                            });
                    });
                    ui.add_space(4.0);
                }
            });
        ui.ctx().request_repaint_after(std::time::Duration::from_millis(120));
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

pub fn bubble(ui: &mut Ui, text: &str, side: Side, time: Option<&str>) -> Response {
    let theme = Theme::get(ui.ctx());
    let (fill, fg) = match side {
        Side::Right => (theme.primary, theme.on_primary),
        Side::Left => (theme.surface, theme.text),
    };
    let pad = egui::vec2(10.0, 8.0);
    let max_width = (ui.available_width() - pad.x * 2.0).clamp(1.0, 320.0);
    let galley = ui
        .painter()
        .layout(text.to_owned(), FontId::proportional(14.0), fg, max_width);
    let time_galley = time.map(|t| {
        ui.painter()
            .layout_no_wrap(t.to_owned(), FontId::proportional(10.0), theme.text_muted)
    });
    let time_size = time_galley
        .as_ref()
        .map_or(Vec2::ZERO, |g| g.size() + egui::vec2(0.0, 2.0));
    let size = egui::vec2(
        galley.size().x.max(time_size.x),
        galley.size().y + time_size.y,
    ) + pad * 2.0;

    ui.horizontal(|ui| {
        if side == Side::Right {
            let gap = ui.available_width() - size.x;
            if gap.is_finite() && gap > 0.0 {
                ui.add_space(gap);
            }
        }
        let (rect, response) = ui.allocate_exact_size(size, Sense::hover());
        if ui.is_rect_visible(rect) {
            let radius = CornerRadius::same((theme.radius as f32 + 6.0) as u8);
            let painter = ui.painter();
            painter.rect_filled(rect, radius, fill);
            if side == Side::Left {
                let stroke = Stroke::new(1.0, theme.border);
                painter.rect_stroke(rect, radius, stroke, StrokeKind::Inside);
            }
            let origin = rect.min + pad;
            painter.galley(origin, galley.clone(), fg);
            if let Some(tg) = &time_galley {
                let pos = egui::pos2(
                    rect.right() - pad.x - tg.size().x,
                    origin.y + galley.size().y + 2.0,
                );
                painter.galley(pos, tg.clone(), theme.text_muted);
            }
        }
        response
    })
    .inner
}

pub fn marker(ui: &mut Ui, text: &str) -> Response {
    let theme = Theme::get(ui.ctx());
    let width = ui.available_width();
    let (rect, response) = ui.allocate_exact_size(egui::vec2(width, 18.0), Sense::hover());
    if ui.is_rect_visible(rect) {
        let painter = ui.painter();
        let galley =
            painter.layout_no_wrap(text.to_owned(), FontId::proportional(11.0), theme.text_muted);
        let chip_size = egui::vec2(galley.size().x + 16.0, rect.height());
        let chip = Rect::from_center_size(rect.center(), chip_size);
        let stroke = Stroke::new(1.0, theme.border);
        let y = rect.center().y;
        let left_end = (chip.left() - 8.0).max(rect.left());
        let right_start = (chip.right() + 8.0).min(rect.right());
        painter.line_segment([egui::pos2(rect.left(), y), egui::pos2(left_end, y)], stroke);
        painter.line_segment([egui::pos2(right_start, y), egui::pos2(rect.right(), y)], stroke);
        painter.rect_filled(chip, CornerRadius::same(9), theme.surface);
        painter.galley(chip.center() - galley.size() / 2.0, galley, theme.text_muted);
    }
    response
}

pub fn message(ui: &mut Ui, author: &str, text: &str, side: Side, time: &str) -> Response {
    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing.y = 4.0;
        let layout = match side {
            Side::Left => egui::Layout::left_to_right(egui::Align::Center),
            Side::Right => egui::Layout::right_to_left(egui::Align::Center),
        };
        let row = egui::vec2(ui.available_width(), 28.0);
        ui.allocate_ui_with_layout(row, layout, |ui| {
            ui.spacing_mut().item_spacing.x = 8.0;
            avatar(ui, author, 28.0);
            ui.label(egui::RichText::new(author).size(12.0).strong());
        });
        bubble(ui, text, side, Some(time));
    })
    .response
}

/// Virtualized list: only visible rows render. `ui.virtual_list(180.0, n, |ui, i| ...)`.
pub fn virtual_list(ui: &mut Ui, height: f32, len: usize, mut row: impl FnMut(&mut Ui, usize)) {
    let row_h = 22.0;
    egui::ScrollArea::vertical()
        .max_height(height)
        .show_rows(ui, row_h, len, |ui, range| {
            for i in range {
                ui.allocate_ui(egui::vec2(ui.available_width(), row_h), |ui| row(ui, i));
            }
        });
}

/// Column-aligned key/value grid, any number of columns: `ui.grid_layout("specs", &rows)`.
pub fn grid_layout(ui: &mut Ui, id: &str, rows: &[Vec<String>]) -> Response {
    let theme = Theme::get(ui.ctx());
    let cols = rows.first().map_or(1, Vec::len).max(1);
    egui::Grid::new(egui::Id::unique(id))
        .num_columns(cols)
        .spacing([16.0, 6.0])
        .show(ui, |ui| {
            for row in rows {
                for (c, cell) in row.iter().enumerate() {
                    let text = RichText::new(cell);
                    ui.label(if c == 0 { text.color(theme.text_muted) } else { text });
                }
                ui.end_row();
            }
        })
        .response
}

/// Input with a prefix/suffix chip: `ui.input_group(&mut text, "https://", "@example.com")`.
pub fn input_group(ui: &mut Ui, text: &mut String, prefix: &str, suffix: &str) -> Response {
    let theme = Theme::get(ui.ctx());
    let chip = |ui: &mut Ui, label: &str| {
        ui.label(RichText::new(label).small().color(theme.text_muted));
    };
    egui::Frame::new()
        .fill(theme.surface)
        .stroke(Stroke::new(1.0, theme.border))
        .corner_radius(CornerRadius::same(theme.radius as u8))
        .inner_margin(6.0)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                if !prefix.is_empty() {
                    chip(ui, prefix);
                }
                let suffix_w = ui
                    .painter()
                    .layout_no_wrap(suffix.to_owned(), FontId::proportional(11.0), theme.text_muted)
                    .size()
                    .x
                    + 12.0;
                ui.add(
                    egui::TextEdit::singleline(text)
                        .frame(egui::Frame::NONE)
                        .desired_width((ui.available_width() - suffix_w).max(60.0)),
                );
                if !suffix.is_empty() {
                    chip(ui, suffix);
                }
            });
        })
        .response
}

/// Image with a themed placeholder when it cannot be loaded.
/// Real files/URLs need an image loader installed (egui_extras' "image" feature);
/// without one this always shows the placeholder.
pub fn kit_image(ui: &mut Ui, uri: &str, max_width: f32) -> Response {
    let theme = Theme::get(ui.ctx());
    let loaded = ui.ctx().try_load_texture(
        uri,
        egui::TextureOptions::LINEAR,
        egui::load::SizeHint::default(),
    );
    if loaded.is_ok() {
        return ui.add(
            egui::Image::new(uri)
                .max_width(max_width)
                .corner_radius(CornerRadius::same(theme.radius as u8)),
        );
    }
    let (rect, resp) = ui.allocate_exact_size(egui::vec2(max_width, 80.0), Sense::hover());
    if ui.is_rect_visible(rect) {
        let radius = CornerRadius::same(theme.radius as u8);
        ui.painter().rect_filled(rect, radius, theme.surface);
        ui.painter().rect_stroke(rect, radius, Stroke::new(1.0, theme.border), StrokeKind::Inside);
        ui.painter().text(
            rect.center(),
            Align2::CENTER_CENTER,
            "no image",
            FontId::proportional(12.0),
            theme.text_muted,
        );
    }
    resp
}

/// Text label with the kit's type ramp: `ui.text("Title", LabelKind::Heading)`.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LabelKind {
    Heading,
    Body,
    Muted,
    Small,
}

pub fn text(ui: &mut Ui, value: &str, kind: LabelKind) -> Response {
    let theme = Theme::get(ui.ctx());
    let text = match kind {
        LabelKind::Heading => RichText::new(value).size(18.0).strong(),
        LabelKind::Body => RichText::new(value),
        LabelKind::Muted => RichText::new(value).color(theme.text_muted),
        LabelKind::Small => RichText::new(value).small().color(theme.text_muted),
    };
    ui.label(text)
}

/// Small icon-button toggle with a drawn check mark: `ui.toggle("bold", &mut on)`.
pub fn toggle(ui: &mut Ui, id: &str, on: &mut bool) -> Response {
    let theme = Theme::get(ui.ctx());
    let (rect, mut resp) = ui.allocate_exact_size(egui::vec2(30.0, 26.0), Sense::click());
    if resp.clicked() {
        *on = !*on;
        resp.mark_changed();
    }
    let _ = id;
    if ui.is_rect_visible(rect) {
        let radius = CornerRadius::same(theme.radius as u8);
        let (fill, stroke) = if *on {
            (theme.primary.gamma_multiply(0.25), theme.primary)
        } else if resp.hovered() {
            (theme.surface_hover, theme.border)
        } else {
            (theme.surface, theme.border)
        };
        ui.painter().rect_filled(rect, radius, fill);
        ui.painter().rect_stroke(rect, radius, Stroke::new(1.0, stroke), StrokeKind::Inside);
        let c = rect.center();
        let s = 5.0;
        let color = if *on { theme.primary } else { theme.text_muted };
        ui.painter().line_segment(
            [c + egui::vec2(-s, 0.0), c + egui::vec2(-2.0, s * 0.8)],
            Stroke::new(2.0, color),
        );
        ui.painter().line_segment(
            [c + egui::vec2(-2.0, s * 0.8), c + egui::vec2(s, -s * 0.7)],
            Stroke::new(2.0, color),
        );
    }
    resp
}

/// Copy-to-clipboard button that confirms on click: `ui.copy_button("code", "cargo add egui-kit")`.
pub fn copy_button(ui: &mut Ui, id: &str, value: &str) -> Response {
    let theme = Theme::get(ui.ctx());
    let done_id = egui::Id::unique(id).with("copied");
    let copied_at: Option<f64> = ui.ctx().data(|d| d.get_temp(done_id));
    let now = ui.input(|i| i.time);
    let just_copied = copied_at.is_some_and(|t| now - t < 1.2);
    let label = if just_copied { "copied" } else { "copy" };
    let color = if just_copied { theme.success } else { theme.text_muted };
    let resp = ui.add(Button::new(RichText::new(label).color(color)).fill(theme.surface));
    if resp.clicked() {
        ui.ctx().copy_text(value.to_owned());
        ui.ctx().data_mut(|d| d.insert_temp(done_id, now));
    }
    if just_copied {
        ui.ctx().request_repaint_after(std::time::Duration::from_millis(200));
    }
    resp
}

// `Theme` is assumed to be in scope, with Color32 colour fields and a numeric `radius`.

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Icon { Check, Cross, Plus, Minus, ChevronRight, ChevronDown, Search, Dot }

pub fn icon(ui: &mut Ui, kind: Icon, size: f32, color: Color32) -> Response {
    let (rect, response) = ui.allocate_exact_size(vec2(size, size), Sense::hover());
    if ui.is_rect_visible(rect) {
        let p = ui.painter();
        let stroke = Stroke::new(2.0, color);
        let at = |x: f32, y: f32| pos2(rect.left() + x * size, rect.top() + y * size);
        match kind {
            Icon::Check => {
                p.line_segment([at(0.20, 0.55), at(0.42, 0.75)], stroke);
                p.line_segment([at(0.42, 0.75), at(0.80, 0.28)], stroke);
            }
            Icon::Cross => {
                p.line_segment([at(0.25, 0.25), at(0.75, 0.75)], stroke);
                p.line_segment([at(0.75, 0.25), at(0.25, 0.75)], stroke);
            }
            Icon::Plus => {
                p.line_segment([at(0.20, 0.50), at(0.80, 0.50)], stroke);
                p.line_segment([at(0.50, 0.20), at(0.50, 0.80)], stroke);
            }
            Icon::Minus => {
                p.line_segment([at(0.20, 0.50), at(0.80, 0.50)], stroke);
            }
            Icon::ChevronRight => {
                p.line_segment([at(0.38, 0.22), at(0.66, 0.50)], stroke);
                p.line_segment([at(0.66, 0.50), at(0.38, 0.78)], stroke);
            }
            Icon::ChevronDown => {
                p.line_segment([at(0.22, 0.38), at(0.50, 0.66)], stroke);
                p.line_segment([at(0.50, 0.66), at(0.78, 0.38)], stroke);
            }
            Icon::Search => {
                p.circle_stroke(at(0.44, 0.44), size * 0.26, stroke);
                p.line_segment([at(0.63, 0.63), at(0.82, 0.82)], stroke);
            }
            Icon::Dot => {
                p.circle_filled(rect.center(), size * 0.18, color);
            }
        }
    }
    response
}

pub struct Question<'a> {
    pub prompt: &'a str,
    pub options: Vec<&'a str>,
}

pub fn questionnaire(
    ui: &mut Ui,
    id: &str,
    questions: &[Question<'_>],
    answers: &mut Vec<Option<usize>>,
) -> Response {
    let theme = Theme::get(ui.ctx());
    // egui 0.37 has no `Id::unique`; `Id::new` is the hashing constructor.
    let step_id = egui::Id::unique(id).with("step");
    answers.resize(questions.len(), None);
    let last_index = questions.len().saturating_sub(1);
    let mut step = ui.ctx().data(|d| d.get_temp::<usize>(step_id)).unwrap_or(0).min(last_index);
    let mut changed = false;

    let mut response = ui
        .vertical(|ui| {
            let Some(question) = questions.get(step) else { return };
            ui.label(RichText::new(question.prompt).strong().color(theme.text));
            ui.add_space(4.0);

            let radius = CornerRadius::same(theme.radius as u8);
            let font = egui::TextStyle::Body.resolve(ui.style());
            for (i, option) in question.options.iter().enumerate() {
                let selected = answers[step] == Some(i);
                let (rect, row) = ui.allocate_exact_size(vec2(ui.available_width(), 30.0), Sense::click());
                let row = row.on_hover_cursor(egui::CursorIcon::PointingHand);
                row.widget_info(|| {
                    egui::WidgetInfo::selected(egui::WidgetType::RadioButton, true, selected, *option)
                });
                if ui.is_rect_visible(rect) {
                    let (fill, edge, text_color) = if selected {
                        (theme.primary.gamma_multiply(0.15), theme.primary, theme.primary)
                    } else if row.hovered() {
                        (theme.surface_hover, theme.border, theme.text)
                    } else {
                        (theme.surface, theme.border, theme.text)
                    };
                    let p = ui.painter();
                    p.rect_filled(rect, radius, fill);
                    p.rect_stroke(rect, radius, Stroke::new(1.0, edge), StrokeKind::Inside);
                    p.text(
                        rect.left_center() + vec2(10.0, 0.0),
                        Align2::LEFT_CENTER,
                        *option,
                        font.clone(),
                        text_color,
                    );
                }
                if row.clicked() && !selected {
                    answers[step] = Some(i);
                    changed = true;
                }
                ui.add_space(4.0);
            }

            ui.add_space(4.0);
            ui.horizontal(|ui| {
                if ui.add_enabled(step > 0, egui::Button::new("Back")).clicked() {
                    step -= 1;
                    changed = true;
                }
                let is_last = step == last_index;
                let label = if is_last { "Finish" } else { "Next" };
                if ui.add(egui::Button::new(label)).clicked() && !is_last {
                    step += 1;
                    changed = true;
                }
            });
        })
        .response;

    ui.ctx().data_mut(|d| d.insert_temp(step_id, step));
    if changed {
        response.mark_changed();
    }
    response
}

/// Two-pane split: drag the handle to resize. `ui.resizable("side", 240.0, |ui| ...)`.
pub fn resizable<R>(ui: &mut Ui, id: &str, default_width: f32, add: impl FnOnce(&mut Ui) -> R) -> R {
    egui::Resize::default()
        .id_salt(id)
        .default_width(default_width)
        .show(ui, add)
}

/// Left navigation panel of fixed width.
pub fn sidebar<R>(ui: &mut Ui, id: &str, width: f32, add: impl FnOnce(&mut Ui) -> R) -> R {
    egui::Panel::left(id)
        .resizable(false)
        .exact_size(width)
        .show(ui, add)
        .inner
}

/// Settings block: groups labelled rows under a title.
pub fn settings(ui: &mut Ui, title: &str, body: impl FnOnce(&mut Ui)) -> Response {
    let theme = Theme::get(ui.ctx());
    egui::Frame::new()
        .fill(theme.panel)
        .stroke(Stroke::new(1.0, theme.border))
        .corner_radius(CornerRadius::same(theme.radius as u8))
        .inner_margin(12.0)
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            ui.label(RichText::new(title).strong());
            ui.add_space(6.0);
            body(ui);
        })
        .response
}

/// Root provider: applies the theme, then your content.
pub fn root<R>(ui: &mut Ui, dark: bool, add: impl FnOnce(&mut Ui) -> R) -> R {
    if dark {
        Theme::dark().apply(ui.ctx());
    } else {
        Theme::light().apply(ui.ctx());
    }
    ui.scope(add).inner
}

/// Keyboard-focus target covering the available area (overlays use this).
pub fn focus_trap(ui: &mut Ui, id: &str) -> Response {
    let resp = ui.interact(ui.available_rect_before_wrap(), egui::Id::unique(id), Sense::click());
    if resp.clicked() {
        resp.request_focus();
    }
    resp
}

/// Chat transcript that sticks to the newest row.
pub fn message_scroller(ui: &mut Ui, height: f32, len: usize, mut row: impl FnMut(&mut Ui, usize)) {
    egui::ScrollArea::vertical()
        .max_height(height)
        .stick_to_bottom(true)
        .show_rows(ui, 44.0, len, |ui, range| {
            for i in range {
                ui.allocate_ui(egui::vec2(ui.available_width(), 44.0), |ui| row(ui, i));
            }
        });
}

/// Thicker, smoother spinner than egui's default thin arc: `ui.kit_spinner(20.0)`.
pub fn kit_spinner(ui: &mut Ui, size: f32) -> Response {
    let theme = Theme::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(egui::vec2(size, size), Sense::hover());
    if ui.is_rect_visible(rect) {
        ui.ctx().request_repaint();
        let time = ui.input(|i| i.time) as f32;
        let radius = (size * 0.5 - 2.0).max(1.0);
        let center = rect.center();
        let start = time * std::f32::consts::TAU * 0.9;
        // 300 degrees of arc, drawn in 32 segments so it reads as smooth
        let sweep = std::f32::consts::TAU * (5.0 / 6.0);
        let points: Vec<egui::Pos2> = (0..=32)
            .map(|i| {
                let a = start + sweep * (i as f32 / 32.0);
                center + radius * egui::vec2(a.cos(), a.sin())
            })
            .collect();
        ui.painter().add(egui::Shape::line(
            points,
            Stroke::new((size * 0.14).clamp(2.0, 4.0), theme.primary),
        ));
    }
    response
}

pub fn alert_dismissable(
    ui: &mut Ui,
    id: &str,
    text: &str,
    variant: Variant,
    dismissed: &mut bool,
) -> Response {
    let theme = Theme::get(ui.ctx());
    let accent = match variant {
        Variant::Danger => theme.danger,
        Variant::Warning => theme.warning,
        Variant::Success => theme.success,
        _ => theme.primary,
    };
    let anim_id = egui::Id::unique(id).with("dismiss");
    let t = 1.0 - ui.ctx().animate_bool(anim_id, *dismissed);
    if t <= 0.0 {
        return ui
            .allocate_exact_size(egui::Vec2::ZERO, egui::Sense::hover())
            .1;
    }

    let (rect, response) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), 38.0), egui::Sense::hover());
    ui.multiply_opacity(t);
    let painter = ui.painter_at(rect);
    painter.rect_filled(
        rect,
        theme.radius,
        Color32::from_rgba_unmultiplied(accent.r(), accent.g(), accent.b(), 36),
    );
    painter.rect_stroke(
        rect,
        theme.radius,
        egui::Stroke::new(
            1.0,
            Color32::from_rgba_unmultiplied(accent.r(), accent.g(), accent.b(), 115),
        ),
        egui::StrokeKind::Inside,
    );

    let badge_c = egui::pos2(rect.left() + 21.0, rect.center().y);
    painter.circle_filled(badge_c, 9.0, accent);
    let badge_glyph = match variant {
        Variant::Danger | Variant::Warning => "!",
        _ => "i",
    };
    painter.text(
        badge_c,
        egui::Align2::CENTER_CENTER,
        badge_glyph,
        FontId::proportional(12.0),
        readable_on(accent, &theme),
    );
    painter.text(
        egui::pos2(rect.left() + 38.0, rect.center().y),
        egui::Align2::LEFT_CENTER,
        text,
        FontId::proportional(13.0),
        theme.text,
    );

    let close_c = egui::pos2(rect.right() - 18.0, rect.center().y);
    let close_rect = egui::Rect::from_center_size(close_c, egui::vec2(20.0, 20.0));
    let close_resp = ui.interact(close_rect, anim_id.with("close"), egui::Sense::click());
    if close_resp.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }
    let r = 5.0_f32;
    let xs = egui::Stroke::new(
        1.5,
        if close_resp.hovered() {
            theme.text
        } else {
            theme.text_muted
        },
    );
    painter.line_segment([close_c - egui::vec2(r, r), close_c + egui::vec2(r, r)], xs);
    painter.line_segment([close_c - egui::vec2(r, -r), close_c + egui::vec2(r, -r)], xs);
    if close_resp.clicked() {
        *dismissed = true;
    }
    response
}

pub fn textarea_hint(ui: &mut Ui, text: &mut String, rows: usize, hint: &str) -> Response {
    let theme = Theme::get(ui.ctx());
    let out = egui::Frame::new()
        .fill(theme.surface)
        .stroke(egui::Stroke::new(1.0, theme.border))
        .corner_radius(theme.radius)
        .inner_margin(egui::Margin::same(8))
        .show(ui, |ui| {
            egui::TextEdit::multiline(text)
                .desired_rows(rows)
                .hint_text(hint)
                .frame(egui::Frame::NONE)
                .desired_width(f32::INFINITY)
                .show(ui)
        });
    let resp = out.inner.response.response;
    let focus_t = ui
        .ctx()
        .animate_bool(resp.id.with("focus"), resp.has_focus());
    let border = mix(theme.border, theme.primary, focus_t);
    let frame_rect = out.response.rect;
    ui.painter_at(frame_rect).rect_stroke(
        frame_rect,
        theme.radius,
        egui::Stroke::new(1.0, border),
        egui::StrokeKind::Inside,
    );
    resp
}

pub fn stepper_rail(ui: &mut Ui, step: usize, total: usize) -> Response {
    let theme = Theme::get(ui.ctx());
    // 44px pitch leaves a visible run of rail between the 22px circles
    const PITCH: f32 = 44.0;
    let n = total.max(1) as f32;
    let width = (n - 1.0) * PITCH + 22.0;
    let (rect, response) = ui.allocate_exact_size(egui::vec2(width, 22.0), egui::Sense::hover());
    let painter = ui.painter();
    let cy = rect.center().y;
    let center = |i: usize| egui::pos2(rect.left() + 11.0 + i as f32 * PITCH, cy);

    // rails first, so the circles paint over their ends
    for i in 0..total.saturating_sub(1) {
        let t = ui.ctx().animate_bool(response.id.with(i), i < step);
        let stroke = egui::Stroke::new(2.5, mix(theme.border, theme.primary, t));
        painter.hline(center(i).x..=center(i + 1).x, cy, stroke);
    }

    for i in 0..total {
        let c = center(i);
        let label = (i + 1).to_string();
        let font = FontId::proportional(11.5);
        if i < step {
            painter.circle_filled(c, 11.0, theme.primary);
            painter.text(
                c,
                egui::Align2::CENTER_CENTER,
                &label,
                font,
                readable_on(theme.primary, &theme),
            );
        } else if i == step {
            painter.circle_stroke(c, 10.25, egui::Stroke::new(1.5, theme.primary));
            painter.text(c, egui::Align2::CENTER_CENTER, &label, font, theme.text);
        } else {
            painter.circle_filled(c, 11.0, theme.surface);
            painter.circle_stroke(c, 10.5, egui::Stroke::new(1.0, theme.border));
            painter.text(c, egui::Align2::CENTER_CENTER, &label, font, theme.text_muted);
        }
    }
    response
}

pub fn menu_button(ui: &mut Ui, id: &str, label: &str, items: &[&str]) -> Option<usize> {
    let theme = Theme::get(ui.ctx());
    let root_id = egui::Id::unique(id);
    let mut open = ui
        .ctx()
        .data_mut(|d| d.get_temp::<bool>(root_id).unwrap_or(false));
    let mut selected: Option<usize> = None;

    let text_w = ui
        .painter()
        .layout_no_wrap(label.to_owned(), FontId::proportional(13.0), theme.text)
        .size()
        .x;
    let (rect, resp) =
        ui.allocate_exact_size(egui::vec2(20.0 + text_w + 18.0, 27.0), egui::Sense::click());
    let painter = ui.painter();
    painter.rect_filled(
        rect,
        theme.radius,
        if resp.hovered() {
            theme.surface_hover
        } else {
            theme.surface
        },
    );
    painter.rect_stroke(
        rect,
        theme.radius,
        egui::Stroke::new(1.0, theme.border),
        egui::StrokeKind::Inside,
    );
    painter.text(
        egui::pos2(rect.left() + 10.0, rect.center().y),
        egui::Align2::LEFT_CENTER,
        label,
        FontId::proportional(13.0),
        theme.text,
    );

    let cc = egui::pos2(rect.right() - 15.0, rect.center().y);
    let r = 3.5_f32;
    let (y_top, y_mid) = if open {
        (cc.y + r, cc.y - r)
    } else {
        (cc.y - r, cc.y + r)
    };
    let chev = egui::Stroke::new(1.5, theme.text_muted);
    painter.line_segment([egui::pos2(cc.x - r, y_top), egui::pos2(cc.x, y_mid)], chev);
    painter.line_segment([egui::pos2(cc.x, y_mid), egui::pos2(cc.x + r, y_top)], chev);

    if resp.clicked() {
        open = !open;
    }

    if open {
        let popup = egui::Area::new(root_id.with("popup"))
            .order(egui::Order::Foreground)
            .fixed_pos(egui::pos2(rect.left(), rect.bottom() + 4.0))
            .show(ui.ctx(), |ui| {
                egui::Frame::new()
                    .fill(theme.panel)
                    .stroke(egui::Stroke::new(1.0, theme.border))
                    .corner_radius(theme.radius)
                    .inner_margin(egui::Margin::same(4))
                    .show(ui, |ui| {
                        for (i, item) in items.iter().enumerate() {
                            let (row, row_resp) = ui.allocate_exact_size(
                                egui::vec2(ui.available_width(), 26.0),
                                egui::Sense::click(),
                            );
                            if row_resp.hovered() {
                                ui.painter()
                                    .rect_filled(row, theme.radius, theme.surface_hover);
                            }
                            ui.painter().text(
                                egui::pos2(row.left() + 8.0, row.center().y),
                                egui::Align2::LEFT_CENTER,
                                *item,
                                FontId::proportional(13.0),
                                theme.text,
                            );
                            if row_resp.clicked() {
                                selected = Some(i);
                            }
                        }
                    });
            });
        let popup_rect = popup.response.rect;
        let escape = ui.ctx().input(|i| i.key_pressed(egui::Key::Escape));
        let clicked = ui.ctx().input(|i| i.pointer.primary_clicked());
        let pointer = ui
            .ctx()
            .input(|i| i.pointer.latest_pos())
            .unwrap_or(egui::Pos2::ZERO);
        if escape
            || selected.is_some()
            || (clicked && !rect.contains(pointer) && !popup_rect.contains(pointer))
        {
            open = false;
        }
    }
    ui.ctx().data_mut(|d| d.insert_temp(root_id, open));
    selected
}

pub fn toast_card(ui: &mut Ui, text: &str, variant: Variant, t01: f32) {
    let theme = Theme::get(ui.ctx());
    let accent = match variant {
        Variant::Danger => theme.danger,
        Variant::Warning => theme.warning,
        Variant::Success => theme.success,
        _ => theme.primary,
    };
    let t = t01.clamp(0.0, 1.0);
    let slide = (1.0 - t) * 24.0;
    let mut card_max = ui.available_rect_before_wrap();
    card_max.min.x += slide;
    ui.multiply_opacity(t);
    let shadow = egui::epaint::Shadow {
        offset: [0, 4],
        blur: 12,
        spread: 0,
        color: Color32::from_black_alpha(80),
    };
    ui.scope_builder(egui::UiBuilder::new().max_rect(card_max), |ui| {
        let out = egui::Frame::new()
            .fill(theme.panel)
            .stroke(egui::Stroke::new(
                1.0,
                Color32::from_rgba_unmultiplied(accent.r(), accent.g(), accent.b(), 153),
            ))
            .corner_radius(theme.radius)
            .shadow(shadow)
            .inner_margin(egui::Margin::symmetric(10, 8))
            .show(ui, |ui| {
                ui.label(RichText::new(text).size(13.0).color(theme.text));
            });
        let fr = out.response.rect;
        ui.painter().rect_filled(
            egui::Rect::from_min_max(
                egui::pos2(fr.left(), fr.top()),
                egui::pos2(fr.left() + 3.0, fr.bottom()),
            ),
            theme.radius,
            accent,
        );
    });
}
