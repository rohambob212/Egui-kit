//! `KitUi`: every widget is one method on `&mut Ui`. Import this trait and that's it.
use crate::{basic, Variant};
use egui::{Response, Ui};

pub trait KitUi {
    /// Small pill label: `ui.badge("new")`.
    fn badge(&mut self, text: impl Into<String>) -> Response;
    /// Button with a variant: `ui.button_v("Save", Variant::Primary)`.
    fn button_v(&mut self, text: impl Into<String>, variant: Variant) -> Response;
    /// Small neutral pill label: `ui.tag("design")`.
    fn tag(&mut self, text: impl Into<String>) -> Response;
    /// Keyboard shortcut chip: `ui.kbd("Ctrl+K")`.
    fn kbd(&mut self, keys: impl Into<String>) -> Response;
    /// 1px horizontal rule across the full width.
    fn divider(&mut self);
    /// Thin progress bar, 0..=1: `ui.progress(0.4)`.
    fn progress(&mut self, value: f32) -> Response;
    /// Circular initials avatar: `ui.avatar("Ada Lovelace", 32.0)`.
    fn avatar(&mut self, name: &str, size: f32) -> Response;
    /// Full-width banner: `ui.alert("Saved.", Variant::Primary)`.
    fn alert(&mut self, text: &str, variant: Variant) -> Response;
    /// Shimmering loading block: `ui.skeleton(180.0, 12.0)`.
    fn skeleton(&mut self, width: f32, height: f32) -> Response;
    /// Star rating, 0..=max: `ui.rating(&mut value, 5)`.
    fn rating(&mut self, value: &mut u8, max: u8) -> Response;
    /// Numbered progress circles: `ui.stepper(1, 4)`.
    fn stepper(&mut self, step: usize, total: usize) -> Response;
    /// Prev/numbers/Next: `ui.pagination(&mut page, 10)`.
    fn pagination(&mut self, page: &mut usize, total_pages: usize) -> Response;
    /// Underline tabs: `ui.tabs(&["One", "Two"], &mut selected)`.
    fn tabs(&mut self, labels: &[&str], selected: &mut usize) -> Response;
    /// Collapsible section header: `ui.accordion("Advanced", &mut open, |ui| ...)`.
    fn accordion(&mut self, title: &str, open: &mut bool, body: impl FnOnce(&mut Ui)) -> Response;
    /// Digit boxes for a one-time code: `ui.otp_input(&mut code, 6)`.
    fn otp_input(&mut self, code: &mut String, len: usize) -> Response;
    /// Hour/minute drag fields: `ui.time_field(&mut hh, &mut mm)`.
    fn time_field(&mut self, hh: &mut u8, mm: &mut u8) -> Response;
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

    fn tag(&mut self, text: impl Into<String>) -> Response {
        basic::tag(self, text.into())
    }

    fn kbd(&mut self, keys: impl Into<String>) -> Response {
        basic::kbd(self, &keys.into())
    }

    fn divider(&mut self) {
        basic::divider(self)
    }

    fn progress(&mut self, value: f32) -> Response {
        basic::progress(self, value)
    }

    fn avatar(&mut self, name: &str, size: f32) -> Response {
        basic::avatar(self, name, size)
    }

    fn alert(&mut self, text: &str, variant: Variant) -> Response {
        basic::alert(self, text, variant)
    }

    fn skeleton(&mut self, width: f32, height: f32) -> Response {
        basic::skeleton(self, width, height)
    }

    fn rating(&mut self, value: &mut u8, max: u8) -> Response {
        basic::rating(self, "rating", value, max)
    }

    fn stepper(&mut self, step: usize, total: usize) -> Response {
        basic::stepper(self, step, total)
    }

    fn pagination(&mut self, page: &mut usize, total_pages: usize) -> Response {
        basic::pagination(self, page, total_pages)
    }

    fn tabs(&mut self, labels: &[&str], selected: &mut usize) -> Response {
        basic::tabs(self, labels, selected)
    }

    fn accordion(&mut self, title: &str, open: &mut bool, body: impl FnOnce(&mut Ui)) -> Response {
        basic::accordion(self, title, open, body)
    }

    fn otp_input(&mut self, code: &mut String, len: usize) -> Response {
        basic::otp_input(self, code, len)
    }

    fn time_field(&mut self, hh: &mut u8, mm: &mut u8) -> Response {
        basic::time_field(self, hh, mm)
    }
}
