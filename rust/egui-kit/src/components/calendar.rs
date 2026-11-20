//! Zero-dependency Gregorian Calendar grid mutating `&mut SimpleDate` with instant Month/Year views.

use crate::theme::active_theme;
use crate::tokens::{ease_out_cubic, lerp_color};
use egui::{Align2, FontId, Frame, Id, Pos2, Rect, Response, Rounding, Sense, Stroke, Ui, Vec2, Widget};

/// Lightweight calendar date value (`month` 1..=12, `day` 1..=31).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SimpleDate {
    pub year: i32,
    pub month: u8,
    pub day: u8,
}

impl SimpleDate {
    pub const fn new(year: i32, month: u8, day: u8) -> Self {
        Self { year, month, day }
    }

    pub fn format_ymd(self) -> String {
        format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

impl Default for SimpleDate {
    fn default() -> Self {
        Self::new(2025, 3, 15)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum CalendarView {
    #[default]
    Days,
    Months,
    Years,
}

/// Interactive monthly Calendar grid with clickable Month & Year drill-down grids and range support.
pub struct Calendar<'a> {
    id_source: &'a str,
    selected: &'a mut SimpleDate,
    range_end: Option<&'a mut SimpleDate>,
    cell_size: f32,
}

impl<'a> Calendar<'a> {
    pub fn new(id_source: &'a str, selected: &'a mut SimpleDate) -> Self {
        Self {
            id_source,
            selected,
            range_end: None,
            cell_size: 34.0,
        }
    }

    /// Enables start..=end range selection bound to a second `&mut SimpleDate`.
    pub fn range(mut self, range_end: &'a mut SimpleDate) -> Self {
        self.range_end = Some(range_end);
        self
    }

    pub fn cell_size(mut self, size: f32) -> Self {
        self.cell_size = size.clamp(26.0, 44.0);
        self
    }
}

impl<'a> Widget for Calendar<'a> {
    fn ui(mut self, ui: &mut Ui) -> Response {
        let tokens = active_theme(ui.ctx());
        let state_id = Id::new(("egui_kit_calendar", self.id_source));
        let (mut view_year, mut view_month, mut view_mode) = ui.ctx().data(|d| {
            d.get_temp::<(i32, u8, CalendarView)>(state_id)
                .unwrap_or((self.selected.year, self.selected.month.clamp(1, 12), CalendarView::Days))
        });

        let grid_w = self.cell_size * 7.0;
        let frame_resp = Frame::none()
            .fill(tokens.card)
            .stroke(Stroke::new(1.0, tokens.border))
            .rounding(Rounding::same(8.0))
            .inner_margin(egui::Margin::same(12.0))
            .show(ui, |ui| {
                ui.set_width(grid_w);

                // Header bar: [<<] [<] [Month] [Year] [>] [>>]
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 3.0;
                    let nav_w = 22.0;

                    if header_nav_btn(ui, state_id.with("py"), "<<", &tokens).clicked() {
                        view_year -= match view_mode {
                            CalendarView::Years => 12,
                            _ => 1,
                        };
                    }
                    if header_nav_btn(ui, state_id.with("pm"), "<", &tokens).clicked() {
                        match view_mode {
                            CalendarView::Days => {
                                if view_month == 1 {
                                    view_month = 12;
                                    view_year -= 1;
                                } else {
                                    view_month -= 1;
                                }
                            }
                            CalendarView::Months => view_year -= 1,
                            CalendarView::Years => view_year -= 12,
                        }
                    }

                    let pills_w = (grid_w - nav_w * 4.0 - 15.0).max(96.0);
                    let month_w = pills_w * 0.56;
                    let year_w = pills_w - month_w;

                    let m_active = view_mode == CalendarView::Months;
                    if header_pill_btn(ui, state_id.with("mpill"), month_short(view_month), month_w, m_active, &tokens).clicked() {
                        view_mode = if m_active { CalendarView::Days } else { CalendarView::Months };
                    }

                    let y_active = view_mode == CalendarView::Years;
                    if header_pill_btn(ui, state_id.with("ypill"), &view_year.to_string(), year_w, y_active, &tokens).clicked() {
                        view_mode = if y_active { CalendarView::Days } else { CalendarView::Years };
                    }

                    if header_nav_btn(ui, state_id.with("nm"), ">", &tokens).clicked() {
                        match view_mode {
                            CalendarView::Days => {
                                if view_month == 12 {
                                    view_month = 1;
                                    view_year += 1;
                                } else {
                                    view_month += 1;
                                }
                            }
                            CalendarView::Months => view_year += 1,
                            CalendarView::Years => view_year += 12,
                        }
                    }
                    if header_nav_btn(ui, state_id.with("ny"), ">>", &tokens).clicked() {
                        view_year += match view_mode {
                            CalendarView::Years => 12,
                            _ => 1,
                        };
                    }
                });

                ui.add_space(8.0);
                let mut changed = false;

                match view_mode {
                    CalendarView::Years => {
                        // 4 rows x 3 cols decade grid for single-click multi-year jumps
                        let decade_start = view_year.div_euclid(12) * 12;
                        let cell_w = grid_w / 3.0;
                        let cell_h = (self.cell_size * 7.0 - 16.0) / 4.0;
                        for r in 0..4 {
                            let (row_rect, _) = ui.allocate_exact_size(Vec2::new(grid_w, cell_h), Sense::hover());
                            for c in 0..3 {
                                let yr = decade_start + (r * 3 + c) as i32;
                                let rect = Rect::from_min_size(
                                    Pos2::new(row_rect.left() + c as f32 * cell_w, row_rect.top()),
                                    Vec2::new(cell_w, cell_h),
                                )
                                .shrink(3.0);
                                let resp = ui.interact(rect, state_id.with(("yr", yr)), Sense::click());
                                let is_cur = yr == view_year;
                                let hov_t = ui.ctx().animate_bool_with_time(resp.id.with("h"), resp.hovered(), 0.10);
                                if is_cur || hov_t > 0.01 {
                                    let fill = if is_cur {
                                        tokens.primary
                                    } else {
                                        lerp_color(tokens.card, tokens.accent, hov_t * 0.7)
                                    };
                                    ui.painter().rect_filled(rect, Rounding::same(6.0), fill);
                                }
                                ui.painter().text(
                                    rect.center(),
                                    Align2::CENTER_CENTER,
                                    yr.to_string(),
                                    FontId::monospace(12.5),
                                    if is_cur { tokens.primary_foreground } else { tokens.foreground },
                                );
                                if resp.clicked() {
                                    view_year = yr;
                                    view_mode = CalendarView::Months;
                                }
                            }
                        }
                    }
                    CalendarView::Months => {
                        // 4 rows x 3 cols month grid
                        let cell_w = grid_w / 3.0;
                        let cell_h = (self.cell_size * 7.0 - 16.0) / 4.0;
                        for r in 0..4 {
                            let (row_rect, _) = ui.allocate_exact_size(Vec2::new(grid_w, cell_h), Sense::hover());
                            for c in 0..3 {
                                let m = (r * 3 + c + 1) as u8;
                                let rect = Rect::from_min_size(
                                    Pos2::new(row_rect.left() + c as f32 * cell_w, row_rect.top()),
                                    Vec2::new(cell_w, cell_h),
                                )
                                .shrink(3.0);
                                let resp = ui.interact(rect, state_id.with(("mo", m)), Sense::click());
                                let is_cur = m == view_month;
                                let hov_t = ui.ctx().animate_bool_with_time(resp.id.with("h"), resp.hovered(), 0.10);
                                if is_cur || hov_t > 0.01 {
                                    let fill = if is_cur {
                                        tokens.primary
                                    } else {
                                        lerp_color(tokens.card, tokens.accent, hov_t * 0.7)
                                    };
                                    ui.painter().rect_filled(rect, Rounding::same(6.0), fill);
                                }
                                ui.painter().text(
                                    rect.center(),
                                    Align2::CENTER_CENTER,
                                    month_short(m),
                                    FontId::proportional(12.5),
                                    if is_cur { tokens.primary_foreground } else { tokens.foreground },
                                );
                                if resp.clicked() {
                                    view_month = m;
                                    view_mode = CalendarView::Days;
                                }
                            }
                        }
                    }
                    CalendarView::Days => {
                        // Weekday header row
                        let (wk_rect, _) = ui.allocate_exact_size(Vec2::new(grid_w, 20.0), Sense::hover());
                        for (col, label) in ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"].iter().enumerate() {
                            let cx = wk_rect.left() + (col as f32 + 0.5) * self.cell_size;
                            ui.painter().text(
                                Pos2::new(cx, wk_rect.center().y),
                                Align2::CENTER_CENTER,
                                *label,
                                FontId::proportional(11.0),
                                tokens.muted_foreground,
                            );
                        }

                        // 6x7 day cells
                        let first_wd = weekday_sunday0(view_year, view_month, 1);
                        let days = days_in_month(view_year, view_month);

                        for row in 0..6 {
                            let (row_rect, _) = ui.allocate_exact_size(Vec2::new(grid_w, self.cell_size), Sense::hover());
                            for col in 0..7 {
                                let slot = row * 7 + col;
                                let cell_rect = Rect::from_min_size(
                                    Pos2::new(row_rect.left() + col as f32 * self.cell_size, row_rect.top()),
                                    Vec2::splat(self.cell_size),
                                )
                                .shrink(2.0);

                                if slot < first_wd || (slot - first_wd) >= days as usize {
                                    continue;
                                }

                                let day = (slot - first_wd + 1) as u8;
                                let candidate = SimpleDate::new(view_year, view_month, day);
                                let cell_resp = ui.interact(cell_rect, state_id.with((view_year, view_month, day)), Sense::click());

                                let is_start = candidate == *self.selected;
                                let is_end = self.range_end.as_ref().is_some_and(|e| candidate == **e);
                                let in_range = self.range_end.as_ref().is_some_and(|e| {
                                    let (lo, hi) = if *self.selected <= **e {
                                        (*self.selected, **e)
                                    } else {
                                        (**e, *self.selected)
                                    };
                                    candidate > lo && candidate < hi
                                });

                                let is_edge = is_start || is_end;
                                let sel_t = ease_out_cubic(ui.ctx().animate_bool_with_time(cell_resp.id.with("s"), is_edge, 0.14));
                                let hov_t = ui.ctx().animate_bool_with_time(cell_resp.id.with("h"), cell_resp.hovered(), 0.10);

                                if in_range {
                                    ui.painter().rect_filled(cell_rect, Rounding::same(4.0), lerp_color(tokens.card, tokens.primary, 0.16));
                                } else if sel_t > 0.01 || hov_t > 0.01 {
                                    let fill = if sel_t > 0.01 {
                                        lerp_color(tokens.card, tokens.primary, sel_t)
                                    } else {
                                        lerp_color(tokens.card, tokens.accent, hov_t * 0.6)
                                    };
                                    ui.painter().rect_filled(cell_rect, Rounding::same(6.0), fill);
                                }

                                let fg = if is_edge { tokens.primary_foreground } else { tokens.foreground };
                                ui.painter().text(
                                    cell_rect.center(),
                                    Align2::CENTER_CENTER,
                                    day.to_string(),
                                    FontId::monospace(12.0),
                                    fg,
                                );

                                if cell_resp.clicked() {
                                    if let Some(ref mut end_date) = self.range_end {
                                        if candidate < *self.selected {
                                            **end_date = *self.selected;
                                            *self.selected = candidate;
                                        } else {
                                            **end_date = candidate;
                                        }
                                    } else {
                                        *self.selected = candidate;
                                    }
                                    changed = true;
                                }
                            }
                        }
                    }
                }
                changed
            });

        ui.ctx().data_mut(|d| d.insert_temp(state_id, (view_year, view_month, view_mode)));
        let mut resp = frame_resp.response;
        if frame_resp.inner {
            resp.mark_changed();
        }
        resp
    }
}

