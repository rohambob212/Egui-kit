//! Root window coordinator managing the primary panel, toast queue, and modal overlay layer.

use crate::theme::active_theme;
use crate::tokens::{ease_out_cubic, Size};
use egui::{Align2, Area, CentralPanel, Context, Frame, Id, Order, RichText, Rounding, Stroke, Ui};

/// Semantic tone for toast notifications hosted by [`Root`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToastTone {
    Info,
    Success,
    Warning,
    Destructive,
}

/// Toast entry rendered in the [`Root`] overlay stack.
#[derive(Debug, Clone)]
pub struct ToastItem {
    pub id: u64,
    pub title: String,
    pub message: Option<String>,
    pub tone: ToastTone,
}

#[derive(Debug, Clone, Default)]
struct RootOverlayState {
    toasts: Vec<ToastItem>,
    next_id: u64,
    modal_open: bool,
    modal_title: String,
    modal_body: String,
}

const ROOT_STATE_KEY: &str = "egui_kit_root_state";

/// Application-level container wrapping `CentralPanel` and rendering overlay portals
/// (toasts, popovers, modal scrims) in proper z-order.
pub struct Root;

impl Root {
    /// Pushes a toast notification into the global `Root` overlay stack.
    pub fn push_toast(
        ctx: &Context,
        title: impl Into<String>,
        message: Option<String>,
        tone: ToastTone,
    ) {
        ctx.data_mut(|d| {
            let mut state = d
                .get_temp::<RootOverlayState>(Id::new(ROOT_STATE_KEY))
                .unwrap_or_default();
            state.next_id += 1;
            let id = state.next_id;
            state.toasts.push(ToastItem {
                id,
                title: title.into(),
                message,
                tone,
            });
            d.insert_temp(Id::new(ROOT_STATE_KEY), state);
        });
    }

    /// Opens a centered overlay dialog hosted by [`Root`].
    pub fn open_modal(ctx: &Context, title: impl Into<String>, body: impl Into<String>) {
        ctx.data_mut(|d| {
            let mut state = d
                .get_temp::<RootOverlayState>(Id::new(ROOT_STATE_KEY))
                .unwrap_or_default();
            state.modal_open = true;
            state.modal_title = title.into();
            state.modal_body = body.into();
            d.insert_temp(Id::new(ROOT_STATE_KEY), state);
        });
    }

    /// Renders the root central panel and flushes active overlay layers above it.
    pub fn show(ctx: &Context, add_contents: impl FnOnce(&mut Ui)) {
        crate::init(ctx);
        let tokens = active_theme(ctx);

        CentralPanel::default()
            .frame(
                Frame::none()
                    .fill(tokens.background)
                    .inner_margin(egui::Margin::same(16.0)),
            )
            .show(ctx, add_contents);

        Self::render_overlays(ctx);
    }

    fn render_overlays(ctx: &Context) {
        let tokens = active_theme(ctx);
        let mut state = ctx
            .data(|d| d.get_temp::<RootOverlayState>(Id::new(ROOT_STATE_KEY)))
            .unwrap_or_default();

        let mut dismissed_id: Option<u64> = None;

        if !state.toasts.is_empty() {
            Area::new(Id::new("egui_kit_root_toasts"))
                .order(Order::Foreground)
                .anchor(Align2::RIGHT_BOTTOM, [-16.0, -16.0])
                .show(ctx, |ui| {
                    ui.set_max_width(320.0);
                    for toast in &state.toasts {
                        let raw_t = ctx.animate_bool_with_time(Id::new(("egui_kit_toast", toast.id)), true, 0.2);
                        let t = ease_out_cubic(raw_t);
                        let slide_x = (1.0 - t) * 24.0;
                        let accent = match toast.tone {
                            ToastTone::Info => tokens.primary,
                            ToastTone::Success => tokens.success,
                            ToastTone::Warning => tokens.warning,
                            ToastTone::Destructive => tokens.destructive,
                        };
                        ui.horizontal(|ui| {
                            ui.add_space(slide_x);
                            Frame::none()
                                .fill(tokens.card.linear_multiply(0.35 + 0.65 * t))
                                .stroke(Stroke::new(1.0, tokens.border.linear_multiply(t)))
                                .rounding(Size::Md.rounding())
                                .inner_margin(egui::Margin::same(12.0))
                                .show(ui, |ui| {
                                    ui.set_opacity(t);
                                    ui.horizontal(|ui| {
                                        let (dot_rect, _) =
                                            ui.allocate_exact_size(egui::vec2(8.0, 8.0), egui::Sense::hover());
                                        ui.painter().circle_filled(dot_rect.center(), 4.0, accent);
                                        ui.vertical(|ui| {
                                            ui.label(
                                                RichText::new(&toast.title)
                                                    .strong()
                                                    .color(tokens.card_foreground),
                                            );
                                            if let Some(msg) = &toast.message {
                                                ui.label(
                                                    RichText::new(msg)
                                                        .small()
                                                        .color(tokens.muted_foreground),
                                                );
                                            }
                                        });
                                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                                            if ui.small_button("×").clicked() {
                                                dismissed_id = Some(toast.id);
                                            }
                                        });
                                    });
                                });
                        });
                        ui.add_space(6.0);
                    }
                });
        }

        if let Some(id) = dismissed_id {
            state.toasts.retain(|t| t.id != id);
            ctx.data_mut(|d| d.insert_temp(Id::new(ROOT_STATE_KEY), state.clone()));
        }

        let modal_t = ease_out_cubic(ctx.animate_bool_with_time(
            Id::new("egui_kit_modal_anim"),
            state.modal_open,
            0.18,
        ));
        if modal_t > 0.01 {
            let screen = ctx.screen_rect();
            let scrim_alpha = (120.0 * modal_t) as u8;
            Area::new(Id::new("egui_kit_root_modal_scrim"))
                .order(Order::Middle)
                .fixed_pos(screen.min)
                .show(ctx, |ui| {
                    let sense = if state.modal_open { egui::Sense::click() } else { egui::Sense::hover() };
                    let (_, resp) = ui.allocate_exact_size(screen.size(), sense);
                    ui.painter().rect_filled(
                        screen,
                        Rounding::ZERO,
                        egui::Color32::from_black_alpha(scrim_alpha),
                    );
                    if resp.clicked() && state.modal_open {
                        state.modal_open = false;
                    }
                });

            let lift_y = (1.0 - modal_t) * 14.0;
            Area::new(Id::new("egui_kit_root_modal"))
                .order(Order::Foreground)
                .anchor(Align2::CENTER_CENTER, [0.0, lift_y])
                .show(ctx, |ui| {
                    ui.set_opacity(modal_t);
                    Frame::none()
                        .fill(tokens.card)
                        .stroke(Stroke::new(1.0, tokens.border))
                        .rounding(Rounding::same(10.0))
                        .inner_margin(egui::Margin::same(20.0))
                        .show(ui, |ui| {
                            ui.set_width(360.0);
                            ui.label(
                                RichText::new(&state.modal_title)
                                    .size(16.0)
                                    .strong()
                                    .color(tokens.card_foreground),
                            );
                            ui.add_space(6.0);
                            ui.label(
                                RichText::new(&state.modal_body).color(tokens.muted_foreground),
                            );
                            ui.add_space(14.0);
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui
                                    .add(crate::components::Button::new("Close"))
                                    .clicked()
                                {
                                    state.modal_open = false;
                                }
                            });
                        });
                });

            ctx.data_mut(|d| d.insert_temp(Id::new(ROOT_STATE_KEY), state));
        }
    }
}
