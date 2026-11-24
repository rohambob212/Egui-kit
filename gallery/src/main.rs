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
        let page_str = page_from_url().unwrap_or_else(|| env_or("GALLERY_PAGE", ""));
        App {
            dark: env_or("GALLERY_LIGHT", "").is_empty(),
            page: match page_str.as_str() {
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
            shown: Page::Theme,
            code: String::from("fn main() {\n    // edit me\n    println!(\"hello\");\n}"),
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
    shown: Page,
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

/// Every page is a centered column of this width, so lines stay readable.
const MAX_W: f32 = 780.0;

fn content<R>(ui: &mut egui::Ui, add: impl FnOnce(&mut egui::Ui) -> R) -> R {
    let avail = ui.available_rect_before_wrap();
    let w = avail.width().min(MAX_W);
    let rect = egui::Rect::from_min_size(
        egui::pos2(avail.left() + (avail.width() - w) * 0.5, avail.top()),
        egui::vec2(w, avail.height()),
    );
    ui.scope_builder(egui::UiBuilder::new().max_rect(rect), |ui| {
        ui.set_width(w);
        add(ui)
    })
    .inner
}

/// Page section: title, optional caption, then the demo in a card.
fn section(ui: &mut egui::Ui, title: &str, caption: &str, body: impl FnOnce(&mut egui::Ui)) {
    let t = Theme::get(ui.ctx());
    ui.add_space(18.0);
    ui.label(egui::RichText::new(title).size(15.0).strong().color(t.text));
    if !caption.is_empty() {
        ui.label(egui::RichText::new(caption).size(12.0).color(t.text_muted));
    }
    ui.add_space(8.0);
    egui::Frame::new()
        .fill(t.surface)
        .stroke(egui::Stroke::new(1.0, t.border))
        .corner_radius(egui::CornerRadius::same(t.radius as u8))
        .inner_margin(14.0)
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            body(ui);
        });
}

/// Page header: big title + dim subtitle.
fn header(ui: &mut egui::Ui, title: &str, subtitle: &str) {
    let t = Theme::get(ui.ctx());
    ui.add_space(20.0);
    ui.label(egui::RichText::new(title).size(24.0).strong().color(t.text));
    ui.label(egui::RichText::new(subtitle).size(13.0).color(t.text_muted));
}

/// WCAG contrast ratio between two colours.
fn contrast(a: egui::Color32, b: egui::Color32) -> f32 {
    fn lum(c: egui::Color32) -> f32 {
        let f = |v: u8| {
            let s = v as f32 / 255.0;
            if s <= 0.03928 {
                s / 12.92
            } else {
                ((s + 0.055) / 1.055).powf(2.4)
            }
        };
        0.2126 * f(c.r()) + 0.7152 * f(c.g()) + 0.0722 * f(c.b())
    }
    let (x, y) = (lum(a), lum(b));
    let (hi, lo) = if x > y { (x, y) } else { (y, x) };
    (hi + 0.05) / (lo + 0.05)
}

/// `?page=apps` on the web, so every page is linkable.
#[cfg(target_arch = "wasm32")]
fn page_from_url() -> Option<String> {
    let search = web_sys::window()?.location().search().ok()?;
    search
        .trim_start_matches('?')
        .split('&')
        .find_map(|kv| kv.strip_prefix("page=").map(|v| v.to_owned()))
}

