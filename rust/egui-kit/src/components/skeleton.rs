//! Pulsing layout-preserving Skeleton placeholder widget.

use crate::theme::active_theme;
use crate::tokens::lerp_color;
use egui::{Response, Rounding, Sense, Ui, Vec2, Widget};

enum SkeletonKind {
    Rect { width: f32, height: f32, rounding: f32 },
    Circle { diameter: f32 },
    Lines { count: usize, line_height: f32 },
}

/// Pulsing loading placeholder for text blocks, cards, and avatars.
pub struct Skeleton {
    kind: SkeletonKind,
    animated: bool,
}

impl Skeleton {
    pub fn rect(width: f32, height: f32) -> Self {
        Self {
            kind: SkeletonKind::Rect {
                width,
                height,
                rounding: 6.0,
            },
            animated: true,
        }
    }

    pub fn circle(diameter: f32) -> Self {
        Self {
            kind: SkeletonKind::Circle { diameter },
            animated: true,
        }
    }

    pub fn lines(count: usize) -> Self {
        Self {
            kind: SkeletonKind::Lines {
                count: count.max(1),
                line_height: 12.0,
            },
            animated: true,
        }
    }

    pub fn rounding(mut self, rounding: f32) -> Self {
        if let SkeletonKind::Rect { rounding: r, .. } = &mut self.kind {
            *r = rounding;
        }
        self
    }

    pub fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }
}

impl Widget for Skeleton {
    fn ui(self, ui: &mut Ui) -> Response {
        let tokens = active_theme(ui.ctx());
        let pulse = if self.animated {
            ui.ctx().request_repaint();
            let t = ui.input(|i| i.time) as f32;
            0.5 + 0.5 * (t * 2.8).sin()
        } else {
            0.0
        };
        let fill = lerp_color(tokens.muted, tokens.accent, pulse * 0.65);

        match self.kind {
            SkeletonKind::Rect {
                width,
                height,
                rounding,
            } => {
                let (rect, response) = ui.allocate_exact_size(Vec2::new(width, height), Sense::hover());
                if ui.is_rect_visible(rect) {
                    ui.painter().rect_filled(rect, Rounding::same(rounding), fill);
                }
                response
            }
            SkeletonKind::Circle { diameter } => {
                let (rect, response) = ui.allocate_exact_size(Vec2::splat(diameter), Sense::hover());
                if ui.is_rect_visible(rect) {
                    ui.painter().circle_filled(rect.center(), diameter * 0.5, fill);
                }
                response
            }
            SkeletonKind::Lines { count, line_height } => {
                let gap = 8.0;
                let max_w = ui.available_width().clamp(120.0, 320.0);
                let total_h = (count as f32) * line_height + (count.saturating_sub(1) as f32) * gap;
                let (rect, response) = ui.allocate_exact_size(Vec2::new(max_w, total_h), Sense::hover());
                if ui.is_rect_visible(rect) {
                    for i in 0..count {
                        let w_factor = if i + 1 == count && count > 1 { 0.68 } else { 1.0 };
                        let y = rect.top() + (i as f32) * (line_height + gap);
                        let line_rect = egui::Rect::from_min_size(
                            egui::Pos2::new(rect.left(), y),
                            Vec2::new(rect.width() * w_factor, line_height),
                        );
                        ui.painter().rect_filled(line_rect, Rounding::same(4.0), fill);
                    }
                }
                response
            }
        }
    }
}
