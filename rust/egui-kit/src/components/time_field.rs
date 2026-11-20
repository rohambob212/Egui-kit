//! Segmented HH:MM:SS TimeField widget mutating `&mut SimpleTime` with keyboard stepping.

use crate::theme::active_theme;
use crate::tokens::{lerp_color, Size};
use egui::{Align2, FontId, Id, Key, Pos2, Rect, Response, Rounding, Sense, Stroke, Ui, Vec2, Widget};

/// Lightweight 24-hour time value (`hour` 0..=23, `minute` 0..=59, `second` 0..=59).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SimpleTime {
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

impl SimpleTime {
    pub const fn new(hour: u8, minute: u8, second: u8) -> Self {
        Self {
            hour: if hour > 23 { 23 } else { hour },
            minute: if minute > 59 { 59 } else { minute },
            second: if second > 59 { 59 } else { second },
        }
    }

    pub fn format_24h(self) -> String {
        format!("{:02}:{:02}:{:02}", self.hour, self.minute, self.second)
    }
}

impl Default for SimpleTime {
    fn default() -> Self {
        Self::new(9, 30, 0)
    }
}

/// Segmented HH:MM:SS time editor supporting 12h/24h display and ArrowUp/ArrowDown stepping.
pub struct TimeField<'a> {
    id_source: &'a str,
    time: &'a mut SimpleTime,
    use_24h: bool,
    show_seconds: bool,
    size: Size,
    disabled: bool,
}

impl<'a> TimeField<'a> {
    pub fn new(id_source: &'a str, time: &'a mut SimpleTime) -> Self {
        Self {
            id_source,
            time,
            use_24h: true,
            show_seconds: true,
            size: Size::Md,
            disabled: false,
        }
    }

    pub fn use_24h(mut self, use_24h: bool) -> Self {
        self.use_24h = use_24h;
        self
    }