#[cfg(not(target_arch = "wasm32"))]
fn page_from_url() -> Option<String> {
    None
}

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
                ui.label(
                    egui::RichText::new("gpui-kit components for egui")
                        .size(13.0)
                        .color(t.text_muted),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui
                        .kit_button(if self.dark { "Light" } else { "Dark" })
                        .clicked()
                    {
                        self.dark = !self.dark;
                    }
                });
            });
        });

        ui.status_bar("status", "ready", "local build");

        egui::Panel::left("nav").resizable(false).exact_size(190.0).show(ui, |ui| {
            ui.add_space(10.0);
            let width = ui.available_width() - 16.0;
            for (page, name) in PAGES {
                let selected = self.page == *page;
                if ui
                    .add_sized(
                        [width, 28.0],
                        egui::Button::selectable(selected, egui::RichText::new(*name).size(14.0)),
                    )
                    .clicked()
                {
                    self.page = *page;
                }
                ui.add_space(2.0);
            }
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                let t = Theme::get(ui.ctx());
                ui.add_space(10.0);
                ui.label(
                    egui::RichText::new("0.1.0 · 65 widgets")
                        .size(11.0)
                        .color(t.text_muted),
                );
            });
        });

        egui::CentralPanel::default_margins().show(ui, |ui| {
            let page = self.page;
            // page changes fade in instead of snapping
            let now = ui.input(|i| i.time);
            let fade = egui::Id::unique("page_fade");
            if self.shown != page {
                self.shown = page;
                ui.ctx().data_mut(|d| d.insert_temp(fade, now));
            }
            let start: f64 = ui.ctx().data(|d| d.get_temp(fade)).unwrap_or(0.0);
            let t = (((now - start) / 0.20) as f32).clamp(0.0, 1.0);
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.multiply_opacity(0.4 + 0.6 * t);
                ui.add_space((1.0 - t) * 6.0);
                match page {
                    Page::Theme => content(ui, |ui| theme_page(ui)),
                    Page::Widgets => content(ui, |ui| widgets_page(ui, self)),
                    Page::Misc => content(ui, |ui| misc_page(ui, self)),
                    Page::Data => content(ui, |ui| data_page(ui, self)),
                    Page::Apps => content(ui, |ui| apps_page(ui, self)),
                    Page::Overlays => content(ui, |ui| overlays_page(ui, self)),
                }
                ui.add_space(40.0);
            });
        });

        let mut toasts = std::mem::take(&mut self.toasts);
        toasts.show(ui);
        self.toasts = toasts;
    }
}

fn theme_page(ui: &mut egui::Ui) {
    let t = Theme::get(ui.ctx());
    header(
        ui,
        "Theme",
        "Semantic tokens. Theme::apply(ctx) once and every widget follows.",
    );

    section(ui, "Neutral", "Four background steps and a border.", |ui| {
        egui::Grid::new("tokens_neutral")
            .num_columns(4)
            .spacing([18.0, 8.0])
            .show(ui, |ui| {
                for (name, color) in [
                    ("bg", t.bg),
                    ("panel", t.panel),
                    ("surface", t.surface),
                    ("surface_hover", t.surface_hover),
                    ("border", t.border),
                ] {
                    let (rect, _) =
                        ui.allocate_exact_size(egui::vec2(46.0, 24.0), egui::Sense::hover());
                    ui.painter().rect_filled(rect, egui::CornerRadius::same(4), color);
                    ui.painter().rect_stroke(
                        rect,
                        egui::CornerRadius::same(4),
                        egui::Stroke::new(1.0, t.border),
                        egui::StrokeKind::Inside,
                    );
                    ui.label(egui::RichText::new(name).monospace().size(12.5).color(t.text));
                    ui.label(
                        egui::RichText::new(format!(
                            "#{:02x}{:02x}{:02x}",
                            color.r(),
                            color.g(),
                            color.b()
                        ))
                        .monospace()
                        .size(12.0)
                        .color(t.text_muted),
                    );
                    ui.label(
                        egui::RichText::new(format!("{:.1}:1", contrast(color, t.bg)))
                            .monospace()
                            .size(11.0)
                            .color(t.text_muted),
                    );
                    ui.end_row();
                }
            });
    });

    section(
        ui,
        "Text and accent",
        "Ratio measured against the page background.",
        |ui| {
            egui::Grid::new("tokens_semantic")
                .num_columns(4)
                .spacing([18.0, 8.0])
                .show(ui, |ui| {
                    for (name, color) in [
                        ("text", t.text),
                        ("text_muted", t.text_muted),
                        ("primary", t.primary),
                        ("danger", t.danger),
                        ("warning", t.warning),
                        ("success", t.success),
                    ] {
                        let (rect, _) =
                            ui.allocate_exact_size(egui::vec2(46.0, 24.0), egui::Sense::hover());
                        ui.painter().rect_filled(rect, egui::CornerRadius::same(4), color);
                        ui.label(egui::RichText::new(name).monospace().size(12.5).color(t.text));
                        ui.label(
                            egui::RichText::new(format!(
                                "#{:02x}{:02x}{:02x}",
                                color.r(),
                                color.g(),
                                color.b()
                            ))
                            .monospace()
                            .size(12.0)
                            .color(t.text_muted),
                        );
                        let ratio = contrast(color, t.bg);
                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new(format!("{ratio:.1}:1"))
                                    .monospace()
                                    .size(11.0)
                                    .color(t.text_muted),
                            );
                            if ratio >= 4.5 {
                                ui.tag("AA");
                            }
                        });
                        ui.end_row();
                    }
                });
        },
    );

    section(ui, "Buttons", "Five variants, one call each.", |ui| {
        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing = egui::vec2(8.0, 8.0);
            ui.kit_button("Primary");
            ui.button_v("Default", Variant::Default);
            ui.button_v("Outline", Variant::Outline);
            ui.button_v("Ghost", Variant::Ghost);
            ui.button_v("Danger", Variant::Danger);
        });
    });

    section(ui, "Badges, tags and keys", "Inline metadata.", |ui| {
        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing = egui::vec2(8.0, 6.0);
            ui.badge("new");
            ui.badge("12");
            ui.tag("design");
            ui.tag("egui 0.37");
            ui.kbd("Ctrl+K");
            ui.kbd("Esc");
        });
    });

    section(ui, "Feedback", "Progress, spinners, skeletons and alerts.", |ui| {
        ui.progress_labeled(0.42, "porting widgets");
        ui.add_space(12.0);
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 12.0;
            ui.kit_spinner(22.0);
            ui.avatar("Ada Lovelace", 30.0);
            ui.avatar("Grace Hopper", 30.0);
            ui.skeleton(140.0, 16.0);
            ui.skeleton(70.0, 16.0);
        });
        ui.add_space(14.0);
        ui.alert("Saved to the project.", Variant::Primary);
        ui.add_space(6.0);
        ui.alert("Could not reach the server. Retrying in 4s.", Variant::Danger);
        ui.add_space(6.0);
        ui.alert("Disk is 92% full.", Variant::Warning);
    });
}

