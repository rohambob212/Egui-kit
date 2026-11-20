//! Badge widget for status dots, numeric counters, and anchored indicators.

use crate::theme::active_theme;
use egui::{Align2, FontId, Pos2, Response, Rounding, Sense, Stroke, Ui, Vec2, Widget};

/// Visual tone for [`Badge`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BadgeVariant {
    #[default]
    Primary,
    Secondary,
    Success,
    Warning,
    Destructive,
}

/// Compact counter or status indicator, usable standalone or anchored to an existing widget rect.
pub struct Badge {
    count: Option<u32>,
    max_count: u32,
    dot: bool,
    label: Option<String>,
    variant: BadgeVariant,
}

impl Badge {
    /// Creates a numeric counter badge.
    pub fn count(count: u32) -> Self {
        Self {
            count: Some(count),
            max_count: 99,
            dot: false,
            label: None,
            variant: BadgeVariant::Destructive,
        }
    }

    /// Creates a compact status dot badge.
    pub fn dot() -> Self {
        Self {
            count: None,
            max_count: 99,
            dot: true,
            label: None,
            variant: BadgeVariant::Success,
        }
    }

    /// Creates a text status badge.
    pub fn text(label: impl Into<String>) -> Self {
        Self {
            count: None,
            max_count: 99,
            dot: false,
            label: Some(label.into()),
            variant: BadgeVariant::Primary,
        }
    }

    pub fn max_count(mut self, max: u32) -> Self {
        self.max_count = max;
        self
    }

    pub fn variant(mut self, variant: BadgeVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Anchors this badge onto the top-right corner of an existing `egui::Response`.
    pub fn attach_to(self, ui: &mut Ui, target: &Response) {
        let anchor = Pos2::new(target.rect.right() - 2.0, target.rect.top() + 2.0);
        self.paint_at(ui, anchor);
    }

    fn paint_at(&self, ui: &mut Ui, center: Pos2) {
        let tokens = active_theme(ui.ctx());
        let (bg, fg) = match self.variant {
            BadgeVariant::Primary => (tokens.primary, tokens.primary_foreground),
            BadgeVariant::Secondary => (tokens.secondary, tokens.secondary_foreground),
            BadgeVariant::Success => (tokens.success, tokens.primary_foreground),
            BadgeVariant::Warning => (tokens.warning, tokens.primary_foreground),
            BadgeVariant::Destructive => (tokens.destructive, tokens.primary_foreground),
        };

        let painter = ui.painter();
        if self.dot {
            painter.circle(center, 5.0, bg, Stroke::new(1.5, tokens.background));
            return;
        }

        let display = self.display_text();
        let galley = painter.layout_no_wrap(display, FontId::proportional(10.5), fg);
        let size = Vec2::new((galley.size().x + 10.0).max(18.0), 18.0);
        let rect = egui::Rect::from_center_size(center, size);
        painter.rect(rect, Rounding::same(9.0), bg, Stroke::new(1.5, tokens.background));
        painter.galley(
            Pos2::new(rect.center().x - galley.size().x * 0.5, rect.center().y - galley.size().y * 0.5),
            galley,
            fg,
        );
    }

    fn display_text(&self) -> String {
        if let Some(label) = &self.label {
            return label.clone();
        }
        match self.count {
            Some(c) if c > self.max_count => format!("{}+", self.max_count),
            Some(c) => c.to_string(),
            None => String::new(),
        }
    }
}

impl Widget for Badge {
    fn ui(self, ui: &mut Ui) -> Response {
        let tokens = active_theme(ui.ctx());
        let (bg, fg) = match self.variant {
            BadgeVariant::Primary => (tokens.primary, tokens.primary_foreground),
            BadgeVariant::Secondary => (tokens.secondary, tokens.secondary_foreground),
            BadgeVariant::Success => (tokens.success, tokens.primary_foreground),
            BadgeVariant::Warning => (tokens.warning, tokens.primary_foreground),
            BadgeVariant::Destructive => (tokens.destructive, tokens.primary_foreground),
        };

        if self.dot {
            let (rect, response) = ui.allocate_exact_size(Vec2::splat(10.0), Sense::hover());
            if ui.is_rect_visible(rect) {
                ui.painter().circle_filled(rect.center(), 4.5, bg);
            }
            return response;
        }

        let text = self.display_text();
        let galley = ui.painter().layout_no_wrap(text, FontId::proportional(11.0), fg);
        let desired = Vec2::new((galley.size().x + 12.0).max(20.0), 20.0);
        let (rect, response) = ui.allocate_exact_size(desired, Sense::hover());
        if ui.is_rect_visible(rect) {
            ui.painter().rect_filled(rect, Rounding::same(10.0), bg);
            ui.painter().text(
                rect.center(),
                Align2::CENTER_CENTER,
                self.display_text(),
                FontId::proportional(11.0),
                fg,
            );
        }
        response
    }
}
