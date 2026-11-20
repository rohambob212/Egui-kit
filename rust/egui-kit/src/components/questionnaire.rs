//! Interactive multi-step Questionnaire survey widget with animated progress bar and card choices.

use crate::components::button::{Button, ButtonVariant};
use crate::components::icon::{paint_icon, IconName};
use crate::components::progress::Progress;
use crate::theme::active_theme;
use crate::tokens::{ease_out_cubic, lerp_color, Size};
use egui::{FontId, Frame, Id, Pos2, Rect, Response, RichText, Sense, Stroke, Ui, Vec2, Widget};

/// Single prompt with selectable options inside a [`Questionnaire`].
#[derive(Debug, Clone)]
pub struct Question {
    pub prompt: String,
    pub subtitle: Option<String>,
    pub options: Vec<String>,
}

impl Question {
    pub fn new(prompt: impl Into<String>, options: Vec<String>) -> Self {
        Self {
            prompt: prompt.into(),
            subtitle: None,
            options,
        }
    }

    pub fn subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }
}

/// Interactive step-by-step Questionnaire mutating `&mut Vec<Option<usize>>` directly.
pub struct Questionnaire<'a> {
    id_source: &'a str,
    questions: &'a [Question],
    answers: &'a mut Vec<Option<usize>>,
    width: f32,
}

impl<'a> Questionnaire<'a> {
    pub fn new(id_source: &'a str, questions: &'a [Question], answers: &'a mut Vec<Option<usize>>) -> Self {
        Self {
            id_source,
            questions,
            answers,
            width: 420.0,
        }
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width.max(240.0);
        self
    }
}

impl<'a> Widget for Questionnaire<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let tokens = active_theme(ui.ctx());
        let id = Id::new(("egui_kit_questionnaire", self.id_source));
        let total = self.questions.len().max(1);
        if self.answers.len() < self.questions.len() {
            self.answers.resize(self.questions.len(), None);
        }

        let step_id = id.with("step");
        let mut step = ui.ctx().data(|d| d.get_temp::<usize>(step_id).unwrap_or(0)).min(total - 1);
        let answered_count = self.answers.iter().filter(|a| a.is_some()).count();
        let progress_frac = answered_count as f32 / total as f32;

        let resp = Frame::none()
            .fill(tokens.card)
            .stroke(Stroke::new(1.0, tokens.border))
            .rounding(Size::Lg.rounding())
            .inner_margin(egui::Margin::same(16.0))
            .show(ui, |ui| {
                ui.set_width(self.width - 32.0);
                ui.vertical(|ui| {
                    ui.spacing_mut().item_spacing.y = 10.0;
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new(format!("Question {} of {}", step + 1, total))
                                .small()
                                .monospace()
                                .color(tokens.muted_foreground),
                        );
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(
                                RichText::new(format!("{answered_count}/{total} answered"))
                                    .small()
                                    .monospace()
                                    .color(tokens.primary),
                            );
                        });
                    });

                    ui.add(Progress::new(progress_frac).size(Size::Sm));

                    if let Some(q) = self.questions.get(step) {
                        ui.label(RichText::new(&q.prompt).strong().size(15.0).color(tokens.card_foreground));
                        if let Some(sub) = &q.subtitle {
                            ui.label(RichText::new(sub).small().color(tokens.muted_foreground));
                        }

                        for (opt_idx, label) in q.options.iter().enumerate() {
                            let is_sel = self.answers[step] == Some(opt_idx);
                            let sel_t = ease_out_cubic(ui.ctx().animate_bool_with_time(
                                id.with(("opt", step, opt_idx)),
                                is_sel,
                                0.14,
                            ));
                            let (opt_rect, opt_resp) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 36.0), Sense::click());
                            let hov_t = ui.ctx().animate_bool_with_time(id.with(("hov", step, opt_idx)), opt_resp.hovered(), 0.1);
                            let bg = lerp_color(lerp_color(tokens.background, tokens.muted, hov_t), tokens.accent, sel_t);
                            let border = lerp_color(tokens.border, tokens.primary, sel_t);

                            ui.painter().rect(opt_rect, Size::Md.rounding(), bg, Stroke::new(1.0 + 0.5 * sel_t, border));
                            ui.painter().text(
                                Pos2::new(opt_rect.left() + 12.0, opt_rect.center().y),
                                egui::Align2::LEFT_CENTER,
                                label,
                                FontId::proportional(13.5),
                                tokens.card_foreground,
                            );
                            if sel_t > 0.05 {
                                paint_icon(
                                    ui.painter(),
                                    Rect::from_center_size(Pos2::new(opt_rect.right() - 16.0, opt_rect.center().y), Vec2::splat(14.0 * sel_t)),
                                    IconName::Check,
                                    tokens.primary,
                                );
                            }
                            if opt_resp.clicked() {
                                self.answers[step] = Some(opt_idx);
                            }
                        }
                    }

                    ui.add_space(4.0);
                    ui.horizontal(|ui| {
                        if ui.add(Button::new("Back").variant(ButtonVariant::Outline).size(Size::Sm).disabled(step == 0)).clicked() {
                            step = step.saturating_sub(1);
                        }
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if step + 1 < total {
                                if ui.add(Button::new("Next").variant(ButtonVariant::Primary).size(Size::Sm)).clicked() {
                                    step += 1;
                                }
                            } else {
                                ui.add(Button::new("Completed").variant(ButtonVariant::Primary).size(Size::Sm).disabled(answered_count < total));
                            }
                        });
                    });
                });
            })
            .response;

        ui.ctx().data_mut(|d| d.insert_temp(step_id, step));
        resp
    }
}
