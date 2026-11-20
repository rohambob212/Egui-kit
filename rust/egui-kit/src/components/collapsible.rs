//! Single toggleable Collapsible section with animated chevron and body reveal.

use crate::theme::active_theme;
use crate::tokens::{ease_out_cubic, lerp_color};
use egui::{FontId, Frame, Id, Margin, Pos2, Rounding, Sense, Stroke, Ui, Vec2};

/// Single disclosure container persisting expanded state in `egui::Memory` or `&mut bool`.
pub struct Collapsible<'a> {
    id_source: &'a str,
    title: String,
    external_open: Option<&'a mut bool>,
    default_open: bool,
}

impl<'a> Collapsible<'a> {
    pub fn new(id_source: &'a str, title: impl Into<String>) -> Self {
        Self {
            id_source,
            title: title.into(),
            external_open: None,
            default_open: false,
        }
    }

    /// Optional direct `&mut bool` binding; when omitted, state lives in `egui::Memory`.
    pub fn open(mut self, open: &'a mut bool) -> Self {
        self.external_open = Some(open);
        self
    }

    pub fn default_open(mut self, default_open: bool) -> Self {
        self.default_open = default_open;
        self
    }

    pub fn show(
        mut self,
        ui: &mut Ui,
        add_contents: impl FnOnce(&mut Ui),
    ) -> bool {
        let tokens = active_theme(ui.ctx());
        let mem_id = Id::new(("egui_kit_collapsible", self.id_source));

        let mut is_open = match self.external_open.as_deref() {
            Some(b) => *b,
            None => ui
                .ctx()
                .data(|d| d.get_temp::<bool>(mem_id))
                .unwrap_or(self.default_open),
        };

        Frame::none()
            .fill(tokens.card)
            .stroke(Stroke::new(1.0, tokens.border))
            .rounding(Rounding::same(8.0))
            .show(ui, |ui| {
                let avail_w = ui.available_width().max(200.0);
                let (hdr_rect, hdr_resp) = ui.allocate_exact_size(Vec2::new(avail_w, 40.0), Sense::click());
                if hdr_resp.clicked() {
                    is_open = !is_open;
                }

                let hov_t = ease_out_cubic(ui.ctx().animate_bool_with_time(mem_id.with("hov"), hdr_resp.hovered(), 0.12));
                let open_t = ease_out_cubic(ui.ctx().animate_bool_with_time(mem_id.with("open"), is_open, 0.18));

                if ui.is_rect_visible(hdr_rect) {
                    if hov_t > 0.01 {
                        ui.painter().rect_filled(
                            hdr_rect,
                            Rounding::same(8.0),
                            lerp_color(tokens.card, tokens.muted, hov_t * 0.55),
                        );
                    }
                    ui.painter().text(
                        Pos2::new(hdr_rect.left() + 14.0, hdr_rect.center().y),
                        egui::Align2::LEFT_CENTER,
                        &self.title,
                        FontId::proportional(13.5),
                        if is_open { tokens.foreground } else { tokens.card_foreground },
                    );

                    let c = Pos2::new(hdr_rect.right() - 20.0, hdr_rect.center().y);
                    let angle = open_t * std::f32::consts::FRAC_PI_2;
                    let rot = |v: Vec2| -> Vec2 {
                        Vec2::new(v.x * angle.cos() - v.y * angle.sin(), v.x * angle.sin() + v.y * angle.cos())
                    };
                    let col = lerp_color(tokens.muted_foreground, tokens.primary, open_t);
                    ui.painter().line_segment([c + rot(Vec2::new(-2.0, -4.0)), c + rot(Vec2::new(2.5, 0.0))], Stroke::new(1.6, col));
                    ui.painter().line_segment([c + rot(Vec2::new(2.5, 0.0)), c + rot(Vec2::new(-2.0, 4.0))], Stroke::new(1.6, col));
                }

                if open_t > 0.02 {
                    let line_rect = ui.available_rect_before_wrap();
                    ui.painter().line_segment(
                        [line_rect.left_top(), Pos2::new(line_rect.left() + avail_w, line_rect.top())],
                        Stroke::new(1.0, tokens.border),
                    );
                    ui.scope(|ui| {
                        ui.set_opacity(open_t);
                        Frame::none()
                            .inner_margin(Margin::same(12.0))
                            .show(ui, |ui| {
                                ui.set_width((avail_w - 24.0).max(120.0));
                                add_contents(ui);
                            });
                    });
                }
            });

        if let Some(ext) = self.external_open {
            *ext = is_open;
        }
        ui.ctx().data_mut(|d| d.insert_temp(mem_id, is_open));
        is_open
    }
}
