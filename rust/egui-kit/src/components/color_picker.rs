//! Interactive HSV ColorPicker rendered via `egui::Mesh` mutating `&mut Color32`.

use crate::theme::active_theme;
use egui::{ecolor::Hsva, Color32, FontId, Frame, Id, Mesh, Pos2, Rect, Response, Rounding, Sense, Stroke, Ui, Vec2, Widget};

/// Interactive HSV saturation-value square, hue bar, alpha bar, hex readout, and swatches.
pub struct ColorPicker<'a> {
    id_source: &'a str,
    color: &'a mut Color32,
    show_alpha: bool,
    width: f32,
}

impl<'a> ColorPicker<'a> {
    pub fn new(id_source: &'a str, color: &'a mut Color32) -> Self {
        Self {
            id_source,
            color,
            show_alpha: true,
            width: 240.0,
        }
    }

    pub fn show_alpha(mut self, show: bool) -> Self {
        self.show_alpha = show;
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width.clamp(180.0, 320.0);
        self
    }
}

impl<'a> Widget for ColorPicker<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let tokens = active_theme(ui.ctx());
        let hsv_id = Id::new(("egui_kit_color_hsv", self.id_source));
        let mut hsv = Hsva::from(*self.color);
        if let Some(prev) = ui.ctx().data(|d| d.get_temp::<Hsva>(hsv_id)) {
            if hsv.a == 0.0 {
                hsv.h = prev.h;
                hsv.s = prev.s;
                hsv.v = prev.v;
            } else if hsv.v == 0.0 || hsv.s == 0.0 {
                hsv.h = prev.h;
            }
        }

        let mut changed = false;
        let inner_w = self.width - 20.0;

        let frame_resp = Frame::none()
            .fill(tokens.card)
            .stroke(Stroke::new(1.0, tokens.border))
            .rounding(Rounding::same(8.0))
            .inner_margin(egui::Margin::same(10.0))
            .show(ui, |ui| {
                ui.set_width(inner_w);

                // 2D Saturation-Value gradient mesh
                let sv_h = 136.0;
                let (sv_rect, sv_resp) = ui.allocate_exact_size(Vec2::new(inner_w, sv_h), Sense::click_and_drag());
                if sv_resp.dragged() || sv_resp.clicked() {
                    if let Some(pos) = sv_resp.interact_pointer_pos() {
                        hsv.s = ((pos.x - sv_rect.left()) / sv_rect.width()).clamp(0.0, 1.0);
                        hsv.v = (1.0 - (pos.y - sv_rect.top()) / sv_rect.height()).clamp(0.0, 1.0);
                        changed = true;
                    }
                }

                if ui.is_rect_visible(sv_rect) {
                    let pure_hue = Color32::from(Hsva::new(hsv.h, 1.0, 1.0, 1.0));
                    let mut mesh = Mesh::default();
                    let tl = mesh.colored_vertex(sv_rect.left_top(), Color32::WHITE);
                    let tr = mesh.colored_vertex(sv_rect.right_top(), pure_hue);
                    let bl = mesh.colored_vertex(sv_rect.left_bottom(), Color32::BLACK);
                    let br = mesh.colored_vertex(sv_rect.right_bottom(), Color32::BLACK);
                    mesh.add_triangle(tl, tr, br);
                    mesh.add_triangle(tl, br, bl);
                    ui.painter().add(mesh);
                    ui.painter().rect_stroke(sv_rect, Rounding::same(4.0), Stroke::new(1.0, tokens.border));

                    let handle = Pos2::new(
                        sv_rect.left() + hsv.s * sv_rect.width(),
                        sv_rect.top() + (1.0 - hsv.v) * sv_rect.height(),
                    );
                    ui.painter().circle_stroke(handle, 6.0, Stroke::new(2.0, Color32::WHITE));
                    ui.painter().circle_stroke(handle, 7.2, Stroke::new(1.0, Color32::from_black_alpha(150)));
                }

                ui.add_space(10.0);

                // Hue spectrum bar with expanded hit target and double-ring thumb
                let (hue_hit, hue_resp) = ui.allocate_exact_size(Vec2::new(inner_w, 20.0), Sense::click_and_drag());
                let hue_rect = Rect::from_center_size(hue_hit.center(), Vec2::new(inner_w, 12.0));
                let thumb_pad = 6.0;
                let hue_span = (hue_rect.width() - thumb_pad * 2.0).max(1.0);
                if hue_resp.dragged() || hue_resp.clicked() {
                    if let Some(pos) = hue_resp.interact_pointer_pos() {
                        hsv.h = ((pos.x - (hue_rect.left() + thumb_pad)) / hue_span).clamp(0.0, 1.0);
                        changed = true;
                    }
                }
                if ui.is_rect_visible(hue_rect) {
                    let steps = 12;
                    let mut mesh = Mesh::default();
                    for i in 0..=steps {
                        let frac = i as f32 / steps as f32;
                        let x = hue_rect.left() + frac * hue_rect.width();
                        let col = Color32::from(Hsva::new(frac, 1.0, 1.0, 1.0));
                        mesh.colored_vertex(Pos2::new(x, hue_rect.top()), col);
                        mesh.colored_vertex(Pos2::new(x, hue_rect.bottom()), col);
                        if i > 0 {
                            let base = (i as u32 - 1) * 2;
                            mesh.add_triangle(base, base + 2, base + 3);
                            mesh.add_triangle(base, base + 3, base + 1);
                        }
                    }
                    ui.painter().add(mesh);
                    ui.painter().rect_stroke(hue_rect, Rounding::same(3.0), Stroke::new(1.0, tokens.border));
                    let hx = hue_rect.left() + thumb_pad + hsv.h * hue_span;
                    let h_pos = Pos2::new(hx, hue_rect.center().y);
                    ui.painter().circle_filled(h_pos, 5.5, Color32::from(Hsva::new(hsv.h, 1.0, 1.0, 1.0)));
                    ui.painter().circle_stroke(h_pos, 6.0, Stroke::new(2.0, Color32::WHITE));
                    ui.painter().circle_stroke(h_pos, 7.2, Stroke::new(1.0, Color32::from_black_alpha(150)));
                }

                if self.show_alpha {
                    ui.add_space(6.0);
                    let (a_hit, a_resp) = ui.allocate_exact_size(Vec2::new(inner_w, 20.0), Sense::click_and_drag());
                    let a_rect = Rect::from_center_size(a_hit.center(), Vec2::new(inner_w, 12.0));
                    let a_span = (a_rect.width() - thumb_pad * 2.0).max(1.0);
                    if a_resp.dragged() || a_resp.clicked() {
                        if let Some(pos) = a_resp.interact_pointer_pos() {
                            hsv.a = ((pos.x - (a_rect.left() + thumb_pad)) / a_span).clamp(0.0, 1.0);
                            changed = true;
                        }
                    }
                    if ui.is_rect_visible(a_rect) {
                        // Subtle checkerboard backdrop for transparency perception
                        let cell = 6.0;
                        let cols = (a_rect.width() / cell).ceil() as i32;
                        for cx in 0..cols {
                            for cy in 0..2 {
                                let x0 = a_rect.left() + (cx as f32) * cell;
                                let y0 = a_rect.top() + (cy as f32) * cell;
                                let tile = Rect::from_min_max(
                                    Pos2::new(x0, y0),
                                    Pos2::new((x0 + cell).min(a_rect.right()), (y0 + cell).min(a_rect.bottom())),
                                );
                                let light = (cx + cy) % 2 == 0;
                                let c = if light { tokens.background } else { tokens.muted };
                                ui.painter().rect_filled(tile, Rounding::ZERO, c);
                            }
                        }

                        let solid = Color32::from(Hsva::new(hsv.h, hsv.s, hsv.v, 1.0));
                        let mut mesh = Mesh::default();
                        let l0 = mesh.colored_vertex(a_rect.left_top(), Color32::TRANSPARENT);
                        let l1 = mesh.colored_vertex(a_rect.left_bottom(), Color32::TRANSPARENT);
                        let r0 = mesh.colored_vertex(a_rect.right_top(), solid);
                        let r1 = mesh.colored_vertex(a_rect.right_bottom(), solid);
                        mesh.add_triangle(l0, r0, r1);
                        mesh.add_triangle(l0, r1, l1);
                        ui.painter().add(mesh);
                        ui.painter().rect_stroke(a_rect, Rounding::same(3.0), Stroke::new(1.0, tokens.border));

                        let ax = a_rect.left() + thumb_pad + hsv.a * a_span;
                        let a_pos = Pos2::new(ax, a_rect.center().y);
                        ui.painter().circle_filled(a_pos, 5.5, Color32::from(hsv));
                        ui.painter().circle_stroke(a_pos, 6.0, Stroke::new(2.0, Color32::WHITE));
                        ui.painter().circle_stroke(a_pos, 7.2, Stroke::new(1.0, Color32::from_black_alpha(150)));
                    }
                }

                ui.add_space(10.0);

                // Preview + Hex readout + Theme Swatches
                let opaque_rgb = Color32::from(Hsva::new(hsv.h, hsv.s, hsv.v, 1.0));
                let current_rgba = Color32::from(hsv);
                let (hex_row, _) = ui.allocate_exact_size(Vec2::new(inner_w, 24.0), Sense::hover());
                let swatch_box = Rect::from_min_size(hex_row.min, Vec2::splat(22.0));
                ui.painter().rect_filled(swatch_box, Rounding::same(4.0), tokens.muted);
                ui.painter().rect(swatch_box, Rounding::same(4.0), current_rgba, Stroke::new(1.0, tokens.border));
                ui.painter().text(
                    Pos2::new(swatch_box.right() + 8.0, hex_row.center().y),
                    egui::Align2::LEFT_CENTER,
                    format!("#{:02X}{:02X}{:02X} · {:.0}%", opaque_rgb.r(), opaque_rgb.g(), opaque_rgb.b(), hsv.a * 100.0),
                    FontId::monospace(12.0),
                    tokens.foreground,
                );

                ui.add_space(8.0);
                let palette = [
                    tokens.primary,
                    tokens.success,
                    tokens.warning,
                    tokens.destructive,
                    Color32::from_rgb(56, 189, 248),
                    Color32::from_rgb(168, 85, 247),
                ];
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 6.0;
                    for preset in palette {
                        let (sr, s_resp) = ui.allocate_exact_size(Vec2::splat(20.0), Sense::click());
                        ui.painter().rect(sr, Rounding::same(4.0), preset, Stroke::new(1.0, tokens.border));
                        if s_resp.clicked() {
                            let prev_a = hsv.a;
                            hsv = Hsva::from(preset);
                            hsv.a = prev_a;
                            changed = true;
                        }
                    }
                });
            });

        if changed {
            *self.color = Color32::from(hsv);
        }
        ui.ctx().data_mut(|d| d.insert_temp(hsv_id, hsv));

        let mut resp = frame_resp.response;
        if changed {
            resp.mark_changed();
        }
        resp
    }
}
