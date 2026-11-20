//! High-intent AlertDialog confirmation modal preventing accidental outside-click dismissal.

use crate::components::button::{Button, ButtonVariant};
use crate::components::icon::{paint_icon, IconName};
use crate::theme::active_theme;
use crate::tokens::ease_out_cubic;
use egui::{
    Align2, Area, Color32, Context, FontId, Frame, Id, Key, Margin, Order, Rect, Rounding, Sense,
    Stroke, Vec2,
};

/// Result emitted on the frame a confirmation action is triggered.
#[derive(Debug, Clone, Copy, Default)]
pub struct AlertDialogResponse {
    pub confirmed: bool,
    pub cancelled: bool,
}

/// High-intent modal confirmation dialog bound directly to `&mut bool`.
pub struct AlertDialog<'a> {
    id_source: &'a str,
    title: String,
    description: String,
    open: &'a mut bool,
    confirm_label: String,
    cancel_label: String,
    destructive: bool,
}

impl<'a> AlertDialog<'a> {
    pub fn confirm(title: impl Into<String>, open: &'a mut bool) -> Self {
        Self {
            id_source: "alert_dialog",
            title: title.into(),
            description: String::new(),
            open,
            confirm_label: "Confirm".to_string(),
            cancel_label: "Cancel".to_string(),
            destructive: false,
        }
    }

    pub fn id(mut self, id_source: &'a str) -> Self {
        self.id_source = id_source;
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    pub fn confirm_label(mut self, label: impl Into<String>) -> Self {
        self.confirm_label = label.into();
        self
    }

    pub fn cancel_label(mut self, label: impl Into<String>) -> Self {
        self.cancel_label = label.into();
        self
    }

    pub fn destructive(mut self, destructive: bool) -> Self {
        self.destructive = destructive;
        self
    }

    pub fn show(self, ctx: &Context) -> AlertDialogResponse {
        let mut out = AlertDialogResponse::default();
        let anim_t = ctx.animate_bool_with_time(Id::new(self.id_source).with("open"), *self.open, 0.15);
        if !*self.open && anim_t <= 0.01 {
            return out;
        }

        let eased = ease_out_cubic(anim_t);
        let tokens = active_theme(ctx);
        let screen = ctx.screen_rect();

        if *self.open && ctx.input(|i| i.key_pressed(Key::Escape)) {
            *self.open = false;
            out.cancelled = true;
        }

        // Intentional blocking backdrop (does NOT close on outside click)
        let scrim_alpha = (135.0 * eased) as u8;
        Area::new(Id::new(self.id_source).with("scrim"))
            .order(Order::Middle)
            .fixed_pos(screen.min)
            .show(ctx, |ui| {
                let (rect, _) = ui.allocate_exact_size(screen.size(), Sense::click());
                ui.painter().rect_filled(
                    rect,
                    Rounding::ZERO,
                    Color32::from_rgba_unmultiplied(8, 12, 20, scrim_alpha),
                );
            });

        let card_w = 420.0_f32.min(screen.width() - 32.0);
        let slide_y = (1.0 - eased) * 12.0;

        Area::new(Id::new(self.id_source).with("modal"))
            .order(Order::Foreground)
            .anchor(Align2::CENTER_CENTER, Vec2::new(0.0, slide_y))
            .show(ctx, |ui| {
                Frame::none()
                    .fill(tokens.card)
                    .stroke(Stroke::new(1.0, tokens.border))
                    .rounding(Rounding::same(10.0))
                    .inner_margin(Margin::same(20.0))
                    .show(ui, |ui| {
                        ui.set_width(card_w);
                        ui.horizontal(|ui| {
                            let (badge_rect, _) = ui.allocate_exact_size(Vec2::splat(34.0), Sense::hover());
                            let tone = if self.destructive {
                                tokens.destructive
                            } else {
                                tokens.warning
                            };
                            ui.painter().rect_filled(
                                badge_rect,
                                Rounding::same(8.0),
                                Color32::from_rgba_unmultiplied(tone.r(), tone.g(), tone.b(), 28),
                            );
                            paint_icon(
                                ui.painter(),
                                Rect::from_center_size(badge_rect.center(), Vec2::splat(17.0)),
                                IconName::AlertTriangle,
                                tone,
                            );

                            ui.add_space(6.0);
                            ui.vertical(|ui| {
                                ui.label(
                                    egui::RichText::new(&self.title)
                                        .font(FontId::proportional(15.0))
                                        .strong()
                                        .color(tokens.card_foreground),
                                );
                                if !self.description.is_empty() {
                                    ui.add_space(3.0);
                                    ui.label(
                                        egui::RichText::new(&self.description)
                                            .font(FontId::proportional(12.5))
                                            .color(tokens.muted_foreground),
                                    );
                                }
                            });
                        });

                        ui.add_space(18.0);
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let variant = if self.destructive {
                                ButtonVariant::Destructive
                            } else {
                                ButtonVariant::Primary
                            };
                            if ui.add(Button::new(&self.confirm_label).variant(variant)).clicked() {
                                *self.open = false;
                                out.confirmed = true;
                            }
                            if ui
                                .add(Button::new(&self.cancel_label).variant(ButtonVariant::Outline))
                                .clicked()
                            {
                                *self.open = false;
                                out.cancelled = true;
                            }
                        });
                    });
            });

        out
    }
}
