//! Interactive star Rating widget supporting whole or half-star precision.

use crate::theme::active_theme;
use crate::tokens::{ease_out_cubic, Size};
use egui::{Color32, Painter, Pos2, Rect, Response, Sense, Shape, Stroke, Ui, Vec2, Widget};
use std::f32::consts::{FRAC_PI_2, TAU};

/// Interactive star rating mutating `&mut f32` (0.0..=max_stars).
pub struct Rating<'a> {
    value: &'a mut f32,
    max_stars: u8,
    allow_half: bool,
    size: Size,
    disabled: bool,
}

impl<'a> Rating<'a> {
    pub fn new(value: &'a mut f32) -> Self {
        Self {
            value,
            max_stars: 5,
            allow_half: false,
            size: Size::Md,
            disabled: false,
        }
    }

    pub fn max_stars(mut self, max: u8) -> Self {
        self.max_stars = max.max(1);
        self
    }

    pub fn allow_half(mut self, allow: bool) -> Self {
        self.allow_half = allow;
        self
    }

    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl<'a> Widget for Rating<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let tokens = active_theme(ui.ctx());
        let star_size = self.size.icon_size() + 4.0;
        let gap = 4.0;
        let count = self.max_stars as usize;
        let total_w = (count as f32) * star_size + ((count.saturating_sub(1)) as f32) * gap;

        let sense = if self.disabled { Sense::hover() } else { Sense::click_and_drag() };
        let (rect, mut response) = ui.allocate_exact_size(Vec2::new(total_w, star_size), sense);

        let suppress_id = response.id.with("suppress_hov");
        let mut suppressed_pos = ui.ctx().data(|d| d.get_temp::<Option<Pos2>>(suppress_id).flatten());
        let mut preview = None;
        let mut hovered_idx = None;
        if !self.disabled {
            let active_pos = response.interact_pointer_pos().or_else(|| response.hover_pos());
            if let Some(pos) = active_pos {
                if let Some(cleared_at) = suppressed_pos {
                    if pos.distance(cleared_at) > 3.0 || response.dragged() {
                        suppressed_pos = None;
                    }
                }
                if suppressed_pos.is_none() || response.clicked() {
                    let rel_x = (pos.x - rect.left()).clamp(0.0, total_w);
                    let step_w = star_size + gap;
                    let idx = (rel_x / step_w).floor().min((count - 1) as f32);
                    hovered_idx = Some(idx as usize);
                    let local_frac = ((rel_x - idx * step_w) / star_size).clamp(0.0, 1.0);
                    let val = if self.allow_half && local_frac <= 0.5 {
                        idx + 0.5
                    } else {
                        idx + 1.0
                    };
                    preview = Some(val.clamp(0.0, self.max_stars as f32));
                }
            } else {
                suppressed_pos = None;
            }

            if response.clicked() {
                if let Some(next) = preview {
                    if (*self.value - next).abs() < 0.01 {
                        *self.value = 0.0;
                        suppressed_pos = active_pos;
                        preview = None;
                        hovered_idx = None;
                    } else {
                        *self.value = next;
                        suppressed_pos = None;
                    }
                    response.mark_changed();
                }
            } else if response.dragged() || response.drag_stopped() {
                if let Some(next) = preview {
                    if (*self.value - next).abs() > f32::EPSILON {
                        *self.value = next;
                        response.mark_changed();
                    }
                }
            }
            ui.ctx().data_mut(|d| d.insert_temp(suppress_id, suppressed_pos));
            response = response.on_hover_cursor(egui::CursorIcon::PointingHand);
        }

        let target = preview.unwrap_or(*self.value).clamp(0.0, self.max_stars as f32);
        let anim_secs = if response.dragged() { 0.0 } else { 0.12 };
        let shown = ui.ctx().animate_value_with_time(response.id.with("val"), target, anim_secs);
        if ui.is_rect_visible(rect) {
            let active_col = if self.disabled {
                tokens.warning.linear_multiply(0.5)
            } else {
                tokens.warning
            };
            let painter = ui.painter();

            for i in 0..count {
                let x = rect.left() + (i as f32) * (star_size + gap);
                let base_rect = Rect::from_min_size(Pos2::new(x, rect.top()), Vec2::splat(star_size));
                let pop_t = ease_out_cubic(ui.ctx().animate_bool_with_time(
                    response.id.with(("star_hov", i)),
                    hovered_idx == Some(i),
                    0.12,
                ));
                let star_rect = Rect::from_center_size(
                    base_rect.center(),
                    Vec2::splat(star_size * (1.0 + 0.14 * pop_t)),
                );
                let fill_amt = (shown - i as f32).clamp(0.0, 1.0);

                paint_star(painter, star_rect, tokens.muted, Stroke::new(1.0, tokens.border));
                if fill_amt > 0.0 {
                    let clip = Rect::from_min_size(
                        star_rect.min,
                        Vec2::new(star_rect.width() * fill_amt, star_rect.height()),
                    );
                    paint_star(&painter.with_clip_rect(clip), star_rect, active_col, Stroke::new(1.0, active_col));
                }
            }
        }

        response
    }
}

fn paint_star(painter: &Painter, rect: Rect, fill: Color32, stroke: Stroke) {
    let c = rect.center();
    let r_out = rect.width().min(rect.height()) * 0.46;
    let r_in = r_out * 0.42;
    let mut perimeter = Vec::with_capacity(10);

    for i in 0..5 {
        let a_mid = (i as f32) * (TAU / 5.0) - FRAC_PI_2;
        let a_prev = a_mid - (TAU / 10.0);
        let a_next = a_mid + (TAU / 10.0);
        let p_in = c + Vec2::angled(a_prev) * r_in;
        let p_out = c + Vec2::angled(a_mid) * r_out;
        let p_next = c + Vec2::angled(a_next) * r_in;
        perimeter.push(p_in);
        perimeter.push(p_out);
        painter.add(Shape::convex_polygon(vec![c, p_in, p_out, p_next], fill, Stroke::NONE));
    }
    if stroke != Stroke::NONE {
        painter.add(Shape::closed_line(perimeter, stroke));
    }
}
