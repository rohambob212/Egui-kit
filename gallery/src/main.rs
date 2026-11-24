use eframe::egui;
use egui_kit::{Icon, KitUi, Question, Side, Theme, Toasts, Variant};

/// Reads a process env var on native, returns the default on wasm.
fn env_or(key: &str, default: &str) -> String {
    #[cfg(not(target_arch = "wasm32"))]
    {
        std::env::var(key).unwrap_or_else(|_| default.to_owned())
    }
    #[cfg(target_arch = "wasm32")]
    {
        let _ = key;
        default.to_owned()
    }
}

impl App {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Theme::dark().apply(&cc.egui_ctx);
        App {
            dark: env_or("GALLERY_LIGHT", "").is_empty(),
            page: match env_or("GALLERY_PAGE", "").as_str() {
                "widgets" => Page::Widgets,
                "misc" => Page::Misc,
                "data" => Page::Data,
                "apps" => Page::Apps,
                "overlays" => Page::Overlays,
                _ => Page::Theme,
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
            pal_open: env_or("GALLERY_OPEN", "") == "palette",
            pal_q: String::new(),
            last_cmd: None,
            dlg_open: env_or("GALLERY_OPEN", "") == "dialog",
            pop_open: false,
            sheet_open: env_or("GALLERY_OPEN", "") == "sheet",
            slide: 0,
            toasts: Toasts::default(),
            misc_on: true,
            answers: Vec::new(),
            code: String::from("fn main() {\n    // edit me\n    println!(\"hello\");\n}\n"),
            dock_state: {
                let mut tabs = Vec::new();
                tabs.push(String::from("Files"));
                tabs.push(String::from("Editor"));
                tabs.push(String::from("Terminal"));
                egui_kit::DockState::new(tabs)
            },
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
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
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Page {
    Theme,
    Widgets,
    Misc,
    Data,
    Apps,
    Overlays,
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
    pal_open: bool,
    pal_q: String,
    last_cmd: Option<usize>,
    dlg_open: bool,
    pop_open: bool,
    sheet_open: bool,
    slide: usize,
    toasts: Toasts,
    misc_on: bool,
    answers: Vec<Option<usize>>,
    code: String,
    dock_state: egui_kit::DockState<String>,
}

const PAGES: &[(Page, &str)] = &[
    (Page::Theme, "Theme"),
    (Page::Widgets, "Widgets"),
    (Page::Misc, "Misc"),
    (Page::Data, "Data"),
    (Page::Apps, "Apps"),
    (Page::Overlays, "Overlays"),
];

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        if self.dark {
            Theme::dark().apply(&ctx);
        } else {
            Theme::light().apply(&ctx);
        }

        ui.toolbar("top", |ui| {
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

        ui.status_bar("status", "egui-kit 0.1", "ready");

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
                Page::Misc => misc_page(ui, self),
                Page::Data => data_page(ui, self),
                Page::Apps => apps_page(ui, self),
                Page::Overlays => overlays_page(ui, self),
            });
        });

        let mut toasts = std::mem::take(&mut self.toasts);
        toasts.show(ui);
        self.toasts = toasts;
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
    ui.add_space(12.0);

    ui.label("Chat");
    ui.marker("Today");
    ui.message("Grace Hopper", "Found the bug — it was an off-by-one.", Side::Left, "09:12");
    ui.add_space(4.0);
    ui.message("Ada Lovelace", "Nice. Ship it.", Side::Right, "09:13");
    ui.add_space(4.0);
    ui.marker("Yesterday");
    ui.bubble("Standup moved to 10:00.", Side::Left, Some("16:40"));
}


