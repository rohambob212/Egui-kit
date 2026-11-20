//! Pressable two-state Toggle button for toolbars and formatting bars.

use crate::components::icon::{paint_icon, IconName};
use crate::theme::active_theme;
use crate::tokens::{lerp_color, Size};
use egui::{FontId, Pos2, Rect, Response, Sense, Stroke, Ui, Vec2, Widget};

/// Pressable toggle button mutating `&mut bool`.
pub struct Toggle<'a> {
    pressed: &'a mut bool,
    label: String,
    icon: Option<IconName>,
    size: Size,
    disabled: bool,
}

impl<'a> Toggle<'a> {
    pub fn new(pressed: &'a mut bool, label: impl Into<String>) -> Self {
        Self {
            pressed,
            label: label.into(),
            icon: None,
            size: Size::Md,
            disabled: false,
        }
    }

    pub fn icon(mut self, icon: IconName) -> Self {
        self.icon = Some(icon);
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
}

impl<'a> Widget for Toggle<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let tokens = active_theme(ui.ctx());
        let sense = if self.disabled { Sense::hover() } else { Sense::click() };
        let font_id = FontId::proportional(self.size.font_size());
        let galley = ui.painter().layout_no_wrap(self.label, font_id, tokens.foreground);

        let icon_w = self.size.icon_size();
        let icon_gap = if self.icon.is_some() && !galley.text().is_empty() { 6.0 } else { 0.0 };
        let leading_w = if self.icon.is_some() { icon_w + icon_gap } else { 0.0 };
        let content_w = leading_w + galley.size().x;
        let desired = Vec2::new((content_w + self.size.padding_x() * 2.0).max(self.size.height()), self.size.height());

        let (rect, mut response) = ui.allocate_exact_size(desired, sense);
        if response.clicked() && !self.disabled {
            *self.pressed = !*self.pressed;
            response.mark_changed();
        }

        if ui.is_rect_visible(rect) {
            let on_t = ui.ctx().animate_bool_with_time(response.id.with("on"), *self.pressed, 0.14);
            let hover_t = ui.ctx().animate_bool_with_time(
                response.id.with("hov"),
                response.hovered() && !self.disabled,
                0.12,
            );
            let down_t = ui.ctx().animate_bool_with_time(
                response.id.with("down"),
                response.is_pointer_button_down_on() && !self.disabled,
                0.08,
            );

            let off_bg = lerp_color(tokens.card, tokens.accent, hover_t);
            let off_fg = lerp_color(tokens.foreground, tokens.accent_foreground, hover_t);
            let mut bg = lerp_color(off_bg, tokens.primary, on_t);
            let mut fg = lerp_color(off_fg, tokens.primary_foreground, on_t);
            let border = Stroke::new(1.0, lerp_color(tokens.border, tokens.primary, on_t));

            if self.disabled {
                bg = bg.linear_multiply(0.55);
                fg = fg.linear_multiply(0.6);
            }

            let paint_rect = rect.shrink(down_t * 0.75);
            let painter = ui.painter();
            painter.rect(paint_rect, self.size.rounding(), bg, border);

            let mut x = paint_rect.center().x - content_w * 0.5;
            if let Some(icon) = self.icon {
                let icon_rect = Rect::from_center_size(Pos2::new(x + icon_w * 0.5, paint_rect.center().y), Vec2::splat(icon_w));
                paint_icon(painter, icon_rect, icon, fg);
                x += icon_w + icon_gap;
            }
            painter.galley(Pos2::new(x, paint_rect.center().y - galley.size().y * 0.5), galley, fg);
        }

        if !self.disabled {
            response = response.on_hover_cursor(egui::CursorIcon::PointingHand);
        }
        response
    }
}