fn widgets_page(ui: &mut egui::Ui, app: &mut App) {
    header(ui, "Widgets", "Every widget is one call on &mut Ui.");

    section(ui, "Rating", "Click a star to set the value.", |ui| {
        ui.rating(&mut app.rating, 5);
    });

    section(ui, "Stepper", "Done steps fill, the active step rings.", |ui| {
        if ui.stepper(app.step, 4).clicked() {
            app.step = (app.step + 1) % 4;
        }
    });

    section(ui, "Pagination", "Prev / numbers / Next, current page filled.", |ui| {
        ui.pagination(&mut app.page_num, 10);
    });

    section(ui, "Tabs", "The underline slides between tabs.", |ui| {
        ui.tabs(&["Overview", "Activity", "Settings"], &mut app.tab);
        ui.add_space(6.0);
        ui.label(format!(
            "{} selected",
            ["Overview", "Activity", "Settings"][app.tab.min(2)]
        ));
    });

    section(ui, "Accordion", "Fades and slides open.", |ui| {
        ui.accordion("Advanced options", &mut app.acc_open, |ui| {
            ui.label("Hidden until the header is clicked.");
        });
    });

    section(ui, "One-time code", "Six digit boxes, type straight through.", |ui| {
        ui.otp_input(&mut app.otp, 6);
    });

    section(ui, "Time", "Two drag-able fields, clamped.", |ui| {
        ui.time_field(&mut app.hh, &mut app.mm);
        ui.label(format!("{:02}:{:02}", app.hh, app.mm));
    });

    section(ui, "Select", "Plain dropdown.", |ui| {
        ui.select(&["Apple", "Banana", "Cherry"], &mut app.choice);
    });

    section(ui, "Combobox", "Filter as you type inside the popup.", |ui| {
        let fruits = [
            "Apple", "Apricot", "Banana", "Blueberry", "Cherry", "Grape", "Kiwi", "Mango", "Peach",
            "Plum",
        ];
        ui.combobox(&fruits, &mut app.query, &mut app.choice);
    });

    section(ui, "Toggles", "Native egui controls, restyled by the theme.", |ui| {
        ui.horizontal_wrapped(|ui| {
            ui.checkbox(&mut app.checked, "Enabled");
            ui.radio_value(&mut app.ratio, 0, "Low");
            ui.radio_value(&mut app.ratio, 1, "Medium");
            ui.radio_value(&mut app.ratio, 2, "High");
            ui.switch("notify", &mut app.notify);
            if ui.menu("Menu", &["Rename", "Duplicate", "Delete"]).is_some_and(|i| i == 2) {
                app.notes.clear();
            }
            ui.tooltip("Hover text works", |ui| ui.button("Hover me"));
        });
    });

    section(ui, "Inputs", "Text, textarea, number and slider.", |ui| {
        ui.text_input(&mut app.name, "Name");
        ui.add_space(4.0);
        ui.textarea(&mut app.notes, 3);
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            ui.number_input(&mut app.qty, 0.0..=99.0);
        });
        ui.add_space(6.0);
        ui.slider(&mut app.level, 0.0..=100.0);
    });

    section(ui, "Details", "Grouped key/value rows.", |ui| {
        ui.description_list(&[
            ("Plan", "Infinite"),
            ("Seats", "3"),
            ("Region", "eu-west"),
        ]);
    });

    section(ui, "List", "Scrollable, one selected row.", |ui| {
        ui.scrollable(110.0, |ui| {
            for (i, name) in ["alpha", "beta", "gamma", "delta", "epsilon", "zeta", "eta"]
                .iter()
                .enumerate()
            {
                if ui.list_item(app.list_sel == i, name) {
                    app.list_sel = i;
                }
            }
        });
    });

    section(ui, "Form", "Label on the left, control on the right.", |ui| {
        ui.form_row("Full name", |ui| {
            ui.text_input(&mut app.name, "Ada Lovelace");
        });
        ui.form_row("Notes", |ui| {
            ui.textarea(&mut app.notes, 2);
        });
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            ui.hover_card(
                |ui| {
                    ui.label("A rich hover card");
                },
                |ui| ui.button("Hover card"),
            );
            if !app.removed {
                ui.attachment("report.pdf", &mut app.removed);
            } else if ui.button("restore attachment").clicked() {
                app.removed = false;
            }
        });
    });

    section(ui, "Date picker", "Popover calendar over a typed date.", |ui| {
        ui.date_picker("due", &mut app.year, &mut app.month, &mut app.day);
    });

    section(ui, "Calendar", "Monday-first month grid.", |ui| {
        ui.calendar(&mut app.year, &mut app.month, &mut app.day);
    });

    section(ui, "Tree", "Expandable file tree, selection tracked by id.", |ui| {
        let nodes = [
            egui_kit::basic::Node::branch(
                "src",
                vec![
                    egui_kit::basic::Node::leaf("main.rs"),
                    egui_kit::basic::Node::branch(
                        "ui",
                        vec![
                            egui_kit::basic::Node::leaf("button.rs"),
                            egui_kit::basic::Node::leaf("theme.rs"),
                        ],
                    ),
                ],
            ),
            egui_kit::basic::Node::leaf("Cargo.toml"),
        ];
        ui.tree("files", &nodes, &mut app.tree_sel);
    });

    section(ui, "Diff", "Added, removed and context lines coloured.", |ui| {
        ui.diff(
            r#"@@ -1,4 +1,5 @@
 fn main() {
-    let x = 1;
+    let x = 2;
+    println!("{x}");
 }
"#,
        );
    });

    section(ui, "Empty state", "When a list has nothing to show.", |ui| {
        ui.empty("Nothing here yet", "Create your first item");
    });

    section(ui, "Chat", "Bubbles, day markers, both sides.", |ui| {
        ui.marker("Today");
        ui.message(
            "Grace Hopper",
            "Found the bug — it was an off-by-one.",
            Side::Left,
            "09:12",
        );
        ui.add_space(4.0);
        ui.message("Ada Lovelace", "Nice. Ship it.", Side::Right, "09:13");
        ui.add_space(4.0);
        ui.marker("Yesterday");
        ui.bubble("Standup moved to 10:00.", Side::Left, Some("16:40"));
    });
}

