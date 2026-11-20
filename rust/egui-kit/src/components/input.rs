//! Single-line themed text Input widget with animated focus ring, icon slot, password mask, and clear action.

use crate::components::icon::{paint_icon, IconName};
use crate::theme::active_theme;
use crate::tokens::{ease_out_cubic, lerp_color, Size};
use egui::{FontId, Id, Pos2, Rect, Response, Sense, Stroke, TextEdit, Ui, Vec2, Widget};

/// Single-line text input mutating `&mut String` directly.
pub struct Input<'a> {
    value: &'a mut String,
    placeholder: String,
    prefix_icon: Option<IconName>,
    clearable: bool,
    password: bool,
    error: bool,
    size: Size,
    width: f32,
    disabled: bool,
}

impl<'a> Input<'a> {
    pub fn new(value: &'a mut String) -> Self {
        Self {
            value,
            placeholder: String::new(),
            prefix_icon: None,
            clearable: false,
            password: false,
            error: false,
            size: Size::Md,
            width: 260.0,
            disabled: false,
        }
    }

    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn prefix_icon(mut self, icon: IconName) -> Self {
        self.prefix_icon = Some(icon);
        self
    }

    pub fn clearable(mut self, clearable: bool) -> Self {
        self.clearable = clearable;
        self
    }

    pub fn password(mut self, password: bool) -> Self {
        self.password = password;
        self
    }

    pub fn error(mut self, error: bool) -> Self {
        self.error = error;
        self
    }

    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width.max(80.0);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl<'a> Widget for Input<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let tokens = active_theme(ui.ctx());
        let h = self.size.height();
        let (outer_rect, mut resp) = ui.allocate_exact_size(Vec2::new(self.width, h), Sense::hover());

        let base_id = resp.id;
        let was_focused = ui.ctx().data(|d| d.get_temp::<bool>(base_id.with("focused")).unwrap_or(false));
        let focus_t = ease_out_cubic(ui.ctx().animate_bool_with_time(
            base_id.with("focus_anim"),
            was_focused && !self.disabled,
            0.14,
        ));
        let error_t = ease_out_cubic(ui.ctx().animate_bool_with_time(
            base_id.with("err_anim"),
            self.error,
            0.14,
        ));
        let show_clear = self.clearable && !self.value.is_empty() && !self.disabled;
        let clear_t = ease_out_cubic(ui.ctx().animate_bool_with_time(
            base_id.with("clear_anim"),
            show_clear,
            0.12,
        ));

        let bg = if self.disabled { tokens.muted } else { tokens.background };
        let base_border = lerp_color(tokens.input, tokens.ring, focus_t);
        let border_col = lerp_color(base_border, tokens.destructive, error_t);

        if ui.is_rect_visible(outer_rect) {
            let painter = ui.painter();
            if focus_t > 0.02 {
                let halo_col = lerp_color(tokens.ring, tokens.destructive, error_t)
                    .linear_multiply(0.22 * focus_t);
                painter.rect_stroke(
                    outer_rect.expand(1.5 * focus_t),
                    self.size.rounding(),
                    Stroke::new(2.5 * focus_t, halo_col),
                );
            }
            painter.rect(outer_rect, self.size.rounding(), bg, Stroke::new(1.0 + 0.5 * focus_t, border_col));
        }

        let px = self.size.padding_x() * 0.75;
        let icon_edge = self.size.icon_size();
        let mut left_x = outer_rect.left() + px;
        let mut right_x = outer_rect.right() - px;

        if let Some(icon) = self.prefix_icon {
            let icon_rect = Rect::from_center_size(
                Pos2::new(left_x + icon_edge * 0.5, outer_rect.center().y),
                Vec2::splat(icon_edge),
            );
            let icon_col = lerp_color(tokens.muted_foreground, tokens.foreground, focus_t);
            paint_icon(ui.painter(), icon_rect, icon, icon_col);
            left_x += icon_edge + 6.0;
        }

        let mut cleared = false;
        if clear_t > 0.05 {
            let clear_rect = Rect::from_center_size(
                Pos2::new(right_x - icon_edge * 0.5, outer_rect.center().y),
                Vec2::splat(icon_edge + 4.0),
            );
            let clear_resp = ui.interact(clear_rect, Id::new((base_id, "clear")), Sense::click());
            let c_col = if clear_resp.hovered() { tokens.foreground } else { tokens.muted_foreground };
            paint_icon(
                ui.painter(),
                Rect::from_center_size(clear_rect.center(), Vec2::splat(icon_edge * 0.85 * clear_t)),
                IconName::Close,
                c_col.linear_multiply(clear_t),
            );
            if clear_resp.clicked() {
                self.value.clear();
                cleared = true;
            }
            right_x -= icon_edge + 8.0;
        }

        let edit_rect = Rect::from_min_max(
            Pos2::new(left_x, outer_rect.top() + 2.0),
            Pos2::new(right_x.max(left_x + 24.0), outer_rect.bottom() - 2.0),
        );

        let mut edit_ui = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(edit_rect)
                .layout(egui::Layout::left_to_right(egui::Align::Center)),
        );
        edit_ui.set_enabled(!self.disabled);

        let font = FontId::proportional(self.size.font_size());
        let edit_resp = edit_ui.add_sized(
            edit_rect.size(),
            TextEdit::singleline(self.value)
                .hint_text(self.placeholder)
                .password(self.password)
                .font(font)
                .text_color(if self.disabled { tokens.muted_foreground } else { tokens.foreground })
                .frame(false),
        );

        let edit_focused = edit_resp.has_focus();
        ui.ctx().data_mut(|d| d.insert_temp(base_id.with("focused"), edit_focused));
        if cleared {
            resp.mark_changed();
        }
        resp.union(edit_resp)
    }
}
