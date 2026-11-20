import type { PartInfo, PartNumber, ThemePreset, WidgetCategory, WidgetSpec } from '../types/kit';

export const PARTS_ROADMAP: PartInfo[] = [
  { part: 1, title: 'Part 1: Core Foundation & Primitives', focus: 'Root, Theme engine, and 8 core UI primitives', status: 'completed' },
  { part: 2, title: 'Part 2: Selection & Feedback States', focus: 'Checkbox, Radio, Switch, Slider, Progress, and Skeletons', status: 'completed' },
  { part: 3, title: 'Part 3: Text Inputs & Form Controls', focus: 'Input, TextArea, Select, Combobox, OTP, and Forms', status: 'completed' },
  { part: 4, title: 'Part 4: Alerts, Media & Pickers', focus: 'Alert, Calendar, DatePicker, ColorPicker, and Speech', status: 'completed' },
  { part: 5, title: 'Part 5: Layout, Disclosure & Navigation', focus: 'Accordion, Tabs, Stepper, Pagination, and Resizable', status: 'completed' },
  { part: 6, title: 'Part 6: Overlays & Command Surfaces', focus: 'Dialog, Sheet, Popover, Tooltip, Menu, and Command', status: 'completed' },
  { part: 7, title: 'Part 7: Desktop Shell, Code & Chat', focus: 'TitleBar, Sidebar, Dock, Editor, Diff, and Bubble', status: 'scheduled' },
  { part: 8, title: 'Part 8: Collections & Data Visualization', focus: 'VirtualList, Tree, DataTable, Chart, Plot, and Carousel', status: 'scheduled' },
];

export interface WidgetFilterOptions {
  query?: string;
  category?: WidgetCategory | 'All';
  part?: PartNumber | 'All';
  onlyReady?: boolean;
}

export function filterWidgets(widgets: WidgetSpec[], options: WidgetFilterOptions): WidgetSpec[] {
  const q = (options.query ?? '').trim().toLowerCase();
  return widgets.filter((widget) => {
    if (options.onlyReady && widget.status !== 'ready') return false;
    if (options.category && options.category !== 'All' && widget.category !== options.category) {
      return false;
    }
    if (options.part && options.part !== 'All' && widget.part !== options.part) {
      return false;
    }
    if (q.length > 0) {
      const hay = `${widget.name} ${widget.slug} ${widget.summary} ${widget.category}`.toLowerCase();
      return hay.includes(q);
    }
    return true;
  });
}

export function groupWidgetsByPart(widgets: WidgetSpec[]): Map<PartNumber, WidgetSpec[]> {
  const groups = new Map<PartNumber, WidgetSpec[]>();
  for (let p = 1 as PartNumber; p <= 8; p = (p + 1) as PartNumber) {
    groups.set(p, []);
  }
  for (const widget of widgets) {
    const list = groups.get(widget.part) ?? [];
    list.push(widget);
    groups.set(widget.part, list);
  }
  return groups;
}

export function countByStatus(widgets: WidgetSpec[]): { ready: number; planned: number; total: number } {
  let ready = 0;
  let planned = 0;
  for (const w of widgets) {
    if (w.status === 'ready') ready += 1;
    else planned += 1;
  }
  return { ready, planned, total: widgets.length };
}

export function filterThemes(
  themes: ThemePreset[],
  mode: 'all' | 'light' | 'dark',
  query: string
): ThemePreset[] {
  const q = query.trim().toLowerCase();
  return themes.filter((t) => {
    if (mode !== 'all' && t.mode !== mode) return false;
    if (q.length > 0) {
      return t.name.toLowerCase().includes(q) || t.id.toLowerCase().includes(q) || t.source.toLowerCase().includes(q);
    }
    return true;
  });
}