fn misc_page(ui: &mut egui::Ui, app: &mut App) {
    header(
        ui,
        "Misc",
        "Text scale, input group, grid, virtual list, toggle, copy, image.",
    );

    section(ui, "Text scale", "", |ui| {
        ui.text("Heading text", egui_kit::LabelKind::Heading);
        ui.text("Body text", egui_kit::LabelKind::Body);
        ui.text("Muted text", egui_kit::LabelKind::Muted);
        ui.text("Small text", egui_kit::LabelKind::Small);
    });

    section(ui, "Input group", "Prefix and suffix around an input.", |ui| {
        ui.input_group(&mut app.name, "https://", "@example.com");
    });

    section(ui, "Grid layout", "", |ui| {
        let rows: Vec<Vec<String>> = vec![
            vec!["Region".into(), "eu-west".into(), "12 ms".into()],
            vec!["Plan".into(), "Infinite".into(), "active".into()],
            vec!["Seats".into(), "3".into(), "2 used".into()],
        ];
        ui.grid_layout("specs", &rows);
    });

    section(
        ui,
        "Virtual list",
        "2000 rows, only the visible ones render.",
        |ui| {
            ui.virtual_list(120.0, 2000, |ui, i| {
                ui.label(format!("row {i}"));
            });
        },
    );

    section(ui, "Toggle and copy", "", |ui| {
        ui.horizontal(|ui| {
            ui.toggle("opt", &mut app.misc_on);
            ui.label("toggle");
            ui.copy_button("copy-demo", "cargo add egui-kit");
        });
    });

    section(ui, "Icons", "Drawn, not font glyphs.", |ui| {
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
    });

    section(ui, "Questionnaire", "One question at a time with Back/Next.", |ui| {
        let questions = [
            Question {
                prompt: "Which platform first?",
                options: vec!["Desktop", "Web", "Mobile"],
            },
            Question {
                prompt: "How do you ship it?",
                options: vec!["CI", "Locally", "Both"],
            },
            Question {
                prompt: "Ship the wasm site?",
                options: vec!["Yes", "Not yet"],
            },
        ];
        ui.questionnaire("onboarding", &questions, &mut app.answers);
        ui.add_space(8.0);
        ui.text(
            &format!("answers so far: {:?}", app.answers),
            egui_kit::LabelKind::Small,
        );
    });

    section(ui, "Image", "Placeholder when the file is missing.", |ui| {
        ui.kit_image("missing-image.png", 160.0);
    });
}

