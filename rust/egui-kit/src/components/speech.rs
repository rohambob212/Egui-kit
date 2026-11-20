//! Voice dictation Speech trigger with animated concentric pulse and equalizer bars.

use crate::theme::active_theme;
use crate::tokens::{ease_out_cubic, lerp_color};
use egui::{FontId, Id, Pos2, Rect, Response, Rounding, Sense, Stroke, Ui, Vec2, Widget};

/// Voice dictation toggle mutating `&mut bool` with live Painter equalizer animation.
pub struct Speech<'a> {
    id_source: &'a str,
    listening: &'a mut bool,
    label: Option<String>,
    disabled: bool,
}

impl<'a> Speech<'a> {
    pub fn new(id_source: &'a str, listening: &'a mut bool) -> Self {
        Self {
            id_source,
            listening,
            label: None,
            disabled: false,
        }
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl<'a> Widget for Speech<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let tokens = active_theme(ui.ctx());
        let anim_id = Id::new(("egui_kit_speech", self.id_source));
        let timer_id = anim_id.with("started_at");
        let sense = if self.disabled { Sense::hover() } else { Sense::click() };

        let width = 296.0;
        let height = 56.0;
        let (rect, mut response) = ui.allocate_exact_size(Vec2::new(width, height), sense);

        if response.clicked() && !self.disabled {
            *self.listening = !*self.listening;
            response.mark_changed();
        }

        let now = ui.input(|i| i.time);
        let active = *self.listening && !self.disabled;

        // Track listening start time in egui::Memory for a live MM:SS readout
        let started_at: Option<f64> = ui.ctx().data_mut(|d| {
            let prev = d.get_temp::<Option<f64>>(timer_id).flatten();
            let next = if active { Some(prev.unwrap_or(now)) } else { None };
            d.insert_temp(timer_id, next);
            next
        });

        let hover_t = ease_out_cubic(
            ui.ctx()
                .animate_bool_with_time(anim_id.with("hov"), response.hovered() && !self.disabled, 0.14),
        );
        let active_t = ease_out_cubic(ui.ctx().animate_bool_with_time(anim_id.with("act"), active, 0.2));
        if active {
            ui.ctx().request_repaint();
        }

        if ui.is_rect_visible(rect) {
            let idle_bg = lerp_color(tokens.card, tokens.muted, hover_t * 0.45);
            let bg = lerp_color(idle_bg, tokens.primary, active_t * 0.12);
            let border = lerp_color(
                lerp_color(tokens.border, tokens.ring, hover_t * 0.45),
                tokens.primary,
                active_t,
            );
            ui.painter().rect(
                rect,
                Rounding::same(14.0),
                bg,
                Stroke::new(1.0 + 0.5 * active_t, border),
            );

            // Tactile microphone circle with dual concentric acoustic pulse rings
            let mic_center = Pos2::new(rect.left() + 28.0, rect.center().y);
            if active_t > 0.04 {
                let phase_a = ((now as f32 * 0.75) % 1.0).clamp(0.0, 1.0);
                let phase_b = (((now as f32 * 0.75) + 0.5) % 1.0).clamp(0.0, 1.0);
                for phase in [phase_a, phase_b] {
                    let ring_r = 16.0 + phase * 10.0;
                    let alpha = ((1.0 - phase) * 0.45 * active_t * 255.0) as u8;
                    let ring_col = egui::Color32::from_rgba_unmultiplied(
                        tokens.primary.r(),
                        tokens.primary.g(),
                        tokens.primary.b(),
                        alpha,
                    );
                    ui.painter()
                        .circle_stroke(mic_center, ring_r, Stroke::new(1.5, ring_col));
                }
            }

            let mic_bg = lerp_color(tokens.muted, tokens.primary, active_t);
            let mic_fg = lerp_color(tokens.foreground, tokens.primary_foreground, active_t);
            ui.painter().circle_filled(mic_center, 16.0, mic_bg);

            // Painter microphone glyph
            let cap = Rect::from_center_size(mic_center - Vec2::new(0.0, 2.5), Vec2::new(6.0, 9.5));
            ui.painter().rect_stroke(cap, Rounding::same(3.0), Stroke::new(1.5, mic_fg));
            ui.painter().line_segment(
                [mic_center + Vec2::new(0.0, 3.0), mic_center + Vec2::new(0.0, 7.5)],
                Stroke::new(1.5, mic_fg),
            );
            ui.painter().line_segment(
                [mic_center + Vec2::new(-3.5, 7.5), mic_center + Vec2::new(3.5, 7.5)],
                Stroke::new(1.5, mic_fg),
            );

            // Primary title + live elapsed sublabel
            let caption = self.label.unwrap_or_else(|| {
                if active {
                    "Listening...".to_string()
                } else {
                    "Tap to speak".to_string()
                }
            });
            ui.painter().text(
                Pos2::new(rect.left() + 56.0, rect.center().y - 7.5),
                egui::Align2::LEFT_CENTER,
                caption,
                FontId::proportional(13.0),
                if active { tokens.foreground } else { tokens.card_foreground },
            );

            let subtext = if let Some(start) = started_at {
                let secs = (now - start).max(0.0) as u64;
                format!("{:02}:{:02} · Capturing", secs / 60, secs % 60)
            } else if self.disabled {
                "Mic unavailable".to_string()
            } else {
                "Voice dictation ready".to_string()
            };
            ui.painter().text(
                Pos2::new(rect.left() + 56.0, rect.center().y + 8.5),
                egui::Align2::LEFT_CENTER,
                subtext,
                FontId::monospace(10.5),
                if active { tokens.primary } else { tokens.muted_foreground },
            );

            // Recessed 7-bar symmetrical acoustic spectrum well on right
            let well_rect = Rect::from_center_size(
                Pos2::new(rect.right() - 42.0, rect.center().y),
                Vec2::new(62.0, 32.0),
            );
            let well_bg = lerp_color(tokens.muted, tokens.primary, active_t * 0.16);
            ui.painter().rect_filled(well_rect, Rounding::same(9.0), well_bg);

            let rest_profile = [0.28_f32, 0.48, 0.72, 0.95, 0.72, 0.48, 0.28];
            let bar_color = lerp_color(tokens.muted_foreground, tokens.primary, active_t);
            let start_x = well_rect.center().x - 18.0;
            for (idx, rest) in rest_profile.iter().enumerate() {
                let osc = if active {
                    ((now as f32 * 6.5 + (idx as f32) * 0.9).sin() * 0.5 + 0.5) * 0.75 + 0.25
                } else {
                    *rest * 0.35
                };
                let bar_h = 4.0 + osc * 16.0 * (0.45 + 0.55 * active_t);
                let bx = start_x + (idx as f32) * 6.0;
                let bar_rect = Rect::from_center_size(Pos2::new(bx, well_rect.center().y), Vec2::new(3.0, bar_h));
                ui.painter().rect_filled(bar_rect, Rounding::same(1.5), bar_color);
            }
        }

        response
    }
}
