//! Accessible modal Dialog overlay with backdrop scrim and smooth scale/fade motion.

use crate::components::icon::{paint_icon, IconName};
use crate::theme::active_theme;
use crate::tokens::ease_out_cubic;
use egui::{
    Align2, Area, Color32, Context, FontId, Frame, Id, Key, Margin, Order, Pos2, Rect, Rounding,
    Sense, Stroke, Ui, Vec2,
};

/// Modal dialog window bound directly to `&mut bool`.
pub struct Dialog<'a> {
    id_source: &'a str,
    open: &'a mut bool,
    title: String,
    description: Option<String>,
    width: f32,
    dismiss_on_backdrop: bool,
    show_close_button: bool,
}

impl<'a> Dialog<'a> {
    pub fn new(id_source: &'a str, open: &'a mut bool) -> Self {
        Self {
            id_source,
            open,
            title: String::new(),
            description: None,
            width: 440.0,
            dismiss_on_backdrop: true,
            show_close_button: true,
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width.clamp(280.0, 760.0);
        self
    }

    pub fn dismiss_on_backdrop(mut self, enabled: bool) -> Self {
        self.dismiss_on_backdrop = enabled;
        self
    }

    pub fn show_close_button(mut self, enabled: bool) -> Self {
        self.show_close_button = enabled;
        self
    }

    pub fn show<R>(
        self,
        ctx: &Context,
        add_body: impl FnOnce(&mut Ui) -> R,
    ) -> Option<R> {
        let anim_t = ctx.animate_bool_with_time(Id::new(self.id_source).with("open"), *self.open, 0.16);
        if !*self.open && anim_t <= 0.01 {
            return None;
        }

        let eased = ease_out_cubic(anim_t);
        let tokens = active_theme(ctx);
        let screen = ctx.screen_rect();

        if *self.open && ctx.input(|i| i.key_pressed(Key::Escape)) {
            *self.open = false;
        }

        // Backdrop scrim layer
        let scrim_alpha = (115.0 * eased) as u8;
        Area::new(Id::new(self.id_source).with("backdrop"))
            .order(Order::Middle)
            .fixed_pos(screen.min)
            .show(ctx, |ui| {
                let (rect, resp) = ui.allocate_exact_size(screen.size(), Sense::click());
                ui.painter().rect_filled(
                    rect,
                    Rounding::ZERO,
                    Color32::from_rgba_unmultiplied(8, 12, 20, scrim_alpha),
                );
                if self.dismiss_on_backdrop && resp.clicked() {
                    *self.open = false;
                }
            });

        // Centered modal card
        let slide_y = (1.0 - eased) * 14.0;
        let dialog_w = self.width.min(screen.width() - 32.0);
        let mut inner_result = None;

        Area::new(Id::new(self.id_source).with("card"))
            .order(Order::Foreground)
            .anchor(Align2::CENTER_CENTER, Vec2::new(0.0, slide_y))
            .show(ctx, |ui| {
                Frame::none()
                    .fill(tokens.card)
                    .stroke(Stroke::new(1.0, tokens.border))
                    .rounding(Rounding::same(10.0))
                    .inner_margin(Margin::same(20.0))
                    .show(ui, |ui| {
                        ui.set_width(dialog_w);
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                if !self.title.is_empty() {
                                    ui.label(
                                        egui::RichText::new(&self.title)
                                            .font(FontId::proportional(16.0))
                                            .strong()
                                            .color(tokens.card_foreground),
                                    );
                                }
                                if let Some(desc) = &self.description {
                                    ui.add_space(2.0);
                                    ui.label(
                                        egui::RichText::new(desc)
                                            .font(FontId::proportional(12.5))
                                            .color(tokens.muted_foreground),
                                    );
                                }
                            });
                            if self.show_close_button {
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                                    let (close_rect, close_resp) =
                                        ui.allocate_exact_size(Vec2::splat(24.0), Sense::click());
                                    if close_resp.hovered() {
                                        ui.painter().rect_filled(close_rect, Rounding::same(5.0), tokens.muted);
                                    }
                                    paint_icon(
                                        ui.painter(),
                                        Rect::from_center_size(close_rect.center(), Vec2::splat(13.0)),
                                        IconName::Close,
                                        tokens.muted_foreground,
                                    );
                                    if close_resp.clicked() {
                                        *self.open = false;
                                    }
                                });
                            }
                        });

                        ui.add_space(12.0);
                        let sep_y = ui.cursor().top();
                        let bounds = ui.available_rect_before_wrap();
                        ui.painter().line_segment(
                            [Pos2::new(bounds.left(), sep_y), Pos2::new(bounds.right(), sep_y)],
                            Stroke::new(1.0, tokens.border),
                        );
                        ui.add_space(12.0);

                        inner_result = Some(add_body(ui));
                    });
            });

        inner_result
    }
}
