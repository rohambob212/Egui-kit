//! One-click Clipboard copy widget calling `ctx.copy_text()` with animated "Copied!" feedback.

use crate::components::icon::{paint_icon, IconName};
use crate::theme::active_theme;
use crate::tokens::{ease_out_cubic, lerp_color, Size};
use egui::{FontId, Id, Pos2, Rect, Response, Sense, Stroke, Ui, Vec2, Widget};

/// One-click copy-to-clipboard button with timed confirmation state in `egui::Memory`.
pub struct Clipboard<'a> {
    id_source: &'a str,
    text: &'a str,
    label: String,
    copied_label: String,
    size: Size,
    show_text: bool,
}

impl<'a> Clipboard<'a> {
    pub fn new(id_source: &'a str, text: &'a str) -> Self {
        Self {
            id_source,
            text,
            label: "Copy".to_string(),
            copied_label: "Copied!".to_string(),
            size: Size::Md,
            show_text: true,
        }
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }

    pub fn copied_label(mut self, copied_label: impl Into<String>) -> Self {
        self.copied_label = copied_label.into();
        self
    }

    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub fn show_text(mut self, show_text: bool) -> Self {
        self.show_text = show_text;
        self
    }
}

impl<'a> Widget for Clipboard<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let tokens = active_theme(ui.ctx());
        let id = Id::new(("egui_kit_clipboard", self.id_source));
        let now = ui.input(|i| i.time);
        let copied_until = ui.ctx().data(|d| d.get_temp::<f64>(id.with("until")).unwrap_or(0.0));
        let is_copied = now < copied_until;
        if is_copied {
            ui.ctx().request_repaint();
        }

        let font = FontId::proportional(self.size.font_size());
        let caption = if is_copied { &self.copied_label } else { &self.label };
        let galley = ui.painter().layout_no_wrap(caption.clone(), font, tokens.foreground);
        let icon_w = self.size.icon_size();
        let text_w = if self.show_text { galley.size().x + 6.0 } else { 0.0 };
        let width = self.size.padding_x() * 2.0 + icon_w + text_w;

        let (rect, mut response) = ui.allocate_exact_size(Vec2::new(width, self.size.height()), Sense::click());
        if response.clicked() {
            ui.ctx().copy_text(self.text.to_string());
            ui.ctx().data_mut(|d| d.insert_temp(id.with("until"), now + 1.6));
            response.mark_changed();
        }

        let copied_t = ease_out_cubic(ui.ctx().animate_bool_with_time(id.with("copied_anim"), is_copied, 0.16));
        let hover_t = ui.ctx().animate_bool_with_time(id.with("hov_anim"), response.hovered(), 0.12);

        if ui.is_rect_visible(rect) {
            let bg = lerp_color(lerp_color(tokens.card, tokens.accent, hover_t), tokens.success.linear_multiply(0.18), copied_t);
            let border = lerp_color(lerp_color(tokens.border, tokens.ring, hover_t), tokens.success, copied_t);
            let fg = lerp_color(tokens.foreground, tokens.success, copied_t);

            let painter = ui.painter();
            painter.rect(rect, self.size.rounding(), bg, Stroke::new(1.0, border));

            let icon_center = Pos2::new(
                rect.left() + self.size.padding_x() + icon_w * 0.5,
                rect.center().y,
            );
            let icon_rect = Rect::from_center_size(icon_center, Vec2::splat(icon_w));
            paint_icon(
                painter,
                icon_rect,
                if copied_t > 0.5 { IconName::Check } else { IconName::Copy },
                fg,
            );

            if self.show_text {
                painter.galley(
                    Pos2::new(icon_rect.right() + 6.0, rect.center().y - galley.size().y * 0.5),
                    galley,
                    fg,
                );
            }
        }

        response.on_hover_cursor(egui::CursorIcon::PointingHand)
    }
}
