//! Split horizontal or vertical Resizable panels with draggable divider handle.

use crate::theme::active_theme;
use crate::tokens::{ease_out_cubic, lerp_color, Orientation};
use egui::{CursorIcon, Frame, Id, Margin, Pos2, Rect, Rounding, Sense, Stroke, Ui, Vec2};

/// Two-pane resizable split container persisting split ratio in `egui::Memory` or `&mut f32`.
pub struct Resizable<'a> {
    id_source: &'a str,
    orientation: Orientation,
    external_ratio: Option<&'a mut f32>,
    default_ratio: f32,
    min_ratio: f32,
    max_ratio: f32,
    height: f32,
}

impl<'a> Resizable<'a> {
    pub fn horizontal(id_source: &'a str) -> Self {
        Self {
            id_source,
            orientation: Orientation::Horizontal,
            external_ratio: None,
            default_ratio: 0.42,
            min_ratio: 0.2,
            max_ratio: 0.8,
            height: 220.0,
        }
    }

    pub fn vertical(id_source: &'a str) -> Self {
        Self {
            id_source,
            orientation: Orientation::Vertical,
            external_ratio: None,
            default_ratio: 0.45,
            min_ratio: 0.2,
            max_ratio: 0.8,
            height: 240.0,
        }
    }

    /// Optional direct `&mut f32` binding for the split ratio (`0.0..=1.0`).
    pub fn ratio(mut self, ratio: &'a mut f32) -> Self {
        self.external_ratio = Some(ratio);
        self
    }

    pub fn default_ratio(mut self, ratio: f32) -> Self {
        self.default_ratio = ratio.clamp(0.1, 0.9);
        self
    }

    pub fn bounds(mut self, min_ratio: f32, max_ratio: f32) -> Self {
        self.min_ratio = min_ratio.clamp(0.1, 0.8);
        self.max_ratio = max_ratio.clamp(self.min_ratio + 0.05, 0.9);
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.height = height.max(100.0);
        self
    }

    pub fn show(
        mut self,
        ui: &mut Ui,
        first_pane: impl FnOnce(&mut Ui),
        second_pane: impl FnOnce(&mut Ui),
    ) -> f32 {
        let tokens = active_theme(ui.ctx());
        let mem_id = Id::new(("egui_kit_resizable", self.id_source));

        let mut ratio = match self.external_ratio.as_deref() {
            Some(r) => *r,
            None => ui
                .ctx()
                .data(|d| d.get_temp::<f32>(mem_id))
                .unwrap_or(self.default_ratio),
        };
        ratio = ratio.clamp(self.min_ratio, self.max_ratio);

        let avail_w = ui.available_width().max(220.0);
        let total_size = Vec2::new(avail_w, self.height);
        let (container_rect, _) = ui.allocate_exact_size(total_size, Sense::hover());

        ui.painter().rect(
            container_rect,
            Rounding::same(8.0),
            tokens.card,
            Stroke::new(1.0, tokens.border),
        );

        let handle_thickness = 10.0;
        let (first_rect, handle_rect, second_rect) = match self.orientation {
            Orientation::Horizontal => {
                let usable_w = (container_rect.width() - handle_thickness).max(40.0);
                let first_w = usable_w * ratio;
                let r1 = Rect::from_min_size(container_rect.min, Vec2::new(first_w, container_rect.height()));
                let hr = Rect::from_min_size(
                    Pos2::new(r1.right(), container_rect.top()),
                    Vec2::new(handle_thickness, container_rect.height()),
                );
                let r2 = Rect::from_min_max(Pos2::new(hr.right(), container_rect.top()), container_rect.max);
                (r1, hr, r2)
            }
            Orientation::Vertical => {
                let usable_h = (container_rect.height() - handle_thickness).max(40.0);
                let first_h = usable_h * ratio;
                let r1 = Rect::from_min_size(container_rect.min, Vec2::new(container_rect.width(), first_h));
                let hr = Rect::from_min_size(
                    Pos2::new(container_rect.left(), r1.bottom()),
                    Vec2::new(container_rect.width(), handle_thickness),
                );
                let r2 = Rect::from_min_max(Pos2::new(container_rect.left(), hr.bottom()), container_rect.max);
                (r1, hr, r2)
            }
        };

        let handle_resp = ui.interact(handle_rect, mem_id.with("divider"), Sense::click_and_drag());
        if handle_resp.hovered() || handle_resp.dragged() {
            ui.ctx().set_cursor_icon(match self.orientation {
                Orientation::Horizontal => CursorIcon::ResizeHorizontal,
                Orientation::Vertical => CursorIcon::ResizeVertical,
            });
        }
        if handle_resp.double_clicked() {
            ratio = self.default_ratio.clamp(self.min_ratio, self.max_ratio);
        } else if handle_resp.dragged() {
            let delta = handle_resp.drag_delta();
            let span = match self.orientation {
                Orientation::Horizontal => (container_rect.width() - handle_thickness).max(1.0),
                Orientation::Vertical => (container_rect.height() - handle_thickness).max(1.0),
            };
            let d_px = match self.orientation {
                Orientation::Horizontal => delta.x,
                Orientation::Vertical => delta.y,
            };
            ratio = (ratio + d_px / span).clamp(self.min_ratio, self.max_ratio);
        }

        let active_t = ease_out_cubic(ui.ctx().animate_bool_with_time(
            mem_id.with("active"),
            handle_resp.hovered() || handle_resp.dragged(),
            0.14,
        ));
        let line_color = lerp_color(tokens.border, tokens.primary, active_t);
        let grip_color = lerp_color(tokens.muted_foreground, tokens.primary, active_t);

        match self.orientation {
            Orientation::Horizontal => {
                let cx = handle_rect.center().x;
                ui.painter().line_segment(
                    [Pos2::new(cx, handle_rect.top() + 1.0), Pos2::new(cx, handle_rect.bottom() - 1.0)],
                    Stroke::new(1.0 + active_t, line_color),
                );
                let grip = Rect::from_center_size(handle_rect.center(), Vec2::new(4.0, 26.0));
                ui.painter().rect_filled(grip, Rounding::same(2.0), grip_color);
            }
            Orientation::Vertical => {
                let cy = handle_rect.center().y;
                ui.painter().line_segment(
                    [Pos2::new(handle_rect.left() + 1.0, cy), Pos2::new(handle_rect.right() - 1.0, cy)],
                    Stroke::new(1.0 + active_t, line_color),
                );
                let grip = Rect::from_center_size(handle_rect.center(), Vec2::new(26.0, 4.0));
                ui.painter().rect_filled(grip, Rounding::same(2.0), grip_color);
            }
        }

        let mut pane1_ui = ui.new_child(egui::UiBuilder::new().max_rect(first_rect.shrink(10.0)));
        pane1_ui.set_clip_rect(first_rect.shrink(4.0));
        Frame::none().inner_margin(Margin::ZERO).show(&mut pane1_ui, first_pane);

        let mut pane2_ui = ui.new_child(egui::UiBuilder::new().max_rect(second_rect.shrink(10.0)));
        pane2_ui.set_clip_rect(second_rect.shrink(4.0));
        Frame::none().inner_margin(Margin::ZERO).show(&mut pane2_ui, second_pane);

        if let Some(ext) = self.external_ratio {
            *ext = ratio;
        }
        ui.ctx().data_mut(|d| d.insert_temp(mem_id, ratio));
        ratio
    }
}
