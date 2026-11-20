//! Themed Scrollable viewport container wrapping `egui::ScrollArea`.

use crate::theme::active_theme;
use crate::tokens::Orientation;
use egui::{Frame, Margin, Rounding, ScrollArea, Stroke, Ui};

/// Themed scroll container with directional axes, max height, and stick-to-bottom mode.
pub struct Scrollable {
    id_source: String,
    direction: Orientation,
    max_height: f32,
    bordered: bool,
    stick_to_bottom: bool,
    always_show_scroll: bool,
}

impl Scrollable {
    pub fn vertical(id_source: impl Into<String>) -> Self {
        Self {
            id_source: id_source.into(),
            direction: Orientation::Vertical,
            max_height: 220.0,
            bordered: true,
            stick_to_bottom: false,
            always_show_scroll: false,
        }
    }

    pub fn horizontal(id_source: impl Into<String>) -> Self {
        Self {
            id_source: id_source.into(),
            direction: Orientation::Horizontal,
            max_height: 180.0,
            bordered: true,
            stick_to_bottom: false,
            always_show_scroll: false,
        }
    }

    pub fn max_height(mut self, max_height: f32) -> Self {
        self.max_height = max_height.max(60.0);
        self
    }

    pub fn bordered(mut self, bordered: bool) -> Self {
        self.bordered = bordered;
        self
    }

    pub fn stick_to_bottom(mut self, stick: bool) -> Self {
        self.stick_to_bottom = stick;
        self
    }

    pub fn always_show_scroll(mut self, always: bool) -> Self {
        self.always_show_scroll = always;
        self
    }

    pub fn show<R>(
        self,
        ui: &mut Ui,
        add_contents: impl FnOnce(&mut Ui) -> R,
    ) -> egui::scroll_area::ScrollAreaOutput<R> {
        let tokens = active_theme(ui.ctx());
        let stroke = if self.bordered {
            Stroke::new(1.0, tokens.border)
        } else {
            Stroke::NONE
        };

        Frame::none()
            .fill(tokens.card)
            .stroke(stroke)
            .rounding(Rounding::same(8.0))
            .inner_margin(Margin::same(8.0))
            .show(ui, |ui| {
                let mut area = match self.direction {
                    Orientation::Vertical => ScrollArea::vertical(),
                    Orientation::Horizontal => ScrollArea::horizontal(),
                };
                area = area
                    .id_salt(&self.id_source)
                    .max_height(self.max_height)
                    .auto_shrink([false, false])
                    .stick_to_bottom(self.stick_to_bottom)
                    .scroll_bar_visibility(if self.always_show_scroll {
                        egui::scroll_area::ScrollBarVisibility::AlwaysVisible
                    } else {
                        egui::scroll_area::ScrollBarVisibility::VisibleWhenNeeded
                    });

                area.show(ui, add_contents)
            })
            .inner
    }
}
