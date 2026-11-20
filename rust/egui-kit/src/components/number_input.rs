//! Numeric NumberInput widget with +/- stepper buttons, min/max clamping, and step snapping.

use crate::components::icon::{paint_icon, IconName};
use crate::theme::active_theme;
use crate::tokens::{ease_out_cubic, lerp_color, Size};
use egui::{FontId, Id, Key, Pos2, Rect, Response, Sense, Stroke, TextEdit, Ui, Vec2, Widget};
use std::ops::RangeInclusive;

/// Interactive numeric input mutating `&mut f32` directly.
pub struct NumberInput<'a> {
    value: &'a mut f32,
    range: RangeInclusive<f32>,
    step: f32,
    precision: usize,
    prefix: Option<String>,
    suffix: Option<String>,
    size: Size,
    width: f32,
    disabled: bool,
}

impl<'a> NumberInput<'a> {
    pub fn new(value: &'a mut f32, range: RangeInclusive<f32>) -> Self {
        Self {
            value,
            range,
            step: 1.0,
            precision: 0,
            prefix: None,
            suffix: None,
            size: Size::Md,
            width: 180.0,
            disabled: false,
        }
    }

    pub fn step(mut self, step: f32) -> Self {
        self.step = step.max(0.0001);
        self
    }

    pub fn precision(mut self, precision: usize) -> Self {
        self.precision = precision.min(4);
        self
    }

    pub fn prefix(mut self, prefix: impl Into<String>) -> Self {
        self.prefix = Some(prefix.into());
        self
    }

    pub fn suffix(mut self, suffix: impl Into<String>) -> Self {
        self.suffix = Some(suffix.into());
        self
    }

    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width.max(120.0);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    fn clamp_step(&self, next: f32) -> f32 {
        let min = *self.range.start();
        let max = *self.range.end();
        let clamped = if next.is_finite() { next.clamp(min, max) } else { min };
        let snapped = ((clamped - min) / self.step).round() * self.step + min;
        snapped.clamp(min, max)
    }
}

