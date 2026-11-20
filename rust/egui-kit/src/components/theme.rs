//! ThemePicker widget for switching `gpui-kit` presets at runtime.

use crate::theme::{active_theme, set_theme, ThemeId, ThemeTokens};
use crate::tokens::Size;
use egui::{ComboBox, Response, RichText, Rounding, Stroke, Ui, Widget};

/// Interactive theme selector widget allowing users to pick among built-in
/// `gpui-kit` presets and preview color swatches inline.
pub struct ThemePicker {
    id_source: &'static str,
    size: Size,
    show_swatches: bool,
}

impl ThemePicker {
    /// Creates a new `ThemePicker` bound to the active `egui::Context` theme.
    pub fn new(id_source: &'static str) -> Self {
        Self {
            id_source,
            size: Size::Md,
            show_swatches: true,
        }
    }

    /// Sets the control scale.
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    /// Enables or disables inline color swatch preview dots next to each preset.
    pub fn show_swatches(mut self, show: bool) -> Self {
        self.show_swatches = show;
        self
    }
}

impl Widget for ThemePicker {
    fn ui(self, ui: &mut Ui) -> Response {
        let current = active_theme(ui.ctx());
        let mut selected = current.id;

        let resp = ComboBox::from_id_salt(self.id_source)
            .selected_text(RichText::new(selected.label()).size(self.size.font_size()))
            .height(260.0)
            .show_ui(ui, |ui| {
                for &preset in ThemeId::all() {
                    let tokens = ThemeTokens::from_preset(preset);
                    ui.horizontal(|ui| {
                        if self.show_swatches {
                            let (rect, _) =
                                ui.allocate_exact_size(egui::vec2(26.0, 14.0), egui::Sense::hover());
                            ui.painter().rect(
                                rect,
                                Rounding::same(3.0),
                                tokens.background,
                                Stroke::new(1.0, tokens.border),
                            );
                            let dot = egui::pos2(rect.right() - 7.0, rect.center().y);
                            ui.painter().circle_filled(dot, 4.0, tokens.primary);
                        }
                        if ui
                            .selectable_value(&mut selected, preset, preset.label())
                            .changed()
                        {
                            set_theme(ui.ctx(), preset);
                        }
                    });
                }
            })
            .response;

        if selected != current.id {
            set_theme(ui.ctx(), selected);
        }

        resp
    }
}
