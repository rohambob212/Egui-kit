//! Determinate and indeterminate Progress bar widget.

use crate::theme::active_theme;
use crate::tokens::{ease_out_cubic, Size, Variant};
use egui::{Pos2, Rect, Response, RichText, Rounding, Sense, Ui, Vec2, Widget};

/// Linear progress indicator (`0.0..=1.0` or indeterminate sweep).
pub struct Progress {
    value: Option<f32>,
    show_label: bool,
    tone: Variant,
    size: Size,
}

impl Progress {
    /// Creates a determinate progress bar clamped to `0.0..=1.0`.
    pub fn new(value: f32) -> Self {
        Self {
            value: Some(value.clamp(0.0, 1.0)),
            show_label: false,
            tone: Variant::Primary,
            size: Size::Md,
        }
    }

    /// Creates an animated indeterminate progress bar.
    pub fn indeterminate() -> Self {
        Self {
            value: None,
            show_label: false,
            tone: Variant::Primary,
            size: Size::Md,
        }
    }

    pub fn show_label(mut self, show: bool) -> Self {
        self.show_label = show;
        self
    }

    pub fn tone(mut self, tone: Variant) -> Self {
        self.tone = tone;
        self
    }

    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }
}

impl Widget for Progress {
    fn ui(self, ui: &mut Ui) -> Response {
        let tokens = active_theme(ui.ctx());
        let height = match self.size {
            Size::Xs => 4.0,
            Size::Sm => 6.0,
            Size::Md => 8.0,
            Size::Lg => 12.0,
        };
        let fill_color = match self.tone {
            Variant::Success => tokens.success,
            Variant::Warning => tokens.warning,
            Variant::Destructive => tokens.destructive,
            Variant::Secondary => tokens.secondary_foreground,
            _ => tokens.primary,
        };

        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 10.0;
            let reserved_label = if self.show_label && self.value.is_some() { 42.0 } else { 0.0 };
            let width = (ui.available_width() - reserved_label).clamp(100.0, 340.0);
            let (rect, response) = ui.allocate_exact_size(Vec2::new(width, height), Sense::hover());

            if ui.is_rect_visible(rect) {
                let painter = ui.painter();
                let rounding = Rounding::same(height * 0.5);
                painter.rect_filled(rect, rounding, tokens.muted);

                match self.value {
                    Some(v) => {
                        let smooth_v = ui.ctx().animate_value_with_time(response.id.with("p"), v, 0.2);
                        let fill_w = rect.width() * smooth_v.clamp(0.0, 1.0);
                        if fill_w > 0.5 {
                            let fill_rect = Rect::from_min_size(rect.min, Vec2::new(fill_w, height));
                            painter.rect_filled(fill_rect, rounding, fill_color);
                        }
                    }
                    None => {
                        ui.ctx().request_repaint();
                        let t = ease_out_cubic((ui.input(|i| i.time) as f32 * 0.85).fract());
                        let seg_w = rect.width() * 0.32;
                        let start_x = egui::lerp((rect.left() - seg_w)..=rect.right(), t);
                        let raw = Rect::from_min_size(Pos2::new(start_x, rect.top()), Vec2::new(seg_w, height));
                        let clipped = raw.intersect(rect);
                        if clipped.width() > 0.5 {
                            painter.rect_filled(clipped, rounding, fill_color);
                        }
                    }
                }
            }

            if self.show_label {
                if let Some(v) = self.value {
                    ui.label(
                        RichText::new(format!("{:.0}%", v * 100.0))
                            .size(self.size.font_size() - 1.0)
                            .monospace()
                            .color(tokens.muted_foreground),
                    );
                }
            }

            response
        })
        .inner
    }
}
