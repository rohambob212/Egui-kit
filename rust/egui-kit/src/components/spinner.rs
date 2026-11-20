//! Lightweight rotating vector arc Spinner widget.

use crate::theme::active_theme;
use crate::tokens::Size;
use egui::{Color32, Pos2, Response, RichText, Sense, Shape, Stroke, Ui, Vec2, Widget};
use std::f32::consts::TAU;

/// Rotating activity indicator scaled by [`Size`].
pub struct Spinner {
    size: Size,
    color: Option<Color32>,
    label: Option<String>,
}

impl Default for Spinner {
    fn default() -> Self {
        Self::new()
    }
}

impl Spinner {
    pub fn new() -> Self {
        Self {
            size: Size::Md,
            color: None,
            label: None,
        }
    }

    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub fn color(mut self, color: Color32) -> Self {
        self.color = Some(color);
        self
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }
}

impl Widget for Spinner {
    fn ui(self, ui: &mut Ui) -> Response {
        let tokens = active_theme(ui.ctx());
        let diameter = self.size.icon_size() + 4.0;
        let stroke_w = match self.size {
            Size::Xs => 1.75,
            Size::Sm => 2.0,
            Size::Md => 2.25,
            Size::Lg => 2.75,
        };
        let color = self.color.unwrap_or(tokens.primary);

        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 8.0;
            let (rect, response) = ui.allocate_exact_size(Vec2::splat(diameter), Sense::hover());

            if ui.is_rect_visible(rect) {
                ui.ctx().request_repaint();
                let time = ui.input(|i| i.time) as f32;
                let center = rect.center();
                let radius = diameter * 0.42;

                ui.painter().circle_stroke(center, radius, Stroke::new(stroke_w, tokens.muted));

                let start_angle = time * 5.0;
                let breath = 0.5 + 0.5 * (time * 2.6).sin();
                let span = TAU * (0.22 + 0.48 * breath);
                let steps = 18;
                let points: Vec<Pos2> = (0..=steps)
                    .map(|i| {
                        let a = start_angle + span * (i as f32 / steps as f32);
                        center + Vec2::angled(a) * radius
                    })
                    .collect();
                ui.painter().add(Shape::line(points, Stroke::new(stroke_w, color)));
            }

            if let Some(label) = &self.label {
                ui.label(
                    RichText::new(label)
                        .size(self.size.font_size())
                        .color(tokens.muted_foreground),
                );
            }

            response
        })
        .inner
    }
}
