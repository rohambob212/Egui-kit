//! `KitUi`: every widget is one method on `&mut Ui`. Import this trait and that's it.
use crate::{basic, Variant};
use egui::{Response, Ui};

pub trait KitUi {
    /// Small pill label: `ui.badge("new")`.
    fn badge(&mut self, text: impl Into<String>) -> Response;
    /// Button with a variant: `ui.button_v("Save", Variant::Primary)`.
    fn button_v(&mut self, text: impl Into<String>, variant: Variant) -> Response;
    /// Primary button, the common case.
    fn kit_button(&mut self, text: impl Into<String>) -> Response {
        self.button_v(text, Variant::Primary)
    }
}

impl KitUi for Ui {
    fn badge(&mut self, text: impl Into<String>) -> Response {
        basic::badge(self, text.into())
    }

    fn button_v(&mut self, text: impl Into<String>, variant: Variant) -> Response {
        basic::button(self, &text.into(), variant)
    }
}
