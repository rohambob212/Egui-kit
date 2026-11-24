use eframe::egui;
use egui_kit::{KitUi, Theme, Variant};

fn main() -> eframe::Result<()> {
    let height: f32 = std::env::var("GALLERY_H").ok().and_then(|h| h.parse().ok()).unwrap_or(760.0);
    let opts = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1100.0, height])
            .with_title("egui-kit gallery"),
        ..Default::default()
    };
    eframe::run_native(
        "egui-kit gallery",
        opts,
        Box::new(|cc| {
            Theme::dark().apply(&cc.egui_ctx);
            Ok(Box::new(App {
                dark: std::env::var("GALLERY_LIGHT").is_err(),
                page: if std::env::var("GALLERY_PAGE").as_deref() == Ok("widgets") {
                    Page::Widgets
                } else {
                    Page::Theme
                },
                rating: 3,
                step: 1,
                page_num: 0,
                tab: 0,
                acc_open: false,
                otp: String::new(),
                hh: 14,
                mm: 30,
                choice: 0,
                query: String::new(),
                checked: true,
                ratio: 0,
                notify: false,
                level: 40.0,
                name: String::new(),
                notes: String::new(),
                qty: 3.0,
                list_sel: 0,
                removed: false,
                year: 2026,
                month: 11,
                day: None,
                tree_sel: None,
            }))
        }),
    )
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Page {
    Theme,
    Widgets,
}

struct App {
    page: Page,
    dark: bool,
    rating: u8,
    step: usize,
    page_num: usize,
    tab: usize,
    acc_open: bool,
    otp: String,
    hh: u8,
    mm: u8,
    choice: usize,
    query: String,
    checked: bool,
    ratio: usize,
    notify: bool,
    level: f32,
    name: String,
    notes: String,
    qty: f64,
    list_sel: usize,
    removed: bool,
    year: i32,
    month: u32,
    day: Option<u32>,
    tree_sel: Option<Vec<usize>>,
}

const PAGES: &[(Page, &str)] = &[(Page::Theme, "Theme"), (Page::Widgets, "Widgets")];

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
            let page = self.page;
            egui::ScrollArea::vertical().show(ui, |ui| match page {
                Page::Theme => theme_page(ui),
                Page::Widgets => widgets_page(ui, self),
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
        ui.tag("design");
        ui.kbd("Ctrl+K");
    });
    ui.add_space(10.0);
    ui.divider();
    ui.add_space(10.0);
    ui.progress(0.4);
    ui.add_space(12.0);

    ui.heading("Feedback");
    ui.horizontal(|ui| {
        ui.avatar("Ada Lovelace", 34.0);
        ui.avatar("Grace Hopper", 34.0);
        ui.spinner();
        ui.skeleton(160.0, 14.0);
    });
    ui.add_space(6.0);
    ui.alert("Saved to the project.", Variant::Primary);
    ui.add_space(4.0);
    ui.alert("Could not reach the server.", Variant::Danger);
}

