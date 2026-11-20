//! Multi-line themed TextArea widget with animated focus border and live character budget indicator.

use crate::theme::active_theme;
use crate::tokens::{ease_out_cubic, lerp_color, Size};
use egui::{FontId, Pos2, Response, RichText, ScrollArea, Stroke, TextEdit, Ui, Vec2, Widget};

/// Multi-line text editor mutating `&mut String` directly.
pub struct TextArea<'a> {
    value: &'a mut String,
    placeholder: String,
    rows: usize,
    max_chars: Option<usize>,
    error: bool,
    size: Size,
    width: f32,
    disabled: bool,
}

impl<'a> TextArea<'a> {
    pub fn new(value: &'a mut String) -> Self {
        Self {
            value,
            placeholder: String::new(),
            rows: 4,
            max_chars: None,
            error: false,
            size: Size::Md,
            width: 320.0,
            disabled: false,
        }
    }

    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn rows(mut self, rows: usize) -> Self {
        self.rows = rows.max(2);
        self
    }

    pub fn max_chars(mut self, max_chars: usize) -> Self {
        self.max_chars = Some(max_chars.max(1));
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
        self.width = width.max(140.0);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl<'a> Widget for TextArea<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let tokens = active_theme(ui.ctx());
        let id = ui.make_persistent_id(("egui_kit_textarea", self.rows));
        let was_focused = ui.ctx().data(|d| d.get_temp::<bool>(id.with("focused")).unwrap_or(false));
        let focus_t = ease_out_cubic(ui.ctx().animate_bool_with_time(
            id.with("focus_anim"),
            was_focused && !self.disabled,
            0.14,
        ));

        let char_count = self.value.chars().count();
        let over_limit = self.max_chars.is_some_and(|max| char_count > max);
        let near_limit = self.max_chars.is_some_and(|max| char_count * 100 >= max * 85);
        let err_t = ease_out_cubic(ui.ctx().animate_bool_with_time(
            id.with("err_anim"),
            self.error || over_limit,
            0.14,
        ));

        let bg = if self.disabled { tokens.muted } else { tokens.background };
        let border_col = lerp_color(lerp_color(tokens.input, tokens.ring, focus_t), tokens.destructive, err_t);
        let line_h = self.size.font_size() * 1.45;
        let box_h = self.rows as f32 * line_h + 16.0;

        let (outer_rect, mut resp) = ui.allocate_exact_size(Vec2::new(self.width, box_h), egui::Sense::hover());

        if ui.is_rect_visible(outer_rect) {
            let painter = ui.painter();
            if focus_t > 0.02 {
                let halo = lerp_color(tokens.ring, tokens.destructive, err_t).linear_multiply(0.2 * focus_t);
                painter.rect_stroke(outer_rect.expand(1.5 * focus_t), self.size.rounding(), Stroke::new(2.5 * focus_t, halo));
            }
            painter.rect(outer_rect, self.size.rounding(), bg, Stroke::new(1.0 + 0.5 * focus_t, border_col));
        }

        let inner_rect = outer_rect.shrink2(Vec2::new(10.0, 8.0));
        let mut child_ui = ui.new_child(egui::UiBuilder::new().max_rect(inner_rect));
        child_ui.set_enabled(!self.disabled);

        let font = FontId::proportional(self.size.font_size());
        let edit_resp = ScrollArea::vertical()
            .id_salt(id.with("scroll"))
            .max_height(inner_rect.height())
            .show(&mut child_ui, |ui| {
                ui.add_sized(
                    inner_rect.size(),
                    TextEdit::multiline(self.value)
                        .hint_text(self.placeholder)
                        .desired_rows(self.rows)
                        .font(font)
                        .text_color(if self.disabled { tokens.muted_foreground } else { tokens.foreground })
                        .frame(false),
                )
            })
            .inner;

        ui.ctx().data_mut(|d| d.insert_temp(id.with("focused"), edit_resp.has_focus()));

        if let Some(max) = self.max_chars {
            let count_col = if over_limit {
                tokens.destructive
            } else if near_limit {
                tokens.warning
            } else {
                tokens.muted_foreground
            };
            ui.painter().text(
                Pos2::new(outer_rect.right() - 4.0, outer_rect.bottom() + 4.0),
                egui::Align2::RIGHT_TOP,
                RichText::new(format!("{char_count} / {max}")).text().to_owned(),
                FontId::monospace(11.0),
                count_col,
            );
            ui.add_space(16.0);
        }

        resp = resp.union(edit_resp);
        resp
    }
}
