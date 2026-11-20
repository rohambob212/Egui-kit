//! Edge-sliding Notification toast stack with countdown progress bars and action triggers.

use crate::components::button::{Button, ButtonVariant};
use crate::components::icon::{paint_icon, IconName};
use crate::theme::active_theme;
use crate::tokens::ease_out_cubic;
use egui::{Area, Color32, Context, FontId, Frame, Id, Margin, Order, Pos2, Rect, Rounding, Sense, Stroke, Vec2};

/// Semantic tone for a [`Notification`] toast.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NotificationTone {
    #[default]
    Info,
    Success,
    Warning,
    Destructive,
}

/// Screen edge corner from which notifications slide in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NotificationEdge {
    TopRight,
    #[default]
    BottomRight,
    TopLeft,
    BottomLeft,
}

/// Toast notification specification pushed into [`NotificationManager`].
#[derive(Debug, Clone)]
pub struct Notification {
    pub id: u64,
    pub title: String,
    pub message: Option<String>,
    pub tone: NotificationTone,
    pub duration_secs: f64,
    pub action_label: Option<String>,
    created_at: f64,
}

impl Notification {
    pub fn info(title: impl Into<String>) -> Self {
        Self::new(title, NotificationTone::Info)
    }

    pub fn success(title: impl Into<String>) -> Self {
        Self::new(title, NotificationTone::Success)
    }

    pub fn warning(title: impl Into<String>) -> Self {
        Self::new(title, NotificationTone::Warning)
    }

    pub fn destructive(title: impl Into<String>) -> Self {
        Self::new(title, NotificationTone::Destructive)
    }

    pub fn new(title: impl Into<String>, tone: NotificationTone) -> Self {
        Self {
            id: 0,
            title: title.into(),
            message: None,
            tone,
            duration_secs: 4.5,
            action_label: None,
            created_at: 0.0,
        }
    }

    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }

    pub fn duration(mut self, secs: f64) -> Self {
        self.duration_secs = secs.max(0.5);
        self
    }

    pub fn action(mut self, label: impl Into<String>) -> Self {
        self.action_label = Some(label.into());
        self
    }
}

/// Screen-edge notification toast stack persisted in `egui::Context` memory.
pub struct NotificationManager;

impl NotificationManager {
    fn storage_id() -> Id {
        Id::new("egui_kit_notification_stack")
    }

    fn next_seq_id() -> Id {
        Id::new("egui_kit_notification_seq")
    }

    /// Pushes a [`Notification`] onto the edge stack and returns its assigned ID.
    pub fn push(ctx: &Context, mut toast: Notification) -> u64 {
        let now = ctx.input(|i| i.time);
        let id = ctx.data_mut(|d| {
            let next = d.get_temp::<u64>(Self::next_seq_id()).unwrap_or(1);
            d.insert_temp(Self::next_seq_id(), next + 1);
            next
        });
        toast.id = id;
        toast.created_at = now;
        ctx.data_mut(|d| {
            let mut list = d.get_temp::<Vec<Notification>>(Self::storage_id()).unwrap_or_default();
            list.push(toast);
            if list.len() > 5 {
                list.remove(0);
            }
            d.insert_temp(Self::storage_id(), list);
        });
        id
    }

