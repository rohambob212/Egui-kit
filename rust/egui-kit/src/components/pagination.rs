//! Compact Pagination bar with smart ellipsis windowing mutating `&mut usize`.

use crate::theme::active_theme;
use crate::tokens::{ease_out_cubic, lerp_color};
use egui::{FontId, Id, Response, Rounding, Sense, Stroke, Ui, Vec2, Widget};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PageSlot {
    Page(usize),
    Ellipsis,
}

/// Interactive 1-based Pagination bar mutating `&mut usize` directly.
pub struct Pagination<'a> {
    page: &'a mut usize,
    total_pages: usize,
    sibling_count: usize,
}

impl<'a> Pagination<'a> {
    pub fn new(page: &'a mut usize, total_pages: usize) -> Self {
        Self {
            page,
            total_pages: total_pages.max(1),
            sibling_count: 1,
        }
    }

    pub fn sibling_count(mut self, count: usize) -> Self {
        self.sibling_count = count.clamp(0, 3);
        self
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        ui.add(self)
    }

    fn build_slots(current: usize, total: usize, siblings: usize) -> Vec<PageSlot> {
        if total <= 5 + siblings * 2 {
            return (1..=total).map(PageSlot::Page).collect();
        }
        let mut out = vec![PageSlot::Page(1)];
        let left = current.saturating_sub(siblings).max(2);
        let right = (current + siblings).min(total - 1);

        if left > 2 {
            out.push(PageSlot::Ellipsis);
        }
        for p in left..=right {
            out.push(PageSlot::Page(p));
        }
        if right < total - 1 {
            out.push(PageSlot::Ellipsis);
        }
        out.push(PageSlot::Page(total));
        out
    }
}

impl<'a> Widget for Pagination<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let tokens = active_theme(ui.ctx());
        let total = self.total_pages;
        *self.page = (*self.page).clamp(1, total);
        let current = *self.page;
        let slots = Self::build_slots(current, total, self.sibling_count);
        let mut changed = false;
        let base_id = Id::new("egui_kit_pagination");

        let mut resp = ui
            .horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 4.0;

                // Previous button
                let can_prev = current > 1;
                let (prev_r, prev_resp) = ui.allocate_exact_size(
                    Vec2::new(32.0, 30.0),
                    if can_prev { Sense::click() } else { Sense::hover() },
                );
                let prev_bg = if can_prev && prev_resp.hovered() { tokens.muted } else { tokens.card };
                ui.painter().rect(prev_r, Rounding::same(6.0), prev_bg, Stroke::new(1.0, tokens.border));
                ui.painter().text(
                    prev_r.center(),
                    egui::Align2::CENTER_CENTER,
                    "‹",
                    FontId::proportional(15.0),
                    if can_prev { tokens.foreground } else { tokens.muted_foreground },
                );
                if can_prev && prev_resp.clicked() {
                    *self.page = current - 1;
                    changed = true;
                }

                // Numbered & ellipsis slots
                for (slot_idx, slot) in slots.into_iter().enumerate() {
                    match slot {
                        PageSlot::Ellipsis => {
                            let (er, _) = ui.allocate_exact_size(Vec2::new(26.0, 30.0), Sense::hover());
                            ui.painter().text(
                                er.center(),
                                egui::Align2::CENTER_CENTER,
                                "…",
                                FontId::monospace(12.5),
                                tokens.muted_foreground,
                            );
                        }
                        PageSlot::Page(p) => {
                            let is_active = p == current;
                            let (pr, p_resp) = ui.allocate_exact_size(Vec2::new(32.0, 30.0), Sense::click());
                            let hov_t = ease_out_cubic(
                                ui.ctx().animate_bool_with_time(base_id.with(("p", slot_idx, p)), p_resp.hovered(), 0.12),
                            );
                            let bg = if is_active {
                                tokens.primary
                            } else {
                                lerp_color(tokens.card, tokens.muted, hov_t)
                            };
                            let fg = if is_active {
                                tokens.primary_foreground
                            } else {
                                tokens.foreground
                            };
                            let border = if is_active { tokens.primary } else { tokens.border };
                            ui.painter().rect(pr, Rounding::same(6.0), bg, Stroke::new(1.0, border));
                            ui.painter().text(
                                pr.center(),
                                egui::Align2::CENTER_CENTER,
                                format!("{}", p),
                                FontId::monospace(12.5),
                                fg,
                            );
                            if p_resp.clicked() && !is_active {
                                *self.page = p;
                                changed = true;
                            }
                        }
                    }
                }

                // Next button
                let can_next = current < total;
                let (next_r, next_resp) = ui.allocate_exact_size(
                    Vec2::new(32.0, 30.0),
                    if can_next { Sense::click() } else { Sense::hover() },
                );
                let next_bg = if can_next && next_resp.hovered() { tokens.muted } else { tokens.card };
                ui.painter().rect(next_r, Rounding::same(6.0), next_bg, Stroke::new(1.0, tokens.border));
                ui.painter().text(
                    next_r.center(),
                    egui::Align2::CENTER_CENTER,
                    "›",
                    FontId::proportional(15.0),
                    if can_next { tokens.foreground } else { tokens.muted_foreground },
                );
                if can_next && next_resp.clicked() {
                    *self.page = current + 1;
                    changed = true;
                }
            })
            .response;

        if changed {
            resp.mark_changed();
        }
        resp
    }
}