export function buildRustThemeSnippet(theme: ThemePreset): string {
  const v = (key: string, fallback: string) => theme.vars[key] || fallback;
  const modeEnum = theme.mode === 'dark' ? 'ThemeMode::Dark' : 'ThemeMode::Light';
  return `use egui_kit::{ThemeId, ThemeMode, ThemeTokens};
use egui_kit::theme::hex;

pub fn ${theme.id.replace(/[^a-zA-Z0-9]+/g, '_')}_tokens() -> ThemeTokens {
    ThemeTokens {
        id: ThemeId::AuroraLight, // Custom preset: "${theme.name}"
        mode: ${modeEnum},
        background: hex("${v('--background', '#ffffff')}"),
        foreground: hex("${v('--foreground', '#0f172a')}"),
        card: hex("${v('--card', '#ffffff')}"),
        card_foreground: hex("${v('--card-foreground', '#0f172a')}"),
        primary: hex("${v('--primary', '#0f172a')}"),
        primary_foreground: hex("${v('--primary-foreground', '#f8fafc')}"),
        secondary: hex("${v('--secondary', '#e2e8f0')}"),
        secondary_foreground: hex("${v('--secondary-foreground', '#0f172a')}"),
        muted: hex("${v('--muted', '#f1f5f9')}"),
        muted_foreground: hex("${v('--muted-foreground', '#64748b')}"),
        accent: hex("${v('--accent', '#f1f5f9')}"),
        accent_foreground: hex("${v('--accent-foreground', '#0f172a')}"),
        border: hex("${v('--border', '#e2e8f0')}"),
        input: hex("${v('--input', '#cbd5e1')}"),
        ring: hex("${v('--ring', '#0f172a')}"),
        success: hex("${v('--success', '#16a34a')}"),
        warning: hex("${v('--warning', '#ca8a04')}"),
        destructive: hex("${v('--destructive', '#dc2626')}"),
    }
}`;
}

export function formatShortcutKeys(shortcut: string, macosSymbols: boolean): string[] {
  return shortcut
    .split('+')
    .map((raw) => raw.trim())
    .filter(Boolean)
    .map((token) => {
      if (!macosSymbols) return token;
      switch (token.toLowerCase()) {
        case 'cmd':
        case 'command':
        case 'super':
        case 'meta':
          return '⌘';
        case 'shift':
          return '⇧';
        case 'alt':
        case 'opt':
        case 'option':
          return '⌥';
        case 'ctrl':
        case 'control':
          return '⌃';
        case 'enter':
        case 'return':
          return '↵';
        case 'esc':
        case 'escape':
          return '⎋';
        case 'tab':
          return '⇥';
        case 'backspace':
        case 'back':
        case 'delete':
        case 'del':
          return '⌫';
        case 'space':
        case 'spacebar':
          return '␣';
        case 'up':
        case 'arrowup':
          return '↑';
        case 'down':
        case 'arrowdown':
          return '↓';
        case 'left':
        case 'arrowleft':
          return '←';
        case 'right':
        case 'arrowright':
          return '→';
        default:
          return token.toUpperCase();
      }
    });
}

export type KbdIconKind =
  | 'command'
  | 'shift'
  | 'option'
  | 'control'
  | 'enter'
  | 'escape'
  | 'tab'
  | 'backspace'
  | 'space'
  | 'arrowUp'
  | 'arrowDown'
  | 'arrowLeft'
  | 'arrowRight';

export interface KbdToken {
  raw: string;
  label?: string;
  icon?: KbdIconKind;
  ariaLabel: string;
}

