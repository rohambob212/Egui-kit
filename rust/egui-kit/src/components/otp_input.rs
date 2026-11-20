//! Segmented OtpInput verification code widget with auto-advance, paste splitting, and animated slot focus.

use crate::theme::active_theme;
use crate::tokens::{ease_out_cubic, lerp_color, Size};
use egui::{Event, FontId, Id, Key, Pos2, Rect, Response, Sense, Stroke, Ui, Vec2, Widget};

/// Segmented one-time verification code input mutating `&mut String` directly.
pub struct OtpInput<'a> {
    value: &'a mut String,
    length: usize,
    masked: bool,
    error: bool,
    size: Size,
    disabled: bool,
}

impl<'a> OtpInput<'a> {
    pub fn new(value: &'a mut String, length: usize) -> Self {
        Self {
            value,
            length: length.clamp(3, 8),
            masked: false,
            error: false,
            size: Size::Md,
            disabled: false,
        }
    }

    pub fn masked(mut self, masked: bool) -> Self {
        self.masked = masked;
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

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl<'a> Widget for OtpInput<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let tokens = active_theme(ui.ctx());
        let box_edge = (self.size.height() + 8.0).clamp(32.0, 52.0);
        let gap = 8.0;
        let total_w = self.length as f32 * box_edge + (self.length.saturating_sub(1)) as f32 * gap;
        let (outer_rect, mut resp) = ui.allocate_exact_size(Vec2::new(total_w, box_edge), Sense::click());
        let base_id = resp.id;
        let active_slot_id = Id::new((base_id, "active_slot"));

        let mut chars: Vec<char> = self.value.chars().filter(|c| c.is_ascii_digit()).take(self.length).collect();
        let mut active_slot = ui.ctx().data(|d| d.get_temp::<Option<usize>>(active_slot_id).unwrap_or(None));
        let mut changed = false;

        if ui.input(|i| i.pointer.any_click()) && !outer_rect.contains(ui.input(|i| i.pointer.interact_pos().unwrap_or(Pos2::ZERO))) {
            active_slot = None;
        }

        for idx in 0..self.length {
            let x = outer_rect.left() + idx as f32 * (box_edge + gap);
            let slot_rect = Rect::from_min_size(Pos2::new(x, outer_rect.top()), Vec2::splat(box_edge));
            let slot_resp = ui.interact(
                slot_rect,
                base_id.with(("slot", idx)),
                if self.disabled { Sense::hover() } else { Sense::click() },
            );
            if slot_resp.clicked() && !self.disabled {
                let target = idx.min(chars.len());
                active_slot = Some(target);
                ui.memory_mut(|m| m.request_focus(base_id));
            }
            resp = resp.union(slot_resp);
        }

        if resp.clicked() && !self.disabled && active_slot.is_none() {
            active_slot = Some(chars.len().min(self.length - 1));
            ui.memory_mut(|m| m.request_focus(base_id));
        }

        if let Some(slot) = active_slot {
            if !self.disabled {
                let events = ui.input(|i| i.events.clone());
                let mut cur_slot = slot;
                for ev in events {
                    match ev {
                        Event::Text(text) | Event::Paste(text) => {
                            let digits: Vec<char> = text.chars().filter(|c| c.is_ascii_digit()).collect();
                            for d in digits {
                                if cur_slot < chars.len() {
                                    chars[cur_slot] = d;
                                } else if chars.len() < self.length {
                                    chars.push(d);
                                }
                                if cur_slot + 1 < self.length {
                                    cur_slot += 1;
                                }
                                changed = true;
                            }
                        }
                        Event::Key { key: Key::Backspace, pressed: true, .. } => {
                            if cur_slot < chars.len() {
                                chars.remove(cur_slot);
                                changed = true;
                            } else if !chars.is_empty() {
                                chars.pop();
                                cur_slot = cur_slot.saturating_sub(1);
                                changed = true;
                            }
                        }
                        Event::Key { key: Key::ArrowLeft, pressed: true, .. } => {
                            cur_slot = cur_slot.saturating_sub(1);
                        }
                        Event::Key { key: Key::ArrowRight, pressed: true, .. } => {
                            cur_slot = (cur_slot + 1).min(self.length - 1);
                        }
                        _ => {}
                    }
                }
                active_slot = Some(cur_slot);
            }
        }

        if changed {
            *self.value = chars.iter().collect();
            resp.mark_changed();
        }

        if ui.is_rect_visible(outer_rect) {
            let err_t = ease_out_cubic(ui.ctx().animate_bool_with_time(base_id.with("err"), self.error, 0.14));
            let painter = ui.painter();
            for idx in 0..self.length {
                let x = outer_rect.left() + idx as f32 * (box_edge + gap);
                let slot_rect = Rect::from_min_size(Pos2::new(x, outer_rect.top()), Vec2::splat(box_edge));
                let is_active = active_slot == Some(idx) && !self.disabled;
                let has_char = chars.get(idx).is_some();

                let focus_t = ease_out_cubic(ui.ctx().animate_bool_with_time(base_id.with(("f", idx)), is_active, 0.12));
                let fill_t = ease_out_cubic(ui.ctx().animate_bool_with_time(base_id.with(("c", idx)), has_char, 0.14));

                let bg = if self.disabled { tokens.muted } else { tokens.background };
                let border = lerp_color(lerp_color(tokens.input, tokens.ring, focus_t), tokens.destructive, err_t);

                if focus_t > 0.02 {
                    let halo = lerp_color(tokens.ring, tokens.destructive, err_t).linear_multiply(0.24 * focus_t);
                    painter.rect_stroke(slot_rect.expand(1.5 * focus_t), self.size.rounding(), Stroke::new(2.5 * focus_t, halo));
                }
                painter.rect(slot_rect, self.size.rounding(), bg, Stroke::new(1.0 + 0.75 * focus_t, border));

                if let Some(ch) = chars.get(idx) {
                    let display = if self.masked { '•' } else { *ch };
                    let font_pt = self.size.font_size() * (1.2 + 0.15 * fill_t);
                    let fg = if self.disabled { tokens.muted_foreground } else { tokens.foreground };
                    painter.text(
                        slot_rect.center(),
                        egui::Align2::CENTER_CENTER,
                        display,
                        FontId::monospace(font_pt),
                        fg.linear_multiply(fill_t.clamp(0.25, 1.0)),
                    );
                } else if is_active {
                    let pulse = ((ui.input(|i| i.time) * 3.2).sin() * 0.5 + 0.5) as f32;
                    painter.line_segment(
                        [slot_rect.center() - Vec2::new(0.0, 8.0), slot_rect.center() + Vec2::new(0.0, 8.0)],
                        Stroke::new(1.5, tokens.primary.linear_multiply(pulse)),
                    );
                    ui.ctx().request_repaint();
                }
            }
        }

        ui.ctx().data_mut(|d| d.insert_temp(active_slot_id, active_slot));
        resp
    }
}
