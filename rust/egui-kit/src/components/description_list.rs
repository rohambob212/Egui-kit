//! Structured key-value DescriptionList supporting 1-3 columns and optional borders.

use crate::theme::active_theme;
use egui::{FontId, Frame, Margin, Response, Rounding, Stroke, Ui, Widget};

/// Single key-value entry in a [`DescriptionList`].
#[derive(Debug, Clone)]
pub struct DescriptionItem {
    pub label: String,
    pub value: String,
}

impl DescriptionItem {
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
        }
    }
}

/// Multi-column key-value metadata table rendered with theme tokens.
pub struct DescriptionList<'a> {
    items: &'a [(&'a str, &'a str)],
    columns: usize,
    bordered: bool,
    compact: bool,
}

impl<'a> DescriptionList<'a> {
    pub fn new(items: &'a [(&'a str, &'a str)]) -> Self {
        Self {
            items,
            columns: 2,
            bordered: true,
            compact: false,
        }
    }

    pub fn columns(mut self, columns: usize) -> Self {
        self.columns = columns.clamp(1, 3);
        self
    }

    pub fn bordered(mut self, bordered: bool) -> Self {
        self.bordered = bordered;
        self
    }

    pub fn compact(mut self, compact: bool) -> Self {
        self.compact = compact;
        self
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        ui.add(self)
    }
}

impl<'a> Widget for DescriptionList<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let tokens = active_theme(ui.ctx());
        let pad_y = if self.compact { 7.0 } else { 10.0 };
        let pad_x = if self.compact { 10.0 } else { 14.0 };
        let cols = self.columns;
        let stroke = if self.bordered {
            Stroke::new(1.0, tokens.border)
        } else {
            Stroke::NONE
        };

        Frame::none()
            .fill(tokens.card)
            .stroke(stroke)
            .rounding(Rounding::same(8.0))
            .show(ui, |ui| {
                let avail_w = ui.available_width().max(240.0);
                let col_w = avail_w / (cols as f32);

                for (row_idx, chunk) in self.items.chunks(cols).enumerate() {
                    if row_idx > 0 {
                        let rect = ui.available_rect_before_wrap();
                        ui.painter().line_segment(
                            [rect.left_top(), egui::pos2(rect.left() + avail_w, rect.top())],
                            Stroke::new(1.0, tokens.border),
                        );
                    }

                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 0.0;
                        for (c_idx, (label, value)) in chunk.iter().enumerate() {
                            if c_idx > 0 && self.bordered {
                                let r = ui.available_rect_before_wrap();
                                ui.painter().line_segment(
                                    [r.left_top(), egui::pos2(r.left(), r.top() + 44.0)],
                                    Stroke::new(1.0, tokens.border),
                                );
                            }
                            Frame::none()
                                .inner_margin(Margin::symmetric(pad_x, pad_y))
                                .show(ui, |ui| {
                                    ui.set_width((col_w - pad_x * 2.0).max(60.0));
                                    ui.vertical(|ui| {
                                        ui.label(
                                            egui::RichText::new(*label)
                                                .font(FontId::proportional(11.5))
                                                .color(tokens.muted_foreground),
                                        );
                                        ui.add_space(2.0);
                                        ui.label(
                                            egui::RichText::new(*value)
                                                .font(FontId::monospace(12.5))
                                                .color(tokens.foreground),
                                        );
                                    });
                                });
                        }
                    });
                }
            })
            .response
    }
}
