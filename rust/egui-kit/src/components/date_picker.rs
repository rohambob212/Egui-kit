//! Popover DatePicker combining a formatted trigger input with an anchored Calendar.

use crate::components::calendar::{Calendar, SimpleDate};
use crate::theme::active_theme;
use crate::tokens::{ease_out_cubic, lerp_color, Size};
use egui::{Area, FontId, Id, Key, Order, Pos2, Rect, Response, Rounding, Sense, Stroke, Ui, Vec2, Widget};

/// Trigger button + anchored Calendar popup mutating `&mut SimpleDate` directly.
pub struct DatePicker<'a> {
    id_source: &'a str,
    date: &'a mut SimpleDate,
    size: Size,
    width: f32,
    disabled: bool,
}

impl<'a> DatePicker<'a> {
    pub fn new(id_source: &'a str, date: &'a mut SimpleDate) -> Self {
        Self {
            id_source,
            date,
            size: Size::Md,
            width: 220.0,
            disabled: false,
        }
    }

    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width.max(150.0);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl<'a> Widget for DatePicker<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let tokens = active_theme(ui.ctx());
        let popup_id = Id::new(("egui_kit_date_picker", self.id_source));
        let mut is_open = ui.ctx().data(|d| d.get_temp::<bool>(popup_id).unwrap_or(false));

        let sense = if self.disabled { Sense::hover() } else { Sense::click() };
        let (rect, mut response) = ui.allocate_exact_size(Vec2::new(self.width, self.size.height()), sense);

        if response.clicked() && !self.disabled {
            is_open = !is_open;
        }

        let open_t = ease_out_cubic(ui.ctx().animate_bool_with_time(popup_id.with("open"), is_open && !self.disabled, 0.16));
        let hov_t = ui.ctx().animate_bool_with_time(popup_id.with("hov"), (response.hovered() || is_open) && !self.disabled, 0.12);

        if ui.is_rect_visible(rect) {
            let bg = if self.disabled {
                tokens.muted
            } else {
                lerp_color(tokens.background, tokens.accent, hov_t * 0.4)
            };
            let border = lerp_color(tokens.input, tokens.ring, hov_t.max(open_t));
            ui.painter().rect(rect, self.size.rounding(), bg, Stroke::new(1.0 + 0.5 * open_t, border));

            // Mini calendar painter icon on left
            let icon_rect = Rect::from_center_size(
                Pos2::new(rect.left() + self.size.padding_x() + 7.0, rect.center().y),
                Vec2::splat(13.0),
            );
            let icon_color = if is_open { tokens.primary } else { tokens.muted_foreground };
            ui.painter().rect_stroke(icon_rect, Rounding::same(2.0), Stroke::new(1.2, icon_color));
            ui.painter().line_segment(
                [Pos2::new(icon_rect.left(), icon_rect.top() + 4.0), Pos2::new(icon_rect.right(), icon_rect.top() + 4.0)],
                Stroke::new(1.2, icon_color),
            );

            ui.painter().text(
                Pos2::new(icon_rect.right() + 9.0, rect.center().y),
                egui::Align2::LEFT_CENTER,
                self.date.format_ymd(),
                FontId::monospace(self.size.font_size() - 0.5),
                if self.disabled { tokens.muted_foreground } else { tokens.foreground },
            );
        }

        if open_t > 0.01 && !self.disabled {
            if is_open && ui.input(|i| i.key_pressed(Key::Escape)) {
                is_open = false;
            }
            let offset_y = egui::lerp(0.0..=5.0, open_t);
            let before = *self.date;
            let area_resp = Area::new(popup_id.with("area"))
                .order(Order::Foreground)
                .fixed_pos(Pos2::new(rect.left(), rect.bottom() + offset_y))
                .show(ui.ctx(), |ui| {
                    ui.set_opacity(open_t);
                    ui.add(Calendar::new(self.id_source, self.date))
                });

            if *self.date != before {
                response.mark_changed();
                is_open = false;
            }

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
