//! Kbd keycap widget with crisp Painter vector glyphs for modifiers and navigation keys.

use crate::theme::active_theme;
use crate::tokens::Size;
use egui::{Align2, Color32, FontId, Painter, Pos2, Rect, Response, Rounding, Sense, Stroke, Ui, Vec2, Widget};

/// Vector keycap glyph rendered via `egui::Painter` so modifier/navigation keys
/// stay crisp across platforms without depending on system fallback fonts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyGlyph {
    Command,
    Shift,
    Option,
    Control,
    Enter,
    Escape,
    Tab,
    Backspace,
    Space,
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
}

#[derive(Debug, Clone)]
struct KeyCapSpec {
    glyph: Option<KeyGlyph>,
    label: Option<String>,
}

/// Keyboard shortcut keycap widget modeled after `gpui-kit`'s `Kbd`.
pub struct Kbd {
    shortcut: String,
    size: Size,
    macos_symbols: bool,
}

impl Kbd {
    /// Creates a keycap from a shortcut string such as `"Cmd+K"` or `"Ctrl+Shift+P"`.
    pub fn new(shortcut: impl Into<String>) -> Self {
        Self {
            shortcut: shortcut.into(),
            size: Size::Sm,
            macos_symbols: cfg!(target_os = "macos"),
        }
    }

    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub fn macos_symbols(mut self, enabled: bool) -> Self {
        self.macos_symbols = enabled;
        self
    }

    fn parse_token(&self, raw: &str) -> KeyCapSpec {
        let trimmed = raw.trim();
        let lower = trimmed.to_ascii_lowercase();

        let glyph = match lower.as_str() {
            "cmd" | "command" | "super" | "meta" => Some(KeyGlyph::Command),
            "shift" => Some(KeyGlyph::Shift),
            "alt" | "opt" | "option" => Some(KeyGlyph::Option),
            "ctrl" | "control" => Some(KeyGlyph::Control),
            "enter" | "return" => Some(KeyGlyph::Enter),
            "esc" | "escape" => Some(KeyGlyph::Escape),
            "tab" => Some(KeyGlyph::Tab),
            "backspace" | "back" | "delete" | "del" => Some(KeyGlyph::Backspace),
            "space" | "spacebar" => Some(KeyGlyph::Space),
            "up" | "arrowup" => Some(KeyGlyph::ArrowUp),
            "down" | "arrowdown" => Some(KeyGlyph::ArrowDown),
            "left" | "arrowleft" => Some(KeyGlyph::ArrowLeft),
            "right" | "arrowright" => Some(KeyGlyph::ArrowRight),
            _ => None,
        };

        if self.macos_symbols {
            if let Some(g) = glyph {
                return KeyCapSpec {
                    glyph: Some(g),
                    label: None,
                };
            }
            return KeyCapSpec {
                glyph: None,
                label: Some(trimmed.to_ascii_uppercase()),
            };
        }

        // Standard Linux / Windows mode: pair recognizable icons with concise key names.
        let standard_label = match lower.as_str() {
            "cmd" | "command" | "super" | "meta" => "Super".to_string(),
            "shift" => "Shift".to_string(),
            "alt" | "opt" | "option" => "Alt".to_string(),
            "ctrl" | "control" => "Ctrl".to_string(),
            "enter" | "return" => "Enter".to_string(),
            "esc" | "escape" => "Esc".to_string(),
            "tab" => "Tab".to_string(),
            "backspace" | "back" => "Back".to_string(),
            "delete" | "del" => "Del".to_string(),
            "space" | "spacebar" => "Space".to_string(),
            "up" | "arrowup" | "down" | "arrowdown" | "left" | "arrowleft" | "right" | "arrowright" => {
                return KeyCapSpec { glyph, label: None };
            }
            _ => trimmed.to_string(),
        };

        KeyCapSpec {
            glyph,
            label: Some(standard_label),
        }
    }
}

