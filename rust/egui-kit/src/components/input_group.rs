//! Composite InputGroup container joining prefix/suffix addons cleanly inside a single focusable border.

use crate::components::icon::{paint_icon, IconName};
use crate::theme::active_theme;
use crate::tokens::{ease_out_cubic, lerp_color, Size};
use egui::{FontId, Pos2, Rect, Response, Stroke, Ui, Vec2};

/// Composite horizontal group combining prefix/suffix addons with an embedded input control.
pub struct InputGroup {
    prefix: Option<String>,
    suffix: Option<String>,
    prefix_icon: Option<IconName>,
    size: Size,
    error: bool,
    disabled: bool,
    width: f32,
}

impl InputGroup {
    pub fn new() -> Self {
        Self {
            prefix: None,
            suffix: None,
            prefix_icon: None,
            size: Size::Md,
            error: false,
            disabled: false,
            width: 320.0,
        }
    }

    pub fn prefix(mut self, prefix: impl Into<String>) -> Self {
        self.prefix = Some(prefix.into());
        self
    }

    pub fn suffix(mut self, suffix: impl Into<String>) -> Self {
        self.suffix = Some(suffix.into());
        self
    }

    pub fn prefix_icon(mut self, icon: IconName) -> Self {
        self.prefix_icon = Some(icon);
        self
    }

    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub fn error(mut self, error: bool) -> Self {
        self.error = error;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width.max(140.0);
        self
    }

    /// Renders the unified group frame and invokes `add_input` in the center slot.
    pub fn show(self, ui: &mut Ui, add_input: impl FnOnce(&mut Ui) -> Response) -> Response {
        let tokens = active_theme(ui.ctx());
        let h = self.size.height();
        let (outer_rect, mut resp) = ui.allocate_exact_size(Vec2::new(self.width, h), egui::Sense::hover());

        let base_id = resp.id;
        let has_focus = ui.ctx().data(|d| d.get_temp::<bool>(base_id.with("focused")).unwrap_or(false));
        let focus_t = ease_out_cubic(ui.ctx().animate_bool_with_time(
            base_id.with("focus_anim"),
            has_focus && !self.disabled,
            0.14,
        ));
        let err_t = ease_out_cubic(ui.ctx().animate_bool_with_time(
            base_id.with("err_anim"),
            self.error,
            0.14,
        ));

        let font = FontId::proportional(self.size.font_size());
        let mono_font = FontId::monospace((self.size.font_size() - 1.0).max(10.5));
        let pad_x = self.size.padding_x() * 0.75;
        let icon_w = self.size.icon_size();

        let prefix_galley = self
            .prefix
            .as_ref()
            .map(|p| ui.painter().layout_no_wrap(p.clone(), mono_font.clone(), tokens.muted_foreground));
        let suffix_galley = self
            .suffix
            .as_ref()
            .map(|s| ui.painter().layout_no_wrap(s.clone(), mono_font, tokens.muted_foreground));

        let prefix_w = match (&self.prefix_icon, &prefix_galley) {
            (Some(_), Some(g)) => pad_x * 2.0 + icon_w + 6.0 + g.size().x,
            (Some(_), None) => pad_x * 2.0 + icon_w,
            (None, Some(g)) => pad_x * 2.0 + g.size().x,
            (None, None) => 0.0,
        };
        let suffix_w = suffix_galley.as_ref().map_or(0.0, |g| pad_x * 2.0 + g.size().x);

        let bg = if self.disabled { tokens.muted } else { tokens.background };
        let border_col = lerp_color(lerp_color(tokens.input, tokens.ring, focus_t), tokens.destructive, err_t);

        if ui.is_rect_visible(outer_rect) {
            let painter = ui.painter();
            if focus_t > 0.02 {
                let halo = lerp_color(tokens.ring, tokens.destructive, err_t).linear_multiply(0.22 * focus_t);
                painter.rect_stroke(outer_rect.expand(1.5 * focus_t), self.size.rounding(), Stroke::new(2.5 * focus_t, halo));
            }
            painter.rect(outer_rect, self.size.rounding(), bg, Stroke::new(1.0 + 0.5 * focus_t, border_col));

            if prefix_w > 0.0 {
                let pre_rect = Rect::from_min_max(outer_rect.min, Pos2::new(outer_rect.left() + prefix_w, outer_rect.bottom()));
                painter.rect_filled(pre_rect.shrink(1.0), self.size.rounding(), tokens.muted);
                painter.line_segment([Pos2::new(pre_rect.right(), outer_rect.top()), Pos2::new(pre_rect.right(), outer_rect.bottom())], Stroke::new(1.0, tokens.border));

                let mut x = pre_rect.left() + pad_x;
                if let Some(icon) = self.prefix_icon {
                    paint_icon(
                        painter,
                        Rect::from_center_size(Pos2::new(x + icon_w * 0.5, pre_rect.center().y), Vec2::splat(icon_w)),
                        icon,
                        tokens.muted_foreground,
                    );
                    x += icon_w + 6.0;
                }
                if let Some(g) = prefix_galley {
                    painter.galley(Pos2::new(x, pre_rect.center().y - g.size().y * 0.5), g, tokens.muted_foreground);
                }
            }

            if suffix_w > 0.0 {
                let suf_rect = Rect::from_min_max(Pos2::new(outer_rect.right() - suffix_w, outer_rect.top()), outer_rect.max);
                painter.rect_filled(suf_rect.shrink(1.0), self.size.rounding(), tokens.muted);
                painter.line_segment([Pos2::new(suf_rect.left(), outer_rect.top()), Pos2::new(suf_rect.left(), outer_rect.bottom())], Stroke::new(1.0, tokens.border));
                if let Some(g) = suffix_galley {
                    painter.galley(Pos2::new(suf_rect.left() + pad_x, suf_rect.center().y - g.size().y * 0.5), g, tokens.muted_foreground);
                }
            }
        }

        let center_rect = Rect::from_min_max(
            Pos2::new(outer_rect.left() + prefix_w + pad_x * 0.6, outer_rect.top() + 2.0),
            Pos2::new((outer_rect.right() - suffix_w - pad_x * 0.6).max(outer_rect.left() + prefix_w + 32.0), outer_rect.bottom() - 2.0),
        );
        let mut child_ui = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(center_rect)
                .layout(egui::Layout::left_to_right(egui::Align::Center)),
        );
        child_ui.style_mut().override_font_id = Some(font);
        child_ui.set_enabled(!self.disabled);

        let inner_resp = add_input(&mut child_ui);
        ui.ctx().data_mut(|d| d.insert_temp(base_id.with("focused"), inner_resp.has_focus()));
        resp = resp.union(inner_resp);
        resp
    }
}

impl Default for InputGroup {
    fn default() -> Self {
        Self::new()
    }
}
