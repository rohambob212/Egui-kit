//! Showcase of all 10 Part 3 text inputs and form controls.

use eframe::egui;
use egui_kit::prelude::*;

struct Part3App {
    query: String,
    host: String,
    notes: String,
    workers: f32,
    otp_code: String,
    region_idx: usize,
    crate_idx: Option<usize>,
    crate_name: String,
    answers: Vec<Option<usize>>,
}

impl Default for Part3App {
    fn default() -> Self {
        Self {
            query: String::new(),
            host: "docs.rs/egui-kit".into(),
            notes: "Immediate-mode widgets for desktop apps.".into(),
            workers: 4.0,
            otp_code: "482".into(),
            region_idx: 0,
            crate_idx: Some(0),
            crate_name: "egui-kit".into(),
            answers: vec![Some(0), None, None],
        }
    }
}

impl eframe::App for Part3App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let regions = ["us-east-1", "eu-central-1", "ap-northeast-1"];
        let crates = ["egui", "eframe", "wgpu", "serde", "tokio"];
        let questions = vec![
            Question::new("Target platform?", vec!["Desktop".into(), "WebAssembly".into(), "Both".into()]),
            Question::new("Preferred renderer?", vec!["glow".into(), "wgpu".into()]),
            Question::new("Default theme mode?", vec!["Dark".into(), "Light".into()]),
        ];

        Root::show(ctx, |ui| {
            ui.kit_label("Part 3: Text Inputs & Form Controls");
            ui.add(ThemePicker::new("part3_theme"));
            ui.separator();

            Form::new("publish_form").show(ui, |ui| {
                FormField::new("Search Query").show(ui, |ui| {
                    ui.add(Input::new(&mut self.query).prefix_icon(IconName::Search).placeholder("Filter modules...").clearable(true))
                });

                FormField::new("Docs Endpoint").required(true).show(ui, |ui| {
                    InputGroup::new().prefix("https://").suffix(".rs").show(ui, |ui| {
                        ui.add(egui::TextEdit::singleline(&mut self.host).frame(false))
                    })
                });

                FormField::new("Release Notes").show(ui, |ui| {
                    ui.add(TextArea::new(&mut self.notes).rows(3).max_chars(140))
                });

                ui.horizontal(|ui| {
                    ui.add(NumberInput::new(&mut self.workers, 1.0..=32.0).step(1.0));
                    ui.kit_clipboard("copy_cmd", "cargo add egui-kit");
                });

                FormField::new("2FA Verification Code").show(ui, |ui| {
                    ui.kit_otp(&mut self.otp_code, 6)
                });

                ui.horizontal(|ui| {
                    ui.kit_select("region", &mut self.region_idx, &regions);
                    ui.kit_combobox("crate_pick", &mut self.crate_idx, &crates);
                });

                let err = if self.crate_name.trim().is_empty() {
                    Some("Crate name is required")
                } else {
                    None
                };
                FormField::new("Crate Slug").required(true).error(err).show(ui, |ui| {
                    ui.add(Input::new(&mut self.crate_name).error(err.is_some()))
                });
            });

            ui.add_space(12.0);
            ui.add(Questionnaire::new("setup_survey", &questions, &mut self.answers));
        });
    }
}

fn main() -> eframe::Result {
    eframe::run_native(
        "egui-kit Part 3 Showcase",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(Part3App::default()))),
    )
}
