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
        .include_y(0.0)
        .include_x(-1.0)
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
        .include_y(0.0)
        .include_x(-1.0)
        .show(ui, |plot_ui| {
            plot_ui.line(
                Line::new("series", points)
                    .color(theme.primary)
                    .width(1.5)
                    .fill(0.0) // fill down to the zero line
                    .fill_alpha(0.35),
            );
        })
        .response
}

/// Bar chart: `data::bar_chart(ui, "sales", &[4.0, 8.0, 3.0])`.
#[cfg(feature = "chart")]
pub fn bar_chart(ui: &mut Ui, id: &str, values: &[f32]) -> Response {
    use egui_plot::{Bar, BarChart, Plot};
    let theme = Theme::get(ui.ctx());
    let max = values.iter().cloned().fold(1.0_f32, f32::max);
    let bars: Vec<Bar> = values
        .iter()
        .enumerate()
        .map(|(i, v)| {
            Bar::new(i as f64, *v as f64)
                .width(0.62)
                .fill(theme.primary)
        })
        .collect();
    Plot::new(id)
        .height(180.0)
        .allow_drag(false)
        .allow_scroll(false)
        .allow_zoom(false)
        .include_y(0.0)
        .include_y(max as f64 * 1.25)
        .include_x(-0.6)
        .include_x(values.len() as f64 - 0.4)
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
        .vscroll(false)
        .column(Column::auto().at_least(110.0).clip(true))
        .columns(
            Column::auto().at_least(80.0),
            headers.len().saturating_sub(2).max(0),
        )
        .column(Column::remainder().at_least(180.0).clip(true))
        .header(26.0, |mut header| {
            for h in headers {
                header.col(|ui| {
                    // the table clips child widgets to the cell, so the header
                    // band is painted rather than built from a widget
                    let band = ui.max_rect().expand2(egui::vec2(6.0, 5.0));
                    ui.painter()
                        .rect_filled(band, egui::CornerRadius::ZERO, theme.surface_hover.gamma_multiply(0.55));
                    ui.label(
                        egui::RichText::new(*h)
                            .strong()
                            .size(12.0)
                            .color(theme.text_muted),
                    );
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