    /// Renders active notifications sliding out from `edge` and returns the ID of any clicked action button.
    pub fn show(ctx: &Context, edge: NotificationEdge) -> Option<u64> {
        let now = ctx.input(|i| i.time);
        let mut items = ctx.data(|d| d.get_temp::<Vec<Notification>>(Self::storage_id()).unwrap_or_default());
        items.retain(|n| now - n.created_at < n.duration_secs);
        if items.is_empty() {
            ctx.data_mut(|d| d.insert_temp(Self::storage_id(), items));
            return None;
        }

        let tokens = active_theme(ctx);
        let screen = ctx.screen_rect();
        let width = 320.0_f32.min(screen.width() - 32.0);
        let mut dismissed: Vec<u64> = Vec::new();
        let mut action_triggered = None;

        let is_top = matches!(edge, NotificationEdge::TopRight | NotificationEdge::TopLeft);
        let is_right = matches!(edge, NotificationEdge::TopRight | NotificationEdge::BottomRight);
        let mut y_cursor = if is_top { screen.top() + 16.0 } else { screen.bottom() - 16.0 };

        for toast in items.iter().rev() {
            let elapsed = (now - toast.created_at).max(0.0);
            let enter_t = ease_out_cubic((elapsed as f32 / 0.22).clamp(0.0, 1.0));
            let remaining = (1.0 - (elapsed / toast.duration_secs) as f32).clamp(0.0, 1.0);

            let slide_offset = (1.0 - enter_t) * (width + 24.0);
            let x = if is_right {
                screen.right() - width - 16.0 + slide_offset
            } else {
                screen.left() + 16.0 - slide_offset
            };
            let est_h = if toast.message.is_some() { 76.0 } else { 54.0 };
            let y = if is_top { y_cursor } else { y_cursor - est_h };

            let (tone_color, icon) = match toast.tone {
                NotificationTone::Info => (tokens.primary, IconName::Info),
                NotificationTone::Success => (tokens.success, IconName::Check),
                NotificationTone::Warning => (tokens.warning, IconName::AlertTriangle),
                NotificationTone::Destructive => (tokens.destructive, IconName::AlertTriangle),
            };

            let area = Area::new(Id::new("kit_notif").with(toast.id))
                .order(Order::Tooltip)
                .fixed_pos(Pos2::new(x, y))
                .show(ctx, |ui| {
                    let frame = Frame::none()
                        .fill(tokens.card)
                        .stroke(Stroke::new(1.0, tokens.border))
                        .rounding(Rounding::same(8.0))
                        .inner_margin(Margin::same(12.0));
                    let resp = frame.show(ui, |ui| {
                        ui.set_width(width - 24.0);
                        ui.horizontal(|ui| {
                            let (badge_rect, _) = ui.allocate_exact_size(Vec2::splat(22.0), Sense::hover());
                            ui.painter().rect_filled(
                                badge_rect,
                                Rounding::same(6.0),
                                Color32::from_rgba_unmultiplied(tone_color.r(), tone_color.g(), tone_color.b(), 30),
                            );
                            paint_icon(
                                ui.painter(),
                                Rect::from_center_size(badge_rect.center(), Vec2::splat(13.0)),
                                icon,
                                tone_color,
                            );

                            ui.vertical(|ui| {
                                ui.set_width(width - 84.0);
                                ui.label(
                                    egui::RichText::new(&toast.title)
                                        .font(FontId::proportional(13.0))
                                        .strong()
                                        .color(tokens.card_foreground),
                                );
                                if let Some(msg) = &toast.message {
                                    ui.label(
                                        egui::RichText::new(msg)
                                            .font(FontId::proportional(11.5))
                                            .color(tokens.muted_foreground),
                                    );
                                }
                                if let Some(act) = &toast.action_label {
                                    ui.add_space(4.0);
                                    if ui.add(Button::new(act).variant(ButtonVariant::Outline)).clicked() {
                                        action_triggered = Some(toast.id);
                                        dismissed.push(toast.id);
                                    }
                                }
                            });

                            let (close_rect, close_resp) = ui.allocate_exact_size(Vec2::splat(20.0), Sense::click());
                            if close_resp.hovered() {
                                ui.painter().rect_filled(close_rect, Rounding::same(4.0), tokens.muted);
                            }
                            paint_icon(
                                ui.painter(),
                                Rect::from_center_size(close_rect.center(), Vec2::splat(11.0)),
                                IconName::Close,
                                tokens.muted_foreground,
                            );
                            if close_resp.clicked() {
                                dismissed.push(toast.id);
                            }
                        });

                        ui.add_space(6.0);
                        let (bar_rect, _) = ui.allocate_exact_size(Vec2::new(width - 24.0, 2.5), Sense::hover());
                        ui.painter().rect_filled(bar_rect, Rounding::same(1.5), tokens.muted);
                        let fill_rect = Rect::from_min_size(
                            bar_rect.min,
                            Vec2::new(bar_rect.width() * remaining, bar_rect.height()),
                        );
                        ui.painter().rect_filled(fill_rect, Rounding::same(1.5), tone_color);
                    });

                    let r = resp.response.rect;
                    let stripe = Rect::from_min_max(r.min, Pos2::new(r.left() + 3.0, r.bottom()));
                    ui.painter().rect_filled(stripe, Rounding::same(2.0), tone_color);
                });

            let h = area.response.rect.height().max(est_h);
            if is_top {
                y_cursor += h + 10.0;
            } else {
                y_cursor -= h + 10.0;
            }
        }

        items.retain(|n| !dismissed.contains(&n.id));
        ctx.data_mut(|d| d.insert_temp(Self::storage_id(), items));
        ctx.request_repaint();
        action_triggered
    }
}