fn data_page(ui: &mut egui::Ui, app: &mut App) {
    header(
        ui,
        "Data",
        "Feature-gated: chart (egui_plot), table (egui_extras).",
    );

    section(ui, "Line chart", "", |ui| {
        let series: Vec<f32> = (0..40).map(|i| (i as f32 * 0.3).sin() * 10.0 + 20.0).collect();
        egui_kit::data::line_chart(ui, "cpu", &series);
    });

    section(ui, "Area chart", "", |ui| {
        let series: Vec<f32> = (0..40).map(|i| (i as f32 * 0.3).sin() * 10.0 + 20.0).collect();
        egui_kit::data::area_chart(ui, "mem", &series);
    });

    section(ui, "Bar chart", "", |ui| {
        egui_kit::data::bar_chart(ui, "sales", &[4.0, 8.0, 3.0, 6.5, 9.0, 2.0]);
    });

    section(ui, "Table", "", |ui| {
        let rows: Vec<Vec<String>> = (0..12)
            .map(|i| {
                vec![
                    format!("user-{i:02}"),
                    format!("{}", 20 + i),
                    format!("user{i}@example.com"),
                ]
            })
            .collect();
        egui_kit::data::table(ui, "users", &["Name", "Age", "Email"], &rows);
        let _ = app;
    });
}