export function parseKbdTokens(shortcut: string, macosSymbols: boolean): KbdToken[] {
  return shortcut
    .split('+')
    .map((raw) => raw.trim())
    .filter(Boolean)
    .map((raw) => {
      const lower = raw.toLowerCase();
      let icon: KbdIconKind | undefined;
      let standardLabel: string | undefined;
      let ariaLabel = raw;

      switch (lower) {
        case 'cmd':
        case 'command':
        case 'super':
        case 'meta':
          icon = 'command';
          standardLabel = 'Super';
          ariaLabel = macosSymbols ? 'Command' : 'Super';
          break;
        case 'shift':
          icon = 'shift';
          standardLabel = 'Shift';
          ariaLabel = 'Shift';
          break;
        case 'alt':
        case 'opt':
        case 'option':
          icon = 'option';
          standardLabel = 'Alt';
          ariaLabel = macosSymbols ? 'Option' : 'Alt';
          break;
        case 'ctrl':
        case 'control':
          icon = 'control';
          standardLabel = 'Ctrl';
          ariaLabel = 'Control';
          break;
        case 'enter':
        case 'return':
          icon = 'enter';
          standardLabel = 'Enter';
          ariaLabel = macosSymbols ? 'Return' : 'Enter';
          break;
        case 'esc':
        case 'escape':
          icon = 'escape';
          standardLabel = 'Esc';
          ariaLabel = 'Escape';
          break;
        case 'tab':
          icon = 'tab';
          standardLabel = 'Tab';
          ariaLabel = 'Tab';
          break;
        case 'backspace':
        case 'back':
          icon = 'backspace';
          standardLabel = 'Back';
          ariaLabel = 'Backspace';
          break;
        case 'delete':
        case 'del':
          icon = 'backspace';
          standardLabel = 'Del';
          ariaLabel = 'Delete';
          break;
        case 'space':
        case 'spacebar':
          icon = 'space';
          standardLabel = 'Space';
          ariaLabel = 'Space';
          break;
        case 'up':
        case 'arrowup':
          icon = 'arrowUp';
          ariaLabel = 'Up Arrow';
          break;
        case 'down':
        case 'arrowdown':
          icon = 'arrowDown';
          ariaLabel = 'Down Arrow';
          break;
        case 'left':
        case 'arrowleft':
          icon = 'arrowLeft';
          ariaLabel = 'Left Arrow';
          break;
        case 'right':
        case 'arrowright':
          icon = 'arrowRight';
          ariaLabel = 'Right Arrow';
          break;
        default:
          return {
            raw,
            label: macosSymbols ? raw.toUpperCase() : raw,
            ariaLabel: raw,
          };
      }

      if (macosSymbols) {
        return { raw, icon, ariaLabel };
      }

      return {
        raw,
        icon,
        label: standardLabel,
        ariaLabel,
      };
    });
}

export function snapSliderValue(raw: number, min: number, max: number, step?: number): number {
  const clamped = Math.min(max, Math.max(min, raw));
  if (!step || step <= 0) return clamped;
  const snapped = Math.round((clamped - min) / step) * step + min;
  return Number(Math.min(max, Math.max(min, snapped)).toFixed(4));
}

export function clampProgressFraction(value: number): number {
  if (!Number.isFinite(value)) return 0;
  return Math.min(1, Math.max(0, value));
}

export function clampNumberInput(value: number, min: number, max: number, step = 1): number {
  if (!Number.isFinite(value)) return min;
  const clamped = Math.min(max, Math.max(min, value));
  if (!Number.isFinite(step) || step <= 0) return clamped;
  const snapped = Math.round((clamped - min) / step) * step + min;
  return Number(Math.min(max, Math.max(min, snapped)).toFixed(4));
}

export function distributeOtpInput(raw: string, length: number): string {
  const digits = raw.replace(/\D/g, '');
  return digits.slice(0, Math.max(1, length));
}

export function filterComboboxOptions<T extends { label: string; meta?: string }>(
  options: readonly T[],
  query: string
): T[] {
  const q = query.trim().toLowerCase();
  if (!q) return [...options];
  return options.filter(
    (opt) =>
      opt.label.toLowerCase().includes(q) || (opt.meta ? opt.meta.toLowerCase().includes(q) : false)
  );
}

export interface FormValidationErrors {
  crateName?: string;
  email?: string;
}

export function validateFormFields(values: {
  crateName: string;
  email: string;
}): FormValidationErrors {
  const errors: FormValidationErrors = {};
  const name = values.crateName.trim();
  const email = values.email.trim();
  if (name.length < 2) {
    errors.crateName = 'Crate name must be at least 2 characters.';
  } else if (!/^[a-z0-9_-]+$/i.test(name)) {
    errors.crateName = 'Use only letters, numbers, hyphens, or underscores.';
  }
  if (!email || !/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(email)) {
    errors.email = 'Enter a valid maintainer email address.';
  }
  return errors;
}

export function formatByteSize(bytes: number): string {
  if (!Number.isFinite(bytes) || bytes <= 0) return '0 B';
  const units = ['B', 'KB', 'MB', 'GB'];
  let idx = 0;
  let value = bytes;
  while (value >= 1024 && idx < units.length - 1) {
    value /= 1024;
    idx += 1;
  }
  return idx === 0 ? `${Math.round(value)} B` : `${value.toFixed(1)} ${units[idx]}`;
}

