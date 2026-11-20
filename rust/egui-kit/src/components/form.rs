//! Structured Form & FormField layout helpers with animated validation messages.

use crate::theme::active_theme;
use crate::tokens::{ease_out_cubic, Orientation};
use egui::{Id, Response, RichText, Ui};

/// Form container providing consistent field spacing and label orientation.
pub struct Form<'a> {
    id_source: &'a str,
    orientation: Orientation,
    spacing: f32,
}

impl<'a> Form<'a> {
    pub fn new(id_source: &'a str) -> Self {
        Self {
            id_source,
            orientation: Orientation::Vertical,
            spacing: 14.0,
        }
    }

    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing.max(4.0);
        self
    }

    pub fn show(self, ui: &mut Ui, add_fields: impl FnOnce(&mut Ui)) -> Response {
        ui.push_id(("egui_kit_form", self.id_source, self.orientation), |ui| {
            ui.vertical(|ui| {
                ui.spacing_mut().item_spacing.y = self.spacing;
                add_fields(ui);
            })
            .response
        })
        .inner
    }
}

/// Individual FormField pairing a label, required marker, child control, hint, and animated error line.
pub struct FormField<'a> {
    label: String,
    required: bool,
    hint: Option<String>,
    error: Option<&'a str>,
    label_width: f32,
    orientation: Orientation,
}

impl<'a> FormField<'a> {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            required: false,
            hint: None,
            error: None,
            label_width: 130.0,
            orientation: Orientation::Vertical,
        }
    }

    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    pub fn hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = Some(hint.into());
        self
    }

    pub fn error(mut self, error: Option<&'a str>) -> Self {
        self.error = error;
        self
    }

    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn show(self, ui: &mut Ui, add_control: impl FnOnce(&mut Ui) -> Response) -> Response {
        let tokens = active_theme(ui.ctx());
        let id = Id::new(("egui_kit_field", &self.label));
        let err_t = ease_out_cubic(ui.ctx().animate_bool_with_time(id.with("err"), self.error.is_some(), 0.16));

        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing.y = 4.0;
            let control_resp = match self.orientation {
                Orientation::Vertical => {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(&self.label).strong().size(13.0).color(tokens.foreground));
                        if self.required {
                            ui.label(RichText::new("*").strong().color(tokens.destructive));
                        }
                    });
                    add_control(ui)
                }
                Orientation::Horizontal => {
                    ui.horizontal(|ui| {
                        ui.allocate_ui(egui::vec2(self.label_width, 24.0), |ui| {
                            ui.horizontal(|ui| {
                                ui.label(RichText::new(&self.label).strong().size(13.0).color(tokens.foreground));
                                if self.required {
                                    ui.label(RichText::new("*").strong().color(tokens.destructive));
                                }
                            });
                        });
                        add_control(ui)
                    })
                    .inner
                }
            };

            if let Some(err) = self.error {
                ui.label(RichText::new(err).small().color(tokens.destructive.linear_multiply(err_t.clamp(0.2, 1.0))));
            } else if let Some(hint) = &self.hint {
                ui.label(RichText::new(hint).small().color(tokens.muted_foreground));
            }
            control_resp
        })
        .inner
    }
}
