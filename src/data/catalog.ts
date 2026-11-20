import type { PartNumber, WidgetCategory, WidgetSpec } from '../types/kit';

function plannedWidget(
  slug: string,
  name: string,
  category: WidgetCategory,
  part: PartNumber,
  summary: string,
  eguiSignature: string,
  gpuiMappingNote: string,
  gpuiName = name
): WidgetSpec {
  const moduleName = slug.replace(/-/g, '_');
  return {
    slug,
    name,
    gpuiName,
    category,
    part,
    status: 'planned',
    summary,
    eguiSignature,
    gpuiMappingNote,
    rustFilePath: `src/components/${moduleName}.rs`,
    usageRust: `use egui_kit::prelude::*;\n\n// Scheduled for Part ${part}\nui.add(${eguiSignature});`,
    props: [
      {
        name: 'size',
        rustType: 'Size',
        defaultValue: 'Size::Md',
        description: 'Controls component height, padding, and font scale.',
      },
      {
        name: 'disabled',
        rustType: 'bool',
        defaultValue: 'false',
        description: 'Disables pointer/keyboard interaction and dims surface contrast.',
      },
    ],
  };
}

export const WIDGET_CATALOG: WidgetSpec[] = [
  // =========================================================================
  // PART 1 — Core Foundation, Theme Engine & Essential Primitives (10)
  // =========================================================================
  {
    slug: 'root',
    name: 'Root',
    gpuiName: 'Root',
    category: 'Layout',
    part: 1,
    status: 'ready',
    summary:
      'Application-level panel wrapper that initializes egui-kit tokens and renders global toast queues and modal overlay portals in z-order.',
    eguiSignature: 'Root::show(ctx, |ui| { /* app content */ })',
    gpuiMappingNote:
      'Replaces GPUI’s Root window wrapper view with CentralPanel + foreground egui::Area overlay portals stored in egui::Context memory.',
    rustFilePath: 'src/components/root.rs',
    usageRust: `use egui_kit::components::{Button, Root, ToastTone};

Root::show(ctx, |ui| {
    if ui.add(Button::new("Trigger Toast")).clicked() {
        Root::push_toast(
            ctx,
            "Workspace saved",
            Some("All buffers flushed cleanly.".into()),
            ToastTone::Success,
        );
    }

    if ui.add(Button::new("Open Modal Overlay")).clicked() {
        Root::open_modal(
            ctx,
            "Confirm Deployment",
            "Target crate egui-kit v0.1.0 is ready.",
        );
    }
});`,
    props: [
      { name: 'ctx', rustType: '&egui::Context', defaultValue: 'required', description: 'Active egui context hosting overlay memory and central panel.' },
      { name: 'add_contents', rustType: 'impl FnOnce(&mut Ui)', defaultValue: 'required', description: 'Primary application UI closure rendered inside CentralPanel.' },
      { name: 'push_toast', rustType: '(ctx, title, msg, ToastTone)', defaultValue: 'static method', description: 'Enqueues a dismissible toast card anchored bottom-right.' },
      { name: 'open_modal', rustType: '(ctx, title, body)', defaultValue: 'static method', description: 'Activates the Root-hosted centered modal dialog and scrim.' },
    ],
  },
  {
    slug: 'theme',
    name: 'Theme',
    gpuiName: 'Theme / ThemeRegistry',
    category: 'Layout',
    part: 1,
    status: 'ready',
    summary:
      'Runtime theme engine and ThemePicker selector synchronizing 36 gpui-kit color palettes directly into egui::Visuals and widget tokens.',
    eguiSignature: 'ui.add(ThemePicker::new("app_theme"))',
    gpuiMappingNote:
      'Maps GPUI’s Theme global entity to egui::Context temp memory via set_theme(ctx, ThemeId) and apply_to_egui(ctx).',
    rustFilePath: 'src/components/theme.rs',
    usageRust: `use egui_kit::{set_theme, ThemeId};
use egui_kit::components::ThemePicker;

// Apply programmatically on startup
set_theme(ctx, ThemeId::TokyoNight);

// Or render an interactive picker widget with live color swatches
ui.add(
    ThemePicker::new("header_theme")
        .size(Size::Md)
        .show_swatches(true),
);`,
    props: [
      { name: 'id_source', rustType: '&str', defaultValue: 'required', description: 'Unique salt for ComboBox popup state in egui::Memory.' },
      { name: 'size', rustType: 'Size', defaultValue: 'Size::Md', description: 'Font and trigger scale for the selector.' },
      { name: 'show_swatches', rustType: 'bool', defaultValue: 'true', description: 'Draws background + primary accent preview dots per preset.' },
    ],
  },
  {
    slug: 'button',
    name: 'Button',
    gpuiName: 'Button',
    category: 'Basic',
    part: 1,
    status: 'ready',
    summary:
      'Interactive action button supporting 6 semantic variants, 4 size steps, leading/trailing vector icons, focus rings, and animated loading spinners.',
    eguiSignature: 'Button::new("Save").variant(ButtonVariant::Primary).size(Size::Md)',
    gpuiMappingNote:
      'Translates GPUI Button builder + ClickListener into egui::Widget returning Response with custom Painter geometry and request_repaint() spinner.',
    rustFilePath: 'src/components/button.rs',
    usageRust: `use egui_kit::prelude::*;

ui.horizontal(|ui| {
    if ui.add(Button::new("Deploy").variant(ButtonVariant::Primary).icon(IconName::Sparkles)).clicked() {
        // primary click
    }
    ui.add(Button::new("Cancel").variant(ButtonVariant::Outline));
    ui.add(Button::new("Delete").variant(ButtonVariant::Destructive));
    ui.add(Button::new("Saving").loading(true));
});`,
    props: [
      { name: 'label', rustType: 'impl Into<String>', defaultValue: 'required', description: 'Button caption text.' },
      { name: 'variant', rustType: 'ButtonVariant', defaultValue: 'Primary', description: 'Primary, Secondary, Outline, Ghost, Destructive, or Link.' },
      { name: 'size', rustType: 'Size', defaultValue: 'Size::Md', description: 'Xs (24px), Sm (28px), Md (34px), or Lg (40px).' },
      { name: 'icon', rustType: 'Option<IconName>', defaultValue: 'None', description: 'Optional leading vector icon.' },
      { name: 'trailing_icon', rustType: 'Option<IconName>', defaultValue: 'None', description: 'Optional trailing vector icon.' },
      { name: 'loading', rustType: 'bool', defaultValue: 'false', description: 'Replaces leading icon with animated spinner and blocks clicks.' },
      { name: 'disabled', rustType: 'bool', defaultValue: 'false', description: 'Suppresses Sense::click() and applies muted opacity.' },
    ],
  },
  {
    slug: 'dropdown_button',
    name: 'DropdownButton',
    gpuiName: 'DropdownButton',
    category: 'Basic',
    part: 1,
    status: 'ready',
    summary:
      'Button trigger anchored to a floating action menu with keyboard Escape dismissal, shortcuts, destructive items, and automatic outside-click closing.',
    eguiSignature: 'DropdownButton::new("menu", "Actions", items, &mut selected)',
    gpuiMappingNote:
      'Persists popup open state in egui::Memory keyed by id_source and renders items inside a foreground egui::Area.',
    rustFilePath: 'src/components/dropdown_button.rs',
    usageRust: `use egui_kit::prelude::*;

let items = vec![
    DropdownItem::new("rename", "Rename File").shortcut("F2"),
    DropdownItem::new("duplicate", "Duplicate").shortcut("Cmd+D"),
    DropdownItem::new("delete", "Delete").destructive(true),
];

ui.add(DropdownButton::new("file_actions", "Actions", items, &mut self.selected_action));`,
    props: [
      { name: 'id_source', rustType: '&str', defaultValue: 'required', description: 'Unique key for menu open/close memory.' },
      { name: 'label', rustType: 'impl Into<String>', defaultValue: 'required', description: 'Trigger button text.' },
      { name: 'items', rustType: 'Vec<DropdownItem>', defaultValue: 'required', description: 'Menu entries with optional shortcuts and destructive flags.' },
      { name: 'selected_id', rustType: '&mut Option<String>', defaultValue: 'required', description: 'Mutated when a user clicks a menu item.' },
      { name: 'size', rustType: 'Size', defaultValue: 'Size::Md', description: 'Controls trigger height and text scale.' },
    ],
  },
  {
    slug: 'icon',
    name: 'Icon',
    gpuiName: 'Icon / IconName',
    category: 'Basic',
    part: 1,
    status: 'ready',
    summary:
      'Resolution-independent vector icons drawn directly with egui::Painter primitives, eliminating external SVG rasterization overhead.',
    eguiSignature: 'Icon::new(IconName::Search).size(Size::Md)',
    gpuiMappingNote:
      'Replaces GPUI SVG asset loading with lightweight stroke-scaled Painter geometry matching active theme foreground or custom Color32.',
    rustFilePath: 'src/components/icon.rs',
    usageRust: `use egui_kit::prelude::*;

ui.horizontal(|ui| {
    ui.add(Icon::new(IconName::Check).color(ui.kit_theme().success));
    ui.add(Icon::new(IconName::Search).size(Size::Lg));
    ui.add(Icon::new(IconName::AlertTriangle).color(ui.kit_theme().warning));
});`,
    props: [
      { name: 'name', rustType: 'IconName', defaultValue: 'required', description: 'Built-in vector icon glyph variant.' },
      { name: 'size', rustType: 'Size', defaultValue: 'Size::Md', description: 'Edge length from 12px (Xs) to 18px (Lg).' },
      { name: 'color', rustType: 'Option<Color32>', defaultValue: 'Theme foreground', description: 'Overrides icon stroke color.' },
    ],
  },
  {
    slug: 'badge',
    name: 'Badge',
    gpuiName: 'Badge',
    category: 'Basic',
    part: 1,
    status: 'ready',
    summary:
      'Compact counter, status dot, or text badge usable inline or anchored cleanly to the top-right corner of any existing widget Response.',
    eguiSignature: 'Badge::count(12).max_count(99).attach_to(ui, &target_resp)',
    gpuiMappingNote:
      'Instead of wrapping children like GPUI’s Badge container, attach_to(ui, &response) paints directly onto response.rect.right_top().',
    rustFilePath: 'src/components/badge.rs',
    usageRust: `use egui_kit::prelude::*;

let btn = ui.add(Button::new("Notifications").variant(ButtonVariant::Secondary));
Badge::count(self.unread).max_count(99).attach_to(ui, &btn);

ui.add(Badge::dot().variant(BadgeVariant::Success));
ui.add(Badge::text("Beta").variant(BadgeVariant::Primary));`,
    props: [
      { name: 'count', rustType: 'u32', defaultValue: 'None', description: 'Numeric count formatted with overflow cap.' },
      { name: 'max_count', rustType: 'u32', defaultValue: '99', description: 'Displays "{max}+" when exceeded.' },
      { name: 'dot', rustType: 'bool', defaultValue: 'false', description: 'Renders a compact 9px indicator dot.' },
      { name: 'variant', rustType: 'BadgeVariant', defaultValue: 'Primary', description: 'Primary, Secondary, Success, Warning, or Destructive.' },
    ],
  },
  {
    slug: 'tag',
    name: 'Tag',
    gpuiName: 'Tag',
    category: 'Basic',
    part: 1,
    status: 'ready',
    summary:
      'Interactive chip for categories, filters, and removable tokens with selection highlighting and dedicated close-button hit testing.',
    eguiSignature: 'Tag::new("egui").closable(true).show(ui)',
    gpuiMappingNote:
      'Returns TagResponse { response, closed } so callers can filter collections cleanly inside immediate-mode loops.',
    rustFilePath: 'src/components/tag.rs',
    usageRust: `use egui_kit::prelude::*;

if Tag::new("Rust").variant(TagVariant::Primary).selected(true).show(ui).response.clicked() {
    // toggle selection
}

if Tag::new("Dismissible").closable(true).show(ui).closed {
    // remove tag from Vec
}`,
    props: [
      { name: 'label', rustType: 'impl Into<String>', defaultValue: 'required', description: 'Visible text inside the tag.' },
      { name: 'variant', rustType: 'TagVariant', defaultValue: 'Neutral', description: 'Neutral, Primary, Outline, Success, Warning, or Destructive.' },
      { name: 'selected', rustType: 'bool', defaultValue: 'false', description: 'Highlights the tag with active primary fill.' },
      { name: 'closable', rustType: 'bool', defaultValue: 'false', description: 'Adds an interactive close icon on the right edge.' },
    ],
  },
  {
    slug: 'label',
    name: 'Label',
    gpuiName: 'Label',
    category: 'Basic',
    part: 1,
    status: 'ready',
    summary:
      'Semantic typography label with muted/status tones, strong emphasis, ellipsis truncation, and optional required field asterisk.',
    eguiSignature: 'Label::new("Crate Name").required(true).strong(true)',
    gpuiMappingNote:
      'Composes egui::RichText and egui::Label::truncate() with theme-aware semantic color tokens.',
    rustFilePath: 'src/components/label.rs',
    usageRust: `use egui_kit::prelude::*;

ui.add(Label::new("Repository URL").required(true).strong(true));
ui.add(Label::new("Optional remote mirror endpoint").tone(LabelTone::Muted).size(Size::Sm));`,
    props: [
      { name: 'text', rustType: 'impl Into<String>', defaultValue: 'required', description: 'Label text content.' },
      { name: 'tone', rustType: 'LabelTone', defaultValue: 'Default', description: 'Default, Muted, Primary, Success, Warning, or Destructive.' },
      { name: 'size', rustType: 'Size', defaultValue: 'Size::Md', description: 'Controls text point size.' },
      { name: 'required', rustType: 'bool', defaultValue: 'false', description: 'Appends a destructive-tinted asterisk.' },
      { name: 'truncate', rustType: 'bool', defaultValue: 'false', description: 'Clips overflowing text with ellipsis.' },
    ],
  },
  {
    slug: 'kbd',
    name: 'Kbd',
    gpuiName: 'Kbd',
    category: 'Basic',
    part: 1,
    status: 'ready',
    summary:
      'Tactile keyboard shortcut keycap renderer with crisp Painter vector icons (Command, Shift, Option, Control, Enter, Tab, Backspace, Esc, and arrows) plus platform-adaptive labels.',
    eguiSignature: 'Kbd::new("Cmd+K").macos_symbols(true)',
    gpuiMappingNote:
      'Splits chord strings on "+" and paints tactile beveled keycaps with normalized Painter vector glyphs so modifier keys never fall back to missing OS font boxes.',
    rustFilePath: 'src/components/kbd.rs',
    usageRust: `use egui_kit::prelude::*;

ui.horizontal(|ui| {
    ui.add(Label::new("Command Palette"));
    ui.kit_kbd("Cmd+Shift+P");
});`,
    props: [
      { name: 'shortcut', rustType: 'impl Into<String>', defaultValue: 'required', description: 'Plus-delimited key sequence like "Cmd+Shift+P" or "Alt+Tab".' },
      { name: 'size', rustType: 'Size', defaultValue: 'Size::Sm', description: 'Keycap height, vector glyph scale, and monospace font size.' },
      { name: 'macos_symbols', rustType: 'bool', defaultValue: 'cfg!(target_os = "macos")', description: 'Renders pure compact vector symbols on macOS or pairs glyphs with Ctrl/Alt/Enter labels.' },
    ],
  },
  {
    slug: 'avatar',
    name: 'Avatar',
    gpuiName: 'Avatar / AvatarGroup',
    category: 'Basic',
    part: 1,
    status: 'ready',
    summary:
      'Circular or rounded-square identity avatar with automatic initials derivation, presence status rings, and overlapping AvatarGroup stacks.',
    eguiSignature: 'Avatar::new("Ada Lovelace").status(AvatarStatus::Online)',
    gpuiMappingNote:
      'Computes two-letter initials automatically and supports negative item spacing in AvatarGroup with a "+N" overflow circle.',
    rustFilePath: 'src/components/avatar.rs',
    usageRust: `use egui_kit::prelude::*;

ui.add(Avatar::new("Ada Lovelace").size(Size::Lg).status(AvatarStatus::Online));

ui.add(AvatarGroup::new(vec![
    Avatar::new("Ada Lovelace").status(AvatarStatus::Online),
    Avatar::new("Graydon Hoare").status(AvatarStatus::Busy),
    Avatar::new("Niko Matsakis"),
    Avatar::new("Raph Levien"),
]).max_visible(3));`,
    props: [
      { name: 'name', rustType: 'impl Into<String>', defaultValue: 'required', description: 'Full name used for hover tooltip and initials.' },
      { name: 'size', rustType: 'Size', defaultValue: 'Size::Md', description: 'Diameter from 24px (Xs) to 52px (Lg).' },
      { name: 'shape', rustType: 'AvatarShape', defaultValue: 'Circle', description: 'Circle or RoundedSquare.' },
      { name: 'status', rustType: 'Option<AvatarStatus>', defaultValue: 'None', description: 'Online, Away, Busy, or Offline presence dot.' },
    ],
  },

  // =========================================================================
  // PART 2 — Selection Controls, Progress & Feedback States (10)
  // =========================================================================
  {
    slug: 'checkbox',
    name: 'Checkbox',
    gpuiName: 'Checkbox',
    category: 'Basic',
    part: 2,
    status: 'ready',
    summary:
      'Zero-boilerplate checkbox mutating &mut bool directly, with optional indeterminate state and crisp vector checkmarks.',
    eguiSignature: 'ui.kit_checkbox(&mut checked, "Enable telemetry")',
    gpuiMappingNote:
      'Eliminates GPUI callback closures: Checkbox::new(&mut checked, label) or the one-liner ui.kit_checkbox(&mut checked, label) updates your bool in place.',
    rustFilePath: 'src/components/checkbox.rs',
    usageRust: `use egui_kit::prelude::*;

// Simplest 1-liner
ui.kit_checkbox(&mut self.telemetry, "Enable telemetry");

// Or configure indeterminate / size / disabled
ui.add(
    Checkbox::new(&mut self.select_all, "Select all modules")
        .indeterminate(self.partial_selection)
        .size(Size::Lg),
);`,
    props: [
      { name: 'checked', rustType: '&mut bool', defaultValue: 'required', description: 'Boolean mutated directly on click.' },
      { name: 'label', rustType: 'impl Into<String>', defaultValue: 'required', description: 'Clickable label rendered beside the box.' },
      { name: 'indeterminate', rustType: 'bool', defaultValue: 'false', description: 'Draws a centered horizontal dash for partial selection.' },
      { name: 'size', rustType: 'Size', defaultValue: 'Size::Md', description: 'Scales checkbox box and label font size.' },
      { name: 'disabled', rustType: 'bool', defaultValue: 'false', description: 'Locks state and dims surface contrast.' },
    ],
  },
  {
    slug: 'radio',
    name: 'Radio',
    gpuiName: 'Radio / RadioGroup',
    category: 'Basic',
    part: 2,
    status: 'ready',
    summary:
      'Generic PartialEq radio button bound directly to any enum or primitive value with automatic mutual exclusion.',
    eguiSignature: 'ui.kit_radio(&mut profile, BuildProfile::Release, "Release")',
    gpuiMappingNote:
      'No separate group state wrapper required: pass the same &mut T enum across Radio items and egui-kit handles equality and mutation.',
    rustFilePath: 'src/components/radio.rs',
    usageRust: `use egui_kit::prelude::*;

#[derive(PartialEq, Clone, Copy)]
enum Mode { Debug, Release, Bench }

ui.horizontal(|ui| {
    ui.kit_radio(&mut self.mode, Mode::Debug, "Debug");
    ui.kit_radio(&mut self.mode, Mode::Release, "Release");
    ui.kit_radio(&mut self.mode, Mode::Bench, "Bench");
});`,
    props: [
      { name: 'current', rustType: '&mut T (PartialEq)', defaultValue: 'required', description: 'Shared selection binding updated when clicked.' },
      { name: 'value', rustType: 'T', defaultValue: 'required', description: 'Option value represented by this radio button.' },
      { name: 'label', rustType: 'impl Into<String>', defaultValue: 'required', description: 'Clickable caption beside the radio ring.' },
      { name: 'size', rustType: 'Size', defaultValue: 'Size::Md', description: 'Controls outer ring diameter and label scale.' },
      { name: 'disabled', rustType: 'bool', defaultValue: 'false', description: 'Disables selection changes.' },
    ],
  },
  {
    slug: 'switch',
    name: 'Switch',
    gpuiName: 'Switch',
    category: 'Basic',
    part: 2,
    status: 'ready',
    summary:
      'Smooth animated binary toggle switch using egui::Context::animate_bool with optional inline label.',
    eguiSignature: 'ui.kit_switch(&mut enabled, "Auto-save")',
    gpuiMappingNote:
      'Replaces GPUI animation tasks with ctx.animate_bool(id, *on) for zero-allocation 60fps thumb interpolation.',
    rustFilePath: 'src/components/switch.rs',
    usageRust: `use egui_kit::prelude::*;

// One-line labeled switch
ui.kit_switch(&mut self.auto_save, "Auto-save buffers");

// Or customize size cleanly
ui.add(Switch::new(&mut self.compact).label("Compact mode").size(Size::Lg));`,
    props: [
      { name: 'on', rustType: '&mut bool', defaultValue: 'required', description: 'Switch state mutated on click.' },
      { name: 'label', rustType: 'Option<String>', defaultValue: 'None', description: 'Optional clickable label beside track.' },
      { name: 'size', rustType: 'Size', defaultValue: 'Size::Md', description: 'Track dimensions from Xs (28x16) to Lg (48x26).' },
      { name: 'disabled', rustType: 'bool', defaultValue: 'false', description: 'Dims track and prevents toggling.' },
    ],
  },
  {
    slug: 'toggle',
    name: 'Toggle',
    gpuiName: 'Toggle',
    category: 'Basic',
    part: 2,
    status: 'ready',
    summary:
      'Pressable two-state toolbar button with optional vector icon and active primary fill highlight.',
    eguiSignature: 'ui.kit_toggle(&mut active, "Filter")',
    gpuiMappingNote:
      'Directly binds &mut bool instead of requiring controlled on_change callbacks and group entities.',
    rustFilePath: 'src/components/toggle.rs',
    usageRust: `use egui_kit::prelude::*;

ui.horizontal(|ui| {
    ui.kit_toggle(&mut self.live_reload, "Live");
    ui.add(Toggle::new(&mut self.search_mode, "Find").icon(IconName::Search).size(Size::Sm));
});`,
    props: [
      { name: 'pressed', rustType: '&mut bool', defaultValue: 'required', description: 'Pressed state mutated on click.' },
      { name: 'label', rustType: 'impl Into<String>', defaultValue: 'required', description: 'Button text (pass "" for icon-only).' },
      { name: 'icon', rustType: 'Option<IconName>', defaultValue: 'None', description: 'Optional leading vector icon.' },
      { name: 'size', rustType: 'Size', defaultValue: 'Size::Md', description: 'Controls height and padding.' },
      { name: 'disabled', rustType: 'bool', defaultValue: 'false', description: 'Prevents toggle interaction.' },
    ],
  },
  {
    slug: 'slider',
    name: 'Slider',
    gpuiName: 'Slider',
    category: 'Basic',
    part: 2,
    status: 'ready',
    summary:
      'Themed numeric slider mutating &mut f32 directly with step snapping, label header, and tabular value readout.',
    eguiSignature: 'Slider::new(&mut val, 0.0..=100.0).step(5.0).suffix("%")',
    gpuiMappingNote:
      'Binds directly to RangeInclusive<f32> with pointer drag/click handling and zero external slider state structs.',
    rustFilePath: 'src/components/slider.rs',
    usageRust: `use egui_kit::prelude::*;

// Minimal range slider
ui.kit_slider(&mut self.opacity, 0.0..=1.0);

// Labeled slider with step snapping & unit suffix
ui.add(
    Slider::new(&mut self.volume, 0.0..=100.0)
        .step(5.0)
        .label("Master Gain")
        .suffix("%"),
);`,
    props: [
      { name: 'value', rustType: '&mut f32', defaultValue: 'required', description: 'Numeric value updated during click or drag.' },
      { name: 'range', rustType: 'RangeInclusive<f32>', defaultValue: 'required', description: 'Inclusive minimum and maximum bounds.' },
      { name: 'step', rustType: 'Option<f32>', defaultValue: 'None', description: 'Optional snap increment while dragging.' },
      { name: 'label', rustType: 'Option<String>', defaultValue: 'None', description: 'Header caption above track.' },
      { name: 'suffix', rustType: 'String', defaultValue: '""', description: 'Unit suffix appended to numeric readout.' },
      { name: 'show_value', rustType: 'bool', defaultValue: 'true', description: 'Shows formatted monospace value above track.' },
    ],
  },
  {
    slug: 'rating',
    name: 'Rating',
    gpuiName: 'Rating',
    category: 'Basic',
    part: 2,
    status: 'ready',
    summary:
      'Interactive star rating widget mutating &mut f32 with crisp geometric stars, hover preview, and optional half-star precision.',
    eguiSignature: 'Rating::new(&mut stars).allow_half(true)',
    gpuiMappingNote:
      'Paints crisp clipped vector stars on Painter and computes hover precision directly from pointer coordinates.',
    rustFilePath: 'src/components/rating.rs',
    usageRust: `use egui_kit::prelude::*;

ui.kit_rating(&mut self.score);

ui.add(
    Rating::new(&mut self.score)
        .max_stars(5)
        .allow_half(true)
        .size(Size::Lg),
);`,
    props: [
      { name: 'value', rustType: '&mut f32', defaultValue: 'required', description: 'Current star rating mutated on click.' },
      { name: 'max_stars', rustType: 'u8', defaultValue: '5', description: 'Total number of star icons.' },
      { name: 'allow_half', rustType: 'bool', defaultValue: 'false', description: 'Enables 0.5-star selection and clipped half fills.' },
      { name: 'size', rustType: 'Size', defaultValue: 'Size::Md', description: 'Controls star size.' },
      { name: 'disabled', rustType: 'bool', defaultValue: 'false', description: 'Locks rating to read-only display.' },
    ],
  },
  {
    slug: 'progress',
    name: 'Progress',
    gpuiName: 'Progress',
    category: 'Basic',
    part: 2,
    status: 'ready',
    summary:
      'Determinate and indeterminate linear progress bar with semantic tones and inline monospace percentage label.',
    eguiSignature: 'Progress::new(0.72).show_label(true)',
    gpuiMappingNote:
      'Triggers ctx.request_repaint() only when Progress::indeterminate() is rendered so determinate bars stay zero-CPU.',
    rustFilePath: 'src/components/progress.rs',
    usageRust: `use egui_kit::prelude::*;

ui.kit_progress(0.72);

ui.add(Progress::new(0.85).tone(Variant::Success).show_label(true).size(Size::Lg));
ui.add(Progress::indeterminate().tone(Variant::Primary));`,
    props: [
      { name: 'value', rustType: 'f32 (0.0..=1.0) | indeterminate()', defaultValue: 'required', description: 'Clamped fraction or indeterminate sweep constructor.' },
      { name: 'show_label', rustType: 'bool', defaultValue: 'false', description: 'Displays rounded percentage to the right.' },
      { name: 'tone', rustType: 'Variant', defaultValue: 'Variant::Primary', description: 'Primary, Secondary, Success, Warning, or Destructive.' },
      { name: 'size', rustType: 'Size', defaultValue: 'Size::Md', description: 'Track thickness from 4px (Xs) to 12px (Lg).' },
    ],
  },
  {
    slug: 'spinner',
    name: 'Spinner',
    gpuiName: 'Spinner',
    category: 'Basic',
    part: 2,
    status: 'ready',
    summary:
      'Lightweight rotating vector arc spinner scaled by Size tokens with optional status label.',
    eguiSignature: 'Spinner::new().size(Size::Md).label("Compiling...")',
    gpuiMappingNote:
      'Draws smooth arc segments directly on egui::Painter without external SVG or sprite sheets.',
    rustFilePath: 'src/components/spinner.rs',
    usageRust: `use egui_kit::prelude::*;

ui.kit_spinner();

ui.add(
    Spinner::new()
        .size(Size::Lg)
        .label("Compiling egui-kit..."),
);`,
    props: [
      { name: 'size', rustType: 'Size', defaultValue: 'Size::Md', description: 'Controls spinner diameter and stroke weight.' },
      { name: 'color', rustType: 'Option<Color32>', defaultValue: 'Theme primary', description: 'Overrides active arc stroke color.' },
      { name: 'label', rustType: 'Option<String>', defaultValue: 'None', description: 'Optional muted text rendered beside spinner.' },
    ],
  },
  {
    slug: 'skeleton',
    name: 'Skeleton',
    gpuiName: 'Skeleton',
    category: 'Basic',
    part: 2,
    status: 'ready',
    summary:
      'Pulsing layout-preserving loading placeholders with dedicated rect, circle, and multi-line text constructors.',
    eguiSignature: 'Skeleton::lines(3)',
    gpuiMappingNote:
      'Provides immediate constructors Skeleton::rect(w, h), Skeleton::circle(d), and Skeleton::lines(n) without verbose layout boilerplate.',
    rustFilePath: 'src/components/skeleton.rs',
    usageRust: `use egui_kit::prelude::*;

ui.horizontal(|ui| {
    ui.add(Skeleton::circle(40.0));
    ui.vertical(|ui| {
        ui.add(Skeleton::rect(160.0, 14.0));
        ui.add(Skeleton::lines(2));
    });
});`,
    props: [
      { name: 'rect(w, h)', rustType: '(f32, f32)', defaultValue: 'constructor', description: 'Rounded rectangle placeholder block.' },
      { name: 'circle(d)', rustType: 'f32', defaultValue: 'constructor', description: 'Circular avatar/icon placeholder.' },
      { name: 'lines(n)', rustType: 'usize', defaultValue: 'constructor', description: 'Multi-line text block with shorter trailing line.' },
      { name: 'animated', rustType: 'bool', defaultValue: 'true', description: 'Enables smooth pulse between muted and accent tokens.' },
    ],
  },
  {
    slug: 'shimmer',
    name: 'Shimmer',
    gpuiName: 'Shimmer',
    category: 'Basic',
    part: 2,
    status: 'ready',
    summary:
      'Animated diagonal highlight sweep usable standalone via Shimmer::rect or composed over any widgets with Shimmer::wrap.',
    eguiSignature: 'Shimmer::wrap(ui, |ui| { ui.add(Skeleton::lines(3)); })',
    gpuiMappingNote:
      'Shimmer::wrap measures the child scope rect and clips a sweeping highlight polygon right over it in one call.',
    rustFilePath: 'src/components/shimmer.rs',
    usageRust: `use egui_kit::prelude::*;

// Standalone shimmer block
ui.add(Shimmer::rect(240.0, 28.0).rounding(6.0));

// Or sweep shimmer over any Skeleton composition
Shimmer::wrap(ui, |ui| {
    ui.add(Skeleton::lines(3));
});`,
    props: [
      { name: 'rect(w, h)', rustType: '(f32, f32)', defaultValue: 'constructor', description: 'Creates a standalone shimmer placeholder block.' },
      { name: 'wrap(ui, f)', rustType: 'FnOnce(&mut Ui)', defaultValue: 'static method', description: 'Paints a clipped highlight sweep over any child UI.' },
      { name: 'rounding', rustType: 'f32', defaultValue: '6.0', description: 'Corner radius for standalone shimmer blocks.' },
      { name: 'speed', rustType: 'f32', defaultValue: '0.75', description: 'Cycle frequency of the diagonal highlight sweep.' },
    ],
  },

  // =========================================================================
  // PART 3 — Text Inputs, Forms & Validation (10)
  // =========================================================================
  {
    slug: 'input',
    name: 'Input',
    gpuiName: 'Input / TextInput',
    category: 'Form',
    part: 3,
    status: 'ready',
    summary:
      'Single-line text input mutating &mut String directly with animated focus halo, prefix icon, clearable action, password masking, and error highlighting.',
    eguiSignature: 'ui.kit_input(&mut query, "Filter components...")',
    gpuiMappingNote:
      'Replaces GPUI InputState entities with direct &mut String binding inside an animated themed Frame and egui::TextEdit::singleline.',
    rustFilePath: 'src/components/input.rs',
    usageRust: `use egui_kit::prelude::*;

// Zero-boilerplate 1-liner
ui.kit_input(&mut self.query, "Filter modules...");

// Or configure prefix icon, clear button, password mask & error state
ui.add(
    Input::new(&mut self.api_token)
        .placeholder("Enter API token...")
        .prefix_icon(IconName::Search)
        .clearable(true)
        .password(true)
        .error(self.invalid_token),
);`,
    props: [
      { name: 'value', rustType: '&mut String', defaultValue: 'required', description: 'Text buffer mutated in place on keystrokes.' },
      { name: 'placeholder', rustType: 'impl Into<String>', defaultValue: '""', description: 'Muted hint text displayed when empty.' },
      { name: 'prefix_icon', rustType: 'Option<IconName>', defaultValue: 'None', description: 'Optional leading vector icon.' },
      { name: 'clearable', rustType: 'bool', defaultValue: 'false', description: 'Shows animated × button when non-empty.' },
      { name: 'password', rustType: 'bool', defaultValue: 'false', description: 'Masks glyphs using bullet characters.' },
      { name: 'error', rustType: 'bool', defaultValue: 'false', description: 'Applies destructive border and halo tint.' },
    ],
  },
  {
    slug: 'input-group',
    name: 'InputGroup',
    gpuiName: 'InputGroup',
    category: 'Form',
    part: 3,
    status: 'ready',
    summary:
      'Composite input container joining prefix/suffix text or icon addons seamlessly inside one animated focusable border.',
    eguiSignature: 'InputGroup::new().prefix("https://").suffix(".rs").show(ui, |ui| ...)',
    gpuiMappingNote:
      'Tracks inner child focus in egui::Memory so the unified outer frame animates its ring whenever the nested TextEdit receives focus.',
    rustFilePath: 'src/components/input_group.rs',
    usageRust: `use egui_kit::prelude::*;

InputGroup::new()
    .prefix("https://")
    .suffix(".rs")
    .show(ui, |ui| {
        ui.add(egui::TextEdit::singleline(&mut self.host).frame(false))
    });`,
    props: [
      { name: 'prefix', rustType: 'Option<String>', defaultValue: 'None', description: 'Leading muted addon badge inside the group border.' },
      { name: 'suffix', rustType: 'Option<String>', defaultValue: 'None', description: 'Trailing muted addon badge inside the group border.' },
      { name: 'prefix_icon', rustType: 'Option<IconName>', defaultValue: 'None', description: 'Optional vector icon inside the prefix slot.' },
      { name: 'size', rustType: 'Size', defaultValue: 'Size::Md', description: 'Scales group height, rounding, and text size.' },
      { name: 'error', rustType: 'bool', defaultValue: 'false', description: 'Highlights unified frame with destructive token.' },
    ],
  },
  {
    slug: 'textarea',
    name: 'TextArea',
    gpuiName: 'TextArea / MultiLineInput',
    category: 'Form',
    part: 3,
    status: 'ready',
    summary:
      'Multi-line scrollable text editor mutating &mut String with animated focus border and live warning/error character budget counter.',
    eguiSignature: 'TextArea::new(&mut notes).rows(4).max_chars(140)',
    gpuiMappingNote:
      'Embeds ScrollArea + egui::TextEdit::multiline::frame(false) inside a custom Painter border with live budget color lerping.',
    rustFilePath: 'src/components/textarea.rs',
    usageRust: `use egui_kit::prelude::*;

ui.kit_textarea(&mut self.summary, "Short crate summary...");

ui.add(
    TextArea::new(&mut self.notes)
        .rows(4)
        .max_chars(140)
        .placeholder("Describe release changes..."),
);`,
    props: [
      { name: 'value', rustType: '&mut String', defaultValue: 'required', description: 'Multi-line text buffer edited directly.' },
      { name: 'rows', rustType: 'usize', defaultValue: '4', description: 'Visible line height budget.' },
      { name: 'max_chars', rustType: 'Option<usize>', defaultValue: 'None', description: 'Optional character limit with warning/error readout.' },
      { name: 'placeholder', rustType: 'impl Into<String>', defaultValue: '""', description: 'Muted hint text displayed when empty.' },
      { name: 'disabled', rustType: 'bool', defaultValue: 'false', description: 'Prevents editing and dims surface.' },
    ],
  },
  {
    slug: 'number-input',
    name: 'NumberInput',
    gpuiName: 'NumberInput',
    category: 'Form',
    part: 3,
    status: 'ready',
    summary:
      'Numeric stepper control mutating &mut f32 with +/- buttons, step snapping, Up/Down key support, and automatic min/max clamping.',
    eguiSignature: 'ui.kit_number_input(&mut workers, 1.0..=32.0)',
    gpuiMappingNote:
      'Keeps temporary typed text drafts in egui::Memory while focused and commits snapped numbers back to &mut f32 on Enter or blur.',
    rustFilePath: 'src/components/number_input.rs',
    usageRust: `use egui_kit::prelude::*;

ui.kit_number_input(&mut self.workers, 1.0..=32.0);

ui.add(
    NumberInput::new(&mut self.scale, 0.5..=4.0)
        .step(0.5)
        .precision(1),
);`,
    props: [
      { name: 'value', rustType: '&mut f32', defaultValue: 'required', description: 'Numeric binding mutated via steppers or typed commit.' },
      { name: 'range', rustType: 'RangeInclusive<f32>', defaultValue: 'required', description: 'Clamped inclusive minimum and maximum values.' },
      { name: 'step', rustType: 'f32', defaultValue: '1.0', description: 'Increment used by +/- buttons and arrow keys.' },
      { name: 'precision', rustType: 'usize', defaultValue: '0', description: 'Decimal places shown in formatted readout.' },
      { name: 'disabled', rustType: 'bool', defaultValue: 'false', description: 'Locks buttons and text entry.' },
    ],
  },
  {
    slug: 'otp-input',
    name: 'OtpInput',
    gpuiName: 'OtpInput / PinInput',
    category: 'Form',
    part: 3,
    status: 'ready',
    summary:
      'Segmented verification code input mutating &mut String directly with auto-advance, backspace retreat, multi-digit paste split, and blinking caret.',
    eguiSignature: 'ui.kit_otp(&mut code, 6)',
    gpuiMappingNote:
      'Processes egui::Event::Text, Event::Paste, and navigation keys in a single widget ID without juggling N separate TextEdits.',
    rustFilePath: 'src/components/otp_input.rs',
    usageRust: `use egui_kit::prelude::*;

ui.kit_otp(&mut self.code, 6);

ui.add(
    OtpInput::new(&mut self.pin, 4)
        .masked(true)
        .error(self.invalid_pin),
);`,
    props: [
      { name: 'value', rustType: '&mut String', defaultValue: 'required', description: 'Digits-only verification code mutated directly.' },
      { name: 'length', rustType: 'usize', defaultValue: '6', description: 'Number of segmented code boxes (3..=8).' },
      { name: 'masked', rustType: 'bool', defaultValue: 'false', description: 'Replaces digits with • security dots.' },
      { name: 'error', rustType: 'bool', defaultValue: 'false', description: 'Highlights all slots with destructive border.' },
      { name: 'size', rustType: 'Size', defaultValue: 'Size::Md', description: 'Scales box edge size and monospace glyph.' },
    ],
  },
  {
    slug: 'select',
    name: 'Select',
    gpuiName: 'Select',
    category: 'Form',
    part: 3,
    status: 'ready',
    summary:
      'Dropdown select control mutating &mut usize directly with smooth chevron flip, eased foreground popup menu, and Escape/outside-click dismissal.',
    eguiSignature: 'ui.kit_select("region", &mut selected_idx, &options)',
    gpuiMappingNote:
      'Uses generic &[T: AsRef<str>] slices and foreground egui::Area animations so zero boilerplate adapter structs are needed.',
    rustFilePath: 'src/components/select.rs',
    usageRust: `use egui_kit::prelude::*;

let regions = ["us-east-1", "eu-central-1", "ap-northeast-1"];
ui.kit_select("deploy_region", &mut self.region_idx, &regions);

ui.add(
    Select::new("region_lg", &mut self.region_idx, &regions)
        .size(Size::Lg)
        .placeholder("Choose region"),
);`,
    props: [
      { name: 'id_source', rustType: '&str', defaultValue: 'required', description: 'Unique key for popup open state.' },
      { name: 'selected', rustType: '&mut usize', defaultValue: 'required', description: 'Active option index updated when clicked.' },
      { name: 'options', rustType: '&[T] (AsRef<str>)', defaultValue: 'required', description: 'Slice of display option labels.' },
      { name: 'size', rustType: 'Size', defaultValue: 'Size::Md', description: 'Trigger height and popup item typography.' },
      { name: 'disabled', rustType: 'bool', defaultValue: 'false', description: 'Locks trigger and suppresses popup.' },
    ],
  },
  {
    slug: 'combobox',
    name: 'Combobox',
    gpuiName: 'Combobox',
    category: 'Form',
    part: 3,
    status: 'ready',
    summary:
      'Searchable dropdown selector mutating &mut Option<usize> with live case-insensitive query filtering, empty state, and one-click clear.',
    eguiSignature: 'ui.kit_combobox("crate", &mut selected, &items)',
    gpuiMappingNote:
      'Stores ephemeral filter query inside egui::Memory keyed by id_source while mutating your &mut Option<usize> selection cleanly.',
    rustFilePath: 'src/components/combobox.rs',
    usageRust: `use egui_kit::prelude::*;

let crates = ["egui", "eframe", "wgpu", "serde", "tokio"];
ui.kit_combobox("crate_picker", &mut self.crate_idx, &crates);

ui.add(
    Combobox::new("crate_full", &mut self.crate_idx, &crates)
        .placeholder("Search Rust crates...")
        .clearable(true),
);`,
    props: [
      { name: 'id_source', rustType: '&str', defaultValue: 'required', description: 'Unique salt for query buffer and popup state.' },
      { name: 'selected', rustType: '&mut Option<usize>', defaultValue: 'required', description: 'Optional selected index mutated on pick or clear.' },
      { name: 'options', rustType: '&[T] (AsRef<str>)', defaultValue: 'required', description: 'Filterable list of candidate items.' },
      { name: 'clearable', rustType: 'bool', defaultValue: 'true', description: 'Includes quick Clear action inside popup.' },
      { name: 'disabled', rustType: 'bool', defaultValue: 'false', description: 'Disables interaction and dims trigger.' },
    ],
  },
  {
    slug: 'form',
    name: 'Form',
    gpuiName: 'Form / FormField',
    category: 'Form',
    part: 3,
    status: 'ready',
    summary:
      'Structured Form and FormField layout helpers pairing labels, required indicators, helper hints, and animated validation error captions.',
    eguiSignature: 'FormField::new("Crate Name").required(true).error(err).show(ui, |ui| ...)',
    gpuiMappingNote:
      'Supports Vertical and Horizontal label alignment via tokens::Orientation while smoothly fading inline validation text via animate_bool_with_time.',
    rustFilePath: 'src/components/form.rs',
    usageRust: `use egui_kit::prelude::*;

Form::new("publish_form")
    .orientation(Orientation::Vertical)
    .show(ui, |ui| {
        FormField::new("Crate Name")
            .required(true)
            .hint("Lowercase crates.io identifier")
            .error(self.name_error.as_deref())
            .show(ui, |ui| ui.kit_input(&mut self.name, "egui-kit"));
    });`,
    props: [
      { name: 'label', rustType: 'impl Into<String>', defaultValue: 'required', description: 'Field title rendered above or beside input.' },
      { name: 'required', rustType: 'bool', defaultValue: 'false', description: 'Appends destructive * indicator.' },
      { name: 'hint', rustType: 'Option<String>', defaultValue: 'None', description: 'Helper caption rendered when no error is active.' },
      { name: 'error', rustType: 'Option<&str>', defaultValue: 'None', description: 'Animated validation error caption replacing hint.' },
      { name: 'orientation', rustType: 'Orientation', defaultValue: 'Vertical', description: 'Vertical stacked or Horizontal two-column alignment.' },
    ],
  },
  {
    slug: 'questionnaire',
    name: 'Questionnaire',
    gpuiName: 'Questionnaire',
    category: 'Basic',
    part: 3,
    status: 'ready',
    summary:
      'Multi-step interactive survey card binding directly to &mut Vec<Option<usize>> with animated progress bar and selectable choice cards.',
    eguiSignature: 'Questionnaire::new("setup", &questions, &mut answers)',
    gpuiMappingNote:
      'Composes Progress and Button internally while persisting active question index in egui::Memory.',
    rustFilePath: 'src/components/questionnaire.rs',
    usageRust: `use egui_kit::prelude::*;

let questions = vec![
    Question::new("Target platform?", vec!["Desktop".into(), "WebAssembly".into()]),
    Question::new("Renderer?", vec!["wgpu".into(), "glow".into()]),
];
ui.add(Questionnaire::new("setup_survey", &questions, &mut self.answers));`,
    props: [
      { name: 'id_source', rustType: '&str', defaultValue: 'required', description: 'Unique key for active survey step.' },
      { name: 'questions', rustType: '&[Question]', defaultValue: 'required', description: 'Prompt titles, subtitles, and choice lists.' },
      { name: 'answers', rustType: '&mut Vec<Option<usize>>', defaultValue: 'required', description: 'Selected answer indices updated per question.' },
      { name: 'width', rustType: 'f32', defaultValue: '420.0', description: 'Card width in logical pixels.' },
    ],
  },
  {
    slug: 'clipboard',
    name: 'Clipboard',
    gpuiName: 'Clipboard',
    category: 'Basic',
    part: 3,
    status: 'ready',
    summary:
      'One-click copy trigger calling ui.ctx().copy_text() with timed "Copied!" success transition and icon morph.',
    eguiSignature: 'ui.kit_clipboard("install", "cargo add egui-kit")',
    gpuiMappingNote:
      'Uses egui 0.30’s native ctx.copy_text() and stores a 1.6s confirmation expiry timestamp inside egui::Memory.',
    rustFilePath: 'src/components/clipboard.rs',
    usageRust: `use egui_kit::prelude::*;

ui.kit_clipboard("install_cmd", "cargo add egui-kit");

ui.add(
    Clipboard::new("token_copy", &self.token)
        .label("Copy Token")
        .copied_label("Copied!")
        .size(Size::Sm),
);`,
    props: [
      { name: 'id_source', rustType: '&str', defaultValue: 'required', description: 'Unique key for timed copied feedback.' },
      { name: 'text', rustType: '&str', defaultValue: 'required', description: 'String payload sent to OS clipboard.' },
      { name: 'label', rustType: 'impl Into<String>', defaultValue: '"Copy"', description: 'Default trigger caption.' },
      { name: 'copied_label', rustType: 'impl Into<String>', defaultValue: '"Copied!"', description: 'Temporary confirmation text shown for 1.6s.' },
      { name: 'show_text', rustType: 'bool', defaultValue: 'true', description: 'Set false for compact icon-only copy trigger.' },
    ],
  },

  // =========================================================================
  // PART 4 — Alerts, Empty States, Media & Pickers (10)
  // =========================================================================
  {
    slug: 'alert',
    name: 'Alert',
    gpuiName: 'Alert',
    category: 'Basic',
    part: 4,
    status: 'ready',
    summary:
      'Inline status banner with semantic vector icon, title, description, left accent stripe, and animated dismiss control.',
    eguiSignature: 'Alert::info("Update available", "egui-kit 0.4 is out")',
    gpuiMappingNote:
      'Pairs a crisp semantic IconName with high-contrast theme accents and optional &mut bool animated fade-out via show_dismissible().',
    rustFilePath: 'src/components/alert.rs',
    usageRust: `use egui_kit::prelude::*;

ui.kit_alert("Release Notes", "Part 4 widgets are ready.");

Alert::success("Build Ready", "All 40 crate widgets compiled cleanly.")
    .show_dismissible(ui, "build_alert", &mut self.show_alert);`,
    props: [
      { name: 'title', rustType: 'impl Into<String>', defaultValue: 'required', description: 'Primary bold banner headline.' },
      { name: 'tone', rustType: 'AlertTone', defaultValue: 'Info', description: 'Info, Success, Warning, or Destructive semantic accent.' },
      { name: 'description', rustType: 'impl Into<String>', defaultValue: 'None', description: 'Optional wrapped detail message below title.' },
      { name: 'dismissible', rustType: 'bool', defaultValue: 'false', description: 'Renders close button when true or using show_dismissible().' },
    ],
  },
  {
    slug: 'empty',
    name: 'Empty',
    gpuiName: 'Empty',
    category: 'Basic',
    part: 4,
    status: 'ready',
    summary:
      'Zero-state illustration placeholder with Painter emblem, guidance text, and optional primary action button.',
    eguiSignature: 'Empty::new("No results").action("Reset Filters").show(ui)',
    gpuiMappingNote:
      'Centers a themed Frame vertically and returns EmptyResponse { response, action_clicked } for one-line CTA handling.',
    rustFilePath: 'src/components/empty.rs',
    usageRust: `use egui_kit::prelude::*;

if Empty::new("No matching widgets")
    .description("Clear active filters or explore all 80 gpui-kit ports.")
    .action("Reset Filters")
    .show(ui)
    .action_clicked
{
    self.query.clear();
}`,
    props: [
      { name: 'title', rustType: 'impl Into<String>', defaultValue: 'required', description: 'Centered empty-state headline.' },
      { name: 'description', rustType: 'impl Into<String>', defaultValue: 'None', description: 'Optional helpful recovery explanation.' },
      { name: 'action', rustType: 'impl Into<String>', defaultValue: 'None', description: 'Optional primary Button label.' },
      { name: 'width', rustType: 'f32', defaultValue: '360.0', description: 'Card width in logical pixels.' },
    ],
  },
  {
    slug: 'attachment',
    name: 'Attachment',
    gpuiName: 'Attachment',
    category: 'Basic',
    part: 4,
    status: 'ready',
    summary:
      'File attachment card displaying Painter file badge, tabular byte size, upload Progress bar, and remove button.',
    eguiSignature: 'Attachment::new("Cargo.lock", 48_210).show(ui)',
    gpuiMappingNote:
      'Formats tabular KB/MB byte labels automatically and composes Progress internally when progress(Some(fraction)) is set.',
    rustFilePath: 'src/components/attachment.rs',
    usageRust: `use egui_kit::prelude::*;

if Attachment::new("Cargo.lock", 48_210)
    .progress(Some(0.72))
    .show(ui)
    .removed
{
    self.files.retain(|f| f.name != "Cargo.lock");
}`,
    props: [
      { name: 'name', rustType: 'impl Into<String>', defaultValue: 'required', description: 'Filename displayed on primary row.' },
      { name: 'bytes', rustType: 'u64', defaultValue: 'required', description: 'Raw byte count formatted to B / KB / MB.' },
      { name: 'progress', rustType: 'Option<f32>', defaultValue: 'None', description: 'Optional 0.0..=1.0 upload Progress bar.' },
      { name: 'removable', rustType: 'bool', defaultValue: 'true', description: 'Shows trailing dismiss icon button.' },
    ],
  },
  {
    slug: 'image',
    name: 'Image',
    gpuiName: 'Image',
    category: 'Basic',
    part: 4,
    status: 'ready',
    summary:
      'Responsive image frame with aspect-ratio sizing, animated Shimmer loading skeleton, and broken-image fallback.',
    eguiSignature: 'Image::new(texture).aspect_ratio(16.0 / 9.0).rounding(8.0)',
    gpuiMappingNote:
      'Paints Option<egui::TextureId> inside an aspect-locked rect, delegating loading states cleanly to Shimmer.',
    rustFilePath: 'src/components/image.rs',
    usageRust: `use egui_kit::prelude::*;

ui.add(
    Image::new(self.preview_tex)
        .width(280.0)
        .aspect_ratio(16.0 / 9.0)
        .rounding(8.0)
        .loading(self.is_loading)
        .caption("Preview asset"),
);`,
    props: [
      { name: 'texture', rustType: 'Option<egui::TextureId>', defaultValue: 'required', description: 'Optional loaded GPU texture handle.' },
      { name: 'aspect_ratio', rustType: 'f32', defaultValue: '16.0 / 9.0', description: 'Width-to-height proportion.' },
      { name: 'rounding', rustType: 'f32', defaultValue: '8.0', description: 'Corner radius in logical pixels.' },
      { name: 'loading', rustType: 'bool', defaultValue: 'false', description: 'Renders animated Shimmer skeleton while true.' },
      { name: 'failed', rustType: 'bool', defaultValue: 'false', description: 'Shows geometric broken-image fallback.' },
    ],
  },
  {
    slug: 'calendar',
    name: 'Calendar',
    gpuiName: 'Calendar',
    category: 'Advanced',
    part: 4,
    status: 'ready',
    summary:
      'Zero-dependency interactive Gregorian month grid mutating &mut SimpleDate directly with fast Month/Year selectors and optional range selection.',
    eguiSignature: 'Calendar::new("cal", &mut date)',
    gpuiMappingNote:
      'Uses pure Gregorian arithmetic (zero chrono/time deps) and supports clickable Month & Decade/Year grids plus << / >> year steppers stored in egui::Memory.',
    rustFilePath: 'src/components/calendar.rs',
    usageRust: `use egui_kit::prelude::*;

ui.kit_calendar("single_cal", &mut self.date);

ui.add(Calendar::new("trip_range", &mut self.start).range(&mut self.end));`,
    props: [
      { name: 'id_source', rustType: '&str', defaultValue: 'required', description: 'Unique key for visible month/year state.' },
      { name: 'selected', rustType: '&mut SimpleDate', defaultValue: 'required', description: 'Selected start or single date.' },
      { name: 'range', rustType: '&mut SimpleDate', defaultValue: 'None', description: 'Optional range end date bound via .range().' },
      { name: 'cell_size', rustType: 'f32', defaultValue: '34.0', description: 'Day cell edge size in logical pixels.' },
    ],
  },
  {
    slug: 'date-picker',
    name: 'DatePicker',
    gpuiName: 'DatePicker',
    category: 'Form',
    part: 4,
    status: 'ready',
    summary:
      'Formatted YYYY-MM-DD trigger input paired with an eased foreground Calendar popover.',
    eguiSignature: 'ui.kit_date_picker("dob", &mut date)',
    gpuiMappingNote:
      'Anchors Calendar inside a slide-and-fade Order::Foreground Area that closes automatically on selection, Escape, or outside click.',
    rustFilePath: 'src/components/date_picker.rs',
    usageRust: `use egui_kit::prelude::*;

ui.kit_date_picker("release_date", &mut self.release_date);

ui.add(
    DatePicker::new("deadline", &mut self.deadline)
        .size(Size::Lg)
        .width(240.0),
);`,
    props: [
      { name: 'id_source', rustType: '&str', defaultValue: 'required', description: 'Unique key for popover open/close memory.' },
      { name: 'date', rustType: '&mut SimpleDate', defaultValue: 'required', description: 'Date value mutated directly when a day is picked.' },
      { name: 'size', rustType: 'Size', defaultValue: 'Md', description: 'Trigger height token (Xs, Sm, Md, Lg).' },
      { name: 'disabled', rustType: 'bool', defaultValue: 'false', description: 'Prevents opening calendar popover.' },
    ],
  },
  {
    slug: 'time-field',
    name: 'TimeField',
    gpuiName: 'TimeField',
    category: 'Form',
    part: 4,
    status: 'ready',
    summary:
      'Segmented HH:MM:SS time input mutating &mut SimpleTime with 12h/24h formats and ArrowUp/ArrowDown stepping.',
    eguiSignature: 'ui.kit_time_field("start", &mut time)',
    gpuiMappingNote:
      'Tracks active segment focus (hour/minute/second) inside egui::Memory with animated active highlight ring.',
    rustFilePath: 'src/components/time_field.rs',
    usageRust: `use egui_kit::prelude::*;

ui.kit_time_field("meeting_start", &mut self.start_time);

ui.add(
    TimeField::new("alarm", &mut self.alarm_time)
        .use_24h(false)
        .show_seconds(false),
);`,
    props: [
      { name: 'id_source', rustType: '&str', defaultValue: 'required', description: 'Unique key for focused HH/MM/SS segment.' },
      { name: 'time', rustType: '&mut SimpleTime', defaultValue: 'required', description: 'Time struct mutated directly by segment steppers.' },
      { name: 'use_24h', rustType: 'bool', defaultValue: 'true', description: 'Toggle between 24-hour clock and 12h AM/PM.' },
      { name: 'show_seconds', rustType: 'bool', defaultValue: 'true', description: 'Include trailing SS segment.' },
    ],
  },
  {
    slug: 'color-picker',
    name: 'ColorPicker',
    gpuiName: 'ColorPicker',
    category: 'Form',
    part: 4,
    status: 'ready',
    summary:
      'Interactive HSV saturation-value square, hue/alpha sliders, hex readout, and clickable theme swatches.',
    eguiSignature: 'ui.kit_color_picker("accent", &mut color)',
    gpuiMappingNote:
      'Draws GPU-interpolated saturation/value and rainbow hue meshes directly via egui::Mesh triangles.',
    rustFilePath: 'src/components/color_picker.rs',
    usageRust: `use egui_kit::prelude::*;

ui.kit_color_picker("brand_accent", &mut self.accent_color);

ui.add(
    ColorPicker::new("bg_tint", &mut self.bg_tint)
        .show_alpha(false)
        .width(260.0),
);`,
    props: [
      { name: 'id_source', rustType: '&str', defaultValue: 'required', description: 'Unique key preserving HSV hue across black/white drags.' },
      { name: 'color', rustType: '&mut egui::Color32', defaultValue: 'required', description: 'Color value mutated directly on drag or swatch click.' },
      { name: 'show_alpha', rustType: 'bool', defaultValue: 'true', description: 'Include horizontal alpha opacity mesh slider.' },
      { name: 'width', rustType: 'f32', defaultValue: '240.0', description: 'Picker card width in logical pixels.' },
    ],
  },
  {
    slug: 'marker',
    name: 'Marker',
    gpuiName: 'Marker',
    category: 'Basic',
    part: 4,
    status: 'ready',
    summary:
      'Spatial pin, callout, or timeline marker badge for annotations, maps, and release timelines.',
    eguiSignature: 'Marker::new("v0.4").tone(Variant::Primary)',
    gpuiMappingNote:
      'Paints Pin teardrop tip, pulsing Callout dot, or vertical Timeline node cleanly on egui::Painter.',
    rustFilePath: 'src/components/marker.rs',
    usageRust: `use egui_kit::prelude::*;

ui.kit_marker("v0.4");

ui.add(
    Marker::new("Live")
        .tone(Variant::Success)
        .style(MarkerStyle::Callout)
        .pulsing(true),
);`,
    props: [
      { name: 'label', rustType: 'impl Into<String>', defaultValue: 'required', description: 'Short marker caption or version tag.' },
      { name: 'style', rustType: 'MarkerStyle', defaultValue: 'Pin', description: 'Pin, Callout, or Timeline geometry.' },
      { name: 'tone', rustType: 'Variant', defaultValue: 'Primary', description: 'Semantic fill accent.' },
      { name: 'pulsing', rustType: 'bool', defaultValue: 'false', description: 'Animates concentric beacon ring on Callout markers.' },
    ],
  },
  {
    slug: 'speech',
    name: 'Speech',
    gpuiName: 'Speech',
    category: 'Form',
    part: 4,
    status: 'ready',
    summary:
      'Voice dictation control mutating &mut bool with dual concentric pulse rings, live MM:SS capture timer, and a recessed 7-bar spectrum well.',
    eguiSignature: 'ui.kit_speech("mic", &mut listening)',
    gpuiMappingNote:
      'Persists capture start timestamp in egui::Memory for live MM:SS readouts and drives dual acoustic rings plus 7 phase-offset equalizer bars at 60fps.',
    rustFilePath: 'src/components/speech.rs',
    usageRust: `use egui_kit::prelude::*;

if ui.kit_speech("voice_cmd", &mut self.listening).changed() {
    // toggle audio capture stream
}`,
    props: [
      { name: 'id_source', rustType: '&str', defaultValue: 'required', description: 'Unique key for listening state transitions.' },
      { name: 'listening', rustType: '&mut bool', defaultValue: 'required', description: 'Recording active state toggled directly on click.' },
      { name: 'label', rustType: 'Option<String>', defaultValue: 'None', description: 'Optional override for Listening... / Tap to speak copy.' },
      { name: 'disabled', rustType: 'bool', defaultValue: 'false', description: 'Disables dictation toggle and waveform motion.' },
    ],
  },

  // =========================================================================
  // PART 5 — Layout Containers, Disclosure & Navigation (10)
  // =========================================================================
  {
    slug: 'group-box',
    name: 'GroupBox',
    gpuiName: 'GroupBox',
    category: 'Layout',
    part: 5,
    status: 'ready',
    summary:
      'Titled border container grouping related settings or form controls cleanly inside a card or muted surface.',
    eguiSignature: 'ui.kit_group_box("Compiler Options", |ui| { ... })',
    gpuiMappingNote:
      'Wraps egui::Frame with a theme-aware header, hairline divider rule, and optional muted surface variant.',
    rustFilePath: 'src/components/group_box.rs',
    usageRust: `use egui_kit::prelude::*;

GroupBox::new("Compiler Options")
    .subtitle("Target profile settings")
    .show(ui, |ui| {
        ui.kit_checkbox(&mut self.lto, "Enable thin LTO");
        ui.kit_checkbox(&mut self.strip, "Strip debug symbols");
    });`,
    props: [
      { name: 'title', rustType: 'impl Into<String>', defaultValue: 'required', description: 'Section heading rendered above divider rule.' },
      { name: 'subtitle', rustType: 'impl Into<String>', defaultValue: 'None', description: 'Optional muted description below the title.' },
      { name: 'bordered', rustType: 'bool', defaultValue: 'true', description: 'Draws 1px ThemeTokens::border outline.' },
      { name: 'muted_surface', rustType: 'bool', defaultValue: 'false', description: 'Switches container fill from card to muted.' },
    ],
  },
  {
    slug: 'description-list',
    name: 'DescriptionList',
    gpuiName: 'DescriptionList',
    category: 'Layout',
    part: 5,
    status: 'ready',
    summary:
      'Structured key-value metadata grid supporting 1–3 columns, compact padding, and hairline bordered cells.',
    eguiSignature: 'ui.kit_description_list(&pairs)',
    gpuiMappingNote:
      'Chunks key-value pairs by column count and aligns monospace values cleanly beneath muted field labels.',
    rustFilePath: 'src/components/description_list.rs',
    usageRust: `use egui_kit::prelude::*;

let metadata = [
    ("Crate", "egui-kit 0.1.0"),
    ("Target", "wasm32-unknown-unknown"),
    ("Theme", "Tokyo Night"),
    ("Widgets", "50 / 80 ready"),
];

DescriptionList::new(&metadata).columns(2).bordered(true).show(ui);`,
    props: [
      { name: 'items', rustType: "&[(&'a str, &'a str)]", defaultValue: 'required', description: 'Key-value metadata pairs to lay out.' },
      { name: 'columns', rustType: 'usize', defaultValue: '2', description: 'Column tracks clamped between 1 and 3.' },
      { name: 'bordered', rustType: 'bool', defaultValue: 'true', description: 'Renders outer frame and internal cell hairlines.' },
      { name: 'compact', rustType: 'bool', defaultValue: 'false', description: 'Reduces vertical cell padding for dense inspectors.' },
    ],
  },
  {
    slug: 'grid-layout',
    name: 'GridLayout',
    gpuiName: 'Grid',
    category: 'Layout',
    part: 5,
    status: 'ready',
    summary:
      'Responsive column/row layout container computing equal fractional tracks and multi-column cell spans.',
    eguiSignature: 'GridLayout::columns(3).gap(12.0).show(ui, |g| {})',
    gpuiMappingNote:
      'Divides ui.available_width() into fractional tracks and automatically wraps rows when col_span overflows.',
    rustFilePath: 'src/components/grid_layout.rs',
    usageRust: `use egui_kit::prelude::*;

GridLayout::columns(3).gap(12.0).show(ui, |grid| {
    grid.cell(1, |ui| { ui.kit_button("1 col"); });
    grid.cell(2, |ui| { ui.kit_button("2 cols span"); });
    grid.cell(3, |ui| { ui.kit_label("Full-width 3 col row"); });
});`,
    props: [
      { name: 'columns', rustType: 'usize', defaultValue: 'required', description: 'Track count per row (clamped 1..=12).' },
      { name: 'gap', rustType: 'f32', defaultValue: '12.0', description: 'Uniform horizontal and vertical spacing in px.' },
      { name: 'cell(span, fn)', rustType: '(usize, FnOnce)', defaultValue: '—', description: 'Places a widget cell spanning 1..=columns.' },
    ],
  },
  {
    slug: 'scrollable',
    name: 'Scrollable',
    gpuiName: 'Scrollable',
    category: 'Layout',
    part: 5,
    status: 'ready',
    summary:
      'Themed scroll container wrapping ScrollArea with directional axes, max height, and stick-to-bottom log tailing.',
    eguiSignature: 'ui.kit_scrollable("log", 220.0, |ui| {})',
    gpuiMappingNote:
      'Pairs ThemeTokens frame styling with egui::ScrollArea auto-hide or always-visible scrollbars.',
    rustFilePath: 'src/components/scrollable.rs',
    usageRust: `use egui_kit::prelude::*;

Scrollable::vertical("build_log")
    .max_height(200.0)
    .stick_to_bottom(true)
    .show(ui, |ui| {
        for line in &self.logs {
            ui.label(line);
        }
    });`,
    props: [
      { name: 'id_source', rustType: 'impl Into<String>', defaultValue: 'required', description: 'Stable scroll offset ID.' },
      { name: 'max_height', rustType: 'f32', defaultValue: '220.0', description: 'Viewport height cap in logical pixels.' },
      { name: 'stick_to_bottom', rustType: 'bool', defaultValue: 'false', description: 'Pins scroll position to newest appended rows.' },
      { name: 'always_show_scroll', rustType: 'bool', defaultValue: 'false', description: 'Keeps scrollbar thumb permanently visible.' },
    ],
  },
  {
    slug: 'resizable',
    name: 'Resizable',
    gpuiName: 'Resizable',
    category: 'Layout',
    part: 5,
    status: 'ready',
    summary:
      'Split horizontal or vertical panels with draggable divider handle, cursor feedback, double-click reset, and ratio bounds.',
    eguiSignature: 'Resizable::horizontal("split").ratio(&mut ratio).show(ui, a, b)',
    gpuiMappingNote:
      'Persists split ratio in egui::Memory or optional &mut f32 and animates handle highlight on hover/drag.',
    rustFilePath: 'src/components/resizable.rs',
    usageRust: `use egui_kit::prelude::*;

Resizable::horizontal("ide_split")
    .ratio(&mut self.split_ratio)
    .bounds(0.2, 0.8)
    .height(220.0)
    .show(
        ui,
        |left| { left.kit_label("Explorer"); },
        |right| { right.kit_label("Editor Canvas"); },
    );`,
    props: [
      { name: 'id_source', rustType: '&str', defaultValue: 'required', description: 'Key for split ratio persistence.' },
      { name: 'ratio', rustType: '&mut f32', defaultValue: 'Memory (0.42)', description: 'Optional direct split fraction binding.' },
      { name: 'bounds', rustType: '(f32, f32)', defaultValue: '(0.2, 0.8)', description: 'Minimum and maximum split ratio clamps.' },
      { name: 'height', rustType: 'f32', defaultValue: '220.0', description: 'Container height in logical pixels.' },
    ],
  },
  {
    slug: 'accordion',
    name: 'Accordion',
    gpuiName: 'Accordion',
    category: 'Basic',
    part: 5,
    status: 'ready',
    summary:
      'Vertically stacked disclosure sections mutating &mut Option<usize> with rotating vector chevrons and smooth expansion.',
    eguiSignature: 'ui.kit_accordion("faq", &mut open_idx, &items)',
    gpuiMappingNote:
      'Animates chevron angle (0..90°) and section opacity via ctx.animate_bool_with_time().',
    rustFilePath: 'src/components/accordion.rs',
    usageRust: `use egui_kit::components::AccordionItem;
use egui_kit::prelude::*;

let items = [
    AccordionItem::new("Why egui-kit?", "Zero-boilerplate &mut bindings over gpui-kit tokens."),
    AccordionItem::new("Custom themes?", "Includes 36 synchronized presets via ThemeId."),
];

ui.kit_accordion("faq", &mut self.open_idx, &items);`,
    props: [
      { name: 'id_source', rustType: '&str', defaultValue: 'required', description: 'Base key for per-row hover/open animations.' },
      { name: 'open_idx', rustType: '&mut Option<usize>', defaultValue: 'required', description: 'Currently expanded row index, or None.' },
      { name: 'items', rustType: '&[AccordionItem]', defaultValue: 'required', description: 'Slice of section titles and body copy.' },
      { name: 'allow_collapse_all', rustType: 'bool', defaultValue: 'true', description: 'Allows clicking active header to close all rows.' },
    ],
  },
  {
    slug: 'collapsible',
    name: 'Collapsible',
    gpuiName: 'Collapsible',
    category: 'Basic',
    part: 5,
    status: 'ready',
    summary:
      'Single toggleable disclosure container persisting expanded state in egui::Memory or an optional &mut bool binding.',
    eguiSignature: 'ui.kit_collapsible("advanced", "Advanced", |ui| {})',
    gpuiMappingNote:
      'Provides zero-arg memory persistence by default while letting callers opt into .open(&mut bool).',
    rustFilePath: 'src/components/collapsible.rs',
    usageRust: `use egui_kit::prelude::*;

Collapsible::new("linker", "Advanced Linker Settings")
    .open(&mut self.show_linker)
    .show(ui, |ui| {
        ui.kit_checkbox(&mut self.lto, "Thin LTO");
        ui.kit_label("Target flags: -C opt-level=3");
    });`,
    props: [
      { name: 'id_source', rustType: '&str', defaultValue: 'required', description: 'Unique key for open/hover state.' },
      { name: 'title', rustType: 'impl Into<String>', defaultValue: 'required', description: 'Clickable disclosure header text.' },
      { name: 'open', rustType: '&mut bool', defaultValue: 'Memory (false)', description: 'Optional external bool binding.' },
      { name: 'default_open', rustType: 'bool', defaultValue: 'false', description: 'Initial expanded state when using Memory.' },
    ],
  },
  {
    slug: 'tabs',
    name: 'Tabs',
    gpuiName: 'Tabs',
    category: 'Advanced',
    part: 5,
    status: 'ready',
    summary:
      'Horizontal or vertical tab switcher mutating &mut usize with Underline, Pill, and Card styles, smooth sliding indicator, and closable tabs.',
    eguiSignature: 'ui.kit_tabs("editor_tabs", &mut active, &tabs)',
    gpuiMappingNote:
      'Interpolates active underline X position and width smoothly across tab changes with animate_value_with_time().',
    rustFilePath: 'src/components/tabs.rs',
    usageRust: `use egui_kit::components::TabsVariant;
use egui_kit::prelude::*;

let tabs = ["Overview", "Props", "Source"];
Tabs::new("doc_tabs", &mut self.active_tab, &tabs)
    .variant(TabsVariant::Underline)
    .show(ui);`,
    props: [
      { name: 'id_source', rustType: '&str', defaultValue: 'required', description: 'Unique key for indicator slide animation.' },
      { name: 'active', rustType: '&mut usize', defaultValue: 'required', description: 'Selected 0-based tab index.' },
      { name: 'labels', rustType: '&[T: AsRef<str>]', defaultValue: 'required', description: 'Tab label collection.' },
      { name: 'variant', rustType: 'TabsVariant', defaultValue: 'Underline', description: 'Underline, Pill, or Card tab styling.' },
      { name: 'closable', rustType: 'bool', defaultValue: 'false', description: 'Renders close icon and emits TabsResponse::closed.' },
    ],
  },
  {
    slug: 'stepper',
    name: 'Stepper',
    gpuiName: 'Stepper',
    category: 'Basic',
    part: 5,
    status: 'ready',
    summary:
      'Multi-step workflow progress indicator with completed checkmarks, active primary nodes, error badges, and animated connectors.',
    eguiSignature: 'ui.kit_stepper(&steps, active_step)',
    gpuiMappingNote:
      'Draws connected nodes horizontally or vertically and optionally binds clickable steps to &mut usize.',
    rustFilePath: 'src/components/stepper.rs',
    usageRust: `use egui_kit::prelude::*;

let steps = ["Select Crate", "Configure", "Verify", "Publish"];
Stepper::new(&steps, self.active_step)
    .show_clickable(ui, &mut self.active_step);`,
    props: [
      { name: 'steps', rustType: '&[T: AsRef<str>]', defaultValue: 'required', description: 'Ordered workflow step labels.' },
      { name: 'active_step', rustType: 'usize', defaultValue: 'required', description: 'Current 0-based workflow step index.' },
      { name: 'error_step', rustType: 'Option<usize>', defaultValue: 'None', description: 'Highlights a failing step in destructive color.' },
      { name: 'orientation', rustType: 'Orientation', defaultValue: 'Horizontal', description: 'Horizontal or Vertical step layout.' },
    ],
  },
  {
    slug: 'pagination',
    name: 'Pagination',
    gpuiName: 'Pagination',
    category: 'Basic',
    part: 5,
    status: 'ready',
    summary:
      'Compact 1-based page navigation bar mutating &mut usize with smart ellipsis truncation and tabular page numbers.',
    eguiSignature: 'ui.kit_pagination(&mut page, total_pages)',
    gpuiMappingNote:
      'Computes compact sibling window around active page and clamps navigation cleanly between 1 and total_pages.',
    rustFilePath: 'src/components/pagination.rs',
    usageRust: `use egui_kit::prelude::*;

if ui.kit_pagination(&mut self.page, 18).changed() {
    // reload paged query results
}`,
    props: [
      { name: 'page', rustType: '&mut usize', defaultValue: 'required', description: '1-based current page mutated directly.' },
      { name: 'total_pages', rustType: 'usize', defaultValue: 'required', description: 'Total available pages (min 1).' },
      { name: 'sibling_count', rustType: 'usize', defaultValue: '1', description: 'Page buttons shown on each side of active page.' },
    ],
  },

  // =========================================================================
  // PART 6 — Overlays, Popovers & Command Surfaces (10)
  // =========================================================================
  {
    slug: 'dialog',
    name: 'Dialog',
    gpuiName: 'Dialog',
    category: 'Layout',
    part: 6,
    status: 'ready',
    summary:
      'Accessible modal dialog window bound directly to &mut bool with dimmed backdrop scrim, Escape dismissal, and eased slide/fade motion.',
    eguiSignature: 'Dialog::new("prefs", &mut open).title("Preferences").show(ctx, |ui| {})',
    gpuiMappingNote:
      'Pairs Order::Middle scrim with Order::Foreground centered card and ctx.animate_bool_with_time() motion.',
    rustFilePath: 'src/components/dialog.rs',
    usageRust: `use egui_kit::prelude::*;

if ui.kit_button("Open Preferences").clicked() {
    self.dialog_open = true;
}

Dialog::new("workspace_prefs", &mut self.dialog_open)
    .title("Workspace Preferences")
    .description("Configure immediate-mode overlay defaults.")
    .width(440.0)
    .show(ctx, |ui| {
        ui.kit_input(&mut self.workspace_name, "Workspace name");
    });`,
    props: [
      { name: 'id_source', rustType: '&str', defaultValue: 'required', description: 'Unique key for scrim/card animation.' },
      { name: 'open', rustType: '&mut bool', defaultValue: 'required', description: 'Controls modal visibility directly.' },
      { name: 'title', rustType: 'impl Into<String>', defaultValue: '""', description: 'Header title displayed inside modal.' },
      { name: 'width', rustType: 'f32', defaultValue: '440.0', description: 'Target dialog width clamped to viewport.' },
      { name: 'dismiss_on_backdrop', rustType: 'bool', defaultValue: 'true', description: 'Closes dialog when scrim is clicked.' },
    ],
  },
  {
    slug: 'alert-dialog',
    name: 'AlertDialog',
    gpuiName: 'AlertDialog',
    category: 'Layout',
    part: 6,
    status: 'ready',
    summary:
      'High-intent confirmation modal blocking accidental outside-click dismissal for destructive or critical workflow actions.',
    eguiSignature: 'AlertDialog::confirm("Delete release tag?", &mut open).destructive(true).show(ctx)',
    gpuiMappingNote:
      'Consumes scrim clicks without closing &mut bool and returns AlertDialogResponse { confirmed, cancelled }.',
    rustFilePath: 'src/components/alert_dialog.rs',
    usageRust: `use egui_kit::prelude::*;

let res = AlertDialog::confirm("Delete release tag?", &mut self.alert_open)
    .description("This permanently removes the signed tag from the registry.")
    .destructive(true)
    .confirm_label("Delete Tag")
    .show(ctx);

if res.confirmed {
    self.delete_tag();
}`,
    props: [
      { name: 'title', rustType: 'impl Into<String>', defaultValue: 'required', description: 'Confirmation headline.' },
      { name: 'open', rustType: '&mut bool', defaultValue: 'required', description: 'Controls confirmation modal visibility.' },
      { name: 'destructive', rustType: 'bool', defaultValue: 'false', description: 'Renders destructive accent badge & CTA.' },
      { name: 'confirm_label', rustType: 'impl Into<String>', defaultValue: '"Confirm"', description: 'Primary action button label.' },
      { name: 'cancel_label', rustType: 'impl Into<String>', defaultValue: '"Cancel"', description: 'Secondary cancel button label.' },
    ],
  },
  {
    slug: 'sheet',
    name: 'Sheet',
    gpuiName: 'Sheet',
    category: 'Layout',
    part: 6,
    status: 'ready',
    summary:
      'Slide-over drawer panel anchored to Right, Left, Top, or Bottom viewport edges with backdrop scrim and Escape dismissal.',
    eguiSignature: 'Sheet::right("inspector", &mut open).title("Inspector").show(ctx, |ui| {})',
    gpuiMappingNote:
      'Animates edge offset smoothly using ease_out_cubic() inside an Order::Foreground Area.',
    rustFilePath: 'src/components/sheet.rs',
    usageRust: `use egui_kit::prelude::*;

Sheet::right("token_inspector", &mut self.sheet_open)
    .title("Theme Token Inspector")
    .subtitle("Active workspace palette")
    .extent(360.0)
    .show(ctx, |ui| {
        ui.kit_label("Inspecting 36 theme tokens.");
    });`,
    props: [
      { name: 'id_source', rustType: '&str', defaultValue: 'required', description: 'Unique key for slide transition.' },
      { name: 'open', rustType: '&mut bool', defaultValue: 'required', description: 'Drawer visibility binding.' },
      { name: 'side', rustType: 'SheetSide', defaultValue: 'Right', description: 'Right, Left, Top, or Bottom viewport edge.' },
      { name: 'extent', rustType: 'f32', defaultValue: '360.0', description: 'Width (Left/Right) or height (Top/Bottom) in px.' },
    ],
  },
  {
    slug: 'popover',
    name: 'Popover',
    gpuiName: 'Popover',
    category: 'Layout',
    part: 6,
    status: 'ready',
    summary:
      'Click-triggered floating surface anchored cleanly to any target Response with viewport-edge collision clamping.',
    eguiSignature: 'Popover::new("filters").side(PopoverSide::Bottom).show_for(ui, &btn, |ui| {})',
    gpuiMappingNote:
      'Persists open state in egui::Memory keyed by id_source and clamps card bounds inside ctx.screen_rect().',
    rustFilePath: 'src/components/popover.rs',
    usageRust: `use egui_kit::prelude::*;

let trigger = ui.kit_button("Filter Options");
Popover::new("filter_popover")
    .side(PopoverSide::Bottom)
    .width(260.0)
    .show_for(ui, &trigger, |ui| {
        ui.kit_checkbox(&mut self.only_ready, "Ready widgets only");
    });`,
    props: [
      { name: 'id_source', rustType: '&str', defaultValue: 'required', description: 'Unique key for open state in Memory.' },
      { name: 'side', rustType: 'PopoverSide', defaultValue: 'Bottom', description: 'Preferred anchor placement (Bottom, Top, Right, Left).' },
      { name: 'width', rustType: 'f32', defaultValue: '260.0', description: 'Floating card width in logical pixels.' },
    ],
  },
  {
    slug: 'tooltip',
    name: 'Tooltip',
    gpuiName: 'Tooltip',
    category: 'Basic',
    part: 6,
    status: 'ready',
    summary:
      'Themed hover/focus tooltip card replacing egui default tooltips with ThemeTokens styling and optional Kbd shortcut badges.',
    eguiSignature: 'response.kit_tooltip_shortcut("Save crate", "Cmd+S")',
    gpuiMappingNote:
      'Provides both Tooltip::new(...).show_for(&resp) and zero-boilerplate ResponseTooltipExt one-liners.',
    rustFilePath: 'src/components/tooltip.rs',
    usageRust: `use egui_kit::prelude::*;

let save_btn = ui.kit_button("Save Crate");
save_btn.kit_tooltip_shortcut("Save module changes", "Cmd+S");`,
    props: [
      { name: 'text', rustType: 'impl Into<String>', defaultValue: 'required', description: 'Tooltip caption text.' },
      { name: 'shortcut', rustType: 'Option<String>', defaultValue: 'None', description: 'Optional trailing Kbd shortcut chord.' },
    ],
  },
  {
    slug: 'hover-card',
    name: 'HoverCard',
    gpuiName: 'HoverCard',
    category: 'Layout',
    part: 6,
    status: 'ready',
    summary:
      'Delayed hover preview card for user profiles, crate mentions, and symbol docs with pointer-enter grace period.',
    eguiSignature: 'HoverCard::new("crate_preview").show_for(ui, &resp, |ui| {})',
    gpuiMappingNote:
      'Tracks enter_time and leave_time inside egui::Memory so moving the cursor onto the card keeps it open.',
    rustFilePath: 'src/components/hover_card.rs',
    usageRust: `use egui_kit::prelude::*;

let link = ui.kit_label("@egui-kit/overlays");
HoverCard::new("crate_hover")
    .open_delay(0.2)
    .show_for(ui, &link, |ui| {
        ui.kit_label("egui-kit v0.6.0");
        ui.label("60 / 80 immediate-mode widgets ready.");
    });`,
    props: [
      { name: 'id_source', rustType: '&str', defaultValue: 'required', description: 'Unique key for hover timer state.' },
      { name: 'open_delay', rustType: 'f64', defaultValue: '0.2', description: 'Seconds hovered before opening.' },
      { name: 'close_delay', rustType: 'f64', defaultValue: '0.16', description: 'Pointer-leave grace period in seconds.' },
      { name: 'width', rustType: 'f32', defaultValue: '280.0', description: 'Preview card width in logical pixels.' },
    ],
  },
  {
    slug: 'menu',
    name: 'Menu',
    gpuiName: 'Menu',
    category: 'Advanced',
    part: 6,
    status: 'ready',
    summary:
      'Hierarchical context and dropdown menu supporting shortcut actions, checkboxes, radio items, separators, and nested submenus.',
    eguiSignature: 'Menu::context(&response, &items)',
    gpuiMappingNote:
      'Nests Order::Tooltip Areas for cascading submenus and returns Option<String> clicked item IDs.',
    rustFilePath: 'src/components/menu.rs',
    usageRust: `use egui_kit::prelude::*;

let items = [
    MenuItem::action_shortcut("rename", "Rename Symbol", "F2"),
    MenuItem::checkbox("fmt", "Format on Save", self.format_on_save),
    MenuItem::Separator,
    MenuItem::submenu("export", "Export As", vec![
        MenuItem::action("rs", "Rust Module (.rs)"),
        MenuItem::action("json", "Theme JSON"),
    ]),
];

if let Some(id) = Menu::dropdown(ui, "editor_menu", &btn, &mut self.menu_open, &items) {
    self.handle_menu_action(&id);
}`,
    props: [
      { name: 'items', rustType: '&[MenuItem]', defaultValue: 'required', description: 'Action, Checkbox, Radio, Submenu, or Separator items.' },
      { name: 'Menu::context', rustType: '(&Response, &[MenuItem])', defaultValue: '—', description: 'Attaches right-click context menu to target.' },
      { name: 'Menu::dropdown', rustType: '(&mut Ui, &str, &Response, &mut bool, &[MenuItem])', defaultValue: '—', description: 'Controlled dropdown menu anchored to trigger.' },
    ],
  },
  {
    slug: 'notification',
    name: 'Notification',
    gpuiName: 'Notification',
    category: 'Layout',
    part: 6,
    status: 'ready',
    summary:
      'Edge-sliding screen toast stack with semantic tones, countdown progress bars, action buttons, and configurable corner placement.',
    eguiSignature: 'NotificationManager::push(ctx, Notification::success("Saved"))',
    gpuiMappingNote:
      'Stores up to 5 active toasts in ctx.data() and slides them smoothly from TopRight, BottomRight, TopLeft, or BottomLeft.',
    rustFilePath: 'src/components/notification.rs',
    usageRust: `use egui_kit::prelude::*;

if ui.kit_button("Publish Crate").clicked() {
    ui.kit_notify(
        Notification::success("Build Published")
            .message("All 60 widgets compiled cleanly.")
            .action("Inspect")
            .duration(5.0),
    );
}

NotificationManager::show(ctx, NotificationEdge::BottomRight);`,
    props: [
      { name: 'tone', rustType: 'NotificationTone', defaultValue: 'Info', description: 'Info, Success, Warning, or Destructive accent.' },
      { name: 'edge', rustType: 'NotificationEdge', defaultValue: 'BottomRight', description: 'TopRight, BottomRight, TopLeft, or BottomLeft slide origin.' },
      { name: 'duration', rustType: 'f64', defaultValue: '4.5', description: 'Auto-dismiss countdown in seconds.' },
      { name: 'action', rustType: 'Option<String>', defaultValue: 'None', description: 'Optional inline action button returning toast ID.' },
    ],
  },
  {
    slug: 'focus-trap',
    name: 'FocusTrap',
    gpuiName: 'FocusTrap',
    category: 'Layout',
    part: 6,
    status: 'ready',
    summary:
      'Keyboard focus containment scope wrapping Tab / Shift+Tab cleanly between bounded child IDs inside modal overlays.',
    eguiSignature: 'FocusTrap::new("modal_scope").active(true).show(ui, &ids, |ui| {})',
    gpuiMappingNote:
      'Intercepts Key::Tab / Shift+Tab against ordered boundary egui::Ids and optionally paints a ThemeTokens::ring outline.',
    rustFilePath: 'src/components/focus_trap.rs',
    usageRust: `use egui_kit::prelude::*;

let trap_ids = [egui::Id::new("field_a"), egui::Id::new("btn_ok")];
FocusTrap::new("modal_scope")
    .active(self.trap_active)
    .show_ring(true)
    .show(ui, &trap_ids, |ui| {
        ui.kit_input(&mut self.name, "Crate name");
        ui.kit_button("Apply");
    });`,
    props: [
      { name: 'id_source', rustType: '&str', defaultValue: 'required', description: 'Unique key for initial focus tracking.' },
      { name: 'active', rustType: 'bool', defaultValue: 'true', description: 'Enables Tab/Shift+Tab boundary wrapping.' },
      { name: 'show_ring', rustType: 'bool', defaultValue: 'false', description: 'Draws ThemeTokens::ring perimeter outline.' },
      { name: 'focusable_ids', rustType: '&[egui::Id]', defaultValue: 'required', description: 'Ordered child IDs defining trap boundaries.' },
    ],
  },
  {
    slug: 'command',
    name: 'Command',
    gpuiName: 'Command',
    category: 'Advanced',
    part: 6,
    status: 'ready',
    summary:
      'Fuzzy-searchable Cmd+K command palette with subsequence matching, Up/Down/Enter keyboard navigation, and Kbd shortcut badges.',
    eguiSignature: 'Command::new("palette", &mut open, &items).show(ctx)',
    gpuiMappingNote:
      'Subsequence-matches query against item labels and groups while persisting query and selection index in egui::Memory.',
    rustFilePath: 'src/components/command.rs',
    usageRust: `use egui_kit::prelude::*;

let commands = [
    CommandItem::new("theme", "Switch Theme", "Appearance").shortcut("Cmd+T"),
    CommandItem::new("build", "Run Cargo Check", "Workspace").shortcut("Cmd+B"),
];

if let Some(id) = ui.kit_command("global_cmd", &mut self.cmd_open, &commands) {
    self.execute_command(&id);
}`,
    props: [
      { name: 'id_source', rustType: '&str', defaultValue: 'required', description: 'Unique key for search query and selection index.' },
      { name: 'open', rustType: '&mut bool', defaultValue: 'required', description: 'Palette modal visibility binding.' },
      { name: 'items', rustType: '&[CommandItem]', defaultValue: 'required', description: 'Action descriptors with group & optional shortcut.' },
      { name: 'placeholder', rustType: 'impl Into<String>', defaultValue: '"Type a command..."', description: 'Search bar placeholder hint.' },
    ],
  },

  // =========================================================================
  // PART 7 — Desktop Shell, Docking, Code & Messaging (10)
  // =========================================================================
  plannedWidget('title-bar', 'TitleBar', 'Layout', 7, 'Custom borderless window title bar with platform traffic lights, drag zone, and slot actions.', 'TitleBar::new("egui-kit Studio").show(ui, |ui| {})', 'Integrates ViewportCommand::StartDrag and window controls.', 'TitleBar'),
  plannedWidget('sidebar', 'Sidebar', 'Layout', 7, 'Collapsible application navigation sidebar with section headers, nested groups, and footer.', 'Sidebar::new("nav", &mut collapsed).show(ui, |ui| {})', 'Animates width cleanly inside egui::SidePanel.'),
  plannedWidget('toolbar', 'Toolbar', 'Layout', 7, 'Compact horizontal/vertical action strip with separators, toggle groups, and overflow menu.', 'Toolbar::new().show(ui, |ui| {})', 'Measures overflowing children and moves them into a trailing menu.'),
  plannedWidget('status-bar', 'StatusBar', 'Layout', 7, 'Bottom IDE status bar hosting left/right status items, cursor coordinates, and branch triggers.', 'StatusBar::new().show(ui, |left, right| {})', 'Renders inside TopBottomPanel::bottom with compact 24px height.'),
  plannedWidget('dock', 'Dock', 'Advanced', 7, 'Full multi-pane IDE docking system with splitters, draggable tab groups, and panel zoom.', 'Dock::new("workspace", &mut dock_state).show(ui)', 'Maintains binary split tree + tab drag-drop zones in state.'),
  plannedWidget('settings', 'Settings', 'Advanced', 7, 'Searchable preferences dialog/panel with categorized sections and live-bound controls.', 'Settings::new(&categories, &mut config).show(ui)', 'Pairs left category nav with form rows and search filter.'),
  plannedWidget('editor', 'Editor', 'Form', 7, 'Code editor widget with line numbers, syntax highlighting, active line highlight, and diagnostics.', 'Editor::new(&mut source).language("rust")', 'Uses egui::text::LayoutJob for fast per-line syntax coloring.'),
  plannedWidget('text-view', 'TextView', 'Basic', 7, 'Rich selectable Markdown and code-block renderer supporting headings, tables, and links.', 'TextView::markdown(&doc_source)', 'Parses Markdown blocks into styled Galley spans with text selection.'),
  plannedWidget('diff', 'Diff', 'Advanced', 7, 'Unified and side-by-side code diff viewer with line numbers and added/removed gutter markers.', 'Diff::new(&old_src, &new_src).side_by_side(true)', 'Computes LCS line hunks and renders synchronized scroll panes.'),
  plannedWidget('bubble', 'Bubble', 'Basic', 7, 'Conversational chat bubble container with incoming/outgoing alignment, timestamp, and tail.', 'Bubble::outgoing("Hello egui!").timestamp("10:42")', 'Aligns max-width wrapped frame left or right with theme surfaces.'),

  // =========================================================================
  // PART 8 — Conversational Scrollers, Collections & Data Viz (10)
  // =========================================================================
  plannedWidget('message', 'Message', 'Basic', 8, 'Complete chat message row combining Avatar, Bubble, Markdown body, attachments, and actions.', 'Message::new(&msg).show(ui)', 'Composes Avatar, Bubble, TextView, and hover action toolbar.'),
  plannedWidget('message-scroller', 'MessageScroller', 'Basic', 8, 'Auto-stick-to-bottom chat timeline with "Jump to latest" pill and lazy history loading.', 'MessageScroller::new("chat", &messages).show(ui)', 'Tracks bottom-stick state across dynamic message height changes.'),
  plannedWidget('list', 'List', 'Advanced', 8, 'Keyboard-navigable item list with single/multi selection, section headers, and search filter.', 'List::new("files", &items, &mut selected)', 'Handles Up/Down/Home/End keys and focused row scroll-into-view.'),
  plannedWidget('virtual-list', 'VirtualList', 'Advanced', 8, 'High-performance virtualized list rendering 100,000+ variable-height rows at 60fps.', 'VirtualList::new("logs", row_count, |ui, idx| {})', 'Wraps ScrollArea::show_rows / cumulative height cache for O(visible) draw.'),
  plannedWidget('tree', 'Tree', 'Advanced', 8, 'Hierarchical expand/collapse tree view with indent guides, icons, and drag-and-drop.', 'Tree::new("fs", &mut root_nodes).show(ui)', 'Flattens visible tree nodes per frame for virtualized rendering.'),
  plannedWidget('table', 'Table', 'Advanced', 8, 'Clean styled data table with fixed headers, striped rows, and column alignment.', 'Table::new(&columns, &rows).striped(true)', 'Uses custom Painter cells with tabular numerals.', 'Table'),
  plannedWidget('data-table', 'DataTable', 'Advanced', 8, 'Enterprise data grid with column sorting, resizing, pinned columns, pagination, and selection.', 'DataTable::new("users", &mut table_state)', 'Supports sortable headers, drag column dividers, and sticky left/right cols.'),
  plannedWidget('chart', 'Chart', 'Advanced', 8, 'Declarative bar, line, area, and pie charts using theme data-1..5 palette and hover tooltips.', 'Chart::line(&series).show(ui)', 'Paints crisp anti-aliased paths and interactive crosshair tooltips.'),
  plannedWidget('plot', 'Plot', 'Advanced', 8, 'Scientific 2D coordinate plot canvas with zoom/pan, grid axes, legends, and scatter/function series.', 'Plot::new("signal").show(ui, |p| {})', 'Maps world-to-screen transforms cleanly inside Painter clip rect.', 'Plot'),
  plannedWidget('carousel', 'Carousel', 'Advanced', 8, 'Horizontal slide carousel with prev/next controls, pagination indicators, and drag swipe.', 'Carousel::new("slides", &mut index, slide_count)', 'Interpolates horizontal slide offset smoothly via ctx.animate_value().'),
];
