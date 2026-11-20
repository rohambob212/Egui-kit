//! Fuzzy-searchable Cmd+K Command palette overlay with keyboard navigation.

use crate::components::icon::{paint_icon, IconName};
use crate::components::kbd::Kbd;
use crate::theme::active_theme;
use crate::tokens::{ease_out_cubic, Size};
use egui::{
    Align2, Area, Color32, Context, FontId, Frame, Id, Key, Margin, Order, Pos2, Rect, Rounding,
    Sense, Stroke, TextEdit, Vec2,
};

/// Action descriptor listed inside a [`Command`] palette.
#[derive(Debug, Clone)]
pub struct CommandItem {
    pub id: String,
    pub label: String,
    pub group: String,
    pub shortcut: Option<String>,
}

impl CommandItem {
    pub fn new(id: impl Into<String>, label: impl Into<String>, group: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            group: group.into(),
            shortcut: None,
        }
    }

    pub fn shortcut(mut self, shortcut: impl Into<String>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }
}

/// Fuzzy-searchable `Cmd+K` command palette bound directly to `&mut bool`.
pub struct Command<'a> {
    id_source: &'a str,
    open: &'a mut bool,
    items: &'a [CommandItem],
    placeholder: String,
}

impl<'a> Command<'a> {
    pub fn new(id_source: &'a str, open: &'a mut bool, items: &'a [CommandItem]) -> Self {
        Self {
            id_source,
            open,
            items,
            placeholder: "Type a command or search...".to_string(),
        }
    }

    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Shows the palette modal when `*open` is true and returns the executed item's `id` if triggered.
    pub fn show(self, ctx: &Context) -> Option<String> {
        let anim_t = ctx.animate_bool_with_time(Id::new(self.id_source).with("open"), *self.open, 0.14);
        if !*self.open && anim_t <= 0.01 {
            return None;
        }

        let eased = ease_out_cubic(anim_t);
        let tokens = active_theme(ctx);
        let screen = ctx.screen_rect();

        if *self.open && ctx.input(|i| i.key_pressed(Key::Escape)) {
            *self.open = false;
        }

        let query_id = Id::new(self.id_source).with("query");
        let sel_id = Id::new(self.id_source).with("sel_idx");
        let mut query = ctx.data(|d| d.get_temp::<String>(query_id).unwrap_or_default());
        let mut sel_idx = ctx.data(|d| d.get_temp::<usize>(sel_id).unwrap_or(0));

        // Backdrop scrim
        let scrim_alpha = (115.0 * eased) as u8;
        Area::new(Id::new(self.id_source).with("scrim"))
            .order(Order::Middle)
            .fixed_pos(screen.min)
            .show(ctx, |ui| {
                let (rect, resp) = ui.allocate_exact_size(screen.size(), Sense::click());
                ui.painter().rect_filled(
                    rect,
                    Rounding::ZERO,
                    Color32::from_rgba_unmultiplied(8, 12, 20, scrim_alpha),
                );
                if resp.clicked() {
                    *self.open = false;
                }
            });

        let filtered: Vec<&CommandItem> = self
            .items
            .iter()
            .filter(|item| fuzzy_matches(&query, &item.label) || fuzzy_matches(&query, &item.group))
            .collect();

        if filtered.is_empty() {
            sel_idx = 0;
        } else {
            sel_idx = sel_idx.min(filtered.len() - 1);
        }

        if *self.open && !filtered.is_empty() {
            if ctx.input(|i| i.key_pressed(Key::ArrowDown)) {
                sel_idx = (sel_idx + 1) % filtered.len();
            } else if ctx.input(|i| i.key_pressed(Key::ArrowUp)) {
                sel_idx = if sel_idx == 0 { filtered.len() - 1 } else { sel_idx - 1 };
            }
        }

        let mut executed = None;
        let palette_w = 500.0_f32.min(screen.width() - 32.0);
        let top_y = (screen.top() + screen.height() * 0.16) + (1.0 - eased) * 10.0;

        Area::new(Id::new(self.id_source).with("palette"))
            .order(Order::Foreground)
            .fixed_pos(Pos2::new(screen.center().x - palette_w * 0.5, top_y))
            .show(ctx, |ui| {
                Frame::none()
                    .fill(tokens.card)
                    .stroke(Stroke::new(1.0, tokens.border))
                    .rounding(Rounding::same(10.0))
                    .inner_margin(Margin::same(10.0))
                    .show(ui, |ui| {
                        ui.set_width(palette_w - 20.0);

                        // Search bar
                        ui.horizontal(|ui| {
                            let (icon_rect, _) = ui.allocate_exact_size(Vec2::splat(20.0), Sense::hover());
                            paint_icon(
                                ui.painter(),
                                Rect::from_center_size(icon_rect.center(), Vec2::splat(14.0)),
                                IconName::Search,
                                tokens.muted_foreground,
                            );
                            let edit_resp = ui.add(
                                TextEdit::singleline(&mut query)
                                    .hint_text(&self.placeholder)
                                    .frame(false)
                                    .desired_width(ui.available_width() - 40.0),
                            );
                            if *self.open {
                                edit_resp.request_focus();
                            }
                            ui.add(Kbd::new("Esc").size(Size::Xs));
                        });

                        ui.add_space(6.0);
                        ui.separator();
                        ui.add_space(6.0);

                        if filtered.is_empty() {
                            ui.vertical_centered(|ui| {
                                ui.add_space(16.0);
                                ui.label(
                                    egui::RichText::new("No matching commands")
                                        .font(FontId::proportional(13.0))
                                        .color(tokens.muted_foreground),
                                );
                                ui.add_space(16.0);
                            });
                        } else {
                            let enter_pressed = ctx.input(|i| i.key_pressed(Key::Enter));
                            for (idx, item) in filtered.iter().enumerate() {
                                let is_sel = idx == sel_idx;
                                let (row_rect, row_resp) = ui.allocate_exact_size(
                                    Vec2::new(ui.available_width(), 32.0),
                                    Sense::click(),
                                );
                                if row_resp.hovered() {
                                    sel_idx = idx;
                                }
                                if is_sel {
                                    ui.painter().rect_filled(row_rect, Rounding::same(6.0), tokens.accent);
                                }
                                ui.painter().text(
                                    Pos2::new(row_rect.left() + 10.0, row_rect.center().y),
                                    Align2::LEFT_CENTER,
                                    &item.label,
                                    FontId::proportional(13.0),
                                    tokens.card_foreground,
                                );
                                ui.painter().text(
                                    Pos2::new(row_rect.right() - 78.0, row_rect.center().y),
                                    Align2::RIGHT_CENTER,
                                    &item.group,
                                    FontId::proportional(11.0),
                                    tokens.muted_foreground,
                                );
                                if let Some(sc) = &item.shortcut {
                                    ui.painter().text(
                                        Pos2::new(row_rect.right() - 10.0, row_rect.center().y),
                                        Align2::RIGHT_CENTER,
                                        sc,
                                        FontId::monospace(11.0),
                                        tokens.muted_foreground,
                                    );
                                }
                                if row_resp.clicked() || (is_sel && enter_pressed) {
                                    executed = Some(item.id.clone());
                                    *self.open = false;
                                }
                            }
                        }
                    });
            });

        ctx.data_mut(|d| {
            d.insert_temp(query_id, query);
            d.insert_temp(sel_id, sel_idx);
        });

        executed
    }
}

fn fuzzy_matches(query: &str, target: &str) -> bool {
    let q = query.trim().to_ascii_lowercase();
    if q.is_empty() {
        return true;
    }
    let mut chars = q.chars().peekable();
    for tc in target.to_ascii_lowercase().chars() {
        if let Some(&qc) = chars.peek() {
            if tc == qc {
                chars.next();
            }
        }
    }
    chars.peek().is_none()
}
