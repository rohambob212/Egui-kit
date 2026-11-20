//! Responsive Image frame with aspect-ratio sizing, shimmer loading state, and error fallback.

use crate::components::shimmer::Shimmer;
use crate::theme::active_theme;
use crate::tokens::lerp_color;
use egui::{Color32, FontId, Pos2, Rect, Response, Rounding, Sense, Stroke, TextureId, Ui, Vec2, Widget};

/// Framed Image container wrapping an optional `TextureId` with loading and fallback states.
pub struct Image {
    texture: Option<TextureId>,
    width: f32,
    aspect_ratio: f32,
    rounding: f32,
    loading: bool,
    failed: bool,
    caption: Option<String>,
}

impl Image {
    pub fn new(texture: Option<TextureId>) -> Self {
        Self {
            texture,
            width: 300.0,
            aspect_ratio: 16.0 / 9.0,
            rounding: 8.0,
            loading: false,
            failed: false,
            caption: None,
        }
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width.max(80.0);
        self
    }

    pub fn aspect_ratio(mut self, ratio: f32) -> Self {
        self.aspect_ratio = ratio.max(0.25);
        self
    }

    pub fn rounding(mut self, rounding: f32) -> Self {
        self.rounding = rounding.max(0.0);
        self
    }

    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    pub fn failed(mut self, failed: bool) -> Self {
        self.failed = failed;
        self
    }

    pub fn caption(mut self, caption: impl Into<String>) -> Self {
        self.caption = Some(caption.into());
        self
    }
}

impl Widget for Image {
    fn ui(self, ui: &mut Ui) -> Response {
        let tokens = active_theme(ui.ctx());
        let height = (self.width / self.aspect_ratio).round();

        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing.y = 6.0;
            let resp = if self.loading {
                ui.add(Shimmer::rect(self.width, height).rounding(self.rounding))
            } else {
                let (rect, response) = ui.allocate_exact_size(Vec2::new(self.width, height), Sense::hover());
                if ui.is_rect_visible(rect) {
                    let r = Rounding::same(self.rounding);
                    if !self.failed {
                        if let Some(tex) = self.texture {
                            ui.painter().image(
                                tex,
                                rect,
                                Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
                                Color32::WHITE,
                            );
                            ui.painter().rect_stroke(rect, r, Stroke::new(1.0, tokens.border));
                        } else {
                            paint_placeholder_frame(ui, rect, r, false, &tokens);
                        }
                    } else {
                        paint_placeholder_frame(ui, rect, r, true, &tokens);
                    }
                }
                response
            };

            if let Some(cap) = self.caption {
                ui.painter().text(
                    ui.cursor().min + Vec2::new(0.0, 6.0),
                    egui::Align2::LEFT_CENTER,
                    cap,
                    FontId::proportional(11.5),
                    tokens.muted_foreground,
                );
                ui.allocate_exact_size(Vec2::new(self.width, 14.0), Sense::hover());
            }

            resp
        })
        .inner
    }
}

fn paint_placeholder_frame(
    ui: &mut Ui,
    rect: Rect,
    rounding: Rounding,
    failed: bool,
    tokens: &crate::theme::ThemeTokens,
) {
    let bg = lerp_color(tokens.muted, tokens.card, 0.35);
    ui.painter().rect(rect, rounding, bg, Stroke::new(1.0, tokens.border));

    // Geometric landscape / broken-photo vector mark
    let c = rect.center() - Vec2::new(0.0, 8.0);
    let accent = if failed { tokens.destructive } else { tokens.primary };
    let stroke = Stroke::new(1.5, accent);
    let frame = Rect::from_center_size(c, Vec2::new(36.0, 26.0));
    ui.painter().rect_stroke(frame, Rounding::same(4.0), stroke);
    ui.painter().line_segment(
        [
            Pos2::new(frame.left() + 5.0, frame.bottom() - 5.0),
            Pos2::new(frame.center().x - 3.0, frame.center().y + 1.0),
        ],
        stroke,
    );
    ui.painter().line_segment(
        [
            Pos2::new(frame.center().x - 3.0, frame.center().y + 1.0),
            Pos2::new(frame.right() - 6.0, frame.bottom() - 5.0),
        ],
        stroke,
    );
    ui.painter().circle_filled(Pos2::new(frame.right() - 9.0, frame.top() + 8.0), 2.5, accent);

    let msg = if failed { "Image failed to load" } else { "Texture slot ready" };
    ui.painter().text(
        rect.center() + Vec2::new(0.0, 16.0),
        egui::Align2::CENTER_CENTER,
        msg,
        FontId::proportional(12.0),
        tokens.muted_foreground,
    );
}
