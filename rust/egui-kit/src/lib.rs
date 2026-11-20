//! `egui-kit`: Idiomatic `egui.rs` port of `gpui-kit` components and design tokens.
//!
//! # Quick Start
//! ```rust,no_run
//! use egui_kit::{init, set_theme, ThemeId};
//! use egui_kit::components::{Button, ButtonVariant, Root};
//!
//! fn ui_frame(ctx: &egui::Context) {
//!     init(ctx);
//!     Root::show(ctx, |ui| {
//!         if ui.add(Button::new("Save Changes").variant(ButtonVariant::Primary)).clicked() {
//!             // handle click
//!         }
//!     });
//! }
//! ```

pub mod components;
pub mod theme;
pub mod tokens;

pub use theme::{active_theme, set_theme, ThemeId, ThemeMode, ThemeTokens};
pub use tokens::{Alignment, Orientation, Size, Variant};

/// Initializes `egui-kit` defaults and registers the default `ThemeTokens`
/// inside `egui::Context` memory if no theme has been configured yet.
pub fn init(ctx: &egui::Context) {
    let initialized_id = egui::Id::new("egui_kit_initialized");
    let already_init = ctx.data(|d| d.get_temp::<bool>(initialized_id).unwrap_or(false));
    if !already_init {
        let default_theme = if ctx.style().visuals.dark_mode {
            ThemeId::TokyoNight
        } else {
            ThemeId::AuroraLight
        };
        set_theme(ctx, default_theme);
        ctx.data_mut(|d| d.insert_temp(initialized_id, true));
    }
}

/// Convenience extension trait for rendering `egui-kit` widgets directly on `egui::Ui`.
pub trait UiExt {
    /// Returns the active [`ThemeTokens`] stored in the current `egui::Context`.
    fn kit_theme(&self) -> ThemeTokens;

    /// Renders a styled [`components::Button`].
    fn kit_button(&mut self, label: impl Into<String>) -> egui::Response;

    /// Renders a semantic [`components::Label`].
    fn kit_label(&mut self, text: impl Into<String>) -> egui::Response;

    /// Renders a [`components::Kbd`] keyboard shortcut badge.
    fn kit_kbd(&mut self, keys: impl Into<String>) -> egui::Response;

    /// Renders a [`components::Checkbox`] bound directly to `&mut bool`.
    fn kit_checkbox(&mut self, checked: &mut bool, label: impl Into<String>) -> egui::Response;

    /// Renders a [`components::Radio`] choice bound directly to `&mut T`.
    fn kit_radio<T: PartialEq>(&mut self, current: &mut T, value: T, label: impl Into<String>) -> egui::Response;

    /// Renders a labeled [`components::Switch`] bound directly to `&mut bool`.
    fn kit_switch(&mut self, on: &mut bool, label: impl Into<String>) -> egui::Response;

    /// Renders a [`components::Toggle`] button bound directly to `&mut bool`.
    fn kit_toggle(&mut self, pressed: &mut bool, label: impl Into<String>) -> egui::Response;

    /// Renders a [`components::Slider`] over `range`.
    fn kit_slider(&mut self, value: &mut f32, range: std::ops::RangeInclusive<f32>) -> egui::Response;

    /// Renders a 5-star [`components::Rating`].
    fn kit_rating(&mut self, value: &mut f32) -> egui::Response;

    /// Renders a determinate [`components::Progress`] bar (`0.0..=1.0`).
    fn kit_progress(&mut self, value: f32) -> egui::Response;

    /// Renders an animated [`components::Spinner`].
    fn kit_spinner(&mut self) -> egui::Response;

    /// Renders a [`components::Input`] bound directly to `&mut String`.
    fn kit_input(&mut self, value: &mut String, placeholder: impl Into<String>) -> egui::Response;

    /// Renders a [`components::TextArea`] bound directly to `&mut String`.
    fn kit_textarea(&mut self, value: &mut String, placeholder: impl Into<String>) -> egui::Response;

    /// Renders a [`components::NumberInput`] stepper bound directly to `&mut f32`.
    fn kit_number_input(&mut self, value: &mut f32, range: std::ops::RangeInclusive<f32>) -> egui::Response;

    /// Renders a segmented [`components::OtpInput`] bound directly to `&mut String`.
    fn kit_otp(&mut self, code: &mut String, length: usize) -> egui::Response;

    /// Renders a [`components::Select`] dropdown bound directly to `&mut usize`.
    fn kit_select<T: AsRef<str>>(&mut self, id_source: &str, selected: &mut usize, options: &[T]) -> egui::Response;

