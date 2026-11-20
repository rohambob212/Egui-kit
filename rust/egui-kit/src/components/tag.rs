//! Interactive Tag widget supporting semantic tones, toggleable selection, and close triggers.

use crate::components::icon::{paint_icon, IconName};
use crate::theme::active_theme;
use crate::tokens::{lerp_color, Size};
use egui::{FontId, Pos2, Rect, Response, Rounding, Sense, Stroke, Ui, Vec2, Widget};

/// Color variant for [`Tag`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TagVariant {
    #[default]
    Neutral,
    Primary,
    Outline,
    Success,
    Warning,
    Destructive,
}

/// Output from rendering a [`Tag`] with optional dismiss handling.
pub struct TagResponse {
    pub response: Response,
    pub closed: bool,
}

/// Interactive label tag with optional selection and dismiss button.
pub struct Tag {
    label: String,
    variant: TagVariant,
    size: Size,
    closable: bool,
    selected: bool,
}

impl Tag {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            variant: TagVariant::Neutral,
            size: Size::Sm,
            closable: false,
            selected: false,
        }
    }

    pub fn variant(mut self, variant: TagVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub fn closable(mut self, closable: bool) -> Self {
        self.closable = closable;
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// Renders the tag and reports whether its close button was clicked.
    pub fn show(self, ui: &mut Ui) -> TagResponse {
        let tokens = active_theme(ui.ctx());
        let font_id = FontId::proportional(self.size.font_size() - 1.0);
        let galley = ui
            .painter()
            .layout_no_wrap(self.label.clone(), font_id.clone(), tokens.foreground);

        let close_w = if self.closable { 16.0 } else { 0.0 };
        let height = (self.size.height() - 6.0).max(20.0);
        let width = galley.size().x + 16.0 + close_w;

        let (rect, response) = ui.allocate_exact_size(Vec2::new(width, height), Sense::click());

        let sel_t = ui.ctx().animate_bool_with_time(response.id.with("sel"), self.selected, 0.14);
        let (base_bg, base_fg, base_border) = match self.variant {
            TagVariant::Neutral => (
                tokens.muted,
                tokens.foreground,
                Stroke::new(1.0, tokens.border),
            ),
            TagVariant::Primary => (
                tokens.primary.linear_multiply(0.18),
                tokens.primary,
                Stroke::new(1.0, tokens.primary.linear_multiply(0.45)),
            ),
            TagVariant::Outline => (
                egui::Color32::TRANSPARENT,
                tokens.foreground,
                Stroke::new(1.0, tokens.border),
            ),
            TagVariant::Success => (
                tokens.success.linear_multiply(0.18),
                tokens.success,
                Stroke::new(1.0, tokens.success.linear_multiply(0.45)),
            ),
            TagVariant::Warning => (
                tokens.warning.linear_multiply(0.18),
                tokens.warning,
                Stroke::new(1.0, tokens.warning.linear_multiply(0.45)),
            ),
            TagVariant::Destructive => (
                tokens.destructive.linear_multiply(0.18),
                tokens.destructive,
                Stroke::new(1.0, tokens.destructive.linear_multiply(0.45)),
            ),
        };

        let bg = lerp_color(base_bg, tokens.primary, sel_t);
        let fg = lerp_color(base_fg, tokens.primary_foreground, sel_t);
        let border = Stroke::new(1.0, lerp_color(base_border.color, tokens.primary, sel_t));

        let mut closed = false;
        if ui.is_rect_visible(rect) {
            let painter = ui.painter();
            painter.rect(rect, Rounding::same(5.0), bg, border);
            painter.galley(
                Pos2::new(rect.left() + 8.0, rect.center().y - galley.size().y * 0.5),
                galley,
                fg,
            );

            if self.closable {
                let close_rect = Rect::from_center_size(
                    Pos2::new(rect.right() - 10.0, rect.center().y),
                    Vec2::splat(11.0),
                );
                let close_resp = ui.interact(
                    close_rect.expand(3.0),
                    response.id.with("close"),
                    Sense::click(),
                );
                if close_resp.clicked() {
                    closed = true;
                }
                let close_hover_t = ui.ctx().animate_bool_with_time(
                    close_resp.id.with("ch"),
                    close_resp.hovered(),
                    0.12,
                );
                let icon_color = lerp_color(fg, tokens.destructive, close_hover_t);
                paint_icon(painter, close_rect, IconName::Close, icon_color);
            }
        }

        TagResponse { response, closed }
    }
}

impl Widget for Tag {
    fn ui(self, ui: &mut Ui) -> Response {
        self.show(ui).response
    }
}
