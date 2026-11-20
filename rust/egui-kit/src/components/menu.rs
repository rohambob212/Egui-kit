//! Hierarchical context Menu with actions, shortcuts, checkboxes, radio items, and submenus.

use crate::components::icon::{paint_icon, IconName};
use crate::theme::active_theme;
use egui::{Align2, Area, FontId, Frame, Id, Key, Margin, Order, Pos2, Rect, Response, Rounding, Sense, Stroke, Ui, Vec2};

/// Entry rendered inside a [`Menu`] popup.
#[derive(Clone)]
pub enum MenuItem {
    Action {
        id: String,
        label: String,
        shortcut: Option<String>,
        disabled: bool,
    },
    Checkbox {
        id: String,
        label: String,
        checked: bool,
    },
    Radio {
        id: String,
        label: String,
        selected: bool,
    },
    Submenu {
        id: String,
        label: String,
        children: Vec<MenuItem>,
    },
    Separator,
}

impl MenuItem {
    pub fn action(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self::Action {
            id: id.into(),
            label: label.into(),
            shortcut: None,
            disabled: false,
        }
    }

    pub fn action_shortcut(
        id: impl Into<String>,
        label: impl Into<String>,
        shortcut: impl Into<String>,
    ) -> Self {
        Self::Action {
            id: id.into(),
            label: label.into(),
            shortcut: Some(shortcut.into()),
            disabled: false,
        }
    }

    pub fn checkbox(id: impl Into<String>, label: impl Into<String>, checked: bool) -> Self {
        Self::Checkbox {
            id: id.into(),
            label: label.into(),
            checked,
        }
    }

    pub fn radio(id: impl Into<String>, label: impl Into<String>, selected: bool) -> Self {
        Self::Radio {
            id: id.into(),
            label: label.into(),
            selected,
        }
    }

    pub fn submenu(id: impl Into<String>, label: impl Into<String>, children: Vec<MenuItem>) -> Self {
        Self::Submenu {
            id: id.into(),
            label: label.into(),
            children,
        }
    }
}

/// Context and dropdown menu renderer returning the triggered item ID on click.
pub struct Menu;

impl Menu {
    /// Attaches a right-click context menu to `target` and returns the clicked item ID, if any.
    pub fn context(target: &Response, items: &[MenuItem]) -> Option<String> {
        let mut selected = None;
        target.context_menu(|ui| {
            selected = Self::render_list(ui, Id::new(target.id).with("ctx_menu"), items);
            if selected.is_some() {
                ui.close_menu();
            }
        });
        selected
    }

    /// Renders a controlled menu anchored to `anchor` when `*open` is true.
    pub fn dropdown(
        ui: &mut Ui,
        id_source: &str,
        anchor: &Response,
        open: &mut bool,
        items: &[MenuItem],
    ) -> Option<String> {
        if !*open {
            return None;
        }
        if ui.ctx().input(|i| i.key_pressed(Key::Escape)) {
            *open = false;
            return None;
        }

        let tokens = active_theme(ui.ctx());
        let screen = ui.ctx().screen_rect();
        let pos = Pos2::new(
            anchor.rect.left().clamp(screen.left() + 8.0, (screen.right() - 228.0).max(screen.left() + 8.0)),
            (anchor.rect.bottom() + 4.0).clamp(screen.top() + 8.0, (screen.bottom() - 200.0).max(screen.top() + 8.0)),
        );

        let mut clicked = None;
        let area = Area::new(Id::new(id_source).with("menu_popup"))
            .order(Order::Foreground)
            .fixed_pos(pos)
            .show(ui.ctx(), |ui| {
                Frame::none()
                    .fill(tokens.card)
                    .stroke(Stroke::new(1.0, tokens.border))
                    .rounding(Rounding::same(8.0))
                    .inner_margin(Margin::same(6.0))
                    .show(ui, |ui| {
                        ui.set_width(210.0);
                        clicked = Self::render_list(ui, Id::new(id_source), items);
                    });
            });

        if clicked.is_some() {
            *open = false;
        } else if !anchor.clicked()
            && ui.ctx().input(|i| i.pointer.any_pressed())
            && !area.response.rect.contains(
                ui.ctx().input(|i| i.pointer.interact_pos().unwrap_or(Pos2::ZERO)),
            )
            && !anchor.rect.contains(
                ui.ctx().input(|i| i.pointer.interact_pos().unwrap_or(Pos2::ZERO)),
            )
        {
            *open = false;
        }

        clicked
    }