    /// Renders a searchable [`components::Combobox`] bound directly to `&mut Option<usize>`.
    fn kit_combobox<T: AsRef<str>>(&mut self, id_source: &str, selected: &mut Option<usize>, options: &[T]) -> egui::Response;

    /// Renders a one-click [`components::Clipboard`] copy button.
    fn kit_clipboard(&mut self, id_source: &str, text: &str) -> egui::Response;

    /// Renders an inline [`components::Alert`] banner.
    fn kit_alert(&mut self, title: impl Into<String>, description: impl Into<String>) -> egui::Response;

    /// Renders a zero-state [`components::Empty`] placeholder.
    fn kit_empty(&mut self, title: impl Into<String>) -> egui::Response;

    /// Renders a [`components::Calendar`] grid bound directly to `&mut SimpleDate`.
    fn kit_calendar(&mut self, id_source: &str, date: &mut components::SimpleDate) -> egui::Response;

    /// Renders a popover [`components::DatePicker`] bound directly to `&mut SimpleDate`.
    fn kit_date_picker(&mut self, id_source: &str, date: &mut components::SimpleDate) -> egui::Response;

    /// Renders a segmented [`components::TimeField`] bound directly to `&mut SimpleTime`.
    fn kit_time_field(&mut self, id_source: &str, time: &mut components::SimpleTime) -> egui::Response;

    /// Renders an HSV [`components::ColorPicker`] bound directly to `&mut egui::Color32`.
    fn kit_color_picker(&mut self, id_source: &str, color: &mut egui::Color32) -> egui::Response;

    /// Renders a spatial [`components::Marker`] badge.
    fn kit_marker(&mut self, label: impl Into<String>) -> egui::Response;

    /// Renders a voice [`components::Speech`] trigger bound directly to `&mut bool`.
    fn kit_speech(&mut self, id_source: &str, listening: &mut bool) -> egui::Response;

    /// Renders a titled [`components::GroupBox`] container.
    fn kit_group_box<R>(&mut self, title: impl Into<String>, add_contents: impl FnOnce(&mut egui::Ui) -> R) -> egui::InnerResponse<R>;

    /// Renders a 2-column [`components::DescriptionList`] key-value table.
    fn kit_description_list(&mut self, items: &[(&str, &str)]) -> egui::Response;

    /// Renders a vertical [`components::Scrollable`] region with max height.
    fn kit_scrollable<R>(&mut self, id_source: &str, max_height: f32, add_contents: impl FnOnce(&mut egui::Ui) -> R) -> egui::scroll_area::ScrollAreaOutput<R>;

