//! egui-kit: gpui-kit-style components for egui, one trait, zero config.
//!
//! ```no_run
//! egui_kit::Theme::dark().apply(ctx);
//! ui.kit_button("Save");
//! ```
pub mod basic;
pub mod kit;
pub mod theme;

pub use kit::KitUi;
pub use theme::Theme;

/// Shared widget variants.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Variant {
    #[default]
    Default,
    Primary,
    Danger,
    Ghost,
    Outline,
}