    fn render_list(ui: &mut Ui, scope_id: Id, items: &[MenuItem]) -> Option<String> {
        let tokens = active_theme(ui.ctx());
        let mut clicked = None;
        let sub_id = scope_id.with("active_sub");
        let mut open_sub = ui.ctx().data(|d| d.get_temp::<Option<String>>(sub_id).unwrap_or(None));

        for item in items {
            match item {
                MenuItem::Separator => {
                    ui.add_space(3.0);
                    ui.separator();
                    ui.add_space(3.0);
                }
                MenuItem::Action { id, label, shortcut, disabled } => {
                    let (rect, resp) = ui.allocate_exact_size(
                        Vec2::new(ui.available_width(), 28.0),
                        if *disabled { Sense::hover() } else { Sense::click() },
                    );
                    if resp.hovered() && !disabled {
                        ui.painter().rect_filled(rect, Rounding::same(5.0), tokens.accent);
                    }
                    let fg = if *disabled {
                        tokens.muted_foreground
                    } else {
                        tokens.card_foreground
                    };
                    ui.painter().text(
                        Pos2::new(rect.left() + 26.0, rect.center().y),
                        Align2::LEFT_CENTER,
                        label,
                        FontId::proportional(12.5),
                        fg,
                    );
                    if let Some(sc) = shortcut {
                        ui.painter().text(
                            Pos2::new(rect.right() - 8.0, rect.center().y),
                            Align2::RIGHT_CENTER,
                            sc,
                            FontId::monospace(11.0),
                            tokens.muted_foreground,
                        );
                    }
                    if resp.clicked() && !disabled {
                        clicked = Some(id.clone());
                    }
                }
                MenuItem::Checkbox { id, label, checked } => {
                    let (rect, resp) = ui.allocate_exact_size(
                        Vec2::new(ui.available_width(), 28.0),
                        Sense::click(),
                    );
                    if resp.hovered() {
                        ui.painter().rect_filled(rect, Rounding::same(5.0), tokens.accent);
                    }
                    if *checked {
                        paint_icon(
                            ui.painter(),
                            Rect::from_center_size(Pos2::new(rect.left() + 14.0, rect.center().y), Vec2::splat(12.0)),
                            IconName::Check,
                            tokens.primary,
                        );
                    }
                    ui.painter().text(
                        Pos2::new(rect.left() + 26.0, rect.center().y),
                        Align2::LEFT_CENTER,
                        label,
                        FontId::proportional(12.5),
                        tokens.card_foreground,
                    );
                    if resp.clicked() {
                        clicked = Some(id.clone());
                    }
                }
                MenuItem::Radio { id, label, selected } => {
                    let (rect, resp) = ui.allocate_exact_size(
                        Vec2::new(ui.available_width(), 28.0),
                        Sense::click(),
                    );
                    if resp.hovered() {
                        ui.painter().rect_filled(rect, Rounding::same(5.0), tokens.accent);
                    }
                    if *selected {
                        ui.painter().circle_filled(
                            Pos2::new(rect.left() + 14.0, rect.center().y),
                            3.5,
                            tokens.primary,
                        );
                    }
                    ui.painter().text(
                        Pos2::new(rect.left() + 26.0, rect.center().y),
                        Align2::LEFT_CENTER,
                        label,
                        FontId::proportional(12.5),
                        tokens.card_foreground,
                    );
                    if resp.clicked() {
                        clicked = Some(id.clone());
                    }
                }
                MenuItem::Submenu { id, label, children } => {
                    let (rect, resp) = ui.allocate_exact_size(
                        Vec2::new(ui.available_width(), 28.0),
                        Sense::click(),
                    );
                    if resp.hovered() {
                        open_sub = Some(id.clone());
                        ui.painter().rect_filled(rect, Rounding::same(5.0), tokens.accent);
                    }
                    ui.painter().text(
                        Pos2::new(rect.left() + 26.0, rect.center().y),
                        Align2::LEFT_CENTER,
                        label,
                        FontId::proportional(12.5),
                        tokens.card_foreground,
                    );
                    paint_icon(
                        ui.painter(),
                        Rect::from_center_size(Pos2::new(rect.right() - 12.0, rect.center().y), Vec2::splat(11.0)),
                        IconName::ChevronRight,
                        tokens.muted_foreground,
                    );
                    if open_sub.as_deref() == Some(id.as_str()) {
                        let sub_pos = Pos2::new(rect.right() + 4.0, rect.top());
                        Area::new(scope_id.with(id))
                            .order(Order::Tooltip)
                            .fixed_pos(sub_pos)
                            .show(ui.ctx(), |ui| {
                                Frame::none()
                                    .fill(tokens.card)
                                    .stroke(Stroke::new(1.0, tokens.border))
                                    .rounding(Rounding::same(8.0))
                                    .inner_margin(Margin::same(6.0))
                                    .show(ui, |ui| {
                                        ui.set_width(180.0);
                                        if let Some(sub_click) = Self::render_list(ui, scope_id.with(id), children) {
                                            clicked = Some(sub_click);
                                        }
                                    });
                            });
                    }
                }
            }
        }

        ui.ctx().data_mut(|d| d.insert_temp(sub_id, open_sub));
        clicked
    }
}
