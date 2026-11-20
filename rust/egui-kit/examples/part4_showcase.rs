//! Runnable showcase for egui-kit Part 4 widgets (Alerts, Empty, Media & Pickers).
//! Run with: `cargo run --example part4_showcase`

use eframe::egui;
use egui_kit::components::Root;
use egui_kit::prelude::*;

struct Part4Demo {
    show_alert: bool,
    date: SimpleDate,
    range_end: SimpleDate,
    time: SimpleTime,
    color: egui::Color32,
    listening: bool,
    has_file: bool,
}

impl Default for Part4Demo {
    fn default() -> Self {
        Self {
            show_alert: true,
            date: SimpleDate::new(2025, 3, 15),
            range_end: SimpleDate::new(2025, 3, 21),
            time: SimpleTime::new(14, 30, 0),
            color: egui::Color32::from_rgb(99, 102, 241),
            listening: false,
            has_file: true,
        }
    }
}

impl eframe::App for Part4Demo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui_kit::init(ctx);
        Root::show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.kit_label("egui-kit Part 4 — Alerts, Media & Pickers");
                    ui.add(ThemePicker::new("p4_theme"));
                });
                ui.add_space(12.0);

                ui.kit_alert("Release Notes", "Part 4 brings Calendar, DatePicker, TimeField, ColorPicker, and Speech.");
                ui.add_space(6.0);
                Alert::success("Build Ready", "All 10 Part 4 modules compiled cleanly.")
                    .show_dismissible(ui, "p4_ok", &mut self.show_alert);

                ui.add_space(10.0);
                if self.has_file {
                    if Attachment::new("Cargo.lock", 48_210).progress(Some(0.75)).show(ui).removed {
                        self.has_file = false;
                    }
                } else if Empty::new("No attachments")
                    .description("Upload a workspace manifest or reset demo state.")
                    .action("Restore File")
                    .show(ui)
                    .action_clicked
                {
                    self.has_file = true;
                }

                ui.add_space(10.0);
                ui.add(Image::new(None).width(280.0).caption("Placeholder texture slot"));

                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    ui.kit_date_picker("p4_dob", &mut self.date);
                    ui.kit_time_field("p4_time", &mut self.time);
                });

                ui.add_space(10.0);
                ui.horizontal_top(|ui| {
                    ui.add(Calendar::new("p4_cal", &mut self.date).range(&mut self.range_end));
                    ui.add_space(12.0);
                    ui.kit_color_picker("p4_color", &mut self.color);
                });

                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    ui.kit_marker("v0.4");
                    ui.add(Marker::new("Live").tone(Variant::Success).style(MarkerStyle::Callout).pulsing(true));
                    ui.kit_speech("p4_mic", &mut self.listening);
                });
            });
        });
    }
}

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "egui-kit Part 4 Showcase",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(Part4Demo::default()))),
    )
}