fn widgets_page(ui: &mut egui::Ui, app: &mut App) {
    let t = Theme::get(ui.ctx());
    ui.heading("Widgets");
    ui.label(egui::RichText::new("Every widget is one call on &mut Ui.").color(t.text_muted));
    ui.add_space(12.0);

    ui.label("Rating");
    ui.rating(&mut app.rating, 5);
    ui.add_space(12.0);

    ui.label("Stepper");
    if ui.stepper(app.step, 4).clicked() {
        app.step = (app.step + 1) % 4;
    }
    ui.add_space(12.0);

    ui.label("Pagination");
    ui.pagination(&mut app.page_num, 10);
    ui.add_space(12.0);

    ui.label("Tabs");
    ui.tabs(&["Overview", "Activity", "Settings"], &mut app.tab);
    ui.add_space(8.0);
    ui.label(format!("{} selected", ["Overview", "Activity", "Settings"][app.tab.min(2)]));
    ui.add_space(12.0);

    ui.accordion("Advanced options", &mut app.acc_open, |ui| {
        ui.label("Hidden until the header is clicked.");
    });
    ui.add_space(12.0);

    ui.label("One-time code (type digits after clicking)");
    ui.otp_input(&mut app.otp, 6);
    ui.add_space(12.0);

    ui.label("Time");
    ui.time_field(&mut app.hh, &mut app.mm);
    ui.label(format!("{:02}:{:02}", app.hh, app.mm));
    ui.add_space(12.0);

    ui.label("Select");
    ui.select(&["Apple", "Banana", "Cherry"], &mut app.choice);
    ui.add_space(12.0);

    ui.label("Combobox (type inside the popup to filter)");
    let fruits = ["Apple", "Apricot", "Banana", "Blueberry", "Cherry", "Grape", "Kiwi", "Mango", "Peach", "Plum"];
    ui.combobox(&fruits, &mut app.query, &mut app.choice);
    ui.add_space(12.0);

    ui.label("Toggles");
    ui.horizontal(|ui| {
        ui.checkbox(&mut app.checked, "Enabled"); // egui native, themed by our Visuals
        ui.radio_value(&mut app.ratio, 0, "Low");
        ui.radio_value(&mut app.ratio, 1, "Medium");
        ui.radio_value(&mut app.ratio, 2, "High");
        ui.switch("notify", &mut app.notify);
        if ui.menu("Menu", &["Rename", "Duplicate", "Delete"]).is_some_and(|i| i == 2) {
            app.notes.clear();
        }
        ui.tooltip("Hover text works", |ui| ui.button("Hover me"));
    });
    ui.add_space(12.0);

    ui.label("Inputs");
    ui.text_input(&mut app.name, "Name");
    ui.add_space(4.0);
    ui.textarea(&mut app.notes, 3);
    ui.add_space(4.0);
    ui.horizontal(|ui| {
        ui.number_input(&mut app.qty, 0.0..=99.0);
        ui.slider(&mut app.level, 0.0..=100.0);
    });
    ui.add_space(12.0);

    ui.group_box("Details", |ui| {
        ui.description_list(&[("Plan", "Infinite"), ("Seats", "3"), ("Region", "eu-west")]);
    });
    ui.add_space(12.0);

    ui.label("List");
    ui.scrollable(110.0, |ui| {
        for (i, name) in ["alpha", "beta", "gamma", "delta", "epsilon", "zeta", "eta"].iter().enumerate() {
            if ui.list_item(app.list_sel == i, name) {
                app.list_sel = i;
            }
        }
    });
    ui.add_space(12.0);

    ui.label("Form");
    ui.form_row("Full name", |ui| {
        ui.text_input(&mut app.name, "Ada Lovelace");
    });
    ui.form_row("Notes", |ui| {
        ui.textarea(&mut app.notes, 2);
    });

    ui.add_space(4.0);
    ui.horizontal(|ui| {
        ui.hover_card(|ui| { ui.label("A rich hover card"); }, |ui| ui.button("Hover card"));
        if !app.removed {
            ui.attachment("report.pdf", &mut app.removed);
        } else if ui.button("restore attachment").clicked() {
            app.removed = false;
        }
    });
    ui.add_space(12.0);

    ui.label("Date picker");
    ui.date_picker("due", &mut app.year, &mut app.month, &mut app.day);
    ui.add_space(12.0);

    ui.label("Calendar");
    ui.calendar(&mut app.year, &mut app.month, &mut app.day);
    ui.add_space(12.0);

    ui.label("Tree");
    let nodes = [
        egui_kit::basic::Node::branch("src", vec![
            egui_kit::basic::Node::leaf("main.rs"),
            egui_kit::basic::Node::branch("ui", vec![
                egui_kit::basic::Node::leaf("button.rs"),
                egui_kit::basic::Node::leaf("theme.rs"),
            ]),
        ]),
        egui_kit::basic::Node::leaf("Cargo.toml"),
    ];
    ui.tree("files", &nodes, &mut app.tree_sel);
    ui.add_space(12.0);

    ui.label("Diff");
    ui.diff(r#"@@ -1,4 +1,5 @@
 fn main() {
-    let x = 1;
+    let x = 2;
+    println!("{x}");
 }
"#);
    ui.add_space(12.0);
    ui.empty("Nothing here yet", "Create your first item");
}