export function formatYmd(year: number, month1: number, day: number): string {
  const m = String(Math.max(1, Math.min(12, month1))).padStart(2, '0');
  const d = String(Math.max(1, Math.min(31, day))).padStart(2, '0');
  return `${year}-${m}-${d}`;
}

export interface CalendarCell {
  year: number;
  month1: number;
  day: number;
  inMonth: boolean;
  ymd: string;
}

export function getMonthGrid(year: number, monthIndex0: number): CalendarCell[] {
  const firstWeekday = new Date(Date.UTC(year, monthIndex0, 1)).getUTCDay();
  const daysInMonth = new Date(Date.UTC(year, monthIndex0 + 1, 0)).getUTCDate();
  const prevMonthDays = new Date(Date.UTC(year, monthIndex0, 0)).getUTCDate();
  const cells: CalendarCell[] = [];

  for (let i = 0; i < 42; i += 1) {
    if (i < firstWeekday) {
      const d = prevMonthDays - firstWeekday + 1 + i;
      const dt = new Date(Date.UTC(year, monthIndex0 - 1, d));
      const y = dt.getUTCFullYear();
      const m1 = dt.getUTCMonth() + 1;
      cells.push({ year: y, month1: m1, day: d, inMonth: false, ymd: formatYmd(y, m1, d) });
    } else if (i < firstWeekday + daysInMonth) {
      const d = i - firstWeekday + 1;
      const m1 = monthIndex0 + 1;
      cells.push({ year, month1: m1, day: d, inMonth: true, ymd: formatYmd(year, m1, d) });
    } else {
      const d = i - (firstWeekday + daysInMonth) + 1;
      const dt = new Date(Date.UTC(year, monthIndex0 + 1, d));
      const y = dt.getUTCFullYear();
      const m1 = dt.getUTCMonth() + 1;
      cells.push({ year: y, month1: m1, day: d, inMonth: false, ymd: formatYmd(y, m1, d) });
    }
  }
  return cells;
}

export interface SimpleTimeValue {
  hours: number;
  minutes: number;
  seconds: number;
}

export function stepTimeSegment(
  time: SimpleTimeValue,
  segment: 'hours' | 'minutes' | 'seconds',
  delta: number
): SimpleTimeValue {
  const mod = (n: number, m: number) => ((n % m) + m) % m;
  return {
    hours: segment === 'hours' ? mod(time.hours + delta, 24) : time.hours,
    minutes: segment === 'minutes' ? mod(time.minutes + delta, 60) : time.minutes,
    seconds: segment === 'seconds' ? mod(time.seconds + delta, 60) : time.seconds,
  };
}

export function hsvToRgbHex(h: number, s: number, v: number): { r: number; g: number; b: number; hex: string } {
  const hue = ((h % 360) + 360) % 360;
  const sat = Math.min(1, Math.max(0, s));
  const val = Math.min(1, Math.max(0, v));
  const c = val * sat;
  const x = c * (1 - Math.abs(((hue / 60) % 2) - 1));
  const m = val - c;
  let r1 = 0;
  let g1 = 0;
  let b1 = 0;

  if (hue < 60) [r1, g1, b1] = [c, x, 0];
  else if (hue < 120) [r1, g1, b1] = [x, c, 0];
  else if (hue < 180) [r1, g1, b1] = [0, c, x];
  else if (hue < 240) [r1, g1, b1] = [0, x, c];
  else if (hue < 300) [r1, g1, b1] = [x, 0, c];
  else [r1, g1, b1] = [c, 0, x];

  const r = Math.round((r1 + m) * 255);
  const g = Math.round((g1 + m) * 255);
  const b = Math.round((b1 + m) * 255);
  const toHex = (n: number) => n.toString(16).padStart(2, '0').toUpperCase();
  return { r, g, b, hex: `#${toHex(r)}${toHex(g)}${toHex(b)}` };
}

export function getCalendarYearOptions(minYear = 1950, maxYear = 2050): number[] {
  const start = Math.min(minYear, maxYear);
  const end = Math.max(minYear, maxYear);
  const years: number[] = [];
  for (let y = end; y >= start; y -= 1) {
    years.push(y);
  }
  return years;
}

