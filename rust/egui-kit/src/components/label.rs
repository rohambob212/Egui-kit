//! Semantic Label widget with required indicator, tone control, and truncation support.

use crate::theme::active_theme;
use crate::tokens::Size;
use egui::{Response, RichText, Sense, Ui, Widget};

/// Semantic text tone for [`Label`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LabelTone {
    #[default]
    Default,
    Muted,
    Primary,
    Success,
    Warning,
    Destructive,
}

/// Accessible form and UI label widget with optional required indicator and wrapping control.
pub struct Label {
    text: String,
    size: Size,
    tone: LabelTone,
    required: bool,
    strong: bool,
    truncate: bool,
}

impl Label {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            size: Size::Md,
            tone: LabelTone::Default,
            required: false,
            strong: false,
            truncate: false,
        }
    }

    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub fn tone(mut self, tone: LabelTone) -> Self {
        self.tone = tone;
        self
    }

    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    pub fn strong(mut self, strong: bool) -> Self {
        self.strong = strong;
        self
    }

    pub fn truncate(mut self, truncate: bool) -> Self {
        self.truncate = truncate;
        self
    }
}

impl Widget for Label {
    fn ui(self, ui: &mut Ui) -> Response {
        let tokens = active_theme(ui.ctx());
        let color = match self.tone {
            LabelTone::Default => tokens.foreground,
            LabelTone::Muted => tokens.muted_foreground,
            LabelTone::Primary => tokens.primary,
            LabelTone::Success => tokens.success,
            LabelTone::Warning => tokens.warning,
            LabelTone::Destructive => tokens.destructive,
        };

        let mut rt = RichText::new(&self.text)
            .size(self.size.font_size())
            .color(color);
        if self.strong {
            rt = rt.strong();
        }

        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 3.0;
            let mut label = egui::Label::new(rt).sense(Sense::hover());
            if self.truncate {
                label = label.truncate();
            }
            let resp = ui.add(label);
            if self.required {
                ui.label(
                    RichText::new("*")
                        .size(self.size.font_size())
                        .color(tokens.destructive),
                );
            }
            resp
        })
        .inner
    }
}
