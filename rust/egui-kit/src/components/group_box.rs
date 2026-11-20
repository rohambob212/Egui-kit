//! Titled GroupBox container with themed header and hairline frame.

use crate::theme::active_theme;
use egui::{FontId, Frame, Margin, Response, Rounding, Stroke, Ui};

/// Titled container grouping related settings or form controls cleanly.
pub struct GroupBox {
    title: String,
    subtitle: Option<String>,
    bordered: bool,
    muted_surface: bool,
    padding: f32,
}

impl GroupBox {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            subtitle: None,
            bordered: true,
            muted_surface: false,
            padding: 14.0,
        }
    }

    pub fn subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    pub fn bordered(mut self, bordered: bool) -> Self {
        self.bordered = bordered;
        self
    }

    pub fn muted_surface(mut self, muted: bool) -> Self {
        self.muted_surface = muted;
        self
    }

    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = padding.max(4.0);
        self
    }

    pub fn show<R>(self, ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> egui::InnerResponse<R> {
        let tokens = active_theme(ui.ctx());
        let bg = if self.muted_surface { tokens.muted } else { tokens.card };
        let title_fg = if self.muted_surface {
            tokens.foreground
        } else {
            tokens.card_foreground
        };
        let stroke = if self.bordered {
            Stroke::new(1.0, tokens.border)
        } else {
            Stroke::NONE
        };

        Frame::none()
            .fill(bg)
            .stroke(stroke)
            .rounding(Rounding::same(8.0))
            .inner_margin(Margin::same(self.padding))
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.label(
                        egui::RichText::new(&self.title)
                            .font(FontId::proportional(13.5))
                            .strong()
                            .color(title_fg),
                    );
                    if let Some(sub) = &self.subtitle {
                        ui.label(
                            egui::RichText::new(sub)
                                .font(FontId::proportional(12.0))
                                .color(tokens.muted_foreground),
                        );
                    }
                    let divider_y = ui.cursor().top() + 8.0;
                    ui.add_space(18.0);
                    let result = add_contents(ui);
                    let bounds = ui.min_rect();
                    ui.painter().line_segment(
                        [egui::pos2(bounds.left(), divider_y), egui::pos2(bounds.right(), divider_y)],
                        Stroke::new(1.0, tokens.border),
                    );
                    result
                })
                .inner
            })
    }
}

/// Convenience helper for bare `GroupBox` headers without return-value plumbing.
pub fn group_box_header(ui: &mut Ui, title: &str) -> Response {
    let tokens = active_theme(ui.ctx());
    ui.label(
        egui::RichText::new(title)
            .font(FontId::proportional(13.5))
            .strong()
            .color(tokens.foreground),
    )
}
