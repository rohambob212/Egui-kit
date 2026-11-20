//! Runnable showcase for egui-kit Part 5 widgets (Layout, Disclosure & Navigation).
//! Run with: `cargo run --example part5_showcase`

use eframe::egui;
use egui_kit::components::{AccordionItem, Root, TabsVariant};
use egui_kit::prelude::*;

struct Part5Demo {
    open_accordion: Option<usize>,
    advanced_open: bool,
    active_tab: usize,
    active_step: usize,
    page: usize,
    split_ratio: f32,
}

impl Default for Part5Demo {
    fn default() -> Self {
        Self {
            open_accordion: Some(0),
            advanced_open: false,
            active_tab: 0,
            active_step: 1,
            page: 4,
            split_ratio: 0.42,
        }
    }
}

impl eframe::App for Part5Demo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui_kit::init(ctx);
        Root::show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.kit_label("egui-kit Part 5 — Layout, Disclosure & Navigation");
                    ui.add(ThemePicker::new("p5_theme"));
                });
                ui.add_space(12.0);

                ui.kit_group_box("Compiler Target", |ui| {
                    let meta = [
                        ("Crate", "egui-kit 0.1"),
                        ("Target", "aarch64-apple-darwin"),
                        ("Profile", "release-lto"),
                        ("MSRV", "1.78"),
                    ];
                    ui.kit_description_list(&meta);
                });

                ui.add_space(10.0);
                GridLayout::columns(3).gap(10.0).show(ui, |grid| {
                    grid.cell(1, |ui| { ui.kit_button("Cell 1 (1 col)"); });
                    grid.cell(2, |ui| { ui.kit_button("Cell 2 (2 cols)"); });
                });

                ui.add_space(10.0);
                let left_pct = self.split_ratio * 100.0;
                Resizable::horizontal("p5_split")
                    .ratio(&mut self.split_ratio)
                    .height(140.0)
                    .show(
                        ui,
                        |left| {
                            left.kit_label(format!("Left ({left_pct:.0}%)"));
                        },
                        |right| {
                            Scrollable::vertical("p5_log").max_height(110.0).show(right, |ui| {
                                for idx in 1..=12 {
                                    ui.label(format!("[{idx:02}] Frame synced"));
                                }
                            });
                        },
                    );

                ui.add_space(10.0);
                let items = [
                    AccordionItem::new("Zero-dependency Painter", "All icons, waveforms, and dividers draw directly via egui::Painter."),
                    AccordionItem::new("Immediate-Mode State", "Simple &mut bindings avoid retained-tree boilerplate."),
                ];
                ui.kit_accordion("p5_faq", &mut self.open_accordion, &items);

                ui.add_space(8.0);
                Collapsible::new("p5_adv", "Advanced Linker Flags")
                    .open(&mut self.advanced_open)
                    .show(ui, |ui| {
                        ui.kit_label("--codegen lto=thin -C target-cpu=native");
                    });

                ui.add_space(10.0);
                let tabs = ["Overview", "Props", "Crate Source"];
                Tabs::new("p5_tabs", &mut self.active_tab, &tabs)
                    .variant(TabsVariant::Underline)
                    .show(ui);

                ui.add_space(10.0);
                let steps = ["Select Crate", "Configure", "Verify", "Publish"];
                Stepper::new(&steps, self.active_step).show_clickable(ui, &mut self.active_step);

                ui.add_space(10.0);
                ui.kit_pagination(&mut self.page, 12);
            });
        });
    }
}

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "egui-kit Part 5 Showcase",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(Part5Demo::default()))),
    )
}
