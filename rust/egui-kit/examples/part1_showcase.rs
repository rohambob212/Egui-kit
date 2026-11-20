//! Runnable eframe showcase demonstrating all 10 Part 1 `egui-kit` widgets.

use eframe::egui;
use egui_kit::prelude::*;

struct Part1Showcase {
    clicks: u32,
    dropdown_choice: Option<String>,
    tags: Vec<String>,
    unread: u32,
}

impl Default for Part1Showcase {
    fn default() -> Self {
        Self {
            clicks: 0,
            dropdown_choice: None,
            tags: vec!["egui".into(), "gpui-kit".into(), "rust".into()],
            unread: 4,
        }
    }
}

impl eframe::App for Part1Showcase {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        Root::show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.add(Label::new("egui-kit Part 1 Showcase").size(Size::Lg).strong(true));
                ui.add(ThemePicker::new("showcase_theme"));
            });
            ui.separator();

            ui.horizontal(|ui| {
                if ui.add(Button::new("Primary Action").icon(IconName::Sparkles)).clicked() {
                    self.clicks += 1;
                    Root::push_toast(
                        ctx,
                        "Button clicked",
                        Some(format!("Total clicks: {}", self.clicks)),
                        ToastTone::Success,
                    );
                }

                let btn_resp = ui.add(Button::new("Inbox").variant(ButtonVariant::Secondary));
                Badge::count(self.unread).attach_to(ui, &btn_resp);

                ui.add(DropdownButton::new(
                    "actions",
                    "Actions",
                    vec![
                        DropdownItem::new("new", "New File").shortcut("Cmd+N"),
                        DropdownItem::new("dup", "Duplicate").shortcut("Cmd+D"),
                        DropdownItem::new("del", "Delete").destructive(true),
                    ],
                    &mut self.dropdown_choice,
                ));

                ui.add(Kbd::new("Cmd+K"));
            });

            ui.add_space(12.0);
            ui.horizontal(|ui| {
                let mut to_remove: Option<usize> = None;
                for (idx, tag) in self.tags.iter().enumerate() {
                    if Tag::new(tag).closable(true).show(ui).closed {
                        to_remove = Some(idx);
                    }
                }
                if let Some(idx) = to_remove {
                    self.tags.remove(idx);
                }
            });

            ui.add_space(12.0);
            ui.add(AvatarGroup::new(vec![
                Avatar::new("Ada Lovelace").status(AvatarStatus::Online),
                Avatar::new("Linus Torvalds").status(AvatarStatus::Away),
                Avatar::new("Graydon Hoare").status(AvatarStatus::Busy),
                Avatar::new("Niko Matsakis"),
                Avatar::new("Raph Levien"),
            ]));
        });
    }
}

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "egui-kit Part 1 Showcase",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(Part1Showcase::default()))),
    )
}
