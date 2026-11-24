# egui-kit

gpui-kit.com's component vocabulary for [egui](https://github.com/emilk/egui) 0.37 — one trait, zero config.

```rust
use egui_kit::{KitUi, Theme, Variant};

fn main() -> eframe::Result<()> {
    eframe::run_native("app", Default::default(), Box::new(|cc| {
        Theme::dark().apply(&cc.egui_ctx);
        Ok(Box::new(App))
    }))
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _: &mut eframe::Frame) {
        ui.kit_button("Save");
        ui.badge("new");
        ui.tag("design");
        ui.kbd("Ctrl+K");
        ui.divider();
        ui.progress(0.4);
        ui.avatar("Ada Lovelace", 34.0);
        ui.alert("Saved to the project.", Variant::Primary);
        ui.skeleton(160.0, 14.0);
        ui.rating(&mut self.rating, 5);
        ui.stepper(self.step, 4);
        ui.pagination(&mut self.page, 10);
        ui.tabs(&["Overview", "Activity"], &mut self.tab);
        ui.accordion("Advanced options", &mut self.open, |ui| { ui.label("..."); });
    }
}
```

Everything is an extension-trait method on `&mut Ui`: import `KitUi` and the widgets are there. No builders, no config structs, no new dependencies — just `egui`.

## Theme

`Theme::dark()` / `Theme::light()` carry the semantic tokens (bg, panel, surface, border, text tiers,
primary/danger/warning/success, radius) and `apply(ctx)` writes them into egui's `Visuals` for the
matching light/dark `egui::Theme`, plus spacing and text sizes. `Theme::get(ctx)` returns the active
theme for widget code.

## Gallery

`cargo run -p gallery` — every widget on one page, light/dark toggle in the top bar.
`GALLERY_PAGE=widgets` opens straight to the widgets page (used for automated screenshots).

## Status

Widgets land in batches; the crate is versioned 0.1 and the API is not frozen.

Design language credited to [gpui-kit](https://gpui-kit.com) (Longbridge). Code: MIT OR Apache-2.0.