fn misc_page(ui: &mut egui::Ui, app: &mut App) {
    let t = Theme::get(ui.ctx());
    ui.heading("Misc");
    ui.label(egui::RichText::new("Text scale, input group, grid, virtual list, toggle, copy, image.").color(t.text_muted));
    ui.add_space(12.0);

    ui.label("Text scale");
    ui.text("Heading text", egui_kit::LabelKind::Heading);
    ui.text("Body text", egui_kit::LabelKind::Body);
    ui.text("Muted text", egui_kit::LabelKind::Muted);
    ui.text("Small text", egui_kit::LabelKind::Small);
    ui.add_space(12.0);

    ui.label("Input group");
    ui.input_group(&mut app.name, "https://", "@example.com");
    ui.add_space(12.0);

    ui.label("Grid layout");
    let rows: Vec<Vec<String>> = vec![
        vec!["Region".into(), "eu-west".into(), "12 ms".into()],
        vec!["Plan".into(), "Infinite".into(), "active".into()],
        vec!["Seats".into(), "3".into(), "2 used".into()],
    ];
    ui.grid_layout("specs", &rows);
    ui.add_space(12.0);

    ui.label("Virtual list (2000 rows, only visible ones render)");
    ui.virtual_list(120.0, 2000, |ui, i| {
        ui.label(format!("row {i}"));
    });
    ui.add_space(12.0);

    ui.horizontal(|ui| {
        ui.toggle("opt", &mut app.misc_on);
        ui.label("toggle");
        ui.copy_button("copy-demo", "cargo add egui-kit");
    });
    ui.add_space(12.0);

    ui.label("Icons");
    ui.horizontal(|ui| {
        let t = Theme::get(ui.ctx());
        for kind in [
            Icon::Check,
            Icon::Cross,
            Icon::Plus,
            Icon::Minus,
            Icon::ChevronRight,
            Icon::ChevronDown,
            Icon::Search,
            Icon::Dot,
        ] {
            ui.icon(kind, 20.0, t.text);
        }
    });
    ui.add_space(12.0);

    ui.label("Questionnaire");
    let questions = [
        Question { prompt: "Which platform first?", options: vec!["Desktop", "Web", "Mobile"] },
        Question { prompt: "How do you ship it?", options: vec!["CI", "Locally", "Both"] },
        Question { prompt: "Ship the wasm site?", options: vec!["Yes", "Not yet"] },
    ];
    ui.questionnaire("onboarding", &questions, &mut app.answers);
    ui.add_space(8.0);
    ui.text(
        &format!("answers so far: {:?}", app.answers),
        egui_kit::LabelKind::Small,
    );
    ui.add_space(12.0);

    ui.label("Image (placeholder when the file is missing)");
    ui.kit_image("missing-image.png", 160.0);
}

fn overlays_page(ui: &mut egui::Ui, app: &mut App) {
    let t = Theme::get(ui.ctx());
    ui.heading("Overlays");
    ui.label(egui::RichText::new("Palette, dialog, popover, sheet, carousel.").color(t.text_muted));
    ui.add_space(12.0);

    ui.horizontal(|ui| {
        if ui.kit_button("Command palette").clicked() {
            app.pal_open = true;
        }
        if ui.button("Toast").clicked() {
            let now = ui.input(|i| i.time);
            app.toasts.push("Saved to the project.", Variant::Primary, now, 2.5);
        }
        if ui.button("Dialog").clicked() {
            app.dlg_open = true;
        }
        if ui.button("Sheet").clicked() {
            app.sheet_open = true;
        }
        let pop = ui.button("Popover");
        if pop.clicked() {
            app.pop_open = !app.pop_open;
        }
        ui.popover("demo-pop", &pop, &mut app.pop_open, |ui| {
            ui.label("Floating card content");
        });
    });
    ui.add_space(10.0);
    ui.label(match app.last_cmd {
        Some(i) => format!("last command: {}", ["New file", "Open settings", "Toggle theme"][i.min(2)]),
        None => "no command picked yet".to_owned(),
    });
    ui.add_space(14.0);

    ui.horizontal(|ui| {
        ui.link("Documentation");
        ui.dropdown_button("dd", "Deploy", &["Preview", "Production"]).1.map(|i| app.last_cmd = Some(i));
    });
    ui.add_space(10.0);

    ui.label("Carousel");
    ui.carousel(&mut app.slide, 5);
    ui.add_space(14.0);

    ui.palette("demo-palette", &mut app.pal_open, &mut app.pal_q, &["New file", "Open settings", "Toggle theme"], )
        .map(|i| app.last_cmd = Some(i));

    ui.dialog("demo-dialog", &mut app.dlg_open, "Delete project?", |ui| {
        ui.label("This cannot be undone.");
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            if ui.kit_button("Delete").clicked() {
                ui.close();
            }
            if ui.button("Cancel").clicked() {
                ui.close();
            }
        });
    });

    ui.sheet("demo-sheet", &mut app.sheet_open, |ui| {
        ui.label("Slide-in panel content.");
        ui.add_space(6.0);
        ui.kbd("Esc");
    });
}