impl Widget for Kbd {
    fn ui(self, ui: &mut Ui) -> Response {
        let tokens = active_theme(ui.ctx());
        let specs: Vec<KeyCapSpec> = self
            .shortcut
            .split('+')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(|part| self.parse_token(part))
            .collect();

        let font_size = (self.size.font_size() - 1.5).max(10.0);
        let font_id = FontId::monospace(font_size);
        let height = (self.size.height() - 6.0).max(20.0);
        let icon_edge = (height * 0.54).clamp(10.0, 16.0);
        let pad_x = (height * 0.28).max(6.0);
        let inner_gap = 4.0;

        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 5.0;
            for spec in specs {
                let text_galley = spec
                    .label
                    .as_ref()
                    .map(|text| ui.painter().layout_no_wrap(text.clone(), font_id.clone(), tokens.foreground));

                let content_w = match (spec.glyph.is_some(), &text_galley) {
                    (true, Some(g)) => icon_edge + inner_gap + g.size().x,
                    (true, None) => icon_edge,
                    (false, Some(g)) => g.size().x,
                    (false, None) => 0.0,
                };
                let width = (content_w + pad_x * 2.0).max(height);

                let (rect, response) = ui.allocate_exact_size(Vec2::new(width, height), Sense::hover());

                if ui.is_rect_visible(rect) {
                    let painter = ui.painter();
                    let rounding = Rounding::same(5.0);

                    // Tactile bottom keycap bezel lip.
                    let lip_rect = rect.translate(Vec2::new(0.0, 1.5));
                    painter.rect_filled(lip_rect, rounding, tokens.border);

                    let cap_fill = if response.hovered() {
                        tokens.accent
                    } else {
                        tokens.muted
                    };
                    painter.rect(rect, rounding, cap_fill, Stroke::new(1.0, tokens.border));

                    let content_left = rect.center().x - content_w * 0.5;
                    let mut cursor_x = content_left;

                    if let Some(glyph) = spec.glyph {
                        let icon_rect = Rect::from_center_size(
                            Pos2::new(cursor_x + icon_edge * 0.5, rect.center().y),
                            Vec2::splat(icon_edge),
                        );
                        paint_key_glyph(painter, icon_rect, glyph, tokens.foreground);
                        cursor_x += icon_edge + inner_gap;
                    }

                    if let Some(galley) = text_galley {
                        let text_pos = Pos2::new(cursor_x, rect.center().y - galley.size().y * 0.5);
                        painter.galley(text_pos, galley, tokens.foreground);
                    }
                }
            }
        })
        .response
    }
}

