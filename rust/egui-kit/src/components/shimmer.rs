//! Diagonal Shimmer sweep widget composable over placeholder blocks.

use crate::theme::active_theme;
use egui::{Color32, Pos2, Response, Rounding, Sense, Shape, Stroke, Ui, Vec2, Widget};

/// Standalone shimmer block or wrapper painting an animated highlight sweep.
pub struct Shimmer {
    width: f32,
    height: f32,
    rounding: f32,
    speed: f32,
}

impl Shimmer {
    pub fn rect(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
            rounding: 6.0,
            speed: 0.75,
        }
    }

    pub fn rounding(mut self, rounding: f32) -> Self {
        self.rounding = rounding;
        self
    }

    pub fn speed(mut self, speed: f32) -> Self {
        self.speed = speed.max(0.1);
        self
    }

    /// Wraps arbitrary child placeholder content with a clipped shimmer sweep overlay.
    pub fn wrap<R>(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> R {
        let out = ui.scope(add_contents);
        paint_sweep(ui, out.response.rect, 0.75);
        out.inner
    }
}

impl Widget for Shimmer {
    fn ui(self, ui: &mut Ui) -> Response {
        let tokens = active_theme(ui.ctx());
        let (rect, response) = ui.allocate_exact_size(Vec2::new(self.width, self.height), Sense::hover());
        if ui.is_rect_visible(rect) {
            ui.painter().rect_filled(rect, Rounding::same(self.rounding), tokens.muted);
            paint_sweep(ui, rect, self.speed);
        }
        response
    }
}

fn paint_sweep(ui: &mut Ui, rect: egui::Rect, speed: f32) {
    ui.ctx().request_repaint();
    let tokens = active_theme(ui.ctx());
    let raw_t = ((ui.input(|i| i.time) as f32) * speed).fract();
    let t = crate::tokens::ease_out_cubic(raw_t);
    let band_w = (rect.width() * 0.36).max(40.0);
    let core_w = band_w * 0.42;
    let slant = rect.height() * 0.45;
    let x = egui::lerp((rect.left() - band_w - slant)..=(rect.right() + band_w), t);

    let outer_alpha = if tokens.mode == crate::theme::ThemeMode::Dark { 18 } else { 55 };
    let core_alpha = if tokens.mode == crate::theme::ThemeMode::Dark { 34 } else { 110 };

    let outer = Color32::from_rgba_unmultiplied(tokens.card.r(), tokens.card.g(), tokens.card.b(), outer_alpha);
    let core = Color32::from_rgba_unmultiplied(tokens.card.r(), tokens.card.g(), tokens.card.b(), core_alpha);

    let painter = ui.painter().with_clip_rect(rect);
    let outer_pts = vec![
        Pos2::new(x, rect.bottom()),
        Pos2::new(x + slant, rect.top()),
        Pos2::new(x + slant + band_w, rect.top()),
        Pos2::new(x + band_w, rect.bottom()),
    ];
    painter.add(Shape::convex_polygon(outer_pts, outer, Stroke::NONE));

    let cx = x + (band_w - core_w) * 0.5;
    let core_pts = vec![
        Pos2::new(cx, rect.bottom()),
        Pos2::new(cx + slant, rect.top()),
        Pos2::new(cx + slant + core_w, rect.top()),
        Pos2::new(cx + core_w, rect.bottom()),
    ];
    painter.add(Shape::convex_polygon(core_pts, core, Stroke::NONE));
}