fn header_nav_btn(ui: &mut Ui, id: Id, label: &str, tokens: &crate::theme::ThemeTokens) -> Response {
    let (rect, resp) = ui.allocate_exact_size(Vec2::new(22.0, 26.0), Sense::click());
    let hov = ui.ctx().animate_bool_with_time(id.with("h"), resp.hovered(), 0.12);
    ui.painter().rect(
        rect,
        Rounding::same(5.0),
        lerp_color(tokens.muted, tokens.accent, hov),
        Stroke::new(1.0, tokens.border),
    );
    ui.painter().text(
        rect.center(),
        Align2::CENTER_CENTER,
        label,
        FontId::monospace(11.0),
        tokens.foreground,
    );
    resp
}

fn header_pill_btn(
    ui: &mut Ui,
    id: Id,
    label: &str,
    width: f32,
    active: bool,
    tokens: &crate::theme::ThemeTokens,
) -> Response {
    let (rect, resp) = ui.allocate_exact_size(Vec2::new(width, 26.0), Sense::click());
    let hov = ui.ctx().animate_bool_with_time(id.with("h"), resp.hovered(), 0.12);
    let bg = if active {
        tokens.primary
    } else {
        lerp_color(tokens.muted, tokens.accent, hov)
    };
    let fg = if active { tokens.primary_foreground } else { tokens.foreground };
    ui.painter().rect(rect, Rounding::same(5.0), bg, Stroke::new(1.0, tokens.border));
    ui.painter().text(rect.center(), Align2::CENTER_CENTER, label, FontId::proportional(12.5), fg);
    resp
}

/// Returns whether `year` is a Gregorian leap year.
pub const fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

/// Number of days in `(year, month)` where `month` is `1..=12`.
pub const fn days_in_month(year: i32, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => 30,
    }
}

/// Tomohiko Sakamoto weekday algorithm returning `0 = Sunday ..= 6 = Saturday`.
pub fn weekday_sunday0(mut y: i32, m: u8, d: u8) -> usize {
    const T: [i32; 12] = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let m_idx = (m.clamp(1, 12) - 1) as usize;
    if m < 3 {
        y -= 1;
    }
    ((y + y / 4 - y / 100 + y / 400 + T[m_idx] + d as i32).rem_euclid(7)) as usize
}

pub const fn month_short(month: u8) -> &'static str {
    match month {
        1 => "Jan",
        2 => "Feb",
        3 => "Mar",
        4 => "Apr",
        5 => "May",
        6 => "Jun",
        7 => "Jul",
        8 => "Aug",
        9 => "Sep",
        10 => "Oct",
        11 => "Nov",
        12 => "Dec",
        _ => "Mon",
    }
}
