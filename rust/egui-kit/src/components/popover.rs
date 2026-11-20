//! Click-triggered Popover surface anchored cleanly to any target widget response.

use crate::theme::active_theme;
use crate::tokens::ease_out_cubic;
use egui::{Area, Frame, Id, Key, Margin, Order, Pos2, Response, Rounding, Stroke, Ui, Vec2};

/// Preferred placement of a [`Popover`] relative to its anchor widget.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PopoverSide {
    #[default]
    Bottom,
    Top,
    Right,
    Left,
}

/// Click-triggered floating popover anchored to an `egui::Response`.
pub struct Popover<'a> {
    id_source: &'a str,
    side: PopoverSide,
    width: f32,
    gap: f32,
}

impl<'a> Popover<'a> {
    pub fn new(id_source: &'a str) -> Self {
        Self {
            id_source,
            side: PopoverSide::Bottom,
            width: 260.0,
            gap: 6.0,
        }
    }

    pub fn side(mut self, side: PopoverSide) -> Self {
        self.side = side;
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width.clamp(140.0, 480.0);
        self
    }

    pub fn show_for<R>(
        self,
        ui: &mut Ui,
        anchor: &Response,
        add_contents: impl FnOnce(&mut Ui) -> R,
    ) -> Option<R> {
        let open_id = Id::new(self.id_source).with("popover_open");
        let mut is_open = ui.ctx().data(|d| d.get_temp::<bool>(open_id).unwrap_or(false));

        if anchor.clicked() {
            is_open = !is_open;
        }
        if is_open && ui.ctx().input(|i| i.key_pressed(Key::Escape)) {
            is_open = false;
        }

        let anim_t = ui.ctx().animate_bool_with_time(Id::new(self.id_source).with("anim"), is_open, 0.14);
        if !is_open && anim_t <= 0.01 {
            ui.ctx().data_mut(|d| d.insert_temp(open_id, false));
            return None;
        }

        let eased = ease_out_cubic(anim_t);
        let tokens = active_theme(ui.ctx());
        let screen = ui.ctx().screen_rect();
        let anchor_rect = anchor.rect;
        let est_height = 180.0;

        let mut pos = match self.side {
            PopoverSide::Bottom => Pos2::new(anchor_rect.left(), anchor_rect.bottom() + self.gap),
            PopoverSide::Top => Pos2::new(anchor_rect.left(), anchor_rect.top() - est_height - self.gap),
            PopoverSide::Right => Pos2::new(anchor_rect.right() + self.gap, anchor_rect.top()),
            PopoverSide::Left => Pos2::new(anchor_rect.left() - self.width - self.gap, anchor_rect.top()),
        };

        // Screen-edge collision clamping
        pos.x = pos.x.clamp(screen.left() + 8.0, (screen.right() - self.width - 8.0).max(screen.left() + 8.0));
        pos.y = pos.y.clamp(screen.top() + 8.0, (screen.bottom() - 120.0).max(screen.top() + 8.0));
        pos.y += (1.0 - eased) * 6.0;

        let mut inner = None;
        let area_resp = Area::new(Id::new(self.id_source).with("surface"))
            .order(Order::Foreground)
            .fixed_pos(pos)
            .show(ui.ctx(), |ui| {
                Frame::none()
                    .fill(tokens.card)
                    .stroke(Stroke::new(1.0, tokens.border))
                    .rounding(Rounding::same(8.0))
                    .inner_margin(Margin::same(12.0))
                    .show(ui, |ui| {
                        ui.set_width(self.width);
                        inner = Some(add_contents(ui));
                    });
            })
            .response;

        if is_open
            && !anchor.clicked()
            && ui.ctx().input(|i| i.pointer.any_pressed())
            && !area_resp.rect.contains(
                ui.ctx()
                    .input(|i| i.pointer.interact_pos().unwrap_or(Pos2::ZERO)),
            )
            && !anchor_rect.contains(
                ui.ctx()
                    .input(|i| i.pointer.interact_pos().unwrap_or(Pos2::ZERO)),
            )
        {
            is_open = false;
        }

        ui.ctx().data_mut(|d| d.insert_temp(open_id, is_open));
        let _ = Vec2::ZERO;
        inner
    }
}
