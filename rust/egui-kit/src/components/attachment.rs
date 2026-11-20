//! File Attachment chip/card with tabular byte formatting, progress bar, and remove button.

use crate::components::icon::{paint_icon, IconName};
use crate::components::progress::Progress;
use crate::theme::active_theme;
use crate::tokens::{lerp_color, Size};
use egui::{FontId, Pos2, Rect, Response, Rounding, Sense, Stroke, Ui, Vec2, Widget};

/// Response returned from [`Attachment::show`].
pub struct AttachmentResponse {
    pub response: Response,
    pub removed: bool,
}

/// File attachment row displaying filename, formatted size, optional progress, and remove trigger.
pub struct Attachment {
    name: String,
    bytes: u64,
    progress: Option<f32>,
    removable: bool,
    width: f32,
}

impl Attachment {
    pub fn new(name: impl Into<String>, bytes: u64) -> Self {
        Self {
            name: name.into(),
            bytes,
            progress: None,
            removable: true,
            width: 320.0,
        }
    }

    pub fn progress(mut self, progress: Option<f32>) -> Self {
        self.progress = progress.map(|p| p.clamp(0.0, 1.0));
        self
    }

    pub fn removable(mut self, removable: bool) -> Self {
        self.removable = removable;
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width.max(200.0);
        self
    }

    /// Shows the attachment card and reports whether its remove button was clicked.
    pub fn show(self, ui: &mut Ui) -> AttachmentResponse {
        let tokens = active_theme(ui.ctx());
        let mut removed = false;
        let has_progress = self.progress.is_some();
        let height = if has_progress { 64.0 } else { 52.0 };
        let (rect, response) = ui.allocate_exact_size(Vec2::new(self.width, height), Sense::hover());

        if ui.is_rect_visible(rect) {
            let hov_t = ui.ctx().animate_bool_with_time(response.id.with("hov"), response.hovered(), 0.12);
            let bg = lerp_color(tokens.card, tokens.accent, hov_t * 0.25);
            ui.painter().rect(rect, Rounding::same(8.0), bg, Stroke::new(1.0, tokens.border));

            // File icon badge
            let badge_rect = Rect::from_min_size(Pos2::new(rect.left() + 12.0, rect.top() + 10.0), Vec2::splat(32.0));
            ui.painter().rect_filled(badge_rect, Rounding::same(6.0), tokens.muted);
            let doc_rect = Rect::from_center_size(badge_rect.center(), Vec2::new(13.0, 16.0));
            ui.painter().rect_stroke(doc_rect, Rounding::same(2.0), Stroke::new(1.3, tokens.primary));
            ui.painter().line_segment(
                [Pos2::new(doc_rect.left() + 3.0, doc_rect.center().y - 2.0), Pos2::new(doc_rect.right() - 3.0, doc_rect.center().y - 2.0)],
                Stroke::new(1.2, tokens.primary),
            );
            ui.painter().line_segment(
                [Pos2::new(doc_rect.left() + 3.0, doc_rect.center().y + 2.0), Pos2::new(doc_rect.right() - 5.0, doc_rect.center().y + 2.0)],
                Stroke::new(1.2, tokens.primary),
            );

            // Title & tabular size
            let text_x = badge_rect.right() + 10.0;
            ui.painter().text(
                Pos2::new(text_x, rect.top() + 17.0),
                egui::Align2::LEFT_CENTER,
                &self.name,
                FontId::proportional(13.0),
                tokens.foreground,
            );
            let size_label = match self.progress {
                Some(p) => format!("{} · {:.0}%", format_bytes(self.bytes), p * 100.0),
                None => format_bytes(self.bytes),
            };
            ui.painter().text(
                Pos2::new(text_x, rect.top() + 34.0),
                egui::Align2::LEFT_CENTER,
                size_label,
                FontId::monospace(11.0),
                tokens.muted_foreground,
            );

            if let Some(p) = self.progress {
                let bar_rect = Rect::from_min_size(
                    Pos2::new(rect.left() + 12.0, rect.bottom() - 12.0),
                    Vec2::new(rect.width() - 24.0, 4.0),
                );
                ui.allocate_ui_at_rect(bar_rect, |ui| {
                    ui.add(Progress::new(p).size(Size::Xs));
                });
            }
        }

        if self.removable {
            let close_rect = Rect::from_center_size(Pos2::new(rect.right() - 20.0, rect.top() + 26.0), Vec2::splat(22.0));
            let close_resp = ui.interact(close_rect, response.id.with("rm"), Sense::click());
            let hov_t = ui.ctx().animate_bool_with_time(close_resp.id.with("h"), close_resp.hovered(), 0.12);
            if close_resp.hovered() {
                ui.painter().rect_filled(close_rect, Rounding::same(4.0), tokens.muted);
            }
            paint_icon(
                ui.painter(),
                close_rect.shrink(5.0),
                IconName::Close,
                lerp_color(tokens.muted_foreground, tokens.destructive, hov_t),
            );
            if close_resp.clicked() {
                removed = true;
            }
        }

        AttachmentResponse { response, removed }
    }
}

impl Widget for Attachment {
    fn ui(self, ui: &mut Ui) -> Response {
        self.show(ui).response
    }
}

/// Formats raw bytes into compact human-readable units.
pub fn format_bytes(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{bytes} B")
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    }
}
