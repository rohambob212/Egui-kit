use eframe::egui;
use egui_kit::{KitUi, Theme, Variant};

fn main() -> eframe::Result<()> {
    let opts = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1100.0, 760.0])
            .with_title("egui-kit gallery"),
        ..Default::default()
    };
    eframe::run_native(
        "egui-kit gallery",
        opts,
        Box::new(|cc| {
            Theme::dark().apply(&cc.egui_ctx);
            Ok(Box::new(App { dark: true, page: Page::Theme }))
        }),
    )
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Page {
    Theme,
}

struct App {
    page: Page,
    dark: bool,
}

const PAGES: &[(Page, &str)] = &[(Page::Theme, "Theme")];

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        if self.dark {
            Theme::dark().apply(&ctx);
        } else {
            Theme::light().apply(&ctx);
        }

        egui::Panel::top("top").show(ui, |ui| {
            let t = Theme::get(ui.ctx());
            ui.horizontal(|ui| {
                ui.heading("egui-kit");
                ui.add_space(8.0);
                ui.label(egui::RichText::new("gpui-kit components for egui").color(t.text_muted));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button(if self.dark { "light mode" } else { "dark mode" }).clicked() {
                        self.dark = !self.dark;
                    }
                });
            });
        });

        egui::Panel::left("nav").resizable(false).exact_size(190.0).show(ui, |ui| {
            ui.add_space(6.0);
            for (page, name) in PAGES {
                if ui.selectable_label(self.page == *page, *name).clicked() {
                    self.page = *page;
                }
            }
        });

        egui::CentralPanel::default_margins().show(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| match self.page {
                Page::Theme => theme_page(ui),
            });
        });
    }
}

fn theme_page(ui: &mut egui::Ui) {
    let t = Theme::get(ui.ctx());
    ui.heading("Theme");
    ui.label(egui::RichText::new("Semantic tokens, applied once with Theme::apply(ctx).").color(t.text_muted));
    ui.add_space(10.0);

    let tokens: &[(&str, egui::Color32)] = &[
        ("bg", t.bg),
        ("panel", t.panel),
        ("surface", t.surface),
        ("surface_hover", t.surface_hover),
        ("border", t.border),
        ("text", t.text),
        ("text_muted", t.text_muted),
        ("primary", t.primary),
        ("danger", t.danger),
        ("warning", t.warning),
        ("success", t.success),
    ];
    egui::Grid::new("tokens").num_columns(2).spacing([24.0, 6.0]).show(ui, |ui| {
        for (name, color) in tokens {
            let (rect, _) = ui.allocate_exact_size(egui::vec2(120.0, 18.0), egui::Sense::hover());
            ui.painter().rect_filled(rect, t.radius, *color);
            ui.painter().rect_stroke(rect, t.radius, egui::Stroke::new(1.0, t.border), egui::StrokeKind::Inside);
            ui.monospace(*name);
            ui.end_row();
        }
    });

    ui.add_space(14.0);
    ui.heading("API preview");
    ui.horizontal(|ui| {
        ui.kit_button("Primary");
        ui.button_v("Default", Variant::Default);
        ui.button_v("Outline", Variant::Outline);
        ui.button_v("Ghost", Variant::Ghost);
        ui.button_v("Danger", Variant::Danger);
        ui.badge("new");
        ui.badge("12");
    });
}
