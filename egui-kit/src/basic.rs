//! Basic widgets. Thin, themed, stateless.
use crate::{Theme, Variant};
use egui::{Align2, Button, Color32, CursorIcon, FontId, Response, RichText, Sense, Ui, Vec2};
use egui::{pos2, vec2, CornerRadius, Rect, Stroke, StrokeKind, UiBuilder};

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
                theme.border
            };

            painter.rect_filled(box_rect, radius, theme.surface);
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
    ui.add(egui::Slider::new(value, range))
}

/// Single-line text input in a themed frame.
pub fn text_input(ui: &mut Ui, text: &mut String, placeholder: &str) -> Response {
    let theme = Theme::get(ui.ctx());
    frame(ui, theme).show(ui, |ui| {
        ui.add(egui::TextEdit::singleline(text).hint_text(placeholder).frame(egui::Frame::NONE))
            .on_hover_text(placeholder)
    }).inner
}

/// Multi-line text input in a themed frame.
pub fn textarea(ui: &mut Ui, text: &mut String, rows: usize) -> Response {
    let theme = Theme::get(ui.ctx());
    frame(ui, theme).show(ui, |ui| {
        ui.add(egui::TextEdit::multiline(text).desired_rows(rows).frame(egui::Frame::NONE))
    }).inner
}

/// Numeric input with drag-to-change, in a themed frame.
pub fn number_input(ui: &mut Ui, value: &mut f64, range: std::ops::RangeInclusive<f64>) -> Response {
    let theme = Theme::get(ui.ctx());
    frame(ui, theme).show(ui, |ui| {
        ui.add(egui::DragValue::new(value).range(range).speed(0.1))
    }).inner
}

fn frame<'a>(ui: &'a Ui, theme: Theme) -> egui::Frame {
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
    ui.selectable_label(selected, label).clicked()
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
