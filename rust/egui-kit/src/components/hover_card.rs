//! Delayed HoverCard preview floating surface with pointer-enter grace period.

use crate::theme::active_theme;
use crate::tokens::ease_out_cubic;
use egui::{Area, Frame, Id, Margin, Order, Pos2, Response, Rounding, Stroke, Ui};

#[derive(Debug, Clone, Copy)]
struct HoverCardState {
    open: bool,
    enter_time: Option<f64>,
    leave_time: Option<f64>,
    last_card_rect: egui::Rect,
}

impl Default for HoverCardState {
    fn default() -> Self {
        Self {
            open: false,
            enter_time: None,
            leave_time: None,
            // egui::Rect has no Default impl in 0.30
            last_card_rect: egui::Rect::NOTHING,
        }
    }
}

/// Delayed hover preview card for user profiles, link previews, and symbol documentation.
pub struct HoverCard<'a> {
    id_source: &'a str,
    open_delay: f64,
    close_delay: f64,
    width: f32,
}

impl<'a> HoverCard<'a> {
    pub fn new(id_source: &'a str) -> Self {
        Self {
            id_source,
            open_delay: 0.2,
            close_delay: 0.16,
            width: 280.0,
        }
    }

    pub fn open_delay(mut self, seconds: f64) -> Self {
        self.open_delay = seconds.max(0.0);
        self
    }

    pub fn close_delay(mut self, seconds: f64) -> Self {
        self.close_delay = seconds.max(0.0);
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width.clamp(180.0, 460.0);
        self
    }

    pub fn show_for<R>(
        self,
        ui: &mut Ui,
        anchor: &Response,
        add_contents: impl FnOnce(&mut Ui) -> R,
    ) -> Option<R> {
        let state_id = Id::new(self.id_source).with("hover_card_state");
        let mut st = ui.ctx().data(|d| d.get_temp::<HoverCardState>(state_id).unwrap_or_default());
        let now = ui.ctx().input(|i| i.time);
        let pointer = ui.ctx().input(|i| i.pointer.hover_pos());

        let over_card = st.open && pointer.map_or(false, |p| st.last_card_rect.contains(p));
        let hovering = anchor.hovered() || over_card;

        if hovering {
            st.leave_time = None;
            let start = *st.enter_time.get_or_insert(now);
            if now - start >= self.open_delay {
                st.open = true;
            } else {
                ui.ctx().request_repaint_after(std::time::Duration::from_millis(16));
            }
        } else {
            st.enter_time = None;
            if st.open {
                let left_at = *st.leave_time.get_or_insert(now);
                if now - left_at >= self.close_delay {
                    st.open = false;
                } else {
                    ui.ctx().request_repaint_after(std::time::Duration::from_millis(16));
                }
            }
        }

        let anim_t = ui.ctx().animate_bool_with_time(Id::new(self.id_source).with("anim"), st.open, 0.15);
        if !st.open && anim_t <= 0.01 {
            ui.ctx().data_mut(|d| d.insert_temp(state_id, st));
            return None;
        }

        let eased = ease_out_cubic(anim_t);
        let tokens = active_theme(ui.ctx());
        let screen = ui.ctx().screen_rect();
        let pos = Pos2::new(
            anchor
                .rect
                .left()
                .clamp(screen.left() + 8.0, (screen.right() - self.width - 8.0).max(screen.left() + 8.0)),
            (anchor.rect.bottom() + 6.0 + (1.0 - eased) * 5.0)
                .clamp(screen.top() + 8.0, (screen.bottom() - 140.0).max(screen.top() + 8.0)),
        );

        let mut out = None;
        let area = Area::new(Id::new(self.id_source).with("card"))
            .order(Order::Foreground)
            .fixed_pos(pos)
            .show(ui.ctx(), |ui| {
                Frame::none()
                    .fill(tokens.card)
                    .stroke(Stroke::new(1.0, tokens.border))
                    .rounding(Rounding::same(8.0))
                    .inner_margin(Margin::same(14.0))
                    .show(ui, |ui| {
                        ui.set_width(self.width);
                        out = Some(add_contents(ui));
                    });
            });

        st.last_card_rect = area.response.rect;
        ui.ctx().data_mut(|d| d.insert_temp(state_id, st));
        out
    }
}
