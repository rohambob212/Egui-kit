//! Responsive CSS-grid-inspired GridLayout container with column span support.

use egui::{Response, Ui};

/// Multi-column grid layout helper dividing `ui.available_width()` evenly.
pub struct GridLayout {
    columns: usize,
    gap: f32,
}

/// Active grid row/column cursor passed to [`GridLayout::show`].
pub struct GridScope<'a> {
    ui: &'a mut Ui,
    columns: usize,
    gap: f32,
    col_w: f32,
    pending_spans: usize,
    row_items: Vec<Box<dyn FnOnce(&mut Ui) + 'a>>,
}

impl GridLayout {
    pub fn columns(columns: usize) -> Self {
        Self {
            columns: columns.clamp(1, 12),
            gap: 12.0,
        }
    }

    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap.max(0.0);
        self
    }

    /// Renders a uniform collection of `count` items across `columns` equal tracks.
    pub fn show_items(
        self,
        ui: &mut Ui,
        count: usize,
        mut item_ui: impl FnMut(&mut Ui, usize),
    ) -> Response {
        let avail = ui.available_width().max(120.0);
        let cols = self.columns;
        let total_gaps = self.gap * (cols.saturating_sub(1) as f32);
        let col_w = ((avail - total_gaps) / (cols as f32)).max(24.0);

        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing.y = self.gap;
            for row_start in (0..count).step_by(cols) {
                ui.horizontal_top(|ui| {
                    ui.spacing_mut().item_spacing.x = self.gap;
                    let row_end = (row_start + cols).min(count);
                    for idx in row_start..row_end {
                        ui.allocate_ui_with_layout(
                            egui::vec2(col_w, 0.0),
                            egui::Layout::top_down(egui::Align::Min),
                            |ui| {
                                ui.set_width(col_w);
                                item_ui(ui, idx);
                            },
                        );
                    }
                });
            }
        })
        .response
    }

    /// Opens a flexible grid builder supporting variable `col_span` cells.
    pub fn show<'a>(self, ui: &'a mut Ui, build: impl FnOnce(&mut GridScope<'a>)) -> Response {
        let avail = ui.available_width().max(120.0);
        let total_gaps = self.gap * (self.columns.saturating_sub(1) as f32);
        let col_w = ((avail - total_gaps) / (self.columns as f32)).max(24.0);

        let mut scope = GridScope {
            ui,
            columns: self.columns,
            gap: self.gap,
            col_w,
            pending_spans: 0,
            row_items: Vec::new(),
        };
        build(&mut scope);
        scope.flush_row();
        scope.ui.allocate_exact_size(egui::Vec2::ZERO, egui::Sense::hover()).1
    }
}

impl<'a> GridScope<'a> {
    /// Queues a grid cell spanning `span` columns (clamped to `1..=columns`).
    pub fn cell(&mut self, span: usize, add_contents: impl FnOnce(&mut Ui) + 'a) {
        let span = span.clamp(1, self.columns);
        if self.pending_spans + span > self.columns {
            self.flush_row();
        }
        let width = self.col_w * (span as f32) + self.gap * ((span.saturating_sub(1)) as f32);
        let gap = self.gap;
        self.pending_spans += span;
        self.row_items.push(Box::new(move |ui| {
            ui.allocate_ui_with_layout(
                egui::vec2(width, 0.0),
                egui::Layout::top_down(egui::Align::Min),
                |ui| {
                    ui.set_width(width);
                    add_contents(ui);
                },
            );
            let _ = gap;
        }));
    }

    fn flush_row(&mut self) {
        if self.row_items.is_empty() {
            return;
        }
        let gap = self.gap;
        let items = std::mem::take(&mut self.row_items);
        self.pending_spans = 0;
        self.ui.horizontal_top(|ui| {
            ui.spacing_mut().item_spacing.x = gap;
            for item in items {
                item(ui);
            }
        });
        self.ui.add_space(gap);
    }
}
