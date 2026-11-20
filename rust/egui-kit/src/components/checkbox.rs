//! Minimal immediate-mode Checkbox widget with optional indeterminate state.

use crate::theme::active_theme;
use crate::tokens::{ease_out_cubic, lerp_color, Size};
use egui::{Pos2, Response, RichText, Rounding, Sense, Stroke, Ui, Vec2, Widget};

/// Interactive checkbox mutating `&mut bool` directly.
pub struct Checkbox<'a> {
    checked: &'a mut bool,
    label: String,
    indeterminate: bool,
    size: Size,
    disabled: bool,
}

impl<'a> Checkbox<'a> {
    pub fn new(checked: &'a mut bool, label: impl Into<String>) -> Self {
        Self {
            checked,
            label: label.into(),
            indeterminate: false,
            size: Size::Md,
            disabled: false,
        }
    }

    pub fn indeterminate(mut self, indeterminate: bool) -> Self {
        self.indeterminate = indeterminate;
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

impl<'a> Widget for Checkbox<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let tokens = active_theme(ui.ctx());
        let box_edge = (self.size.icon_size() + 2.0).max(14.0);
        let sense = if self.disabled { Sense::hover() } else { Sense::click() };

        let mut resp = ui
            .horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 8.0;
                let (rect, box_resp) = ui.allocate_exact_size(Vec2::splat(box_edge), sense);
                let label_resp = if !self.label.is_empty() {
                    let color = if self.disabled { tokens.muted_foreground } else { tokens.foreground };
                    Some(ui.add(egui::Label::new(RichText::new(&self.label).size(self.size.font_size()).color(color)).sense(sense)))
                } else {
                    None
                };

                let clicked = box_resp.clicked() || label_resp.as_ref().is_some_and(|r| r.clicked());
                if clicked && !self.disabled {
                    *self.checked = !*self.checked;
                }

                let active = *self.checked || self.indeterminate;
                let active_t = ease_out_cubic(ui.ctx().animate_bool_with_time(box_resp.id.with("act"), active, 0.14));
                let hover_t = ui.ctx().animate_bool_with_time(
                    box_resp.id.with("hov"),
                    box_resp.hovered() && !self.disabled,
                    0.12,
                );

                if ui.is_rect_visible(rect) {
                    let target_fill = if self.disabled { tokens.primary.linear_multiply(0.5) } else { tokens.primary };
                    let fill = lerp_color(tokens.background, target_fill, active_t);
                    let rest_border = lerp_color(tokens.input, tokens.ring, hover_t);
                    let border_col = lerp_color(rest_border, target_fill, active_t);

                    let painter = ui.painter();
                    painter.rect(rect, Rounding::same(4.0), fill, Stroke::new(1.5, border_col));

                    if active_t > 0.02 {
                        let stroke = Stroke::new((box_edge * 0.13).clamp(1.6, 2.4), tokens.primary_foreground);
                        if self.indeterminate {
                            let half_w = (rect.width() * 0.5 - 3.5) * active_t;
                            let cx = rect.center().x;
                            let y = rect.center().y;
                            painter.line_segment([Pos2::new(cx - half_w, y), Pos2::new(cx + half_w, y)], stroke);
                        } else {
                            let c = rect.center();
                            let r = box_edge * 0.27;
                            let p0 = Pos2::new(c.x - r * 0.85, c.y + r * 0.05);
                            let p1 = Pos2::new(c.x - r * 0.2, c.y + r * 0.7);
                            let p2 = Pos2::new(c.x + r * 0.9, c.y - r * 0.6);

                            let leg1 = (active_t / 0.42).clamp(0.0, 1.0);
                            if leg1 > 0.0 {
                                painter.line_segment([p0, p0.lerp(p1, leg1)], stroke);
                            }
                            let leg2 = ((active_t - 0.42) / 0.58).clamp(0.0, 1.0);
                            if leg2 > 0.0 {
                                painter.line_segment([p1, p1.lerp(p2, leg2)], stroke);
                            }
                        }
                    }
                }

                let mut combined = box_resp;
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
