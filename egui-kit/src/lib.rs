//! egui-kit: gpui-kit-style components for egui, one trait, zero config.
//!
//! ```no_run
//! egui_kit::Theme::dark().apply(ctx);
//! ui.kit_button("Save");
//! ```
pub mod basic;
pub mod data;
pub mod heavy;

/// Re-exported so callers do not need their own `egui_dock` dependency.
#[cfg(feature = "dock")]
pub use egui_dock::DockState;
pub mod kit;
pub mod theme;

pub use basic::{Icon, LabelKind, Question, Side, Toasts};
pub use kit::KitUi;
pub use theme::Theme;

/// Shared widget variants.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Variant {
    #[default]
    Default,
    Primary,
    Danger,
    Warning,
    Success,
    Ghost,
    Outline,
}