impl<'a> Widget for NumberInput<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let tokens = active_theme(ui.ctx());
        let h = self.size.height();
        let (outer_rect, mut resp) = ui.allocate_exact_size(Vec2::new(self.width, h), Sense::hover());
        let base_id = resp.id;
        let buf_id = Id::new((base_id, "draft_buf"));
        let edit_state_id = Id::new((base_id, "editing"));

        let min = *self.range.start();
        let max = *self.range.end();
        let btn_w = h.min(34.0);
        let dec_rect = Rect::from_min_size(outer_rect.min, Vec2::new(btn_w, h));
        let inc_rect = Rect::from_min_size(Pos2::new(outer_rect.right() - btn_w, outer_rect.top()), Vec2::new(btn_w, h));
        let center_rect = Rect::from_min_max(
            Pos2::new(dec_rect.right(), outer_rect.top()),
            Pos2::new(inc_rect.left(), outer_rect.bottom()),
        );

        let dec_resp = ui.interact(dec_rect, base_id.with("dec"), if self.disabled { Sense::hover() } else { Sense::click() });
        let inc_resp = ui.interact(inc_rect, base_id.with("inc"), if self.disabled { Sense::hover() } else { Sense::click() });

        let mut changed = false;
        if dec_resp.clicked() && !self.disabled && *self.value > min {
            *self.value = self.clamp_step(*self.value - self.step);
            changed = true;
        }
        if inc_resp.clicked() && !self.disabled && *self.value < max {
            *self.value = self.clamp_step(*self.value + self.step);
            changed = true;
        }

        let is_editing = ui.ctx().data(|d| d.get_temp::<bool>(edit_state_id).unwrap_or(false));
        let mut draft = if is_editing && !changed {
            ui.ctx().data(|d| {
                d.get_temp::<String>(buf_id)
                    .unwrap_or_else(|| format!("{:.*}", self.precision, *self.value))
            })
        } else {
            format!("{:.*}", self.precision, *self.value)
        };

        let focus_t = ease_out_cubic(ui.ctx().animate_bool_with_time(
            base_id.with("focus_anim"),
            is_editing && !self.disabled,
            0.14,
        ));
        let dec_hov = ui.ctx().animate_bool_with_time(base_id.with("dec_h"), dec_resp.hovered() && !self.disabled && *self.value > min, 0.1);
        let inc_hov = ui.ctx().animate_bool_with_time(base_id.with("inc_h"), inc_resp.hovered() && !self.disabled && *self.value < max, 0.1);

        if ui.is_rect_visible(outer_rect) {
            let painter = ui.painter();
            let bg = if self.disabled { tokens.muted } else { tokens.background };
            let border = lerp_color(tokens.input, tokens.ring, focus_t);
            if focus_t > 0.02 {
                painter.rect_stroke(outer_rect.expand(1.5 * focus_t), self.size.rounding(), Stroke::new(2.5 * focus_t, tokens.ring.linear_multiply(0.22 * focus_t)));
            }
            painter.rect(outer_rect, self.size.rounding(), bg, Stroke::new(1.0 + 0.5 * focus_t, border));

            painter.rect_filled(dec_rect.shrink(1.0), self.size.rounding(), lerp_color(tokens.muted, tokens.accent, dec_hov));
            painter.rect_filled(inc_rect.shrink(1.0), self.size.rounding(), lerp_color(tokens.muted, tokens.accent, inc_hov));
            painter.line_segment([dec_rect.right_top(), dec_rect.right_bottom()], Stroke::new(1.0, tokens.border));
            painter.line_segment([inc_rect.left_top(), inc_rect.left_bottom()], Stroke::new(1.0, tokens.border));

            let icon_s = Vec2::splat(self.size.icon_size() * 0.85);
            let dec_col = if self.disabled || *self.value <= min { tokens.muted_foreground } else { tokens.foreground };
            let inc_col = if self.disabled || *self.value >= max { tokens.muted_foreground } else { tokens.foreground };
            paint_icon(painter, Rect::from_center_size(dec_rect.center(), icon_s), IconName::Minus, dec_col);
            paint_icon(painter, Rect::from_center_size(inc_rect.center(), icon_s), IconName::Plus, inc_col);
        }

        let mut edit_ui = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(center_rect.shrink2(Vec2::new(6.0, 2.0)))
                .layout(egui::Layout::left_to_right(egui::Align::Center)),
        );
        edit_ui.set_enabled(!self.disabled);
        let edit_resp = edit_ui.add_sized(
            center_rect.shrink2(Vec2::new(6.0, 2.0)).size(),
            TextEdit::singleline(&mut draft)
                .horizontal_align(egui::Align::Center)
                .font(FontId::monospace(self.size.font_size()))
                .text_color(if self.disabled { tokens.muted_foreground } else { tokens.foreground })
                .frame(false),
        );

        if edit_resp.has_focus() && !self.disabled {
            if edit_ui.input(|i| i.key_pressed(Key::ArrowUp)) {
                *self.value = self.clamp_step(*self.value + self.step);
                draft = format!("{:.*}", self.precision, *self.value);
                changed = true;
            } else if edit_ui.input(|i| i.key_pressed(Key::ArrowDown)) {
                *self.value = self.clamp_step(*self.value - self.step);
                draft = format!("{:.*}", self.precision, *self.value);
                changed = true;
            }
        }

        if edit_resp.lost_focus() || (edit_resp.has_focus() && edit_ui.input(|i| i.key_pressed(Key::Enter))) {
            if let Ok(parsed) = draft.trim().parse::<f32>() {
                *self.value = self.clamp_step(parsed);
                changed = true;
            }
            draft = format!("{:.*}", self.precision, *self.value);
        }

        let edit_focused = edit_resp.has_focus();
        ui.ctx().data_mut(|d| {
            d.insert_temp(edit_state_id, edit_focused);
            d.insert_temp(buf_id, draft);
        });

        if changed {
            resp.mark_changed();
        }
        resp.union(dec_resp).union(inc_resp).union(edit_resp)
    }
}