fn data_page(ui: &mut egui::Ui, app: &mut App) {
    let t = Theme::get(ui.ctx());
    ui.heading("Data");
    ui.label(egui::RichText::new("Feature-gated widgets: chart (egui_plot), table (egui_extras).").color(t.text_muted));
    ui.add_space(12.0);

    let series: Vec<f32> = (0..40).map(|i| (i as f32 * 0.3).sin() * 10.0 + 20.0).collect();
    ui.label("Line chart");
    egui_kit::data::line_chart(ui, "cpu", &series);
    ui.add_space(12.0);

    ui.label("Area chart");
    egui_kit::data::area_chart(ui, "mem", &series);
    ui.add_space(12.0);

    ui.label("Bar chart");
    egui_kit::data::bar_chart(ui, "sales", &[4.0, 8.0, 3.0, 6.5, 9.0, 2.0]);
    ui.add_space(12.0);

    ui.label("Table");
    let rows: Vec<Vec<String>> = (0..12)
        .map(|i| vec![format!("user-{i:02}"), format!("{}", 20 + i), format!("user{i}@example.com")])
        .collect();
    egui_kit::data::table(ui, "users", &["Name", "Age", "Email"], &rows);
    let _ = app;
}

#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::JsCast as _;
    console_error_panic_hook::set_once();
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("egui_canvas").expect("canvas #egui_canvas missing");
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into().expect("#egui_canvas is not a canvas");
    wasm_bindgen_futures::spawn_local(async move {
        eframe::WebRunner::new()
            .start(
                canvas,
                eframe::WebOptions::default(),
                Box::new(|cc| Ok(Box::new(App::new(cc)))),
            )
            .await
            .expect("failed to start the wasm app");
    });
}

fn apps_page(ui: &mut egui::Ui, app: &mut App) {
    let t = Theme::get(ui.ctx());
    ui.heading("Apps");
    ui.label(egui::RichText::new("App-level components: dock, editor, resizable split, settings.").color(t.text_muted));
    ui.add_space(12.0);

    ui.label("Dock (drag tabs, split panes)");
    egui_kit::heavy::dock(ui, "dock", 240.0, &mut app.dock_state, &mut |ui, tab| {
        ui.label(format!("contents of {tab}"));
        ui.add_space(4.0);
        ui.kbd("drag the tab");
    });
    ui.add_space(16.0);

    ui.label("Editor");
    egui_kit::heavy::editor(ui, &mut app.code, 8);
    ui.add_space(16.0);

    ui.label("Resizable split");
    ui.resizable("split", 220.0, |ui| {
        ui.group_box("Left pane", |ui| {
            ui.label("drag the handle to resize");
        });
    });
    ui.add_space(16.0);

    ui.label("Settings");
    ui.settings("Preferences", |ui| {
        ui.checkbox(&mut app.checked, "Enable notifications");
        ui.radio_value(&mut app.ratio, 0, "Compact");
        ui.radio_value(&mut app.ratio, 1, "Comfortable");
    });
    ui.add_space(16.0);

    ui.label("Message scroller (sticks to newest)");
    let lines = ["hey", "how's the port going?", "58 widgets so far", "nice", "shipping the wasm build"];
    let n = lines.len();
    ui.message_scroller(160.0, n, |ui, i| {
        ui.message(if i % 2 == 0 { "Ada" } else { "Grace" }, lines[i], if i % 2 == 0 { Side::Left } else { Side::Right }, "12:0");
    });
}