export function getDecadeWindow(year: number): number[] {
  const base = Math.floor(year / 12) * 12;
  return Array.from({ length: 12 }, (_, idx) => base + idx);
}

export type PaginationToken = number | 'ellipsis';

export function computePaginationItems(
  currentPage: number,
  totalPages: number,
  siblingCount = 1
): PaginationToken[] {
  const total = Math.max(1, Math.floor(totalPages));
  const current = Math.min(total, Math.max(1, Math.floor(currentPage)));
  const siblings = Math.min(3, Math.max(0, Math.floor(siblingCount)));

  if (total <= 5 + siblings * 2) {
    return Array.from({ length: total }, (_, idx) => idx + 1);
  }

  const out: PaginationToken[] = [1];
  const left = Math.max(2, current - siblings);
  const right = Math.min(total - 1, current + siblings);

  if (left > 2) out.push('ellipsis');
  for (let p = left; p <= right; p += 1) {
    out.push(p);
  }
  if (right < total - 1) out.push('ellipsis');
  out.push(total);
  return out;
}

export function clampSplitRatio(ratio: number, min = 0.2, max = 0.8): number {
  if (!Number.isFinite(ratio)) return min;
  return Number(Math.min(max, Math.max(min, ratio)).toFixed(4));
}

export function stepAccordionIndex(
  current: number | null,
  clicked: number,
  allowEmpty = true
): number | null {
  if (current === clicked) {
    return allowEmpty ? null : clicked;
  }
  return clicked;
}

export function getNextStepIndex(current: number, totalSteps: number, delta: number): number {
  if (totalSteps <= 1) return 0;
  return Math.min(totalSteps - 1, Math.max(0, current + delta));
}

export function fuzzyMatchesQuery(query: string, target: string): boolean {
  const q = query.trim().toLowerCase();
  if (!q) return true;
  const t = target.toLowerCase();
  let qi = 0;
  for (let ti = 0; ti < t.length && qi < q.length; ti += 1) {
    if (t[ti] === q[qi]) {
      qi += 1;
    }
  }
  return qi === q.length;
}

export function filterCommandItems<T extends { label: string; group: string }>(
  items: readonly T[],
  query: string
): T[] {
  if (!query.trim()) return [...items];
  return items.filter(
    (item) => fuzzyMatchesQuery(query, item.label) || fuzzyMatchesQuery(query, item.group)
  );
}

export function clampNotificationStack<T>(items: readonly T[], maxItems = 5): T[] {
  const cap = Math.max(1, Math.floor(maxItems));
  if (items.length <= cap) return [...items];
  return items.slice(items.length - cap);
}

export function cycleFocusIndex(current: number | null, total: number, backwards: boolean): number {
  if (total <= 0) return 0;
  if (current === null || current < 0 || current >= total) {
    return backwards ? total - 1 : 0;
  }
  return backwards ? (current - 1 + total) % total : (current + 1) % total;
}

export interface FuzzySpan {
  text: string;
  match: boolean;
}

export function splitFuzzyMatchSpans(query: string, target: string): FuzzySpan[] {
  const q = query.trim().toLowerCase();
  if (!q || !target) return target ? [{ text: target, match: false }] : [];

  const tLower = target.toLowerCase();
  const matchedIndices: number[] = [];
  let qi = 0;
  for (let ti = 0; ti < tLower.length && qi < q.length; ti += 1) {
    if (tLower[ti] === q[qi]) {
      matchedIndices.push(ti);
      qi += 1;
    }
  }

  if (qi < q.length) {
    return [{ text: target, match: false }];
  }

  const matchSet = new Set(matchedIndices);
  const spans: FuzzySpan[] = [];
  let buf = '';
  let currentMatch = matchSet.has(0);

  for (let i = 0; i < target.length; i += 1) {
    const isMatch = matchSet.has(i);
    if (isMatch !== currentMatch) {
      if (buf) spans.push({ text: buf, match: currentMatch });
      buf = target[i];
      currentMatch = isMatch;
    } else {
      buf += target[i];
    }
  }
  if (buf) spans.push({ text: buf, match: currentMatch });
  return spans;
}
