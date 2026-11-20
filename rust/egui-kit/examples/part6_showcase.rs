//! Runnable showcase for egui-kit Part 6 widgets (Overlays, Popovers & Command Surfaces).
//! Run with: `cargo run --example part6_showcase`

use eframe::egui;
use egui_kit::components::{
    AlertDialog, Command, CommandItem, Dialog, FocusTrap, HoverCard, Menu, MenuItem,
    Notification, NotificationEdge, NotificationManager, Popover, ResponseTooltipExt, Root, Sheet,
};
use egui_kit::prelude::*;

struct Part6Demo {
    dialog_open: bool,
    alert_open: bool,
    sheet_open: bool,
    command_open: bool,
    trap_active: bool,
    trap_input: String,
    menu_checked: bool,
    last_event: String,
}

impl Default for Part6Demo {
    fn default() -> Self {
        Self {
            dialog_open: false,
            alert_open: false,
            sheet_open: false,
            command_open: false,
            trap_active: false,
            trap_input: "Scoped text".to_string(),
            menu_checked: true,
            last_event: "Idle".to_string(),
        }
    }
}

impl eframe::App for Part6Demo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui_kit::init(ctx);
        Root::show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.kit_label("egui-kit Part 6 — Overlays & Command Surfaces");
                ui.add(ThemePicker::new("p6_theme"));
            });
            ui.add_space(12.0);

            ui.horizontal_wrapped(|ui| {
                let d_btn = ui.kit_button("Open Dialog");
                d_btn.kit_tooltip_shortcut("Launch modal dialog", "Cmd+D");
                if d_btn.clicked() {
                    self.dialog_open = true;
                }

                if ui.kit_button("Open AlertDialog").clicked() {
                    self.alert_open = true;
                }

                if ui.kit_button("Open Right Sheet").clicked() {
                    self.sheet_open = true;
                }

                let pop_btn = ui.kit_button("Toggle Popover");
                Popover::new("p6_pop").show_for(ui, &pop_btn, |ui| {
                    ui.kit_label("Anchored Popover");
                    ui.label("Auto-clamps inside viewport bounds.");
                });

                let hover_btn = ui.kit_button("Hover Preview");
                HoverCard::new("p6_hover").show_for(ui, &hover_btn, |ui| {
                    ui.kit_label("egui-kit v0.1.0");
                    ui.label("60 / 80 widgets ready across Parts 1–6.");
                });

                if ui.kit_button("Push Edge Toast").clicked() {
                    NotificationManager::push(
                        ctx,
                        Notification::success("Build Published")
                            .message("Part 6 overlays compiled cleanly.")
                            .action("View"),
                    );
                }

                if ui.kit_button("Command Palette").clicked() {
                    self.command_open = true;
                }
            });

            ui.add_space(12.0);
            let ctx_zone = ui.kit_button("Right-Click for Menu");
            let menu_items = [
                MenuItem::action_shortcut("rename", "Rename Symbol", "F2"),
                MenuItem::checkbox("format", "Format on Save", self.menu_checked),
                MenuItem::Separator,
                MenuItem::action("delete", "Delete"),
            ];
            if let Some(id) = Menu::context(&ctx_zone, &menu_items) {
                if id == "format" {
                    self.menu_checked = !self.menu_checked;
                }
                self.last_event = format!("Menu: {id}");
            }

            ui.add_space(12.0);
            ui.kit_checkbox(&mut self.trap_active, "Activate FocusTrap Ring");
            let trap_ids = [egui::Id::new("trap_a"), egui::Id::new("trap_b")];
            FocusTrap::new("p6_trap")
                .active(self.trap_active)
                .show_ring(true)
                .show(ui, &trap_ids, |ui| {
                    ui.horizontal(|ui| {
                        ui.kit_input(&mut self.trap_input, "Focusable field");
                        ui.kit_button("Confirm");
                    });
                });

            ui.add_space(8.0);
            ui.kit_label(format!("Last event: {}", self.last_event));
        });

        Dialog::new("p6_dialog", &mut self.dialog_open)
            .title("Workspace Preferences")
            .description("Configure immediate-mode overlay defaults.")
            .show(ctx, |ui| {
                ui.label("Press Escape or click backdrop to dismiss.");
            });

        let alert_res = AlertDialog::confirm("Delete Release Tag?", &mut self.alert_open)
            .description("This cannot be undone and blocks outside-click dismissal.")
            .destructive(true)
            .show(ctx);
        if alert_res.confirmed {
            self.last_event = "AlertDialog confirmed".into();
        }

        Sheet::right("p6_sheet", &mut self.sheet_open)
            .title("Inspector Drawer")
            .subtitle("Edge-anchored overlay panel")
            .show(ctx, |ui| {
                ui.label("Sheet content renders cleanly at 60fps.");
            });

        let commands = [
            CommandItem::new("theme", "Switch Theme", "Appearance").shortcut("Cmd+T"),
            CommandItem::new("build", "Run Cargo Check", "Workspace").shortcut("Cmd+B"),
            CommandItem::new("toast", "Trigger Notification", "Overlays"),
        ];
        if let Some(cmd_id) = Command::new("p6_cmd", &mut self.command_open, &commands).show(ctx) {
            self.last_event = format!("Executed command: {cmd_id}");
        }

        if let Some(id) = NotificationManager::show(ctx, NotificationEdge::BottomRight) {
            self.last_event = format!("Toast action #{id}");
        }
    }
}

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "egui-kit Part 6 Showcase",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(Part6Demo::default()))),
    )
}
