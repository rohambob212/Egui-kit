//! Themed numeric Slider widget with step snapping and inline tabular readout.

use crate::theme::active_theme;
use crate::tokens::Size;
use egui::{Pos2, Rect, Response, RichText, Rounding, Sense, Stroke, Ui, Vec2, Widget};
use std::ops::RangeInclusive;

/// Interactive numeric slider mutating `&mut f32`.
pub struct Slider<'a> {
    value: &'a mut f32,
    range: RangeInclusive<f32>,
    step: Option<f32>,
    label: Option<String>,
    suffix: String,
    show_value: bool,
    size: Size,
    disabled: bool,
}

impl<'a> Slider<'a> {
    pub fn new(value: &'a mut f32, range: RangeInclusive<f32>) -> Self {
        Self {
            value,
            range,
            step: None,
            label: None,
            suffix: String::new(),
            show_value: true,
            size: Size::Md,
            disabled: false,
        }
    }

    pub fn step(mut self, step: f32) -> Self {
        self.step = Some(step.max(f32::EPSILON));
        self
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn suffix(mut self, suffix: impl Into<String>) -> Self {
        self.suffix = suffix.into();
        self
    }

    pub fn show_value(mut self, show: bool) -> Self {
        self.show_value = show;
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

impl<'a> Widget for Slider<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let tokens = active_theme(ui.ctx());
        let min = *self.range.start();
        let max = *self.range.end();
        let span = (max - min).max(f32::EPSILON);

        let mut resp = ui
            .vertical(|ui| {
                ui.spacing_mut().item_spacing.y = 4.0;
                if self.label.is_some() || self.show_value {
                    ui.horizontal(|ui| {
                        if let Some(label) = &self.label {
                            ui.label(RichText::new(label).size(self.size.font_size()).color(tokens.foreground));
                        }
                        if self.show_value {
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.label(
                                    RichText::new(format!("{:.1}{}", *self.value, self.suffix))
                                        .size(self.size.font_size() - 1.0)
                                        .monospace()
                                        .color(tokens.muted_foreground),
                                );
                            });
                        }
                    });
                }

                let track_h = match self.size {
                    Size::Xs | Size::Sm => 4.0,
                    Size::Md => 6.0,
                    Size::Lg => 8.0,
                };
                let thumb_r = track_h + 3.0;
                let width = ui.available_width().clamp(120.0, 320.0);
                let sense = if self.disabled { Sense::hover() } else { Sense::click_and_drag() };
                let (rect, mut track_resp) = ui.allocate_exact_size(Vec2::new(width, thumb_r * 2.4), sense);
                let track_left = rect.left() + thumb_r;
                let track_w = (rect.width() - thumb_r * 2.0).max(1.0);

                if !self.disabled {
                    if let Some(pointer) = track_resp.interact_pointer_pos() {
                        let frac = ((pointer.x - track_left) / track_w).clamp(0.0, 1.0);
                        let mut next = min + frac * span;
                        if let Some(step) = self.step {
                            next = min + ((next - min) / step).round() * step;
                        }
                        let clamped = next.clamp(min, max);
                        if (clamped - *self.value).abs() > f32::EPSILON {
                            *self.value = clamped;
                            track_resp.mark_changed();
                        }
                    }
                }

                if ui.is_rect_visible(rect) {
                    let active_hover = (track_resp.hovered() || track_resp.dragged()) && !self.disabled;
                    let hover_t = ui.ctx().animate_bool_with_time(track_resp.id.with("hov"), active_hover, 0.12);
                    let anim_secs = if track_resp.dragged() { 0.0 } else { 0.1 };
                    let display_val = ui.ctx().animate_value_with_time(
                        track_resp.id.with("val"),
                        (*self.value).clamp(min, max),
                        anim_secs,
                    );

                    let painter = ui.painter();
                    let t = ((display_val - min) / span).clamp(0.0, 1.0);
                    let track_rect = Rect::from_center_size(rect.center(), Vec2::new(track_w, track_h));
                    painter.rect_filled(track_rect, Rounding::same(track_h * 0.5), tokens.muted);

                    let active_w = track_w * t;
                    let active_rect = Rect::from_min_size(track_rect.min, Vec2::new(active_w, track_h));
                    let accent = if self.disabled { tokens.primary.linear_multiply(0.5) } else { tokens.primary };
                    painter.rect_filled(active_rect, Rounding::same(track_h * 0.5), accent);

                    let thumb_center = Pos2::new(track_left + active_w, rect.center().y);
                    if hover_t > 0.02 {
                        painter.circle_filled(
                            thumb_center,
                            thumb_r + 5.0 * hover_t,
                            tokens.ring.linear_multiply(0.18 * hover_t),
                        );
                    }
                    painter.circle(
                        thumb_center,
                        thumb_r + 1.5 * hover_t,
                        tokens.background,
                        Stroke::new(2.0, accent),
                    );
                }
                track_resp
            })
            .inner;

        if !self.disabled {
            resp = resp.on_hover_cursor(egui::CursorIcon::PointingHand);
        }
        resp
    }
}
