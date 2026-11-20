//! Minimal showcase of all 10 Part 2 selection and feedback widgets.

use eframe::egui;
use egui_kit::prelude::*;

#[derive(PartialEq, Clone, Copy)]
enum BuildProfile {
    Debug,
    Release,
    Bench,
}

struct Part2App {
    telemetry: bool,
    autoupdate: bool,
    dark_icons: bool,
    profile: BuildProfile,
    volume: f32,
    rating: f32,
    progress: f32,
}

impl Default for Part2App {
    fn default() -> Self {
        Self {
            telemetry: true,
            autoupdate: false,
            dark_icons: true,
            profile: BuildProfile::Release,
            volume: 64.0,
            rating: 4.5,
            progress: 0.72,
        }
    }
}

impl eframe::App for Part2App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        Root::show(ctx, |ui| {
            ui.kit_label("Part 2: Selection & Feedback States");
            ui.add(ThemePicker::new("part2_theme"));
            ui.separator();

            ui.kit_checkbox(&mut self.telemetry, "Enable crate telemetry");
            ui.horizontal(|ui| {
                ui.kit_radio(&mut self.profile, BuildProfile::Debug, "Debug");
                ui.kit_radio(&mut self.profile, BuildProfile::Release, "Release");
                ui.kit_radio(&mut self.profile, BuildProfile::Bench, "Bench");
            });

            ui.kit_switch(&mut self.autoupdate, "Auto-check updates");
            ui.kit_toggle(&mut self.dark_icons, "High-contrast icons");

            ui.add(Slider::new(&mut self.volume, 0.0..=100.0).step(1.0).label("Audio Gain").suffix("%"));
            ui.add(Rating::new(&mut self.rating).allow_half(true));

            ui.add(Progress::new(self.progress).show_label(true));
            ui.add(Progress::indeterminate().tone(Variant::Success));
            ui.add(Spinner::new().label("Compiling crate..."));

            ui.add(Skeleton::lines(2));
            ui.add(Shimmer::rect(240.0, 28.0));
        });
    }
}

fn main() -> eframe::Result {
    eframe::run_native(
        "egui-kit Part 2 Showcase",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(Part2App::default()))),
    )
}
