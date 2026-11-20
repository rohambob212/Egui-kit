//! Button widget ported from `gpui-kit` with variants, sizes, icons, and loading state.

use crate::components::icon::{paint_icon, IconName};
use crate::theme::active_theme;
use crate::tokens::{lerp_color, Size};
use egui::{FontId, Response, Sense, Stroke, Ui, Vec2, Widget};

/// Visual style variants for [`Button`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
    Outline,
    Ghost,
    Destructive,
    Link,
}

/// Interactive button widget styled via `egui-kit` design tokens.
pub struct Button {
    label: String,
    variant: ButtonVariant,
    size: Size,
    disabled: bool,
    loading: bool,
    icon: Option<IconName>,
    trailing_icon: Option<IconName>,
    full_width: bool,
}

impl Button {
    /// Creates a new button with the given label.
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            variant: ButtonVariant::Primary,
            size: Size::Md,
            disabled: false,
            loading: false,
            icon: None,
            trailing_icon: None,
            full_width: false,
        }
    }

    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    pub fn icon(mut self, icon: IconName) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn trailing_icon(mut self, icon: IconName) -> Self {
        self.trailing_icon = Some(icon);
        self
    }

    pub fn full_width(mut self, full_width: bool) -> Self {
        self.full_width = full_width;
        self
    }
}

impl Widget for Button {
    fn ui(self, ui: &mut Ui) -> Response {
        let tokens = active_theme(ui.ctx());
        let interactive = !self.disabled && !self.loading;
        let sense = if interactive {
            Sense::click()
        } else {
            Sense::hover()
        };

        let font_id = FontId::proportional(self.size.font_size());
        let galley = ui.painter().layout_no_wrap(
            self.label.clone(),
            font_id,
            tokens.primary_foreground,
        );

        let icon_gap = 6.0;
        let icon_w = self.size.icon_size();
        let leading_w = if self.loading || self.icon.is_some() {
            icon_w + icon_gap
        } else {
            0.0
        };
        let trailing_w = if self.trailing_icon.is_some() {
            icon_gap + icon_w
        } else {
            0.0
        };

        let content_w = leading_w + galley.size().x + trailing_w;
        let desired_w = if self.full_width {
            ui.available_width().max(content_w + self.size.padding_x() * 2.0)
        } else {
            content_w + self.size.padding_x() * 2.0
        };
        let desired = Vec2::new(desired_w, self.size.height());

        let (rect, response) = ui.allocate_exact_size(desired, sense);
        if !ui.is_rect_visible(rect) {
            return response;
        }

        let hover_t = ui.ctx().animate_bool_with_time(
            response.id.with("hover"),
            response.hovered() && interactive,
            0.12,
        );
        let press_t = ui.ctx().animate_bool_with_time(
            response.id.with("press"),
            response.is_pointer_button_down_on() && interactive,
            0.08,
        );
        let focus_t = ui.ctx().animate_bool_with_time(
            response.id.with("focus"),
            response.has_focus(),
            0.14,
        );

        let (base_bg, mut fg, mut border) = match self.variant {
            ButtonVariant::Primary => (tokens.primary, tokens.primary_foreground, Stroke::NONE),
            ButtonVariant::Secondary => (
                tokens.secondary,
                tokens.secondary_foreground,
                Stroke::new(1.0, tokens.border),
            ),
            ButtonVariant::Outline => (
                egui::Color32::TRANSPARENT,
                tokens.foreground,
                Stroke::new(1.0, tokens.border),
            ),
            ButtonVariant::Ghost => (egui::Color32::TRANSPARENT, tokens.foreground, Stroke::NONE),
            ButtonVariant::Destructive => (
                tokens.destructive,
                tokens.primary_foreground,
                Stroke::NONE,
            ),
            ButtonVariant::Link => (egui::Color32::TRANSPARENT, tokens.primary, Stroke::NONE),
        };

        let hover_bg = match self.variant {
            ButtonVariant::Primary => tokens.primary.linear_multiply(0.88),
            ButtonVariant::Destructive => tokens.destructive.linear_multiply(0.88),
            ButtonVariant::Secondary | ButtonVariant::Outline | ButtonVariant::Ghost => tokens.accent,
            ButtonVariant::Link => egui::Color32::TRANSPARENT,
        };
        let mut bg = lerp_color(base_bg, hover_bg, hover_t);

        if !interactive {
            bg = base_bg.linear_multiply(0.55);
            fg = fg.linear_multiply(0.6);
            border.color = border.color.linear_multiply(0.55);
        }

        let paint_rect = rect.shrink(press_t * 0.85);
        let painter = ui.painter();
        if bg != egui::Color32::TRANSPARENT || border != Stroke::NONE {
            painter.rect(paint_rect, self.size.rounding(), bg, border);
        }

        if focus_t > 0.01 {
            painter.rect_stroke(
                paint_rect.expand(2.0),
                self.size.rounding(),
                Stroke::new(1.5, tokens.ring.linear_multiply(focus_t)),
            );
        }

        let mut cursor_x = rect.center().x - content_w * 0.5;
        let center_y = rect.center().y;

        if self.loading {
            let spinner_center = egui::pos2(cursor_x + icon_w * 0.5, center_y);
            let time = ui.input(|i| i.time) as f32;
            let angle = time * 6.0;
            let r = icon_w * 0.38;
            let p1 = spinner_center + Vec2::angled(angle) * r;
            let p2 = spinner_center + Vec2::angled(angle + 2.2) * r;
            painter.circle_stroke(spinner_center, r, Stroke::new(1.5, fg.linear_multiply(0.35)));
            painter.line_segment([p1, p2], Stroke::new(2.0, fg));
            ui.ctx().request_repaint();
            cursor_x += icon_w + icon_gap;
        } else if let Some(icon_name) = self.icon {
            let icon_rect = egui::Rect::from_center_size(
                egui::pos2(cursor_x + icon_w * 0.5, center_y),
                Vec2::splat(icon_w),
            );
            paint_icon(painter, icon_rect, icon_name, fg);
            cursor_x += icon_w + icon_gap;
        }

        let text_galley =
            painter.layout_no_wrap(self.label, FontId::proportional(self.size.font_size()), fg);
        let text_pos = egui::pos2(cursor_x, center_y - text_galley.size().y * 0.5);
        painter.galley(text_pos, text_galley, fg);
        cursor_x += content_w - leading_w - trailing_w;

        if self.variant == ButtonVariant::Link && response.hovered() && interactive {
            let underline_y = center_y + self.size.font_size() * 0.55;
            painter.line_segment(
                [egui::pos2(text_pos.x, underline_y), egui::pos2(cursor_x, underline_y)],
                Stroke::new(1.0, fg),
            );
        }

        if let Some(trailing) = self.trailing_icon {
            let icon_rect = egui::Rect::from_center_size(
                egui::pos2(cursor_x + icon_gap + icon_w * 0.5, center_y),
                Vec2::splat(icon_w),
            );
            paint_icon(painter, icon_rect, trailing, fg);
        }

        if interactive {
            response.on_hover_cursor(egui::CursorIcon::PointingHand)
        } else {
            response
        }
    }
}
