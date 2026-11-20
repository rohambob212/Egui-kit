//! Inline semantic Alert banner with optional dismiss transition.

use crate::components::icon::{paint_icon, IconName};
use crate::theme::active_theme;
use crate::tokens::{ease_out_cubic, lerp_color};
use egui::{Color32, FontId, Id, Pos2, Rect, Response, Sense, Stroke, Ui, Vec2, Widget};

/// Visual tone for [`Alert`] banners.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AlertTone {
    #[default]
    Info,
    Success,
    Warning,
    Destructive,
}

/// Inline status banner with semantic icon, title, message, and optional dismiss control.
pub struct Alert {
    tone: AlertTone,
    title: String,
    description: Option<String>,
    dismissible: bool,
    width: Option<f32>,
}

impl Alert {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            tone: AlertTone::Info,
            title: title.into(),
            description: None,
            dismissible: false,
            width: None,
        }
    }

    pub fn info(title: impl Into<String>, description: impl Into<String>) -> Self {
        Self::new(title).tone(AlertTone::Info).description(description)
    }

    pub fn success(title: impl Into<String>, description: impl Into<String>) -> Self {
        Self::new(title).tone(AlertTone::Success).description(description)
    }

    pub fn warning(title: impl Into<String>, description: impl Into<String>) -> Self {
        Self::new(title).tone(AlertTone::Warning).description(description)
    }

    pub fn destructive(title: impl Into<String>, description: impl Into<String>) -> Self {
        Self::new(title).tone(AlertTone::Destructive).description(description)
    }

    pub fn tone(mut self, tone: AlertTone) -> Self {
        self.tone = tone;
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn dismissible(mut self, dismissible: bool) -> Self {
        self.dismissible = dismissible;
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width.max(180.0));
        self
    }

    /// Renders the alert bound directly to `&mut bool` visibility with animated collapse.
    pub fn show_dismissible(mut self, ui: &mut Ui, id_source: &str, visible: &mut bool) -> Option<Response> {
        self.dismissible = true;
        let anim_id = Id::new(("egui_kit_alert_vis", id_source));
        let vis_t = ease_out_cubic(ui.ctx().animate_bool_with_time(anim_id, *visible, 0.16));
        if vis_t <= 0.01 {
            return None;
        }
        let resp = ui
            .scope(|ui| {
                ui.set_opacity(vis_t);
                render_alert(ui, self, Some(visible))
            })
            .inner;
        Some(resp)
    }
}

impl Widget for Alert {
    fn ui(self, ui: &mut Ui) -> Response {
        render_alert(ui, self, None)
    }
}

fn render_alert(ui: &mut Ui, alert: Alert, mut visible: Option<&mut bool>) -> Response {
    let tokens = active_theme(ui.ctx());
    let (accent, icon_name) = match alert.tone {
        AlertTone::Info => (tokens.primary, IconName::Info),
        AlertTone::Success => (tokens.success, IconName::Check),
        AlertTone::Warning => (tokens.warning, IconName::AlertTriangle),
        AlertTone::Destructive => (tokens.destructive, IconName::AlertTriangle),
    };

    let width = alert.width.unwrap_or_else(|| ui.available_width().clamp(220.0, 480.0));
    let pad_x = 14.0;
    let pad_y = 12.0;
    let icon_col = 24.0;
    let close_col = if alert.dismissible { 24.0 } else { 0.0 };
    let text_w = (width - pad_x * 2.0 - icon_col - close_col).max(100.0);

    let title_galley = ui.painter().layout(
        alert.title,
        FontId::proportional(13.5),
        tokens.foreground,
        text_w,
    );
    let desc_galley = alert.description.map(|desc| {
        ui.painter().layout(
            desc,
            FontId::proportional(12.5),
            tokens.muted_foreground,
            text_w,
        )
    });

    let text_h = title_galley.size().y
        + desc_galley
            .as_ref()
            .map(|g| 4.0 + g.size().y)
            .unwrap_or(0.0);
    let total_h = (text_h + pad_y * 2.0).max(46.0);

    let (rect, response) = ui.allocate_exact_size(Vec2::new(width, total_h), Sense::hover());

    if ui.is_rect_visible(rect) {
        let bg = lerp_color(tokens.card, accent, 0.09);
        let border = lerp_color(tokens.border, accent, 0.45);
        let rounding = egui::Rounding::same(8.0);
        ui.painter().rect(rect, rounding, bg, Stroke::new(1.0, border));

        // Left semantic accent stripe
        let stripe = Rect::from_min_max(
            rect.min + Vec2::new(1.0, 6.0),
            Pos2::new(rect.left() + 4.0, rect.bottom() - 6.0),
        );
        ui.painter().rect_filled(stripe, egui::Rounding::same(2.0), accent);

        let icon_rect = Rect::from_min_size(
            Pos2::new(rect.left() + pad_x, rect.top() + pad_y + 1.0),
            Vec2::splat(16.0),
        );
        paint_icon(ui.painter(), icon_rect, icon_name, accent);

        let text_x = rect.left() + pad_x + icon_col;
        let mut cur_y = rect.top() + pad_y;
        ui.painter().galley(Pos2::new(text_x, cur_y), title_galley.clone(), tokens.foreground);
        if let Some(desc) = desc_galley {
            cur_y += title_galley.size().y + 4.0;
            ui.painter().galley(Pos2::new(text_x, cur_y), desc, tokens.muted_foreground);
        }
    }

    if alert.dismissible {
        let close_rect = Rect::from_center_size(
            Pos2::new(rect.right() - pad_x - 8.0, rect.top() + pad_y + 8.0),
            Vec2::splat(20.0),
        );
        let close_resp = ui.interact(close_rect, response.id.with("dismiss"), Sense::click());
        let hov_t = ui.ctx().animate_bool_with_time(close_resp.id.with("hov"), close_resp.hovered(), 0.12);
        if close_resp.hovered() {
            ui.painter().rect_filled(
                close_rect,
                egui::Rounding::same(4.0),
                Color32::from_rgba_unmultiplied(tokens.foreground.r(), tokens.foreground.g(), tokens.foreground.b(), (22.0 * hov_t) as u8),
            );
        }
        paint_icon(
            ui.painter(),
            close_rect.shrink(4.0),
            IconName::Close,
            lerp_color(tokens.muted_foreground, tokens.foreground, hov_t),
        );
        if close_resp.clicked() {
            if let Some(ref mut vis) = visible {
                **vis = false;
            }
        }
    }

    response
}
