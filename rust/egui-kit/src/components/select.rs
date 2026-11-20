//! Styled dropdown Select widget mutating `&mut usize` directly with eased popup animation.

use crate::components::icon::{paint_icon, IconName};
use crate::theme::active_theme;
use crate::tokens::{ease_out_cubic, lerp_color, Size};
use egui::{Area, FontId, Frame, Id, Key, Order, Pos2, Rect, Response, Sense, Stroke, Ui, Vec2, Widget};

/// Interactive dropdown Select mutating the chosen option index (`&mut usize`).
pub struct Select<'a, T: AsRef<str>> {
    id_source: &'a str,
    selected: &'a mut usize,
    options: &'a [T],
    placeholder: String,
    size: Size,
    width: f32,
    disabled: bool,
}

impl<'a, T: AsRef<str>> Select<'a, T> {
    pub fn new(id_source: &'a str, selected: &'a mut usize, options: &'a [T]) -> Self {
        Self {
            id_source,
            selected,
            options,
            placeholder: "Select an option".to_string(),
            size: Size::Md,
            width: 240.0,
            disabled: false,
        }
    }

    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width.max(120.0);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl<'a, T: AsRef<str>> Widget for Select<'a, T> {
    fn ui(self, ui: &mut Ui) -> Response {
        let tokens = active_theme(ui.ctx());
        let popup_id = Id::new(("egui_kit_select", self.id_source));
        let mut is_open = ui.ctx().data(|d| d.get_temp::<bool>(popup_id).unwrap_or(false));

        let sense = if self.disabled { Sense::hover() } else { Sense::click() };
        let (rect, mut response) = ui.allocate_exact_size(Vec2::new(self.width, self.size.height()), sense);

        if response.clicked() && !self.disabled {
            is_open = !is_open;
        }

        let open_t = ease_out_cubic(ui.ctx().animate_bool_with_time(popup_id.with("open"), is_open && !self.disabled, 0.16));
        let hover_t = ui.ctx().animate_bool_with_time(popup_id.with("hov"), (response.hovered() || is_open) && !self.disabled, 0.12);

        let bg = if self.disabled { tokens.muted } else { lerp_color(tokens.background, tokens.accent, hover_t * 0.45) };
        let border = lerp_color(tokens.input, tokens.ring, hover_t.max(open_t));
        let current_label = self.options.get(*self.selected).map(|o| o.as_ref()).unwrap_or(&self.placeholder);
        let fg = if self.disabled || self.options.is_empty() {
            tokens.muted_foreground
        } else {
            tokens.foreground
        };

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();
            painter.rect(rect, self.size.rounding(), bg, Stroke::new(1.0 + 0.5 * open_t, border));
            let font = FontId::proportional(self.size.font_size());
            let galley = painter.layout_no_wrap(current_label.to_string(), font, fg);
            painter.galley(Pos2::new(rect.left() + self.size.padding_x(), rect.center().y - galley.size().y * 0.5), galley, fg);

            // Animated chevron rotation (down -> up)
            let chev_w = self.size.icon_size();
            let cc = Pos2::new(rect.right() - self.size.padding_x() - chev_w * 0.5, rect.center().y);
            let r = chev_w * 0.34;
            let tip_y = egui::lerp(r * 0.45..=-r * 0.45, open_t);
            let wing_y = egui::lerp(-r * 0.3..=r * 0.3, open_t);
            let stroke = Stroke::new((chev_w * 0.1).clamp(1.25, 2.0), tokens.muted_foreground);
            painter.line_segment([cc + Vec2::new(-r * 0.85, wing_y), cc + Vec2::new(0.0, tip_y)], stroke);
            painter.line_segment([cc + Vec2::new(0.0, tip_y), cc + Vec2::new(r * 0.85, wing_y)], stroke);
        }

        if open_t > 0.01 && !self.disabled {
            if is_open && ui.input(|i| i.key_pressed(Key::Escape)) {
                is_open = false;
            }
            let offset_y = egui::lerp(0.0..=4.0, open_t);
            let area_resp = Area::new(popup_id.with("popup"))
                .order(Order::Foreground)
                .fixed_pos(Pos2::new(rect.left(), rect.bottom() + offset_y))
                .show(ui.ctx(), |ui| {
                    ui.set_opacity(open_t);
                    Frame::none()
                        .fill(tokens.card)
                        .stroke(Stroke::new(1.0, tokens.border))
                        .rounding(self.size.rounding())
                        .inner_margin(egui::Margin::same(4.0))
                        .show(ui, |ui| {
                            ui.set_min_width(self.width - 8.0);
                            for (idx, opt) in self.options.iter().enumerate() {
                                let selected = idx == *self.selected;
                                let (item_rect, item_resp) = ui.allocate_exact_size(Vec2::new(self.width - 10.0, self.size.height() - 4.0), Sense::click());
                                if item_resp.hovered() || selected {
                                    ui.painter().rect_filled(item_rect, self.size.rounding(), if selected { tokens.accent } else { tokens.muted });
                                }
                                ui.painter().text(
                                    Pos2::new(item_rect.left() + 10.0, item_rect.center().y),
                                    egui::Align2::LEFT_CENTER,
                                    opt.as_ref(),
                                    FontId::proportional(self.size.font_size()),
                                    tokens.card_foreground,
                                );
                                if selected {
                                    paint_icon(
                                        ui.painter(),
                                        Rect::from_center_size(Pos2::new(item_rect.right() - 14.0, item_rect.center().y), Vec2::splat(13.0)),
                                        IconName::Check,
                                        tokens.primary,
                                    );
                                }
                                if item_resp.clicked() && is_open {
                                    *self.selected = idx;
                                    is_open = false;
                                    response.mark_changed();
                                }
                            }
                        });
                });

            if is_open
                && ui.input(|i| i.pointer.any_click())
                && !response.hovered()
                && !area_resp.response.hovered()
            {
                is_open = false;
            }
        }

        ui.ctx().data_mut(|d| d.insert_temp(popup_id, is_open));
        response
    }
}
