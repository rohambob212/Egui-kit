//! `KitUi`: every widget is one method on `&mut Ui`. Import this trait and that's it.
use crate::{basic, basic::Side, Variant};
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
    /// Themed dropdown: `ui.select(&["One", "Two"], &mut selected)`.
    fn select(&mut self, items: &[&str], selected: &mut usize) -> Response;
    /// Filterable dropdown: `ui.combobox(&items, &mut query, &mut selected)`.
    fn combobox(&mut self, items: &[&str], query: &mut String, selected: &mut usize) -> Response;
    /// Animated switch: `ui.switch("notify", &mut on)`.
    fn switch(&mut self, id: &str, on: &mut bool) -> Response;
    /// Native slider: `ui.slider(&mut value, 0.0..=100.0)`.
    fn slider(&mut self, value: &mut f32, range: std::ops::RangeInclusive<f32>) -> Response;
    /// Single-line text input in a themed frame: `ui.text_input(&mut text, "Name")`.
    fn text_input(&mut self, text: &mut String, placeholder: &str) -> Response;
    /// Multi-line text input: `ui.textarea(&mut text, 3)`.
    fn textarea(&mut self, text: &mut String, rows: usize) -> Response;
    /// Numeric drag input: `ui.number_input(&mut n, 0.0..=10.0)`.
    fn number_input(&mut self, value: &mut f64, range: std::ops::RangeInclusive<f64>) -> Response;
    /// Hover text on rendered content: `ui.tooltip("Saved", |ui| ui.button("ok"))`.
    fn tooltip(&mut self, text: &str, add: impl FnOnce(&mut Ui) -> Response) -> Response;
    /// Dropdown menu button, returns the clicked index.
    fn menu(&mut self, label: &str, items: &[&str]) -> Option<usize>;
    /// Bordered section with a title.
    fn group_box(&mut self, title: &str, body: impl FnOnce(&mut Ui)) -> Response;
    /// Key/value table.
    fn description_list(&mut self, rows: &[(&str, &str)]) -> Response;
    /// Centred empty-state placeholder.
    fn empty(&mut self, title: &str, hint: &str) -> Response;
    /// File chip with a remove button.
    fn attachment(&mut self, name: &str, removed: &mut bool) -> Response;
    /// Vertical scroll region of fixed height.
    fn scrollable<R>(&mut self, height: f32, add: impl FnOnce(&mut Ui) -> R) -> R;
    /// Label above a field, spaced for a form.
    fn form_row(&mut self, label: &str, field: impl FnOnce(&mut Ui)) -> Response;
    /// Content plus a rich hover card.
    fn hover_card(&mut self, content: impl FnOnce(&mut Ui), add: impl FnOnce(&mut Ui) -> Response) -> Response;
    /// Selectable list row; true when picked.
    fn list_item(&mut self, selected: bool, label: &str) -> bool;
    /// Month grid with prev/next navigation: `ui.calendar(&mut y, &mut m, &mut day)`.
    fn calendar(&mut self, year: &mut i32, month: &mut u32, day: &mut Option<u32>) -> Response;
    /// Button that expands the calendar inline: `ui.date_picker("start", &mut y, &mut m, &mut day)`.
    fn date_picker(&mut self, id: &str, year: &mut i32, month: &mut u32, day: &mut Option<u32>) -> Response;
    /// Indented tree with expand/collapse: `ui.tree("files", &nodes, &mut selected_path)`.
    fn tree(&mut self, id: &str, nodes: &[crate::basic::Node<'_>], selected: &mut Option<Vec<usize>>) -> Response;
    /// Unified-diff renderer: `ui.diff(patch_text)`.
    fn diff(&mut self, patch: &str) -> Response;
    /// Modal command palette; returns the picked index.
    fn palette(&mut self, id: &str, open: &mut bool, query: &mut String, items: &[&str]) -> Option<usize>;
    /// Modal dialog with a title bar.
    fn dialog(&mut self, id: &str, open: &mut bool, title: &str, body: impl FnOnce(&mut Ui)) -> Response;
    /// Floating card under an anchor widget.
    fn popover(&mut self, id: &str, anchor: &Response, open: &mut bool, body: impl FnOnce(&mut Ui));
    /// Right-hand slide-in panel.
    fn sheet(&mut self, id: &str, open: &mut bool, body: impl FnOnce(&mut Ui));
    /// Slide carousel with dots.
    fn carousel(&mut self, index: &mut usize, count: usize) -> Response;
    /// Clickable link-styled text.
    fn link(&mut self, text: &str) -> Response;
    /// Split action + caret menu: returns (main click, picked item).
    fn dropdown_button(&mut self, id: &str, label: &str, items: &[&str]) -> (Response, Option<usize>);
    /// Top toolbar panel.
    fn toolbar<R>(&mut self, id: &str, add: impl FnOnce(&mut Ui) -> R) -> R;
    /// Bottom status bar with left and right regions.
    fn status_bar(&mut self, id: &str, left: &str, right: &str) -> Response;
    /// Chat bubble: `ui.bubble("hi", Side::Left, Some("09:12"))`.
    fn bubble(&mut self, text: &str, side: Side, time: Option<&str>) -> Response;
    /// Centred conversation marker/separator.
    fn marker(&mut self, text: &str) -> Response;
    /// Avatar + author + bubble: `ui.message("Ada", "hello", Side::Left, "09:12")`.
    fn message(&mut self, author: &str, text: &str, side: Side, time: &str) -> Response;
    /// Virtualized list of `len` rows inside a fixed-height scroll area.
    fn virtual_list(&mut self, height: f32, len: usize, row: impl FnMut(&mut Ui, usize));
    /// Column-aligned grid of string rows.
    fn grid_layout(&mut self, id: &str, rows: &[Vec<String>]) -> Response;
    /// Input with prefix/suffix chips.
    fn input_group(&mut self, text: &mut String, prefix: &str, suffix: &str) -> Response;
    /// Image with a themed placeholder fallback (egui's own `ui.image` is shadowed, so this is `kit_image`).
    fn kit_image(&mut self, uri: &str, max_width: f32) -> Response;
    /// Text with the kit type ramp.
    fn text(&mut self, value: &str, kind: crate::basic::LabelKind) -> Response;
    /// Small icon-button toggle.
    fn toggle(&mut self, id: &str, on: &mut bool) -> Response;
    /// Copy-to-clipboard button.
    fn copy_button(&mut self, id: &str, value: &str) -> Response;
    /// Vector icon drawn from painter primitives (no icon font needed).
    fn icon(&mut self, kind: crate::basic::Icon, size: f32, color: egui::Color32) -> Response;
    /// Two-pane resizable split.
    fn resizable<R>(&mut self, id: &str, default_width: f32, add: impl FnOnce(&mut Ui) -> R) -> R;
    /// Left navigation panel of fixed width.
    fn sidebar<R>(&mut self, id: &str, width: f32, add: impl FnOnce(&mut Ui) -> R) -> R;
    /// Grouped settings block.
    fn settings(&mut self, title: &str, body: impl FnOnce(&mut Ui)) -> Response;
    /// Theme provider for a subtree: `ui.root(dark, |ui| ...)`.
    fn root<R>(&mut self, dark: bool, add: impl FnOnce(&mut Ui) -> R) -> R;
    /// Keyboard-focus target over the available area.
    fn focus_trap(&mut self, id: &str) -> Response;
    /// Chat transcript that sticks to the newest row.
    fn message_scroller(&mut self, height: f32, len: usize, row: impl FnMut(&mut Ui, usize));
    /// Dismissable banner: `ui.alert_dismissable("a1", "msg", Variant::Danger, &mut dismissed)`.
    fn alert_dismissable(&mut self, id: &str, text: &str, variant: Variant, dismissed: &mut bool) -> Response;
    /// Multi-line input with a hint and a focus-animating border.
    fn textarea_hint(&mut self, text: &mut String, rows: usize, hint: &str) -> Response;
    /// Stepper with connector rails between circles.
    fn stepper_rail(&mut self, step: usize, total: usize) -> Response;
    /// Dropdown button with a drawn chevron: `ui.dropdown_menu("m", "Menu", &["a", "b"])`.
    /// (Named `dropdown_menu` because `Ui::menu_button` already exists in egui.)
    fn dropdown_menu(&mut self, id: &str, label: &str, items: &[&str]) -> Option<usize>;
    /// Thicker spinner with a smooth arc (egui's own `spinner` is thinner).
    fn kit_spinner(&mut self, size: f32) -> Response;
    /// Labeled progress bar with the percentage on the right.
    fn progress_labeled(&mut self, value: f32, label: &str) -> Response;
    /// Multi-step questionnaire with Back/Next.
    fn questionnaire(&mut self, id: &str, questions: &[crate::basic::Question<'_>], answers: &mut Vec<Option<usize>>) -> Response;
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

    fn select(&mut self, items: &[&str], selected: &mut usize) -> Response {
        basic::select(self, items, selected)
    }

    fn combobox(&mut self, items: &[&str], query: &mut String, selected: &mut usize) -> Response {
        basic::combobox(self, items, query, selected)
    }

    fn switch(&mut self, id: &str, on: &mut bool) -> Response {
        basic::switch(self, id, on)
    }

    fn slider(&mut self, value: &mut f32, range: std::ops::RangeInclusive<f32>) -> Response {
        basic::slider(self, value, range)
    }

    fn text_input(&mut self, text: &mut String, placeholder: &str) -> Response {
        basic::text_input(self, text, placeholder)
    }

    fn textarea(&mut self, text: &mut String, rows: usize) -> Response {
        basic::textarea(self, text, rows)
    }

    fn number_input(&mut self, value: &mut f64, range: std::ops::RangeInclusive<f64>) -> Response {
        basic::number_input(self, value, range)
    }

    fn tooltip(&mut self, text: &str, add: impl FnOnce(&mut Ui) -> Response) -> Response {
        basic::tooltip(self, text, add)
    }

    fn menu(&mut self, label: &str, items: &[&str]) -> Option<usize> {
        basic::menu(self, label, items)
    }

    fn group_box(&mut self, title: &str, body: impl FnOnce(&mut Ui)) -> Response {
        basic::group_box(self, title, body)
    }

    fn description_list(&mut self, rows: &[(&str, &str)]) -> Response {
        basic::description_list(self, rows)
    }

    fn empty(&mut self, title: &str, hint: &str) -> Response {
        basic::empty(self, title, hint)
    }

    fn attachment(&mut self, name: &str, removed: &mut bool) -> Response {
        basic::attachment(self, name, removed)
    }

    fn scrollable<R>(&mut self, height: f32, add: impl FnOnce(&mut Ui) -> R) -> R {
        basic::scrollable(self, height, add)
    }

    fn form_row(&mut self, label: &str, field: impl FnOnce(&mut Ui)) -> Response {
        basic::form_row(self, label, field)
    }

    fn hover_card(&mut self, content: impl FnOnce(&mut Ui), add: impl FnOnce(&mut Ui) -> Response) -> Response {
        basic::hover_card(self, content, add)
    }

    fn list_item(&mut self, selected: bool, label: &str) -> bool {
        basic::list_item(self, selected, label)
    }

    fn calendar(&mut self, year: &mut i32, month: &mut u32, day: &mut Option<u32>) -> Response {
        basic::calendar(self, year, month, day)
    }

    fn date_picker(&mut self, id: &str, year: &mut i32, month: &mut u32, day: &mut Option<u32>) -> Response {
        basic::date_picker(self, id, year, month, day)
    }

    fn tree(&mut self, id: &str, nodes: &[crate::basic::Node<'_>], selected: &mut Option<Vec<usize>>) -> Response {
        basic::tree(self, id, nodes, selected)
    }

    fn diff(&mut self, patch: &str) -> Response {
        basic::diff(self, patch)
    }

    fn palette(&mut self, id: &str, open: &mut bool, query: &mut String, items: &[&str]) -> Option<usize> {
        basic::palette(self, id, open, query, items)
    }

    fn dialog(&mut self, id: &str, open: &mut bool, title: &str, body: impl FnOnce(&mut Ui)) -> Response {
        basic::dialog(self, id, open, title, body)
    }

    fn popover(&mut self, id: &str, anchor: &Response, open: &mut bool, body: impl FnOnce(&mut Ui)) {
        basic::popover(self, id, anchor, open, body)
    }

    fn sheet(&mut self, id: &str, open: &mut bool, body: impl FnOnce(&mut Ui)) {
        basic::sheet(self, id, open, body)
    }

    fn carousel(&mut self, index: &mut usize, count: usize) -> Response {
        basic::carousel(self, index, count)
    }

    fn link(&mut self, text: &str) -> Response {
        basic::link(self, text)
    }

    fn dropdown_button(&mut self, id: &str, label: &str, items: &[&str]) -> (Response, Option<usize>) {
        basic::dropdown_button(self, id, label, items)
    }

    fn toolbar<R>(&mut self, id: &str, add: impl FnOnce(&mut Ui) -> R) -> R {
        basic::toolbar(self, id, add)
    }

    fn status_bar(&mut self, id: &str, left: &str, right: &str) -> Response {
        basic::status_bar(self, id, left, right)
    }

    fn bubble(&mut self, text: &str, side: Side, time: Option<&str>) -> Response {
        basic::bubble(self, text, side, time)
    }

    fn marker(&mut self, text: &str) -> Response {
        basic::marker(self, text)
    }

    fn message(&mut self, author: &str, text: &str, side: Side, time: &str) -> Response {
        basic::message(self, author, text, side, time)
    }

    fn virtual_list(&mut self, height: f32, len: usize, row: impl FnMut(&mut Ui, usize)) {
        basic::virtual_list(self, height, len, row)
    }

    fn grid_layout(&mut self, id: &str, rows: &[Vec<String>]) -> Response {
        basic::grid_layout(self, id, rows)
    }

    fn input_group(&mut self, text: &mut String, prefix: &str, suffix: &str) -> Response {
        basic::input_group(self, text, prefix, suffix)
    }

    fn kit_image(&mut self, uri: &str, max_width: f32) -> Response {
        basic::kit_image(self, uri, max_width)
    }

    fn text(&mut self, value: &str, kind: crate::basic::LabelKind) -> Response {
        basic::text(self, value, kind)
    }

    fn toggle(&mut self, id: &str, on: &mut bool) -> Response {
        basic::toggle(self, id, on)
    }

    fn copy_button(&mut self, id: &str, value: &str) -> Response {
        basic::copy_button(self, id, value)
    }

    fn icon(&mut self, kind: crate::basic::Icon, size: f32, color: egui::Color32) -> Response {
        basic::icon(self, kind, size, color)
    }

    fn questionnaire(&mut self, id: &str, questions: &[crate::basic::Question<'_>], answers: &mut Vec<Option<usize>>) -> Response {
        basic::questionnaire(self, id, questions, answers)
    }

    fn resizable<R>(&mut self, id: &str, default_width: f32, add: impl FnOnce(&mut Ui) -> R) -> R {
        basic::resizable(self, id, default_width, add)
    }

    fn sidebar<R>(&mut self, id: &str, width: f32, add: impl FnOnce(&mut Ui) -> R) -> R {
        basic::sidebar(self, id, width, add)
    }

    fn settings(&mut self, title: &str, body: impl FnOnce(&mut Ui)) -> Response {
        basic::settings(self, title, body)
    }

    fn root<R>(&mut self, dark: bool, add: impl FnOnce(&mut Ui) -> R) -> R {
        basic::root(self, dark, add)
    }

    fn focus_trap(&mut self, id: &str) -> Response {
        basic::focus_trap(self, id)
    }

    fn message_scroller(&mut self, height: f32, len: usize, row: impl FnMut(&mut Ui, usize)) {
        basic::message_scroller(self, height, len, row)
    }

    fn alert_dismissable(&mut self, id: &str, text: &str, variant: Variant, dismissed: &mut bool) -> Response {
        basic::alert_dismissable(self, id, text, variant, dismissed)
    }

    fn textarea_hint(&mut self, text: &mut String, rows: usize, hint: &str) -> Response {
        basic::textarea_hint(self, text, rows, hint)
    }

    fn stepper_rail(&mut self, step: usize, total: usize) -> Response {
        basic::stepper_rail(self, step, total)
    }

    fn dropdown_menu(&mut self, id: &str, label: &str, items: &[&str]) -> Option<usize> {
        basic::menu_button(self, id, label, items)
    }

    fn kit_spinner(&mut self, size: f32) -> Response {
        basic::kit_spinner(self, size)
    }

    fn progress_labeled(&mut self, value: f32, label: &str) -> Response {
        basic::progress_labeled(self, value, label)
    }
}
