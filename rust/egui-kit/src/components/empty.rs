//! Centered Empty state placeholder with vector emblem and optional primary action.

use crate::components::button::{Button, ButtonVariant};
use crate::theme::active_theme;
use crate::tokens::lerp_color;
use egui::{Align2, FontId, Pos2, Rect, Response, Rounding, Sense, Stroke, Ui, Vec2, Widget};

/// Return value from rendering an [`Empty`] zero-state card.
pub struct EmptyResponse {
    pub response: Response,
    pub action_clicked: bool,
}

/// Zero-state placeholder widget with subtle illustration, title, description, and action CTA.
pub struct Empty {
    title: String,
    description: Option<String>,
    action_label: Option<String>,
    width: f32,
}

impl Empty {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: None,
            action_label: None,
            width: 360.0,
        }
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn action(mut self, label: impl Into<String>) -> Self {
        self.action_label = Some(label.into());
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width.max(220.0);
        self
    }

    /// Shows the empty state and returns whether the optional action button was clicked.
    pub fn show(self, ui: &mut Ui) -> EmptyResponse {
        let tokens = active_theme(ui.ctx());
        let mut action_clicked = false;

        let inner = ui.allocate_ui(Vec2::new(self.width, 0.0), |ui| {
            let frame_resp = egui::Frame::none()
                .fill(tokens.card)
                .stroke(Stroke::new(1.0, tokens.border))
                .rounding(Rounding::same(10.0))
                .inner_margin(egui::Margin::symmetric(24.0, 26.0))
                .show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        // Painter emblem (clean tray / box outline)
                        let (emblem_rect, _) = ui.allocate_exact_size(Vec2::splat(48.0), Sense::hover());
                        if ui.is_rect_visible(emblem_rect) {
                            let bg = lerp_color(tokens.muted, tokens.primary, 0.08);
                            ui.painter().circle_filled(emblem_rect.center(), 24.0, bg);
                            let c = emblem_rect.center();
                            let box_rect = Rect::from_center_size(c, Vec2::new(22.0, 16.0));
                            let stroke = Stroke::new(1.5, tokens.muted_foreground);
                            ui.painter().rect_stroke(box_rect, Rounding::same(3.0), stroke);
                            ui.painter().line_segment(
                                [
                                    Pos2::new(box_rect.left() + 2.0, c.y + 1.0),
                                    Pos2::new(c.x - 4.0, c.y + 1.0),
                                ],
                                stroke,
                            );
                            ui.painter().line_segment(
                                [
                                    Pos2::new(c.x + 4.0, c.y + 1.0),
                                    Pos2::new(box_rect.right() - 2.0, c.y + 1.0),
                                ],
                                stroke,
                            );
                        }

                        ui.add_space(10.0);
                        ui.painter().text(
                            ui.cursor().min + Vec2::new(ui.available_width() * 0.5, 9.0),
                            Align2::CENTER_CENTER,
                            &self.title,
                            FontId::proportional(15.0),
                            tokens.foreground,
                        );
                        ui.allocate_exact_size(Vec2::new(ui.available_width(), 20.0), Sense::hover());

                        if let Some(ref desc) = self.description {
                            ui.add_space(4.0);
                            let galley = ui.painter().layout(
                                desc.clone(),
                                FontId::proportional(12.5),
                                tokens.muted_foreground,
                                (self.width - 48.0).max(120.0),
                            );
                            let h = galley.size().y;
                            let (r, _) = ui.allocate_exact_size(Vec2::new(galley.size().x, h), Sense::hover());
                            ui.painter().galley(r.min, galley, tokens.muted_foreground);
                        }

                        if let Some(ref label) = self.action_label {
                            ui.add_space(14.0);
                            if ui.add(Button::new(label).variant(ButtonVariant::Primary)).clicked() {
                                action_clicked = true;
                            }
                        }
                    });
                });
            frame_resp.response
        });

        EmptyResponse {
            response: inner.inner,
            action_clicked,
        }
    }
}

impl Widget for Empty {
    fn ui(self, ui: &mut Ui) -> Response {
        self.show(ui).response
    }
}