    /// Renders a stacked [`components::Accordion`] bound directly to `&mut Option<usize>`.
    fn kit_accordion(&mut self, id_source: &str, open_idx: &mut Option<usize>, items: &[components::AccordionItem<'_>]) -> egui::Response;

    /// Renders a [`components::Collapsible`] disclosure block persisted in `egui::Memory`.
    fn kit_collapsible(&mut self, id_source: &str, title: impl Into<String>, add_contents: impl FnOnce(&mut egui::Ui)) -> bool;

    /// Renders a [`components::Tabs`] bar bound directly to `&mut usize`.
    fn kit_tabs<T: AsRef<str>>(&mut self, id_source: &str, active: &mut usize, labels: &[T]) -> components::TabsResponse;

    /// Renders a horizontal [`components::Stepper`] progress indicator.
    fn kit_stepper<T: AsRef<str>>(&mut self, steps: &[T], active_step: usize) -> egui::Response;

    /// Renders a 1-based [`components::Pagination`] bar bound directly to `&mut usize`.
    fn kit_pagination(&mut self, page: &mut usize, total_pages: usize) -> egui::Response;

    /// Renders a modal [`components::Dialog`] bound directly to `&mut bool`.
    fn kit_dialog<R>(&self, id_source: &str, open: &mut bool, title: impl Into<String>, add_body: impl FnOnce(&mut egui::Ui) -> R) -> Option<R>;

    /// Renders an edge-anchored [`components::Sheet`] drawer bound directly to `&mut bool`.
    fn kit_sheet<R>(&self, id_source: &str, open: &mut bool, title: impl Into<String>, add_body: impl FnOnce(&mut egui::Ui) -> R) -> Option<R>;

    /// Pushes an edge-sliding [`components::Notification`] toast into [`components::NotificationManager`].
    fn kit_notify(&self, toast: components::Notification) -> u64;

    /// Renders a fuzzy-searchable [`components::Command`] palette bound to `&mut bool`.
    fn kit_command(&self, id_source: &str, open: &mut bool, items: &[components::CommandItem]) -> Option<String>;
}

impl UiExt for egui::Ui {
    fn kit_theme(&self) -> ThemeTokens {
        active_theme(self.ctx())
    }

    fn kit_button(&mut self, label: impl Into<String>) -> egui::Response {
        self.add(components::Button::new(label))
    }

    fn kit_label(&mut self, text: impl Into<String>) -> egui::Response {
        self.add(components::Label::new(text))
    }

    fn kit_kbd(&mut self, keys: impl Into<String>) -> egui::Response {
        self.add(components::Kbd::new(keys))
    }

    fn kit_checkbox(&mut self, checked: &mut bool, label: impl Into<String>) -> egui::Response {
        self.add(components::Checkbox::new(checked, label))
    }

    fn kit_radio<T: PartialEq>(&mut self, current: &mut T, value: T, label: impl Into<String>) -> egui::Response {
        self.add(components::Radio::new(current, value, label))
    }

    fn kit_switch(&mut self, on: &mut bool, label: impl Into<String>) -> egui::Response {
        self.add(components::Switch::new(on).label(label))
    }

    fn kit_toggle(&mut self, pressed: &mut bool, label: impl Into<String>) -> egui::Response {
        self.add(components::Toggle::new(pressed, label))
    }

    fn kit_slider(&mut self, value: &mut f32, range: std::ops::RangeInclusive<f32>) -> egui::Response {
        self.add(components::Slider::new(value, range))
    }

    fn kit_rating(&mut self, value: &mut f32) -> egui::Response {
        self.add(components::Rating::new(value))
    }

    fn kit_progress(&mut self, value: f32) -> egui::Response {
        self.add(components::Progress::new(value))
    }

    fn kit_spinner(&mut self) -> egui::Response {
        self.add(components::Spinner::new())
    }

    fn kit_input(&mut self, value: &mut String, placeholder: impl Into<String>) -> egui::Response {
        self.add(components::Input::new(value).placeholder(placeholder))
    }

    fn kit_textarea(&mut self, value: &mut String, placeholder: impl Into<String>) -> egui::Response {
        self.add(components::TextArea::new(value).placeholder(placeholder))
    }

    fn kit_number_input(&mut self, value: &mut f32, range: std::ops::RangeInclusive<f32>) -> egui::Response {
        self.add(components::NumberInput::new(value, range))
    }

    fn kit_otp(&mut self, code: &mut String, length: usize) -> egui::Response {
        self.add(components::OtpInput::new(code, length))
    }

    fn kit_select<T: AsRef<str>>(&mut self, id_source: &str, selected: &mut usize, options: &[T]) -> egui::Response {
        self.add(components::Select::new(id_source, selected, options))
    }

    fn kit_combobox<T: AsRef<str>>(&mut self, id_source: &str, selected: &mut Option<usize>, options: &[T]) -> egui::Response {
        self.add(components::Combobox::new(id_source, selected, options))
    }

    fn kit_clipboard(&mut self, id_source: &str, text: &str) -> egui::Response {
        self.add(components::Clipboard::new(id_source, text))
    }

    fn kit_alert(&mut self, title: impl Into<String>, description: impl Into<String>) -> egui::Response {
        self.add(components::Alert::info(title, description))
    }

    fn kit_empty(&mut self, title: impl Into<String>) -> egui::Response {
        self.add(components::Empty::new(title))
    }

    fn kit_calendar(&mut self, id_source: &str, date: &mut components::SimpleDate) -> egui::Response {
        self.add(components::Calendar::new(id_source, date))
    }

    fn kit_date_picker(&mut self, id_source: &str, date: &mut components::SimpleDate) -> egui::Response {
        self.add(components::DatePicker::new(id_source, date))
    }

    fn kit_time_field(&mut self, id_source: &str, time: &mut components::SimpleTime) -> egui::Response {
        self.add(components::TimeField::new(id_source, time))
    }

    fn kit_color_picker(&mut self, id_source: &str, color: &mut egui::Color32) -> egui::Response {
        self.add(components::ColorPicker::new(id_source, color))
    }

    fn kit_marker(&mut self, label: impl Into<String>) -> egui::Response {
        self.add(components::Marker::new(label))
    }

    fn kit_speech(&mut self, id_source: &str, listening: &mut bool) -> egui::Response {
        self.add(components::Speech::new(id_source, listening))
    }

    fn kit_group_box<R>(&mut self, title: impl Into<String>, add_contents: impl FnOnce(&mut egui::Ui) -> R) -> egui::InnerResponse<R> {
        components::GroupBox::new(title).show(self, add_contents)
    }

    fn kit_description_list(&mut self, items: &[(&str, &str)]) -> egui::Response {
        self.add(components::DescriptionList::new(items))
    }

    fn kit_scrollable<R>(&mut self, id_source: &str, max_height: f32, add_contents: impl FnOnce(&mut egui::Ui) -> R) -> egui::scroll_area::ScrollAreaOutput<R> {
        components::Scrollable::vertical(id_source).max_height(max_height).show(self, add_contents)
    }

    fn kit_accordion(&mut self, id_source: &str, open_idx: &mut Option<usize>, items: &[components::AccordionItem<'_>]) -> egui::Response {
        self.add(components::Accordion::new(id_source, open_idx, items))
    }

    fn kit_collapsible(&mut self, id_source: &str, title: impl Into<String>, add_contents: impl FnOnce(&mut egui::Ui)) -> bool {
        components::Collapsible::new(id_source, title).show(self, add_contents)
    }

    fn kit_tabs<T: AsRef<str>>(&mut self, id_source: &str, active: &mut usize, labels: &[T]) -> components::TabsResponse {
        components::Tabs::new(id_source, active, labels).show(self)
    }

    fn kit_stepper<T: AsRef<str>>(&mut self, steps: &[T], active_step: usize) -> egui::Response {
        components::Stepper::new(steps, active_step).show(self)
    }

    fn kit_pagination(&mut self, page: &mut usize, total_pages: usize) -> egui::Response {
        self.add(components::Pagination::new(page, total_pages))
    }

    fn kit_dialog<R>(&self, id_source: &str, open: &mut bool, title: impl Into<String>, add_body: impl FnOnce(&mut egui::Ui) -> R) -> Option<R> {
        components::Dialog::new(id_source, open).title(title).show(self.ctx(), add_body)
    }

    fn kit_sheet<R>(&self, id_source: &str, open: &mut bool, title: impl Into<String>, add_body: impl FnOnce(&mut egui::Ui) -> R) -> Option<R> {
        components::Sheet::right(id_source, open).title(title).show(self.ctx(), add_body)
    }

    fn kit_notify(&self, toast: components::Notification) -> u64 {
        components::NotificationManager::push(self.ctx(), toast)
    }

    fn kit_command(&self, id_source: &str, open: &mut bool, items: &[components::CommandItem]) -> Option<String> {
        components::Command::new(id_source, open, items).show(self.ctx())
    }
}

/// Re-exports of commonly used `egui-kit` types and widgets.
pub mod prelude {
    pub use crate::components::{
        Accordion, AccordionItem, Alert, AlertDialog, AlertDialogResponse, AlertTone, Attachment,
        AttachmentResponse, Avatar, AvatarGroup, AvatarShape, AvatarStatus, Badge, BadgeVariant,
        Button, ButtonVariant, Calendar, Checkbox, Clipboard, Collapsible, ColorPicker, Combobox,
        Command, CommandItem, DatePicker, DescriptionItem, DescriptionList, Dialog, DropdownButton,
        DropdownItem, Empty, EmptyResponse, FocusTrap, Form, FormField, GridLayout, GridScope,
        GroupBox, HoverCard, Icon, IconName, Image, Input, InputGroup, Kbd, Label, LabelTone,
        Marker, MarkerStyle, Menu, MenuItem, Notification, NotificationEdge, NotificationManager,
        NotificationTone, NumberInput, OtpInput, Pagination, Popover, PopoverSide, Progress,
        Question, Questionnaire, Radio, Rating, Resizable, ResponseTooltipExt, Root, Scrollable,
        Select, Sheet, SheetSide, Shimmer, SimpleDate, SimpleTime, Skeleton, Slider, Speech,
        Spinner, Stepper, Switch, Tabs, TabsResponse, TabsVariant, Tag, TagVariant, TextArea,
        ThemePicker, TimeField, ToastItem, ToastTone, Toggle, Tooltip,
    };
    pub use crate::{active_theme, init, set_theme, Orientation, Size, ThemeId, ThemeMode, ThemeTokens, UiExt, Variant};
}
