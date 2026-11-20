//! Minimal immediate-mode Radio button bound directly to a PartialEq value.

use crate::theme::active_theme;
use crate::tokens::{ease_out_cubic, lerp_color, Size};
use egui::{Response, RichText, Sense, Stroke, Ui, Vec2, Widget};

/// Single-choice radio button mutating `&mut T` when clicked.
pub struct Radio<'a, T: PartialEq> {
    current: &'a mut T,
    value: T,
    label: String,
    size: Size,
    disabled: bool,
}

impl<'a, T: PartialEq> Radio<'a, T> {
    pub fn new(current: &'a mut T, value: T, label: impl Into<String>) -> Self {
        Self {
            current,
            value,
            label: label.into(),
            size: Size::Md,
            disabled: false,
        }
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

impl<'a, T: PartialEq> Widget for Radio<'a, T> {
    fn ui(self, ui: &mut Ui) -> Response {
        let tokens = active_theme(ui.ctx());
        let diameter = (self.size.icon_size() + 2.0).max(14.0);
        let sense = if self.disabled { Sense::hover() } else { Sense::click() };

        let mut resp = ui
            .horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 8.0;
                let (rect, circle_resp) = ui.allocate_exact_size(Vec2::splat(diameter), sense);
                let label_resp = if !self.label.is_empty() {
                    let color = if self.disabled { tokens.muted_foreground } else { tokens.foreground };
                    Some(ui.add(egui::Label::new(RichText::new(&self.label).size(self.size.font_size()).color(color)).sense(sense)))
                } else {
                    None
                };

                let clicked = circle_resp.clicked() || label_resp.as_ref().is_some_and(|r| r.clicked());
                if clicked && !self.disabled {
                    *self.current = self.value;
                }

                let selected = *self.current == self.value;
                let sel_t = ease_out_cubic(ui.ctx().animate_bool_with_time(circle_resp.id.with("sel"), selected, 0.15));
                let hover_t = ui.ctx().animate_bool_with_time(
                    circle_resp.id.with("hov"),
                    circle_resp.hovered() && !self.disabled,
                    0.12,
                );

                if ui.is_rect_visible(rect) {
                    let painter = ui.painter();
                    let active_col = if self.disabled { tokens.primary.linear_multiply(0.5) } else { tokens.primary };
                    let rest_ring = lerp_color(tokens.input, tokens.ring, hover_t);
                    let ring_col = lerp_color(rest_ring, active_col, sel_t);

                    painter.circle(
                        rect.center(),
                        diameter * 0.5,
                        tokens.background,
                        Stroke::new(1.5, ring_col),
                    );
                    if sel_t > 0.02 {
                        painter.circle_filled(rect.center(), (diameter * 0.27) * sel_t, active_col);
                    }
                }

                let mut combined = circle_resp;
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
