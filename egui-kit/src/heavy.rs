//! Heavy widgets backed by external crates, each behind its own feature:
//! `dock` (egui_dock), `editor` (egui_code_editor), plus `chart`/`table` in `data`.
use crate::Theme;
use egui::{Response, Ui};

/// Dockable workspace: caller owns the `DockState<String>` (tabs are their titles).
/// `body` renders each tab's contents. The dock fills a bounded box of `height`,
/// so widgets after it still get laid out normally.
#[cfg(feature = "dock")]
pub fn dock(
    ui: &mut Ui,
    id: &str,
    height: f32,
    state: &mut egui_dock::DockState<String>,
    body: &mut dyn FnMut(&mut Ui, &mut String),
) -> Response {
    let _ = Theme::get(ui.ctx());
    let mut viewer = KitTabs { body };
    ui.allocate_ui(egui::vec2(ui.available_width(), height), |ui| {
        egui_dock::DockArea::new(state)
            .id(egui::Id::unique(id))
            .style(egui_dock::Style::from_egui(ui.style()))
            .show_inside(ui, &mut viewer);
    })
    .response
}

/// Tab renderer handed to `egui_dock`; the caller's closure draws the content.
#[cfg(feature = "dock")]
struct KitTabs<'a> {
    body: &'a mut dyn FnMut(&mut Ui, &mut String),
}

#[cfg(feature = "dock")]
impl egui_dock::TabViewer for KitTabs<'_> {
    type Tab = String;

    fn id(&mut self, tab: &mut Self::Tab) -> egui::Id {
        egui::Id::unique(tab.as_str())
    }

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        tab.as_str().into()
    }

    fn ui(&mut self, ui: &mut Ui, tab: &mut Self::Tab) {
        (self.body)(ui, tab);
    }
}

/// Source editor with line numbers and highlighting, themed dark.
#[cfg(feature = "editor")]
pub fn editor(ui: &mut Ui, code: &mut String, rows: usize) -> Response {
    use egui_code_editor::{CodeEditor, Color32Theme, ColorTheme, Syntax};
    let theme: Color32Theme = ColorTheme::GITHUB_DARK.into();
    let mut editor = CodeEditor::default()
        .with_rows(rows)
        .with_theme(&theme)
        .with_numlines(true);
    let syntax = Syntax::simple("//");
    editor.show(ui, code, &syntax).0.response.response
}
