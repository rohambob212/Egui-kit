//! DropdownButton widget combining a primary button trigger with an anchored popup menu.

use crate::theme::active_theme;
use crate::tokens::{ease_out_cubic, lerp_color, Size};
use egui::{Area, Frame, Id, Order, Pos2, Response, RichText, Sense, Stroke, Ui, Vec2, Widget};

/// Single selectable action inside a [`DropdownButton`] menu.
#[derive(Debug, Clone)]
pub struct DropdownItem {
    pub id: String,
    pub label: String,
    pub shortcut: Option<String>,
    pub destructive: bool,
    pub disabled: bool,
}

impl DropdownItem {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            shortcut: None,
            destructive: false,
            disabled: false,
        }
    }

    pub fn shortcut(mut self, shortcut: impl Into<String>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    pub fn destructive(mut self, destructive: bool) -> Self {
        self.destructive = destructive;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// Interactive trigger button with an anchored dropdown menu persisted via `egui::Memory`.
pub struct DropdownButton<'a> {
    id_source: &'a str,
    label: String,
    items: Vec<DropdownItem>,
    selected_id: &'a mut Option<String>,
    size: Size,
    disabled: bool,
}

impl<'a> DropdownButton<'a> {
    pub fn new(
        id_source: &'a str,
        label: impl Into<String>,
        items: Vec<DropdownItem>,
        selected_id: &'a mut Option<String>,
    ) -> Self {
        Self {
            id_source,
            label: label.into(),
            items,
            selected_id,
            size: Size::Md,
            disabled: false,
        }
    }

    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl<'a> Widget for DropdownButton<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let tokens = active_theme(ui.ctx());
        let popup_id = Id::new(("egui_kit_dropdown", self.id_source));
        let mut is_open = ui
            .ctx()
            .data(|d| d.get_temp::<bool>(popup_id).unwrap_or(false));

        let font_id = egui::FontId::proportional(self.size.font_size());
        let galley = ui
            .painter()
            .layout_no_wrap(self.label.clone(), font_id.clone(), tokens.foreground);
        let chevron_w = self.size.icon_size();
        let width = galley.size().x + self.size.padding_x() * 2.0 + chevron_w + 8.0;
        let (rect, response) =
            ui.allocate_exact_size(Vec2::new(width, self.size.height()), Sense::click());

        if response.clicked() && !self.disabled {
            is_open = !is_open;
        }

        let open_t = ease_out_cubic(ui.ctx().animate_bool_with_time(
            popup_id.with("open_anim"),
            is_open && !self.disabled,
            0.16,
        ));
        let hover_t = ui.ctx().animate_bool_with_time(
            popup_id.with("hover_anim"),
            (response.hovered() || is_open) && !self.disabled,
            0.12,
        );

        let bg = if self.disabled {
            tokens.muted
        } else {
            lerp_color(tokens.secondary, tokens.accent, hover_t)
        };

        let fg = if self.disabled {
            tokens.muted_foreground
        } else {
            tokens.secondary_foreground
        };

        let painter = ui.painter();
        painter.rect(rect, self.size.rounding(), bg, Stroke::new(1.0, tokens.border));
        painter.galley(
            egui::pos2(
                rect.left() + self.size.padding_x(),
                rect.center().y - galley.size().y * 0.5,
            ),
            galley,
            fg,
        );

        // Smoothly flip chevron from pointing down (0.0) to up (1.0)
        let chev_center = Pos2::new(
            rect.right() - self.size.padding_x() - chevron_w * 0.5,
            rect.center().y,
        );
        let r = chevron_w * 0.34;
        let tip_y = egui::lerp(r * 0.45..=-r * 0.45, open_t);
        let wing_y = egui::lerp(-r * 0.3..=r * 0.3, open_t);
        let chev_stroke = Stroke::new((chevron_w * 0.1).clamp(1.25, 2.0), fg);
        painter.line_segment([chev_center + Vec2::new(-r * 0.85, wing_y), chev_center + Vec2::new(0.0, tip_y)], chev_stroke);
        painter.line_segment([chev_center + Vec2::new(0.0, tip_y), chev_center + Vec2::new(r * 0.85, wing_y)], chev_stroke);

        if open_t > 0.01 && !self.disabled {
            if is_open && ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                is_open = false;
            }

            let offset_y = egui::lerp(0.0..=4.0, open_t);
            let area_resp = Area::new(popup_id.with("menu"))
                .order(Order::Foreground)
                .fixed_pos(egui::pos2(rect.left(), rect.bottom() + offset_y))
                .show(ui.ctx(), |ui| {
                    ui.set_opacity(open_t);
                    Frame::none()
                        .fill(tokens.card)
                        .stroke(Stroke::new(1.0, tokens.border))
                        .rounding(self.size.rounding())
                        .inner_margin(egui::Margin::same(4.0))
                        .show(ui, |ui| {
                            ui.set_min_width(180.0);
                            for item in &self.items {
                                let text_color = if item.disabled {
                                    tokens.muted_foreground
                                } else if item.destructive {
                                    tokens.destructive
                                } else {
                                    tokens.card_foreground
                                };

                                let row_resp = ui.horizontal(|ui| {
                                    let btn = ui.add_enabled(
                                        !item.disabled,
                                        egui::Button::new(
                                            RichText::new(&item.label).color(text_color),
                                        )
                                        .frame(false),
                                    );
                                    if let Some(sc) = &item.shortcut {
                                        ui.with_layout(
                                            egui::Layout::right_to_left(egui::Align::Center),
                                            |ui| {
                                                ui.label(
                                                    RichText::new(sc)
                                                        .small()
                                                        .monospace()
                                                        .color(tokens.muted_foreground),
                                                );
                                            },
                                        );
                                    }
                                    btn
                                });

                                if row_resp.inner.clicked() && is_open {
                                    *self.selected_id = Some(item.id.clone());
                                    is_open = false;
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