fn apps_page(ui: &mut egui::Ui, app: &mut App) {
    header(
        ui,
        "Apps",
        "App-level components: dock, editor, resizable split, settings.",
    );

    section(ui, "Dock", "Drag tabs, split panes.", |ui| {
        egui_kit::heavy::dock(ui, "dock", 240.0, &mut app.dock_state, &mut |ui, tab| {
            ui.label(format!("contents of {tab}"));
            ui.add_space(4.0);
            ui.kbd("drag the tab");
        });
    });

    section(ui, "Editor", "Line numbers and highlighting.", |ui| {
        egui_kit::heavy::editor(ui, &mut app.code, 8);
    });

    section(ui, "Resizable split", "Drag the divider.", |ui| {
        ui.horizontal(|ui| {
            ui.resizable("split", 240.0, |ui| {
                ui.vertical(|ui| {
                    ui.strong("Left pane");
                    ui.label("drag the right edge to resize");
                });
            });
            ui.separator();
            ui.vertical(|ui| {
                ui.strong("Right pane");
                ui.label("takes the remaining width");
            });
        });
    });

    section(ui, "Settings", "Grouped preference rows.", |ui| {
        ui.settings("Preferences", |ui| {
            ui.checkbox(&mut app.checked, "Enable notifications");
            ui.radio_value(&mut app.ratio, 0, "Compact");
            ui.radio_value(&mut app.ratio, 1, "Comfortable");
        });
    });

    section(ui, "Message scroller", "Sticks to the newest row.", |ui| {
        let lines = [
            "hey",
            "how's the port going?",
            "65 widgets so far",
            "nice",
            "shipping the wasm build",
        ];
        let n = lines.len();
        ui.message_scroller(160.0, n, |ui, i| {
            ui.message(
                if i % 2 == 0 { "Ada" } else { "Grace" },
                lines[i],
                if i % 2 == 0 { Side::Left } else { Side::Right },
                "12:04",
            );
        });
    });
}

fn overlays_page(ui: &mut egui::Ui, app: &mut App) {
    header(ui, "Overlays", "Palette, dialog, popover, sheet, carousel.");

    section(ui, "Triggers", "Everything opens from here.", |ui| {
        ui.horizontal_wrapped(|ui| {
            if ui.kit_button("Command palette").clicked() {
                app.pal_open = true;
            }
            if ui.button("Toast").clicked() {
                let now = ui.input(|i| i.time);
                app.toasts
                    .push("Saved to the project.", Variant::Primary, now, 2.5);
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
        ui.add_space(8.0);
        ui.label(match app.last_cmd {
            Some(i) => format!(
                "last command: {}",
                ["New file", "Open settings", "Toggle theme"][i.min(2)]
            ),
            None => "no command picked yet".to_owned(),
        });
    });

    section(ui, "Links and dropdown", "", |ui| {
        ui.horizontal(|ui| {
            let _ = ui.link("Documentation");
            ui.dropdown_button("dd", "Deploy", &["Preview", "Production"])
                .1
                .map(|i| app.last_cmd = Some(i));
        });
    });

    section(ui, "Carousel", "", |ui| {
        ui.carousel(&mut app.slide, 5);
    });

    section(
        ui,
        "Palette, dialog and sheet",
        "Open one from the triggers above; they fade and slide in.",
        |ui| {
            ui.horizontal_wrapped(|ui| {
                if ui.button("Open palette").clicked() {
                    app.pal_open = true;
                }
                if ui.button("Open dialog").clicked() {
                    app.dlg_open = true;
                }
                if ui.button("Open sheet").clicked() {
                    app.sheet_open = true;
                }
            });
        },
    );

    // these render only while open, so they live outside the section card
    ui.palette(
        "demo-palette",
        &mut app.pal_open,
        &mut app.pal_q,
        &["New file", "Open settings", "Toggle theme"],
    )
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

#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::JsCast as _;
    console_error_panic_hook::set_once();
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document
        .get_element_by_id("egui_canvas")
        .expect("canvas #egui_canvas missing");
    let canvas: web_sys::HtmlCanvasElement =
        canvas.dyn_into().expect("#egui_canvas is not a canvas");
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
