//! Searchable Combobox widget with live option filtering, clearable selection, and animated popup.

use crate::components::icon::{paint_icon, IconName};
use crate::theme::active_theme;
use crate::tokens::{ease_out_cubic, lerp_color, Size};
use egui::{Area, FontId, Frame, Id, Key, Order, Pos2, Rect, Response, Sense, Stroke, TextEdit, Ui, Vec2, Widget};

/// Searchable autocomplete select widget mutating `&mut Option<usize>` directly.
pub struct Combobox<'a, T: AsRef<str>> {
    id_source: &'a str,
    selected: &'a mut Option<usize>,
    options: &'a [T],
    placeholder: String,
    search_hint: String,
    clearable: bool,
    size: Size,
    width: f32,
    disabled: bool,
}

impl<'a, T: AsRef<str>> Combobox<'a, T> {
    pub fn new(id_source: &'a str, selected: &'a mut Option<usize>, options: &'a [T]) -> Self {
        Self {
            id_source,
            selected,
            options,
            placeholder: "Choose item...".to_string(),
            search_hint: "Filter options...".to_string(),
            clearable: true,
            size: Size::Md,
            width: 260.0,
            disabled: false,
        }
    }

    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn search_hint(mut self, hint: impl Into<String>) -> Self {
        self.search_hint = hint.into();
        self
    }

    pub fn clearable(mut self, clearable: bool) -> Self {
        self.clearable = clearable;
        self
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

impl<'a, T: AsRef<str>> Widget for Combobox<'a, T> {
    fn ui(self, ui: &mut Ui) -> Response {
        let tokens = active_theme(ui.ctx());
        let popup_id = Id::new(("egui_kit_combobox", self.id_source));
        let query_id = popup_id.with("query");
        let mut is_open = ui.ctx().data(|d| d.get_temp::<bool>(popup_id).unwrap_or(false));
        let mut query = ui.ctx().data(|d| d.get_temp::<String>(query_id).unwrap_or_default());

        let sense = if self.disabled { Sense::hover() } else { Sense::click() };
        let (rect, mut response) = ui.allocate_exact_size(Vec2::new(self.width, self.size.height()), sense);

        let open_t = ease_out_cubic(ui.ctx().animate_bool_with_time(popup_id.with("open"), is_open && !self.disabled, 0.16));
        let hover_t = ui.ctx().animate_bool_with_time(popup_id.with("hov"), (response.hovered() || is_open) && !self.disabled, 0.12);

        let selected_label = self.selected.and_then(|idx| self.options.get(idx)).map(|o| o.as_ref());
        let display = selected_label.unwrap_or(&self.placeholder);
        let fg = if selected_label.is_some() && !self.disabled { tokens.foreground } else { tokens.muted_foreground };
        let bg = if self.disabled { tokens.muted } else { lerp_color(tokens.background, tokens.accent, hover_t * 0.4) };

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();
            painter.rect(rect, self.size.rounding(), bg, Stroke::new(1.0 + 0.5 * open_t, lerp_color(tokens.input, tokens.ring, hover_t.max(open_t))));
            let font = FontId::proportional(self.size.font_size());
            let galley = painter.layout_no_wrap(display.to_string(), font, fg);
            painter.galley(Pos2::new(rect.left() + self.size.padding_x(), rect.center().y - galley.size().y * 0.5), galley, fg);
            paint_icon(
                painter,
                Rect::from_center_size(Pos2::new(rect.right() - self.size.padding_x() - 6.0, rect.center().y), Vec2::splat(self.size.icon_size())),
                IconName::ChevronDown,
                tokens.muted_foreground,
            );
        }

        if response.clicked() && !self.disabled {
            is_open = !is_open;
        }

        if open_t > 0.01 && !self.disabled {
            if is_open && ui.input(|i| i.key_pressed(Key::Escape)) {
                is_open = false;
            }
            let offset_y = egui::lerp(0.0..=4.0, open_t);
            let area_resp = Area::new(popup_id.with("menu"))
                .order(Order::Foreground)
                .fixed_pos(Pos2::new(rect.left(), rect.bottom() + offset_y))
                .show(ui.ctx(), |ui| {
                    ui.set_opacity(open_t);
                    Frame::none()
                        .fill(tokens.card)
                        .stroke(Stroke::new(1.0, tokens.border))
                        .rounding(self.size.rounding())
                        .inner_margin(egui::Margin::same(6.0))
                        .show(ui, |ui| {
                            ui.set_min_width(self.width - 12.0);
                            ui.horizontal(|ui| {
                                ui.add(crate::components::Icon::new(IconName::Search).size(Size::Sm).color(tokens.muted_foreground));
                                ui.add(
                                    TextEdit::singleline(&mut query)
                                        .hint_text(&self.search_hint)
                                        .frame(false)
                                        .desired_width(self.width - 64.0),
                                );
                            });
                            ui.separator();

                            let needle = query.trim().to_lowercase();
                            let mut matches = 0usize;
                            for (idx, opt) in self.options.iter().enumerate() {
                                if !needle.is_empty() && !opt.as_ref().to_lowercase().contains(&needle) {
                                    continue;
                                }
                                matches += 1;
                                let is_sel = *self.selected == Some(idx);
                                let (row_rect, row_resp) = ui.allocate_exact_size(Vec2::new(self.width - 16.0, 28.0), Sense::click());
                                if row_resp.hovered() || is_sel {
                                    ui.painter().rect_filled(row_rect, self.size.rounding(), if is_sel { tokens.accent } else { tokens.muted });
                                }
                                ui.painter().text(
                                    Pos2::new(row_rect.left() + 8.0, row_rect.center().y),
                                    egui::Align2::LEFT_CENTER,
                                    opt.as_ref(),
                                    FontId::proportional(self.size.font_size()),
                                    tokens.card_foreground,
                                );
                                if is_sel {
                                    paint_icon(
                                        ui.painter(),
                                        Rect::from_center_size(Pos2::new(row_rect.right() - 12.0, row_rect.center().y), Vec2::splat(13.0)),
                                        IconName::Check,
                                        tokens.primary,
                                    );
                                }
                                if row_resp.clicked() && is_open {
                                    *self.selected = Some(idx);
                                    query.clear();
                                    is_open = false;
                                    response.mark_changed();
                                }
                            }
                            if matches == 0 {
                                ui.label(egui::RichText::new("No matching options").small().color(tokens.muted_foreground));
                            } else if self.clearable && self.selected.is_some() {
                                ui.separator();
                                if ui.button("Clear selection").clicked() {
                                    *self.selected = None;
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

        ui.ctx().data_mut(|d| {
            d.insert_temp(popup_id, is_open);
            d.insert_temp(query_id, query);
        });
        response
    }
}
