//! Heavy widgets that need a dependency, each behind its own feature.
//! The core crate stays dependency-free: enable `chart` / `table` to get these.
#[cfg(any(feature = "chart", feature = "table"))]
use crate::Theme;
#[cfg(any(feature = "chart", feature = "table"))]
use egui::{Response, Ui};

/// Line chart of one series: `data::line_chart(ui, "cpu", &series)`.
#[cfg(feature = "chart")]
pub fn line_chart(ui: &mut Ui, id: &str, values: &[f32]) -> Response {
    use egui_plot::{Line, Plot, PlotPoints};
    let theme = Theme::get(ui.ctx());
    let points: PlotPoints = values
        .iter()
        .enumerate()
        .map(|(i, v)| [i as f64, *v as f64])
        .collect();
    Plot::new(id)
        .height(180.0)
        .allow_drag(false)
        .allow_scroll(false)
        .allow_zoom(false)
        .show(ui, |plot_ui| {
            plot_ui.line(Line::new("series", points).color(theme.primary).width(2.0));
        })
        .response
}

/// Two-series area/line chart, for the "area" variant of the set.
#[cfg(feature = "chart")]
pub fn area_chart(ui: &mut Ui, id: &str, values: &[f32]) -> Response {
    use egui_plot::{Line, Plot, PlotPoints};
    let theme = Theme::get(ui.ctx());
    let points: PlotPoints = values
        .iter()
        .enumerate()
        .map(|(i, v)| [i as f64, *v as f64])
        .collect();
    Plot::new(id)
        .height(180.0)
        .allow_drag(false)
        .allow_scroll(false)
        .allow_zoom(false)
        .show(ui, |plot_ui| {
            plot_ui.line(
                Line::new("series", points)
                    .color(theme.primary.gamma_multiply(0.25))
                    .fill(0.25),
            );
        })
        .response
}

/// Bar chart: `data::bar_chart(ui, "sales", &[4.0, 8.0, 3.0])`.
#[cfg(feature = "chart")]
pub fn bar_chart(ui: &mut Ui, id: &str, values: &[f32]) -> Response {
    use egui_plot::{Bar, BarChart, Plot};
    let theme = Theme::get(ui.ctx());
    let bars: Vec<Bar> = values
        .iter()
        .enumerate()
        .map(|(i, v)| Bar::new(i as f64, *v as f64).fill(theme.primary))
        .collect();
    Plot::new(id)
        .height(180.0)
        .allow_drag(false)
        .allow_scroll(false)
        .allow_zoom(false)
        .show(ui, |plot_ui| {
            plot_ui.bar_chart(BarChart::new("bars", bars));
        })
        .response
}

/// Striped, sortable-looking data table: `data::table(ui, id, &["Name", "Age"], &rows)`.
#[cfg(feature = "table")]
pub fn table(ui: &mut Ui, id: &str, headers: &[&str], rows: &[Vec<String>]) -> Response {
    use egui_extras::{Column, TableBuilder};
    let theme = Theme::get(ui.ctx());
    let before = ui.available_rect_before_wrap();
    TableBuilder::new(ui)
        .id_salt(id)
        .striped(true)
        .column(Column::auto().at_least(120.0))
        .columns(Column::auto().at_least(90.0), headers.len().saturating_sub(1))
        .header(24.0, |mut header| {
            for h in headers {
                header.col(|ui| {
                    ui.label(egui::RichText::new(*h).strong().color(theme.text_muted));
                });
            }
        })
        .body(|mut body| {
            for row in rows {
                body.row(22.0, |mut ui_row| {
                    for cell in row {
                        ui_row.col(|ui| {
                            ui.label(cell);
                        });
                    }
                });
            }
        });
    ui.interact(before.intersect(ui.min_rect()), egui::Id::unique(id).with("table"), egui::Sense::hover())
}