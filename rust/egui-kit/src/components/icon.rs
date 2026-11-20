//! Vector painter-based Icon widget with crisp geometric paths at any DPI scale.

use crate::theme::active_theme;
use crate::tokens::Size;
use egui::{Color32, Painter, Pos2, Rect, Response, Sense, Stroke, Ui, Vec2, Widget};

/// Built-in crisp vector icons painted directly on `egui::Painter`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IconName {
    Check,
    Close,
    ChevronDown,
    ChevronRight,
    Plus,
    Minus,
    Search,
    Settings,
    Copy,
    Sparkles,
    Sun,
    Moon,
    AlertTriangle,
    Info,
    User,
    Terminal,
}

/// Vector icon widget rendered cleanly via `egui::Painter`.
pub struct Icon {
    name: IconName,
    size: Size,
    color: Option<Color32>,
}

impl Icon {
    pub fn new(name: IconName) -> Self {
        Self {
            name,
            size: Size::Md,
            color: None,
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
}

impl Widget for Icon {
    fn ui(self, ui: &mut Ui) -> Response {
        let tokens = active_theme(ui.ctx());
        let edge = self.size.icon_size();
        let (rect, response) = ui.allocate_exact_size(Vec2::splat(edge), Sense::hover());
        if ui.is_rect_visible(rect) {
            paint_icon(
                ui.painter(),
                rect,
                self.name,
                self.color.unwrap_or(tokens.foreground),
            );
        }
        response
    }
}

/// Paints a vector [`IconName`] inside `rect` using crisp normalized geometry.
pub fn paint_icon(painter: &Painter, rect: Rect, name: IconName, color: Color32) {
    let c = rect.center();
    let r = rect.width().min(rect.height()) * 0.38;
    let stroke = Stroke::new((rect.width() * 0.1).clamp(1.25, 2.2), color);

    match name {
        IconName::Check => {
            painter.line_segment(
                [Pos2::new(c.x - r * 0.85, c.y + r * 0.05), Pos2::new(c.x - r * 0.2, c.y + r * 0.7)],
                stroke,
            );
            painter.line_segment(
                [Pos2::new(c.x - r * 0.2, c.y + r * 0.7), Pos2::new(c.x + r * 0.9, c.y - r * 0.6)],
                stroke,
            );
        }
        IconName::Close => {
            painter.line_segment([c + Vec2::new(-r, -r), c + Vec2::new(r, r)], stroke);
            painter.line_segment([c + Vec2::new(r, -r), c + Vec2::new(-r, r)], stroke);
        }
        IconName::ChevronDown => {
            painter.line_segment(
                [c + Vec2::new(-r * 0.85, -r * 0.3), c + Vec2::new(0.0, r * 0.45)],
                stroke,
            );
            painter.line_segment(
                [c + Vec2::new(0.0, r * 0.45), c + Vec2::new(r * 0.85, -r * 0.3)],
                stroke,
            );
        }
        IconName::ChevronRight => {
            painter.line_segment(
                [c + Vec2::new(-r * 0.3, -r * 0.85), c + Vec2::new(r * 0.45, 0.0)],
                stroke,
            );
            painter.line_segment(
                [c + Vec2::new(r * 0.45, 0.0), c + Vec2::new(-r * 0.3, r * 0.85)],
                stroke,
            );
        }
        IconName::Plus => {
            painter.line_segment([c + Vec2::new(-r, 0.0), c + Vec2::new(r, 0.0)], stroke);
            painter.line_segment([c + Vec2::new(0.0, -r), c + Vec2::new(0.0, r)], stroke);
        }
        IconName::Minus => {
            painter.line_segment([c + Vec2::new(-r, 0.0), c + Vec2::new(r, 0.0)], stroke);
        }
        IconName::Search => {
            let lens = c + Vec2::new(-r * 0.2, -r * 0.2);
            painter.circle_stroke(lens, r * 0.65, stroke);
            painter.line_segment([lens + Vec2::splat(r * 0.48), c + Vec2::splat(r)], stroke);
        }
        IconName::Settings | IconName::Sun => {
            painter.circle_stroke(c, r * 0.48, stroke);
            for i in 0..6 {
                let a = (i as f32) * std::f32::consts::TAU / 6.0;
                let dir = Vec2::angled(a);
                painter.line_segment([c + dir * (r * 0.72), c + dir * r], stroke);
            }
        }
        IconName::Copy => {
            let back = Rect::from_min_max(c + Vec2::new(-r, -r), c + Vec2::new(r * 0.4, r * 0.4));
            let front = Rect::from_min_max(c + Vec2::new(-r * 0.35, -r * 0.35), c + Vec2::new(r, r));
            painter.rect_stroke(back, 2.0, stroke);
            painter.rect_stroke(front, 2.0, stroke);
        }
        IconName::Sparkles | IconName::Terminal => {
            painter.line_segment(
                [c + Vec2::new(-r * 0.85, -r * 0.5), c + Vec2::new(-r * 0.1, 0.0)],
                stroke,
            );
            painter.line_segment(
                [c + Vec2::new(-r * 0.1, 0.0), c + Vec2::new(-r * 0.85, r * 0.5)],
                stroke,
            );
            painter.line_segment(
                [c + Vec2::new(r * 0.1, r * 0.5), c + Vec2::new(r * 0.9, r * 0.5)],
                stroke,
            );
        }
        IconName::Moon => {
            painter.circle_stroke(c, r * 0.8, stroke);
        }
        IconName::AlertTriangle => {
            let top = c + Vec2::new(0.0, -r * 0.9);
            let bl = c + Vec2::new(-r * 0.9, r * 0.8);
            let br = c + Vec2::new(r * 0.9, r * 0.8);
            painter.line_segment([top, bl], stroke);
            painter.line_segment([bl, br], stroke);
            painter.line_segment([br, top], stroke);
        }
        IconName::Info => {
            painter.circle_stroke(c, r * 0.85, stroke);
            painter.line_segment([c + Vec2::new(0.0, -r * 0.1), c + Vec2::new(0.0, r * 0.45)], stroke);
            painter.circle_filled(c + Vec2::new(0.0, -r * 0.42), stroke.width * 0.65, color);
        }
        IconName::User => {
            painter.circle_stroke(c + Vec2::new(0.0, -r * 0.35), r * 0.4, stroke);
            painter.line_segment(
                [c + Vec2::new(-r * 0.75, r * 0.8), c + Vec2::new(r * 0.75, r * 0.8)],
                stroke,
            );
        }
    }
}
