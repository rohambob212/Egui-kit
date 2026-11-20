//! Animated binary Switch widget using egui::Context::animate_bool.

use crate::theme::active_theme;
use crate::tokens::{ease_out_cubic, lerp_color, Size};
use egui::{Pos2, Response, RichText, Rounding, Sense, Stroke, Ui, Vec2, Widget};

/// Smooth binary toggle switch mutating `&mut bool`.
pub struct Switch<'a> {
    on: &'a mut bool,
    label: Option<String>,
    size: Size,
    disabled: bool,
}

impl<'a> Switch<'a> {
    pub fn new(on: &'a mut bool) -> Self {
        Self {
            on,
            label: None,
            size: Size::Md,
            disabled: false,
        }
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
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

impl<'a> Widget for Switch<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let tokens = active_theme(ui.ctx());
        let (track_w, track_h) = match self.size {
            Size::Xs => (28.0, 16.0),
            Size::Sm => (34.0, 18.0),
            Size::Md => (40.0, 22.0),
            Size::Lg => (48.0, 26.0),
        };
        let sense = if self.disabled { Sense::hover() } else { Sense::click() };

        let mut resp = ui
            .horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 8.0;
                let (rect, track_resp) = ui.allocate_exact_size(Vec2::new(track_w, track_h), sense);
                let label_resp = self.label.as_ref().map(|text| {
                    let color = if self.disabled { tokens.muted_foreground } else { tokens.foreground };
                    ui.add(egui::Label::new(RichText::new(text).size(self.size.font_size()).color(color)).sense(sense))
                });

                let clicked = track_resp.clicked() || label_resp.as_ref().is_some_and(|r| r.clicked());
                if clicked && !self.disabled {
                    *self.on = !*self.on;
                }

                let t = ease_out_cubic(ui.ctx().animate_bool_with_time(track_resp.id, *self.on, 0.16));
                let hover_t = ui.ctx().animate_bool_with_time(
                    track_resp.id.with("hov"),
                    track_resp.hovered() && !self.disabled,
                    0.12,
                );
                if ui.is_rect_visible(rect) {
                    let active_fill = if self.disabled { tokens.primary.linear_multiply(0.5) } else { tokens.primary };
                    let track_fill = lerp_color(tokens.muted, active_fill, t);
                    let border_col = lerp_color(tokens.border, tokens.ring, hover_t);
                    ui.painter().rect(rect, Rounding::same(track_h * 0.5), track_fill, Stroke::new(1.0, border_col));

                    let r = track_h * (0.37 + 0.03 * hover_t);
                    let pad = track_h * 0.5;
                    let thumb_x = egui::lerp((rect.left() + pad)..=(rect.right() - pad), t);
                    ui.painter().circle_filled(
                        Pos2::new(thumb_x, rect.center().y),
                        r,
                        tokens.background,
                    );
                }

                let mut combined = track_resp;
                if let Some(lr) = label_resp {
                    combined = combined.union(lr);
                }
                if clicked {
                    combined.mark_changed();
                }
                combined
            })
            .inner;

        if !self.disabled {
            resp = resp.on_hover_cursor(egui::CursorIcon::PointingHand);
        }
        resp
    }
}
