//! Themed hover Tooltip widget with optional Kbd shortcut badge and Response extension.

use crate::components::kbd::Kbd;
use crate::theme::active_theme;
use crate::tokens::Size;
use egui::{Area, FontId, Frame, Id, Margin, Order, Pos2, Response, Rounding, Stroke};

/// Themed tooltip helper replacing `egui`'s default unstyled tooltip frame.
pub struct Tooltip {
    text: String,
    shortcut: Option<String>,
}

impl Tooltip {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            shortcut: None,
        }
    }

    pub fn shortcut(mut self, shortcut: impl Into<String>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    /// Shows this themed tooltip when `target` is hovered or keyboard-focused.
    pub fn show_for(self, target: &Response) {
        if !(target.hovered() || target.has_focus()) {
            return;
        }

        let ctx = target.ctx.clone();
        let tokens = active_theme(&ctx);
        let screen = ctx.screen_rect();
        let pos = Pos2::new(
            target.rect.left().clamp(screen.left() + 8.0, (screen.right() - 220.0).max(screen.left() + 8.0)),
            (target.rect.top() - 34.0).max(screen.top() + 8.0),
        );

        Area::new(Id::new(target.id).with("kit_tooltip"))
            .order(Order::Tooltip)
            .interactable(false)
            .fixed_pos(pos)
            .show(&ctx, |ui| {
                Frame::none()
                    .fill(tokens.card)
                    .stroke(Stroke::new(1.0, tokens.border))
                    .rounding(Rounding::same(6.0))
                    .inner_margin(Margin::symmetric(10.0, 6.0))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = 8.0;
                            ui.label(
                                egui::RichText::new(&self.text)
                                    .font(FontId::proportional(12.0))
                                    .color(tokens.card_foreground),
                            );
                            if let Some(chord) = &self.shortcut {
                                ui.add(Kbd::new(chord).size(Size::Xs));
                            }
                        });
                    });
            });
    }
}

/// Extension trait adding `.kit_tooltip(...)` one-liners to any `egui::Response`.
pub trait ResponseTooltipExt {
    fn kit_tooltip(&self, text: impl Into<String>);
    fn kit_tooltip_shortcut(&self, text: impl Into<String>, shortcut: impl Into<String>);
}

impl ResponseTooltipExt for Response {
    fn kit_tooltip(&self, text: impl Into<String>) {
        Tooltip::new(text).show_for(self);
    }

    fn kit_tooltip_shortcut(&self, text: impl Into<String>, shortcut: impl Into<String>) {
        Tooltip::new(text).shortcut(shortcut).show_for(self);
    }
}
