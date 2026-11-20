//! Edge-anchored Sheet slide-over drawer panel for inspectors and detail editors.

use crate::components::icon::{paint_icon, IconName};
use crate::theme::active_theme;
use crate::tokens::ease_out_cubic;
use egui::{
    Area, Color32, Context, FontId, Frame, Id, Key, Margin, Order, Pos2, Rect, Rounding, Sense,
    Stroke, Ui, Vec2,
};

/// Viewport edge to anchor a [`Sheet`] drawer against.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SheetSide {
    #[default]
    Right,
    Left,
    Top,
    Bottom,
}

/// Slide-over drawer panel anchored to a viewport edge and bound to `&mut bool`.
pub struct Sheet<'a> {
    id_source: &'a str,
    open: &'a mut bool,
    side: SheetSide,
    title: String,
    subtitle: Option<String>,
    extent: f32,
}

impl<'a> Sheet<'a> {
    pub fn right(id_source: &'a str, open: &'a mut bool) -> Self {
        Self::new(id_source, open, SheetSide::Right)
    }

    pub fn left(id_source: &'a str, open: &'a mut bool) -> Self {
        Self::new(id_source, open, SheetSide::Left)
    }

    pub fn top(id_source: &'a str, open: &'a mut bool) -> Self {
        Self::new(id_source, open, SheetSide::Top)
    }

    pub fn bottom(id_source: &'a str, open: &'a mut bool) -> Self {
        Self::new(id_source, open, SheetSide::Bottom)
    }

    pub fn new(id_source: &'a str, open: &'a mut bool, side: SheetSide) -> Self {
        Self {
            id_source,
            open,
            side,
            title: String::new(),
            subtitle: None,
            extent: 360.0,
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    pub fn extent(mut self, extent: f32) -> Self {
        self.extent = extent.clamp(200.0, 640.0);
        self
    }

    pub fn show<R>(self, ctx: &Context, add_contents: impl FnOnce(&mut Ui) -> R) -> Option<R> {
        let anim_t = ctx.animate_bool_with_time(Id::new(self.id_source).with("open"), *self.open, 0.2);
        if !*self.open && anim_t <= 0.01 {
            return None;
        }

        let eased = ease_out_cubic(anim_t);
        let tokens = active_theme(ctx);
        let screen = ctx.screen_rect();

        if *self.open && ctx.input(|i| i.key_pressed(Key::Escape)) {
            *self.open = false;
        }

        // Backdrop scrim
        let scrim_alpha = (105.0 * eased) as u8;
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
                if resp.clicked() {
                    *self.open = false;
                }
            });

        let (panel_pos, panel_size) = match self.side {
            SheetSide::Right => {
                let w = self.extent.min(screen.width() - 24.0);
                let x = screen.right() - w * eased;
                (Pos2::new(x, screen.top()), Vec2::new(w, screen.height()))
            }
            SheetSide::Left => {
                let w = self.extent.min(screen.width() - 24.0);
                let x = screen.left() - w * (1.0 - eased);
                (Pos2::new(x, screen.top()), Vec2::new(w, screen.height()))
            }
            SheetSide::Top => {
                let h = self.extent.min(screen.height() - 24.0);
                let y = screen.top() - h * (1.0 - eased);
                (Pos2::new(screen.left(), y), Vec2::new(screen.width(), h))
            }
            SheetSide::Bottom => {
                let h = self.extent.min(screen.height() - 24.0);
                let y = screen.bottom() - h * eased;
                (Pos2::new(screen.left(), y), Vec2::new(screen.width(), h))
            }
        };

        let mut result = None;
        Area::new(Id::new(self.id_source).with("drawer"))
            .order(Order::Foreground)
            .fixed_pos(panel_pos)
            .show(ctx, |ui| {
                Frame::none()
                    .fill(tokens.card)
                    .stroke(Stroke::new(1.0, tokens.border))
                    .inner_margin(Margin::same(18.0))
                    .show(ui, |ui| {
                        ui.set_min_size(panel_size - Vec2::splat(36.0));
                        ui.set_max_size(panel_size - Vec2::splat(36.0));

                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                if !self.title.is_empty() {
                                    ui.label(
                                        egui::RichText::new(&self.title)
                                            .font(FontId::proportional(15.5))
                                            .strong()
                                            .color(tokens.card_foreground),
                                    );
                                }
                                if let Some(sub) = &self.subtitle {
                                    ui.label(
                                        egui::RichText::new(sub)
                                            .font(FontId::proportional(12.0))
                                            .color(tokens.muted_foreground),
                                    );
                                }
                            });
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
                        });

                        ui.add_space(10.0);
                        ui.separator();
                        ui.add_space(10.0);

                        result = Some(add_contents(ui));
                    });
            });

        result
    }
}
