//! FocusTrap keyboard containment scope cycling Tab / Shift+Tab inside overlays.

use crate::theme::active_theme;
use egui::{Id, Key, Rounding, Stroke, Ui};

/// Keyboard focus containment wrapper keeping `Tab` / `Shift+Tab` inside modal boundaries.
pub struct FocusTrap<'a> {
    id_source: &'a str,
    active: bool,
    show_ring: bool,
}

impl<'a> FocusTrap<'a> {
    pub fn new(id_source: &'a str) -> Self {
        Self {
            id_source,
            active: true,
            show_ring: false,
        }
    }

    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    pub fn show_ring(mut self, show: bool) -> Self {
        self.show_ring = show;
        self
    }

    /// Runs `add_contents` inside a focus-trapped scope, recording focusable child IDs and
    /// wrapping `Tab` / `Shift+Tab` traversal cleanly between the first and last focusables.
    pub fn show<R>(self, ui: &mut Ui, focusable_ids: &[Id], add_contents: impl FnOnce(&mut Ui) -> R) -> R {
        let tokens = active_theme(ui.ctx());
        let init_id = Id::new(self.id_source).with("trap_init");

        if self.active && !focusable_ids.is_empty() {
            let did_init = ui.ctx().data(|d| d.get_temp::<bool>(init_id).unwrap_or(false));
            if !did_init {
                ui.ctx().memory_mut(|m| m.request_focus(focusable_ids[0]));
                ui.ctx().data_mut(|d| d.insert_temp(init_id, true));
            }

            let (tab, shift_tab) = ui.ctx().input(|i| {
                let pressed = i.key_pressed(Key::Tab);
                (pressed && !i.modifiers.shift, pressed && i.modifiers.shift)
            });

            if tab || shift_tab {
                let current_idx = focusable_ids
                    .iter()
                    .position(|id| ui.ctx().memory(|m| m.has_focus(*id)));
                if let Some(idx) = current_idx {
                    if tab && idx + 1 == focusable_ids.len() {
                        ui.ctx().memory_mut(|m| m.request_focus(focusable_ids[0]));
                    } else if shift_tab && idx == 0 {
                        ui.ctx().memory_mut(|m| m.request_focus(*focusable_ids.last().unwrap()));
                    }
                } else {
                    ui.ctx().memory_mut(|m| m.request_focus(focusable_ids[0]));
                }
            }
        } else if !self.active {
            ui.ctx().data_mut(|d| d.insert_temp(init_id, false));
        }

        let resp = ui.scope(add_contents);
        if self.show_ring && self.active {
            ui.painter().rect_stroke(
                resp.response.rect.expand(3.0),
                Rounding::same(8.0),
                Stroke::new(1.5, tokens.ring),
            );
        }
        resp.inner
    }
}
