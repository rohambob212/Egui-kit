//! Multi-step workflow Stepper indicator with completed, active, and error states.

use crate::components::icon::{paint_icon, IconName};
use crate::theme::active_theme;
use crate::tokens::{ease_out_cubic, lerp_color, Orientation};
use egui::{FontId, Id, Pos2, Rect, Response, Sense, Stroke, Ui, Vec2};

/// Horizontal or vertical multi-step progress indicator.
pub struct Stepper<'a, T: AsRef<str>> {
    steps: &'a [T],
    active_step: usize,
    error_step: Option<usize>,
    orientation: Orientation,
}

impl<'a, T: AsRef<str>> Stepper<'a, T> {
    pub fn new(steps: &'a [T], active_step: usize) -> Self {
        Self {
            steps,
            active_step,
            error_step: None,
            orientation: Orientation::Horizontal,
        }
    }

    pub fn error_step(mut self, step: Option<usize>) -> Self {
        self.error_step = step;
        self
    }

    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Renders read-only stepper progress.
    pub fn show(self, ui: &mut Ui) -> Response {
        self.render(ui, None)
    }

    /// Renders interactive clickable steps mutating `&mut usize` directly.
    pub fn show_clickable(self, ui: &mut Ui, current: &mut usize) -> Response {
        self.render(ui, Some(current))
    }

    fn render(self, ui: &mut Ui, mut clickable: Option<&mut usize>) -> Response {
        let tokens = active_theme(ui.ctx());
        let count = self.steps.len();
        if count == 0 {
            return ui.allocate_exact_size(Vec2::ZERO, Sense::hover()).1;
        }
        let active = clickable.as_deref().copied().unwrap_or(self.active_step).min(count - 1);

        match self.orientation {
            Orientation::Horizontal => {
                let avail_w = ui.available_width().clamp(260.0, 620.0);
                let (rect, mut resp) = ui.allocate_exact_size(Vec2::new(avail_w, 54.0), Sense::hover());
                let step_span = avail_w / (count as f32);

                for idx in 0..count {
                    let cx = rect.left() + step_span * (idx as f32 + 0.5);
                    let cy = rect.top() + 14.0;
                    let center = Pos2::new(cx, cy);

                    if idx + 1 < count {
                        let next_cx = rect.left() + step_span * ((idx + 1) as f32 + 0.5);
                        let done = idx < active;
                        let conn_t = ease_out_cubic(ui.ctx().animate_bool_with_time(
                            Id::new(("egui_kit_step_conn", idx)),
                            done,
                            0.2,
                        ));
                        let line_col = lerp_color(tokens.border, tokens.primary, conn_t);
                        ui.painter().line_segment(
                            [Pos2::new(cx + 16.0, cy), Pos2::new(next_cx - 16.0, cy)],
                            Stroke::new(2.0, line_col),
                        );
                    }

                    let is_err = self.error_step == Some(idx);
                    let is_done = idx < active && !is_err;
                    let is_curr = idx == active && !is_err;

                    let node_rect = Rect::from_center_size(center, Vec2::splat(26.0));
                    if let Some(ref mut cur_mut) = clickable {
                        let node_resp = ui.interact(node_rect, resp.id.with(("step", idx)), Sense::click());
                        if node_resp.clicked() {
                            **cur_mut = idx;
                            resp.mark_changed();
                        }
                    }

                    let (fill, fg, border_col) = if is_err {
                        (tokens.destructive, tokens.primary_foreground, tokens.destructive)
                    } else if is_done || is_curr {
                        (tokens.primary, tokens.primary_foreground, tokens.primary)
                    } else {
                        (tokens.muted, tokens.muted_foreground, tokens.border)
                    };

                    ui.painter().circle(center, 12.0, fill, Stroke::new(1.5, border_col));
                    if is_err {
                        paint_icon(ui.painter(), Rect::from_center_size(center, Vec2::splat(13.0)), IconName::AlertTriangle, fg);
                    } else if is_done {
                        paint_icon(ui.painter(), Rect::from_center_size(center, Vec2::splat(13.0)), IconName::Check, fg);
                    } else {
                        ui.painter().text(
                            center,
                            egui::Align2::CENTER_CENTER,
                            format!("{}", idx + 1),
                            FontId::monospace(11.5),
                            fg,
                        );
                    }

                    ui.painter().text(
                        Pos2::new(cx, rect.top() + 38.0),
                        egui::Align2::CENTER_CENTER,
                        self.steps[idx].as_ref(),
                        FontId::proportional(12.0),
                        if is_curr {
                            tokens.foreground
                        } else if is_err {
                            tokens.destructive
                        } else {
                            tokens.muted_foreground
                        },
                    );
                }
                resp
            }
            Orientation::Vertical => {
                let mut changed = false;
                let mut outer = ui
                    .vertical(|ui| {
                        ui.spacing_mut().item_spacing.y = 10.0;
                        for (idx, label) in self.steps.iter().enumerate() {
                            let is_err = self.error_step == Some(idx);
                            let is_done = idx < active && !is_err;
                            let is_curr = idx == active && !is_err;
                            let (row_rect, row_resp) = ui.allocate_exact_size(
                                Vec2::new(220.0, 28.0),
                                if clickable.is_some() { Sense::click() } else { Sense::hover() },
                            );
                            if row_resp.clicked() {
                                if let Some(ref mut cur_mut) = clickable {
                                    **cur_mut = idx;
                                    changed = true;
                                }
                            }
                            let center = Pos2::new(row_rect.left() + 14.0, row_rect.center().y);
                            let (fill, fg, border_col) = if is_err {
                                (tokens.destructive, tokens.primary_foreground, tokens.destructive)
                            } else if is_done || is_curr {
                                (tokens.primary, tokens.primary_foreground, tokens.primary)
                            } else {
                                (tokens.muted, tokens.muted_foreground, tokens.border)
                            };
                            ui.painter().circle(center, 11.0, fill, Stroke::new(1.5, border_col));
                            if is_done {
                                paint_icon(ui.painter(), Rect::from_center_size(center, Vec2::splat(12.0)), IconName::Check, fg);
                            } else {
                                ui.painter().text(center, egui::Align2::CENTER_CENTER, format!("{}", idx + 1), FontId::monospace(11.0), fg);
                            }
                            ui.painter().text(
                                Pos2::new(row_rect.left() + 34.0, row_rect.center().y),
                                egui::Align2::LEFT_CENTER,
                                label.as_ref(),
                                FontId::proportional(13.0),
                                if is_curr { tokens.foreground } else { tokens.muted_foreground },
                            );
                        }
                    })
                    .response;
                if changed {
                    outer.mark_changed();
                }
                outer
            }
        }
    }
}