    pub fn show_seconds(mut self, show: bool) -> Self {
        self.show_seconds = show;
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

impl<'a> Widget for TimeField<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let tokens = active_theme(ui.ctx());
        let seg_id = Id::new(("egui_kit_time_seg", self.id_source));
        let mut active_seg: usize = ui.ctx().data(|d| d.get_temp::<usize>(seg_id).unwrap_or(0));

        let seg_count = if self.show_seconds { 3 } else { 2 };
        active_seg = active_seg.min(seg_count - 1);

        let h = self.size.height();
        let seg_w = 28.0;
        let step_w = 24.0;
        let ampm_w = if self.use_24h { 0.0 } else { 36.0 };
        let total_w = 28.0 + (seg_count as f32) * (seg_w + 8.0) + ampm_w + step_w * 2.0 + 12.0;

        let (rect, mut response) = ui.allocate_exact_size(Vec2::new(total_w, h), Sense::hover());

        if ui.is_rect_visible(rect) {
            let bg = if self.disabled { tokens.muted } else { tokens.background };
            ui.painter().rect(rect, self.size.rounding(), bg, Stroke::new(1.0, tokens.input));

            // Clock icon
            let clock_c = Pos2::new(rect.left() + 16.0, rect.center().y);
            ui.painter().circle_stroke(clock_c, 6.5, Stroke::new(1.2, tokens.muted_foreground));
            ui.painter().line_segment([clock_c, clock_c + Vec2::new(0.0, -3.5)], Stroke::new(1.2, tokens.muted_foreground));
            ui.painter().line_segment([clock_c, clock_c + Vec2::new(3.0, 1.5)], Stroke::new(1.2, tokens.muted_foreground));
        }

        let display_hour = if self.use_24h {
            self.time.hour
        } else {
            let h12 = self.time.hour % 12;
            if h12 == 0 { 12 } else { h12 }
        };

        let values = [display_hour, self.time.minute, self.time.second];
        let mut x = rect.left() + 30.0;

        for idx in 0..seg_count {
            let seg_rect = Rect::from_min_size(Pos2::new(x, rect.top() + 4.0), Vec2::new(seg_w, h - 8.0));
            let s_resp = ui.interact(
                seg_rect,
                seg_id.with(idx),
                if self.disabled { Sense::hover() } else { Sense::click() },
            );
            if s_resp.clicked() && !self.disabled {
                active_seg = idx;
                s_resp.request_focus();
            }

            let is_active = idx == active_seg && !self.disabled;
            let focus_t = ui.ctx().animate_bool_with_time(seg_id.with(("act", idx)), is_active, 0.12);
            if focus_t > 0.01 {
                ui.painter().rect_filled(
                    seg_rect,
                    Rounding::same(4.0),
                    lerp_color(tokens.background, tokens.primary, focus_t * 0.18),
                );
            }
            ui.painter().text(
                seg_rect.center(),
                Align2::CENTER_CENTER,
                format!("{:02}", values[idx]),
                FontId::monospace(self.size.font_size()),
                if is_active { tokens.primary } else { tokens.foreground },
            );
            x += seg_w;
            if idx + 1 < seg_count {
                ui.painter().text(
                    Pos2::new(x + 4.0, rect.center().y),
                    Align2::CENTER_CENTER,
                    ":",
                    FontId::monospace(self.size.font_size()),
                    tokens.muted_foreground,
                );
                x += 8.0;
            }
        }

        if !self.use_24h {
            let ampm_rect = Rect::from_min_size(Pos2::new(x + 4.0, rect.top() + 4.0), Vec2::new(30.0, h - 8.0));
            let ampm_resp = ui.interact(
                ampm_rect,
                seg_id.with("ampm"),
                if self.disabled { Sense::hover() } else { Sense::click() },
            );
            ui.painter().rect_filled(ampm_rect, Rounding::same(4.0), tokens.muted);
            ui.painter().text(
                ampm_rect.center(),
                Align2::CENTER_CENTER,
                if self.time.hour < 12 { "AM" } else { "PM" },
                FontId::monospace(11.0),
                tokens.foreground,
            );
            if ampm_resp.clicked() && !self.disabled {
                self.time.hour = (self.time.hour + 12) % 24;
                response.mark_changed();
            }
            x += ampm_w;
        }

        // - / + stepper buttons for the focused segment
        if !self.disabled {
            let mut delta: i32 = 0;
            if ui.input(|i| i.key_pressed(Key::ArrowUp)) {
                delta += 1;
            } else if ui.input(|i| i.key_pressed(Key::ArrowDown)) {
                delta -= 1;
            }

            let minus_rect = Rect::from_min_size(Pos2::new(rect.right() - step_w * 2.0 - 4.0, rect.top() + 4.0), Vec2::new(step_w, h - 8.0));
            let plus_rect = Rect::from_min_size(Pos2::new(rect.right() - step_w - 2.0, rect.top() + 4.0), Vec2::new(step_w, h - 8.0));

            let m_resp = ui.interact(minus_rect, seg_id.with("dec"), Sense::click());
            let p_resp = ui.interact(plus_rect, seg_id.with("inc"), Sense::click());
            if m_resp.hovered() {
                ui.painter().rect_filled(minus_rect, Rounding::same(4.0), tokens.muted);
            }
            if p_resp.hovered() {
                ui.painter().rect_filled(plus_rect, Rounding::same(4.0), tokens.muted);
            }
            ui.painter().text(minus_rect.center(), Align2::CENTER_CENTER, "-", FontId::monospace(13.0), tokens.foreground);
            ui.painter().text(plus_rect.center(), Align2::CENTER_CENTER, "+", FontId::monospace(13.0), tokens.foreground);

            if m_resp.clicked() {
                delta -= 1;
            }
            if p_resp.clicked() {
                delta += 1;
            }

            if delta != 0 {
                match active_seg {
                    0 => self.time.hour = ((self.time.hour as i32 + delta).rem_euclid(24)) as u8,
                    1 => self.time.minute = ((self.time.minute as i32 + delta).rem_euclid(60)) as u8,
                    _ => self.time.second = ((self.time.second as i32 + delta).rem_euclid(60)) as u8,
                }
                response.mark_changed();
            }
        }

        ui.ctx().data_mut(|d| d.insert_temp(seg_id, active_seg));
        response
    }
}
