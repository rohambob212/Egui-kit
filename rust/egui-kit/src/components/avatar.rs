//! Avatar and AvatarGroup widgets with initials fallback, shape, and presence status.

use crate::theme::active_theme;
use crate::tokens::{lerp_color, Size};
use egui::{Align2, Color32, FontId, Pos2, Response, Rounding, Sense, Stroke, Ui, Vec2, Widget};

/// Geometry shape of an [`Avatar`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AvatarShape {
    #[default]
    Circle,
    RoundedSquare,
}

/// Optional user presence indicator rendered on the avatar corner.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AvatarStatus {
    Online,
    Away,
    Busy,
    Offline,
}

/// User identity avatar widget with deterministic initials tint and optional presence ring.
pub struct Avatar {
    name: String,
    initials: String,
    size: Size,
    shape: AvatarShape,
    status: Option<AvatarStatus>,
    accent_color: Option<Color32>,
}

impl Avatar {
    pub fn new(name: impl Into<String>) -> Self {
        let name = name.into();
        let initials = name
            .split_whitespace()
            .filter_map(|part| part.chars().next())
            .take(2)
            .collect::<String>()
            .to_uppercase();
        Self {
            name,
            initials,
            size: Size::Md,
            shape: AvatarShape::Circle,
            status: None,
            accent_color: None,
        }
    }

    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub fn shape(mut self, shape: AvatarShape) -> Self {
        self.shape = shape;
        self
    }

    pub fn status(mut self, status: AvatarStatus) -> Self {
        self.status = Some(status);
        self
    }

    pub fn color(mut self, color: Color32) -> Self {
        self.accent_color = Some(color);
        self
    }

    fn diameter(&self) -> f32 {
        match self.size {
            Size::Xs => 24.0,
            Size::Sm => 32.0,
            Size::Md => 40.0,
            Size::Lg => 52.0,
        }
    }
}

impl Widget for Avatar {
    fn ui(self, ui: &mut Ui) -> Response {
        let tokens = active_theme(ui.ctx());
        let d = self.diameter();
        let (rect, response) = ui.allocate_exact_size(Vec2::splat(d), Sense::hover());

        if ui.is_rect_visible(rect) {
            let hover_t = ui.ctx().animate_bool_with_time(response.id, response.hovered(), 0.14);
            let bg = self.accent_color.unwrap_or(tokens.primary);
            let fg = tokens.primary_foreground;
            let rounding = match self.shape {
                AvatarShape::Circle => Rounding::same(d * 0.5),
                AvatarShape::RoundedSquare => Rounding::same(8.0),
            };

            let stroke_col = lerp_color(tokens.border, tokens.ring, hover_t);
            let painter = ui.painter();
            painter.rect(rect, rounding, bg, Stroke::new(1.0 + hover_t * 0.75, stroke_col));
            painter.text(
                rect.center(),
                Align2::CENTER_CENTER,
                &self.initials,
                FontId::proportional(d * 0.36),
                fg,
            );

            if let Some(status) = self.status {
                let status_color = match status {
                    AvatarStatus::Online => tokens.success,
                    AvatarStatus::Away => tokens.warning,
                    AvatarStatus::Busy => tokens.destructive,
                    AvatarStatus::Offline => tokens.muted_foreground,
                };
                let dot_r = (d * 0.14).max(3.5);
                let dot_center = Pos2::new(rect.right() - dot_r, rect.bottom() - dot_r);
                painter.circle(
                    dot_center,
                    dot_r,
                    status_color,
                    Stroke::new(2.0, tokens.background),
                );
            }
        }

        response.on_hover_text(&self.name)
    }
}

/// Stacked horizontal group of [`Avatar`] items with automatic `+N` overflow badge.
pub struct AvatarGroup {
    avatars: Vec<Avatar>,
    max_visible: usize,
    size: Size,
}

impl AvatarGroup {
    pub fn new(avatars: Vec<Avatar>) -> Self {
        Self {
            avatars,
            max_visible: 4,
            size: Size::Md,
        }
    }

    pub fn max_visible(mut self, max: usize) -> Self {
        self.max_visible = max.max(1);
        self
    }

    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }
}

impl Widget for AvatarGroup {
    fn ui(self, ui: &mut Ui) -> Response {
        let tokens = active_theme(ui.ctx());
        let total = self.avatars.len();
        let visible_count = total.min(self.max_visible);
        let overflow = total.saturating_sub(self.max_visible);

        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = -8.0;
            for avatar in self.avatars.into_iter().take(visible_count) {
                ui.add(avatar.size(self.size));
            }
            if overflow > 0 {
                let d = match self.size {
                    Size::Xs => 24.0,
                    Size::Sm => 32.0,
                    Size::Md => 40.0,
                    Size::Lg => 52.0,
                };
                let (rect, _) = ui.allocate_exact_size(Vec2::splat(d), Sense::hover());
                ui.painter().circle(
                    rect.center(),
                    d * 0.5,
                    tokens.muted,
                    Stroke::new(1.5, tokens.background),
                );
                ui.painter().text(
                    rect.center(),
                    Align2::CENTER_CENTER,
                    format!("+{overflow}"),
                    FontId::proportional(d * 0.32),
                    tokens.foreground,
                );
            }
        })
        .response
    }
}
