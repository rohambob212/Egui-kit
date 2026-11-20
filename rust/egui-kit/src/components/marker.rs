//! Spatial pin, callout, and timeline Marker badge widget.

use crate::theme::active_theme;
use crate::tokens::{lerp_color, Variant};
use egui::{FontId, Pos2, Rect, Response, Rounding, Sense, Shape, Stroke, Ui, Vec2, Widget};

/// Visual shape style for [`Marker`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum MarkerStyle {
    #[default]
    Pin,
    Callout,
    Timeline,
}

/// Annotation marker pin or timeline node aligned to an anchor point.
pub struct Marker {
    label: String,
    tone: Variant,
    style: MarkerStyle,
    pulsing: bool,
}

impl Marker {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            tone: Variant::Primary,
            style: MarkerStyle::Pin,
            pulsing: false,
        }
    }

    pub fn tone(mut self, tone: Variant) -> Self {
        self.tone = tone;
        self
    }

    pub fn style(mut self, style: MarkerStyle) -> Self {
        self.style = style;
        self
    }

    pub fn pulsing(mut self, pulsing: bool) -> Self {
        self.pulsing = pulsing;
        self
    }
}

impl Widget for Marker {
    fn ui(self, ui: &mut Ui) -> Response {
        let tokens = active_theme(ui.ctx());
        let (fill, fg) = match self.tone {
            Variant::Success => (tokens.success, tokens.primary_foreground),
            Variant::Warning => (tokens.warning, tokens.primary_foreground),
            Variant::Destructive => (tokens.destructive, tokens.destructive_foreground),
            Variant::Secondary => (tokens.secondary, tokens.secondary_foreground),
            _ => (tokens.primary, tokens.primary_foreground),
        };

        let font = FontId::monospace(11.5);
        let galley = ui.painter().layout_no_wrap(self.label, font, fg);
        let pad_x = 10.0;
        let pill_w = (galley.size().x + pad_x * 2.0).max(32.0);

        match self.style {
            MarkerStyle::Pin => {
                let (rect, response) = ui.allocate_exact_size(Vec2::new(pill_w.max(40.0), 34.0), Sense::click());
                if ui.is_rect_visible(rect) {
                    let hov_t = ui.ctx().animate_bool_with_time(response.id.with("h"), response.hovered(), 0.12);
                    let pill = Rect::from_min_size(Pos2::new(rect.center().x - pill_w * 0.5, rect.top()), Vec2::new(pill_w, 24.0));
                    let color = lerp_color(fill, tokens.foreground, hov_t * 0.12);
                    ui.painter().rect_filled(pill, Rounding::same(12.0), color);
                    // Pointer tip
                    let tip = vec![
                        Pos2::new(pill.center().x - 5.0, pill.bottom() - 1.0),
                        Pos2::new(pill.center().x + 5.0, pill.bottom() - 1.0),
                        Pos2::new(pill.center().x, rect.bottom()),
                    ];
                    ui.painter().add(Shape::convex_polygon(tip, color, Stroke::NONE));
                    ui.painter().galley(
                        Pos2::new(pill.center().x - galley.size().x * 0.5, pill.center().y - galley.size().y * 0.5),
                        galley,
                        fg,
                    );
                }
                response
            }
            MarkerStyle::Callout => {
                let (rect, response) = ui.allocate_exact_size(Vec2::new(pill_w + 14.0, 26.0), Sense::click());
                if ui.is_rect_visible(rect) {
                    let anchor = Pos2::new(rect.left() + 5.0, rect.center().y);
                    if self.pulsing {
                        ui.ctx().request_repaint();
                        let pulse = ((ui.input(|i| i.time) as f32 * 1.4).sin() * 0.5 + 0.5).clamp(0.0, 1.0);
                        ui.painter().circle_stroke(anchor, 5.0 + 4.0 * pulse, Stroke::new(1.2, fill));
                    }
                    ui.painter().circle_filled(anchor, 4.0, fill);
                    let pill = Rect::from_min_size(Pos2::new(rect.left() + 14.0, rect.top() + 1.0), Vec2::new(pill_w, 24.0));
                    ui.painter().rect(pill, Rounding::same(6.0), tokens.card, Stroke::new(1.0, fill));
                    ui.painter().galley(
                        Pos2::new(pill.center().x - galley.size().x * 0.5, pill.center().y - galley.size().y * 0.5),
                        galley,
                        tokens.foreground,
                    );
                }
                response
            }
            MarkerStyle::Timeline => {
                let (rect, response) = ui.allocate_exact_size(Vec2::new(pill_w + 20.0, 26.0), Sense::click());
                if ui.is_rect_visible(rect) {
                    let dot = Pos2::new(rect.left() + 8.0, rect.center().y);
                    ui.painter().line_segment(
                        [Pos2::new(dot.x, rect.top()), Pos2::new(dot.x, rect.bottom())],
                        Stroke::new(2.0, tokens.border),
                    );
                    ui.painter().circle_filled(dot, 5.0, fill);
                    ui.painter().circle_stroke(dot, 5.0, Stroke::new(1.5, tokens.background));
                    ui.painter().galley(
                        Pos2::new(rect.left() + 20.0, rect.center().y - galley.size().y * 0.5),
                        galley,
                        tokens.foreground,
                    );
                }
                response
            }
        }
    }
}
