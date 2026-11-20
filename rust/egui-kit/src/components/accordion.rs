//! Stacked Accordion disclosure widget mutating `&mut Option<usize>` with animated chevrons.

use crate::theme::active_theme;
use crate::tokens::{ease_out_cubic, lerp_color};
use egui::{FontId, Frame, Id, Margin, Pos2, Response, Rounding, Sense, Stroke, Ui, Vec2, Widget};

/// Single collapsible section definition for [`Accordion`].
#[derive(Debug, Clone)]
pub struct AccordionItem<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

impl<'a> AccordionItem<'a> {
    pub const fn new(title: &'a str, body: &'a str) -> Self {
        Self { title, body }
    }
}

/// Vertically stacked disclosure list supporting single active index (`&mut Option<usize>`).
pub struct Accordion<'a> {
    id_source: &'a str,
    open_idx: &'a mut Option<usize>,
    items: &'a [AccordionItem<'a>],
    allow_collapse_all: bool,
}

impl<'a> Accordion<'a> {
    pub fn new(id_source: &'a str, open_idx: &'a mut Option<usize>, items: &'a [AccordionItem<'a>]) -> Self {
        Self {
            id_source,
            open_idx,
            items,
            allow_collapse_all: true,
        }
    }

    pub fn allow_collapse_all(mut self, allow: bool) -> Self {
        self.allow_collapse_all = allow;
        self
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        ui.add(self)
    }
}

impl<'a> Widget for Accordion<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let tokens = active_theme(ui.ctx());
        let base_id = Id::new(("egui_kit_accordion", self.id_source));
        let mut changed = false;

        let frame_resp = Frame::none()
            .fill(tokens.card)
            .stroke(Stroke::new(1.0, tokens.border))
            .rounding(Rounding::same(8.0))
            .show(ui, |ui| {
                let avail_w = ui.available_width().max(220.0);
                for (idx, item) in self.items.iter().enumerate() {
                    if idx > 0 {
                        let rect = ui.available_rect_before_wrap();
                        ui.painter().line_segment(
                            [rect.left_top(), Pos2::new(rect.left() + avail_w, rect.top())],
                            Stroke::new(1.0, tokens.border),
                        );
                    }

                    let is_open = *self.open_idx == Some(idx);
                    let row_id = base_id.with(idx);
                    let (hdr_rect, hdr_resp) = ui.allocate_exact_size(Vec2::new(avail_w, 40.0), Sense::click());
                    let hov_t = ease_out_cubic(ui.ctx().animate_bool_with_time(row_id.with("hov"), hdr_resp.hovered(), 0.12));
                    let open_t = ease_out_cubic(ui.ctx().animate_bool_with_time(row_id.with("open"), is_open, 0.18));

                    if hdr_resp.clicked() {
                        if is_open && self.allow_collapse_all {
                            *self.open_idx = None;
                        } else {
                            *self.open_idx = Some(idx);
                        }
                        changed = true;
                    }

                    if ui.is_rect_visible(hdr_rect) {
                        if hov_t > 0.01 {
                            ui.painter().rect_filled(
                                hdr_rect,
                                Rounding::ZERO,
                                lerp_color(tokens.card, tokens.muted, hov_t * 0.6),
                            );
                        }
                        ui.painter().text(
                            Pos2::new(hdr_rect.left() + 14.0, hdr_rect.center().y),
                            egui::Align2::LEFT_CENTER,
                            item.title,
                            FontId::proportional(13.5),
                            if is_open { tokens.primary } else { tokens.card_foreground },
                        );

                        // Rotating chevron vector glyph (0 -> 90 deg)
                        let c = Pos2::new(hdr_rect.right() - 20.0, hdr_rect.center().y);
                        let angle = open_t * std::f32::consts::FRAC_PI_2;
                        let rot = |v: Vec2| -> Vec2 {
                            Vec2::new(v.x * angle.cos() - v.y * angle.sin(), v.x * angle.sin() + v.y * angle.cos())
                        };
                        let col = lerp_color(tokens.muted_foreground, tokens.primary, open_t);
                        let p1 = c + rot(Vec2::new(-2.0, -4.0));
                        let p2 = c + rot(Vec2::new(2.5, 0.0));
                        let p3 = c + rot(Vec2::new(-2.0, 4.0));
                        ui.painter().line_segment([p1, p2], Stroke::new(1.6, col));
                        ui.painter().line_segment([p2, p3], Stroke::new(1.6, col));
                    }

                    if open_t > 0.02 {
                        ui.scope(|ui| {
                            ui.set_opacity(open_t);
                            Frame::none()
                                .inner_margin(Margin {
                                    left: 14.0,
                                    right: 14.0,
                                    top: 2.0,
                                    bottom: 12.0 * open_t,
                                })
                                .show(ui, |ui| {
                                    ui.set_width((avail_w - 28.0).max(120.0));
                                    ui.label(
                                        egui::RichText::new(item.body)
                                            .font(FontId::proportional(12.5))
                                            .color(tokens.muted_foreground),
                                    );
                                });
                        });
                    }
                }
            });

        let mut resp = frame_resp.response;
        if changed {
            resp.mark_changed();
        }
        resp
    }
}
