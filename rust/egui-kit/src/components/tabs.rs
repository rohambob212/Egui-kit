//! Horizontal or vertical Tabs switcher with sliding indicator and optional closable tabs.

use crate::theme::active_theme;
use crate::tokens::{ease_out_cubic, lerp_color, Orientation};
use egui::{FontId, Id, Pos2, Rect, Response, Rounding, Sense, Stroke, Ui, Vec2};

/// Visual tab bar style for [`Tabs`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum TabsVariant {
    #[default]
    Underline,
    Pill,
    Card,
}

/// Response returned by [`Tabs::show`] including close button events.
pub struct TabsResponse {
    pub response: Response,
    pub changed: bool,
    pub closed: Option<usize>,
}

/// Interactive tab bar mutating `&mut usize` directly with smooth sliding indicator.
pub struct Tabs<'a, T: AsRef<str>> {
    id_source: &'a str,
    active: &'a mut usize,
    labels: &'a [T],
    variant: TabsVariant,
    orientation: Orientation,
    closable: bool,
}

impl<'a, T: AsRef<str>> Tabs<'a, T> {
    pub fn new(id_source: &'a str, active: &'a mut usize, labels: &'a [T]) -> Self {
        Self {
            id_source,
            active,
            labels,
            variant: TabsVariant::Underline,
            orientation: Orientation::Horizontal,
            closable: false,
        }
    }

    pub fn variant(mut self, variant: TabsVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn closable(mut self, closable: bool) -> Self {
        self.closable = closable;
        self
    }

    pub fn show(self, ui: &mut Ui) -> TabsResponse {
        let tokens = active_theme(ui.ctx());
        let base_id = Id::new(("egui_kit_tabs", self.id_source));
        let mut changed = false;
        let mut closed: Option<usize> = None;

        if self.labels.is_empty() {
            let (_, resp) = ui.allocate_exact_size(Vec2::ZERO, Sense::hover());
            return TabsResponse {
                response: resp,
                changed: false,
                closed: None,
            };
        }
        *self.active = (*self.active).min(self.labels.len() - 1);

        let mut tab_rects: Vec<Rect> = Vec::with_capacity(self.labels.len());

        let container_resp = match self.orientation {
            Orientation::Horizontal => {
                let outer = ui
                    .horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 4.0;
                        for (idx, item) in self.labels.iter().enumerate() {
                            let label = item.as_ref();
                            let is_active = *self.active == idx;
                            let text_w = ui
                                .painter()
                                .layout_no_wrap(label.to_string(), FontId::proportional(13.0), tokens.foreground)
                                .size()
                                .x;
                            let close_w = if self.closable { 18.0 } else { 0.0 };
                            let w = text_w + 24.0 + close_w;
                            let (rect, resp) = ui.allocate_exact_size(Vec2::new(w, 34.0), Sense::click());
                            tab_rects.push(rect);

                            let hov_t = ease_out_cubic(
                                ui.ctx().animate_bool_with_time(base_id.with(("h", idx)), resp.hovered(), 0.12),
                            );
                            if self.variant == TabsVariant::Pill {
                                if is_active {
                                    ui.painter().rect_filled(rect, Rounding::same(6.0), tokens.primary);
                                } else if hov_t > 0.01 {
                                    ui.painter().rect_filled(
                                        rect,
                                        Rounding::same(6.0),
                                        lerp_color(tokens.background, tokens.muted, hov_t),
                                    );
                                }
                            } else if self.variant == TabsVariant::Card {
                                let fill = if is_active {
                                    tokens.card
                                } else {
                                    lerp_color(tokens.muted, tokens.card, hov_t * 0.5)
                                };
                                let stroke = if is_active {
                                    Stroke::new(1.0, tokens.border)
                                } else {
                                    Stroke::NONE
                                };
                                ui.painter().rect(rect, Rounding::same(6.0), fill, stroke);
                            } else if hov_t > 0.01 && !is_active {
                                ui.painter().rect_filled(
                                    rect,
                                    Rounding::same(5.0),
                                    lerp_color(tokens.background, tokens.muted, hov_t * 0.6),
                                );
                            }

                            let text_color = if self.variant == TabsVariant::Pill && is_active {
                                tokens.primary_foreground
                            } else if is_active {
                                tokens.foreground
                            } else {
                                lerp_color(tokens.muted_foreground, tokens.foreground, hov_t)
                            };
                            let text_x = rect.left() + 12.0;
                            ui.painter().text(
                                Pos2::new(text_x, rect.center().y),
                                egui::Align2::LEFT_CENTER,
                                label,
                                FontId::proportional(13.0),
                                text_color,
                            );

                            let mut hit_close = false;
                            if self.closable {
                                let cr = Rect::from_center_size(
                                    Pos2::new(rect.right() - 12.0, rect.center().y),
                                    Vec2::splat(14.0),
                                );
                                let c_resp = ui.interact(cr, base_id.with(("x", idx)), Sense::click());
                                ui.painter().text(
                                    cr.center(),
                                    egui::Align2::CENTER_CENTER,
                                    "×",
                                    FontId::proportional(13.0),
                                    text_color,
                                );
                                if c_resp.clicked() {
                                    closed = Some(idx);
                                    hit_close = true;
                                }
                            }

                            if resp.clicked() && !hit_close && *self.active != idx {
                                *self.active = idx;
                                changed = true;
                            }
                        }
                    })
                    .response;

                if self.variant == TabsVariant::Underline && !tab_rects.is_empty() {
                    let bar_y = outer.rect.bottom();
                    ui.painter().line_segment(
                        [Pos2::new(outer.rect.left(), bar_y), Pos2::new(outer.rect.right(), bar_y)],
                        Stroke::new(1.0, tokens.border),
                    );
                    let target = tab_rects[*self.active];
                    let anim_x = ui.ctx().animate_value_with_time(base_id.with("ind_x"), target.left(), 0.16);
                    let anim_w = ui.ctx().animate_value_with_time(base_id.with("ind_w"), target.width(), 0.16);
                    let ind = Rect::from_min_size(Pos2::new(anim_x, bar_y - 2.0), Vec2::new(anim_w, 2.0));
                    ui.painter().rect_filled(ind, Rounding::same(1.0), tokens.primary);
                }
                outer
            }
            Orientation::Vertical => ui
                .vertical(|ui| {
                    ui.spacing_mut().item_spacing.y = 4.0;
                    for (idx, item) in self.labels.iter().enumerate() {
                        let label = item.as_ref();
                        let is_active = *self.active == idx;
                        let (rect, resp) = ui.allocate_exact_size(Vec2::new(168.0, 32.0), Sense::click());
                        let bg = if is_active { tokens.accent } else { tokens.background };
                        ui.painter().rect_filled(rect, Rounding::same(6.0), bg);
                        if is_active {
                            let rail = Rect::from_min_size(rect.min + Vec2::new(0.0, 6.0), Vec2::new(3.0, rect.height() - 12.0));
                            ui.painter().rect_filled(rail, Rounding::same(1.5), tokens.primary);
                        }
                        ui.painter().text(
                            Pos2::new(rect.left() + 12.0, rect.center().y),
                            egui::Align2::LEFT_CENTER,
                            label,
                            FontId::proportional(13.0),
                            if is_active { tokens.foreground } else { tokens.muted_foreground },
                        );
                        if resp.clicked() && *self.active != idx {
                            *self.active = idx;
                            changed = true;
                        }
                    }
                })
                .response,
        };

        TabsResponse {
            response: container_resp,
            changed,
            closed,
        }
    }
}