/// Paints a crisp normalized vector glyph for modifier and navigation keys.
pub fn paint_key_glyph(painter: &Painter, rect: Rect, glyph: KeyGlyph, color: Color32) {
    let c = rect.center();
    let r = rect.width().min(rect.height()) * 0.42;
    let stroke = Stroke::new((rect.width() * 0.115).clamp(1.2, 1.9), color);

    match glyph {
        KeyGlyph::Command => {
            // Classic cloverleaf (⌘): center frame + 4 corner loops.
            let inner = r * 0.42;
            let loop_r = r * 0.36;
            painter.rect_stroke(
                Rect::from_center_size(c, Vec2::splat(inner * 2.0)),
                0.0,
                stroke,
            );
            let offsets = [
                Vec2::new(-inner - loop_r * 0.85, -inner - loop_r * 0.85),
                Vec2::new(inner + loop_r * 0.85, -inner - loop_r * 0.85),
                Vec2::new(-inner - loop_r * 0.85, inner + loop_r * 0.85),
                Vec2::new(inner + loop_r * 0.85, inner + loop_r * 0.85),
            ];
            for off in offsets {
                painter.rect_stroke(
                    Rect::from_center_size(c + off, Vec2::splat(loop_r * 1.7)),
                    loop_r * 0.85,
                    stroke,
                );
            }
        }
        KeyGlyph::Shift => {
            let pts = [
                c + Vec2::new(0.0, -r * 0.9),
                c + Vec2::new(-r * 0.85, r * 0.05),
                c + Vec2::new(-r * 0.35, r * 0.05),
                c + Vec2::new(-r * 0.35, r * 0.85),
                c + Vec2::new(r * 0.35, r * 0.85),
                c + Vec2::new(r * 0.35, r * 0.05),
                c + Vec2::new(r * 0.85, r * 0.05),
                c + Vec2::new(0.0, -r * 0.9),
            ];
            for pair in pts.windows(2) {
                painter.line_segment([pair[0], pair[1]], stroke);
            }
        }
        KeyGlyph::Option => {
            // Stepped diagonal rail + upper branch bar (⌥).
            painter.line_segment([c + Vec2::new(-r * 0.9, -r * 0.55), c + Vec2::new(-r * 0.25, -r * 0.55)], stroke);
            painter.line_segment([c + Vec2::new(-r * 0.25, -r * 0.55), c + Vec2::new(r * 0.35, r * 0.55)], stroke);
            painter.line_segment([c + Vec2::new(r * 0.35, r * 0.55), c + Vec2::new(r * 0.9, r * 0.55)], stroke);
            painter.line_segment([c + Vec2::new(r * 0.15, -r * 0.55), c + Vec2::new(r * 0.9, -r * 0.55)], stroke);
        }
        KeyGlyph::Control => {
            painter.line_segment([c + Vec2::new(-r * 0.75, r * 0.25), c + Vec2::new(0.0, -r * 0.55)], stroke);
            painter.line_segment([c + Vec2::new(0.0, -r * 0.55), c + Vec2::new(r * 0.75, r * 0.25)], stroke);
        }
        KeyGlyph::Enter => {
            painter.line_segment([c + Vec2::new(r * 0.75, -r * 0.65), c + Vec2::new(r * 0.75, r * 0.2)], stroke);
            painter.line_segment([c + Vec2::new(r * 0.75, r * 0.2), c + Vec2::new(-r * 0.7, r * 0.2)], stroke);
            painter.line_segment([c + Vec2::new(-r * 0.25, -r * 0.25), c + Vec2::new(-r * 0.7, r * 0.2)], stroke);
            painter.line_segment([c + Vec2::new(-r * 0.7, r * 0.2), c + Vec2::new(-r * 0.25, r * 0.65)], stroke);
        }
        KeyGlyph::Escape => {
            // Open escape ring + outward northeast arrow (⎋).
            painter.circle_stroke(c, r * 0.68, Stroke::new(stroke.width * 0.9, color));
            painter.line_segment([c + Vec2::new(0.0, 0.0), c + Vec2::new(r * 0.9, -r * 0.9)], stroke);
            painter.line_segment([c + Vec2::new(r * 0.35, -r * 0.9), c + Vec2::new(r * 0.9, -r * 0.9)], stroke);
            painter.line_segment([c + Vec2::new(r * 0.9, -r * 0.35), c + Vec2::new(r * 0.9, -r * 0.9)], stroke);
        }
        KeyGlyph::Tab => {
            painter.line_segment([c + Vec2::new(-r * 0.85, 0.0), c + Vec2::new(r * 0.45, 0.0)], stroke);
            painter.line_segment([c + Vec2::new(0.0, -r * 0.45), c + Vec2::new(r * 0.45, 0.0)], stroke);
            painter.line_segment([c + Vec2::new(r * 0.45, 0.0), c + Vec2::new(0.0, r * 0.45)], stroke);
            painter.line_segment([c + Vec2::new(r * 0.8, -r * 0.65), c + Vec2::new(r * 0.8, r * 0.65)], stroke);
        }
        KeyGlyph::Backspace => {
            let pts = [
                c + Vec2::new(-r * 0.95, 0.0),
                c + Vec2::new(-r * 0.35, -r * 0.65),
                c + Vec2::new(r * 0.85, -r * 0.65),
                c + Vec2::new(r * 0.85, r * 0.65),
                c + Vec2::new(-r * 0.35, r * 0.65),
                c + Vec2::new(-r * 0.95, 0.0),
            ];
            for pair in pts.windows(2) {
                painter.line_segment([pair[0], pair[1]], stroke);
            }
            let x_r = r * 0.28;
            let xc = c + Vec2::new(r * 0.25, 0.0);
            painter.line_segment([xc + Vec2::new(-x_r, -x_r), xc + Vec2::new(x_r, x_r)], stroke);
            painter.line_segment([xc + Vec2::new(x_r, -x_r), xc + Vec2::new(-x_r, x_r)], stroke);
        }
        KeyGlyph::Space => {
            painter.line_segment([c + Vec2::new(-r * 0.85, 0.0), c + Vec2::new(-r * 0.85, r * 0.45)], stroke);
            painter.line_segment([c + Vec2::new(-r * 0.85, r * 0.45), c + Vec2::new(r * 0.85, r * 0.45)], stroke);
            painter.line_segment([c + Vec2::new(r * 0.85, r * 0.45), c + Vec2::new(r * 0.85, 0.0)], stroke);
        }
        KeyGlyph::ArrowUp => {
            painter.line_segment([c + Vec2::new(0.0, r * 0.75), c + Vec2::new(0.0, -r * 0.75)], stroke);
            painter.line_segment([c + Vec2::new(-r * 0.5, -r * 0.25), c + Vec2::new(0.0, -r * 0.75)], stroke);
            painter.line_segment([c + Vec2::new(r * 0.5, -r * 0.25), c + Vec2::new(0.0, -r * 0.75)], stroke);
        }
        KeyGlyph::ArrowDown => {
            painter.line_segment([c + Vec2::new(0.0, -r * 0.75), c + Vec2::new(0.0, r * 0.75)], stroke);
            painter.line_segment([c + Vec2::new(-r * 0.5, r * 0.25), c + Vec2::new(0.0, r * 0.75)], stroke);
            painter.line_segment([c + Vec2::new(r * 0.5, r * 0.25), c + Vec2::new(0.0, r * 0.75)], stroke);
        }
        KeyGlyph::ArrowLeft => {
            painter.line_segment([c + Vec2::new(r * 0.75, 0.0), c + Vec2::new(-r * 0.75, 0.0)], stroke);
            painter.line_segment([c + Vec2::new(-r * 0.25, -r * 0.5), c + Vec2::new(-r * 0.75, 0.0)], stroke);
            painter.line_segment([c + Vec2::new(-r * 0.25, r * 0.5), c + Vec2::new(-r * 0.75, 0.0)], stroke);
        }
        KeyGlyph::ArrowRight => {
            painter.line_segment([c + Vec2::new(-r * 0.75, 0.0), c + Vec2::new(r * 0.75, 0.0)], stroke);
            painter.line_segment([c + Vec2::new(r * 0.25, -r * 0.5), c + Vec2::new(r * 0.75, 0.0)], stroke);
            painter.line_segment([c + Vec2::new(r * 0.25, r * 0.5), c + Vec2::new(r * 0.75, 0.0)], stroke);
        }
    }
}
