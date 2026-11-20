import test from 'node:test';
import assert from 'node:assert/strict';
import {
  buildRustThemeSnippet,
  clampProgressFraction,
  countByStatus,
  filterThemes,
  filterWidgets,
  formatShortcutKeys,
  groupWidgetsByPart,
  parseKbdTokens,
  snapSliderValue,
} from './catalogLogic';
import type { ThemePreset, WidgetSpec } from '../types/kit';

const SAMPLE_WIDGETS: WidgetSpec[] = [
  {
    slug: 'button',
    name: 'Button',
    gpuiName: 'Button',
    category: 'Basic',
    part: 1,
    status: 'ready',
    summary: 'Clickable action trigger with variants.',
    eguiSignature: 'Button::new("Save").variant(ButtonVariant::Primary)',
    gpuiMappingNote: 'Maps GPUI click listener to egui::Response::clicked().',
    usageRust: 'ui.add(Button::new("Save"));',
    rustFilePath: 'src/components/button.rs',
    props: [],
  },
  {
    slug: 'checkbox',
    name: 'Checkbox',
    gpuiName: 'Checkbox',
    category: 'Form',
    part: 2,
    status: 'ready',
    summary: 'Boolean toggle box.',
    eguiSignature: 'Checkbox::new(&mut checked, "Accept")',
    gpuiMappingNote: 'Mutates &mut bool directly.',
    usageRust: 'ui.add(Checkbox::new(&mut checked, "Accept"));',
    rustFilePath: 'src/components/checkbox.rs',
    props: [],
  },
  {
    slug: 'dock',
    name: 'Dock',
    gpuiName: 'DockArea',
    category: 'Advanced',
    part: 7,
    status: 'planned',
    summary: 'Split pane docking layout.',
    eguiSignature: 'Dock::new("workspace", &mut tree)',
    gpuiMappingNote: 'Recursive split tree in egui::Memory.',
    usageRust: 'Dock::new("workspace", &mut tree).ui(ui);',
    rustFilePath: 'src/components/dock.rs',
    props: [],
  },
];

const SAMPLE_THEMES: ThemePreset[] = [
  {
    id: 'aurora-aurora-light',
    name: 'Aurora Light',
    mode: 'light',
    source: 'aurora',
    vars: { '--background': '#ffffff', '--primary': '#0f172a' },
  },
  {
    id: 'tokyonight-tokyo-night',
    name: 'Tokyo Night',
    mode: 'dark',
    source: 'tokyonight',
    vars: { '--background': '#1a1b26', '--primary': '#7aa2f7' },
  },
];

test('filterWidgets filters by query, category, part, and ready status', () => {
  assert.equal(filterWidgets(SAMPLE_WIDGETS, { query: 'butt' }).length, 1);
  assert.equal(filterWidgets(SAMPLE_WIDGETS, { category: 'Form' }).length, 1);
  assert.equal(filterWidgets(SAMPLE_WIDGETS, { part: 7 }).length, 1);
  assert.equal(filterWidgets(SAMPLE_WIDGETS, { onlyReady: true }).length, 2);
});

test('groupWidgetsByPart initializes all 8 parts and distributes widgets', () => {
  const grouped = groupWidgetsByPart(SAMPLE_WIDGETS);
  assert.equal(grouped.size, 8);
  assert.equal(grouped.get(1)?.length, 1);
  assert.equal(grouped.get(2)?.length, 1);
  assert.equal(grouped.get(7)?.length, 1);
  assert.equal(grouped.get(8)?.length, 0);
});

test('countByStatus tallies ready and planned widgets accurately', () => {
  assert.deepEqual(countByStatus(SAMPLE_WIDGETS), { ready: 2, planned: 1, total: 3 });
});

test('filterThemes filters by mode and text query', () => {
  assert.equal(filterThemes(SAMPLE_THEMES, 'light', '').length, 1);
  assert.equal(filterThemes(SAMPLE_THEMES, 'dark', 'tokyo').length, 1);
  assert.equal(filterThemes(SAMPLE_THEMES, 'all', 'unknown').length, 0);
});

test('buildRustThemeSnippet emits valid ThemeTokens Rust code with theme hex colors', () => {
  const snippet = buildRustThemeSnippet(SAMPLE_THEMES[1]);
  assert.match(snippet, /ThemeMode::Dark/);
  assert.match(snippet, /hex\("#1a1b26"\)/);
  assert.match(snippet, /hex\("#7aa2f7"\)/);
});

test('formatShortcutKeys formats macOS glyphs vs standard labels', () => {
  assert.deepEqual(formatShortcutKeys('Cmd+Shift+K', true), ['⌘', '⇧', 'K']);
  assert.deepEqual(formatShortcutKeys('Ctrl+Shift+K', false), ['Ctrl', 'Shift', 'K']);
});

test('parseKbdTokens resolves vector icons for modifiers, navigation, and arrows', () => {
  assert.deepEqual(parseKbdTokens('Cmd+Shift+P', true), [
    { raw: 'Cmd', icon: 'command', ariaLabel: 'Command' },
    { raw: 'Shift', icon: 'shift', ariaLabel: 'Shift' },
    { raw: 'P', label: 'P', ariaLabel: 'P' },
  ]);
  assert.deepEqual(parseKbdTokens('Ctrl+Alt+Enter', false), [
    { raw: 'Ctrl', icon: 'control', label: 'Ctrl', ariaLabel: 'Control' },
    { raw: 'Alt', icon: 'option', label: 'Alt', ariaLabel: 'Alt' },
    { raw: 'Enter', icon: 'enter', label: 'Enter', ariaLabel: 'Enter' },
  ]);
  assert.deepEqual(parseKbdTokens('Esc+Tab+Up', true), [
    { raw: 'Esc', icon: 'escape', ariaLabel: 'Escape' },
    { raw: 'Tab', icon: 'tab', ariaLabel: 'Tab' },
    { raw: 'Up', icon: 'arrowUp', ariaLabel: 'Up Arrow' },
  ]);
});

test('snapSliderValue clamps bounds and snaps to step increments', () => {
  assert.equal(snapSliderValue(37, 0, 100, 5), 35);
  assert.equal(snapSliderValue(120, 0, 100, 5), 100);
  assert.equal(snapSliderValue(-10, 0, 100, 5), 0);
  assert.equal(snapSliderValue(42.5, 0, 100), 42.5);
});

test('clampProgressFraction clamps finite numbers to 0..1 and guards NaN', () => {
  assert.equal(clampProgressFraction(0.45), 0.45);
  assert.equal(clampProgressFraction(-0.2), 0);
  assert.equal(clampProgressFraction(1.4), 1);
  assert.equal(clampProgressFraction(Number.NaN), 0);
});

import {
  clampNumberInput,
  distributeOtpInput,
  filterComboboxOptions,
  validateFormFields,
} from './catalogLogic';

test('clampNumberInput clamps finite values and snaps to step cleanly', () => {
  assert.equal(clampNumberInput(150, 0, 100, 5), 100);
  assert.equal(clampNumberInput(-4, 0, 100, 5), 0);
  assert.equal(clampNumberInput(12.3, 0, 100, 0.5), 12.5);
  assert.equal(clampNumberInput(Number.NaN, 10, 50, 1), 10);
});

test('distributeOtpInput splits pasted code across fixed-length digit slots', () => {
  assert.equal(distributeOtpInput('48-29 10', 6), '482910');
  assert.equal(distributeOtpInput('12345678', 6), '123456');
  assert.equal(distributeOtpInput('abc', 6), '');
});

test('filterComboboxOptions matches label or metadata case-insensitively', () => {
  const opts = [
    { label: 'Tokyo Night', meta: 'Dark' },
    { label: 'Aurora Light', meta: 'Light' },
  ];
  assert.equal(filterComboboxOptions(opts, 'tokyo').length, 1);
  assert.equal(filterComboboxOptions(opts, 'light').length, 1);
  assert.equal(filterComboboxOptions(opts, 'missing').length, 0);
});

test('validateFormFields enforces crate name and email syntax', () => {
  const empty = validateFormFields({ crateName: ' ', email: 'invalid' });
  assert.ok(empty.crateName);
  assert.ok(empty.email);
  const valid = validateFormFields({ crateName: 'egui-kit', email: 'dev@egui.rs' });
  assert.deepEqual(valid, {});
});

import {
  formatByteSize,
  formatYmd,
  getCalendarYearOptions,
  getDecadeWindow,
  getMonthGrid,
  hsvToRgbHex,
  stepTimeSegment,
} from './catalogLogic';

test('formatByteSize formats byte, KB, and MB values cleanly', () => {
  assert.equal(formatByteSize(0), '0 B');
  assert.equal(formatByteSize(512), '512 B');
  assert.equal(formatByteSize(48_210), '47.1 KB');
  assert.equal(formatByteSize(2_621_440), '2.5 MB');
});

test('getMonthGrid generates a 42-cell Gregorian grid including leap years', () => {
  const feb2024 = getMonthGrid(2024, 1);
  assert.equal(feb2024.length, 42);
  assert.equal(feb2024.filter((c) => c.inMonth).length, 29);
  assert.equal(formatYmd(2026, 4, 9), '2026-04-09');
});

test('getCalendarYearOptions and getDecadeWindow cover multi-decade jumps', () => {
  const years = getCalendarYearOptions(2010, 2025);
  assert.equal(years[0], 2025);
  assert.equal(years[years.length - 1], 2010);
  assert.ok(years.includes(2014));
  const win2014 = getDecadeWindow(2014);
  assert.equal(win2014.length, 12);
  assert.ok(win2014.includes(2014));
});

test('stepTimeSegment wraps hours, minutes, and seconds cleanly', () => {
  const t = { hours: 23, minutes: 0, seconds: 59 };
  assert.equal(stepTimeSegment(t, 'hours', 1).hours, 0);
  assert.equal(stepTimeSegment(t, 'minutes', -1).minutes, 59);
  assert.equal(stepTimeSegment(t, 'seconds', 1).seconds, 0);
});

test('hsvToRgbHex converts primary red, green, blue, and white accurately', () => {
  assert.equal(hsvToRgbHex(0, 1, 1).hex, '#FF0000');
  assert.equal(hsvToRgbHex(120, 1, 1).hex, '#00FF00');
  assert.equal(hsvToRgbHex(240, 1, 1).hex, '#0000FF');
  assert.equal(hsvToRgbHex(0, 0, 1).hex, '#FFFFFF');
});

import {
  clampSplitRatio,
  computePaginationItems,
  getNextStepIndex,
  stepAccordionIndex,
} from './catalogLogic';

test('computePaginationItems builds compact window with ellipsis tokens', () => {
  assert.deepEqual(computePaginationItems(1, 5, 1), [1, 2, 3, 4, 5]);
  assert.deepEqual(computePaginationItems(6, 12, 1), [1, 'ellipsis', 5, 6, 7, 'ellipsis', 12]);
  assert.deepEqual(computePaginationItems(1, 12, 1), [1, 2, 'ellipsis', 12]);
  assert.deepEqual(computePaginationItems(12, 12, 1), [1, 'ellipsis', 11, 12]);
});

test('clampSplitRatio and step helpers clamp bounds and toggle cleanly', () => {
  assert.equal(clampSplitRatio(0.95, 0.2, 0.8), 0.8);
  assert.equal(clampSplitRatio(0.05, 0.2, 0.8), 0.2);
  assert.equal(clampSplitRatio(Number.NaN, 0.2, 0.8), 0.2);
  assert.equal(stepAccordionIndex(1, 1, true), null);
  assert.equal(stepAccordionIndex(1, 1, false), 1);
  assert.equal(stepAccordionIndex(null, 2, true), 2);
  assert.equal(getNextStepIndex(3, 4, 1), 3);
  assert.equal(getNextStepIndex(0, 4, -1), 0);
});

import {
  clampNotificationStack,
  cycleFocusIndex,
  filterCommandItems,
  fuzzyMatchesQuery,
  splitFuzzyMatchSpans,
} from './catalogLogic';

test('fuzzyMatchesQuery and filterCommandItems match sub-sequences across label and group', () => {
  assert.equal(fuzzyMatchesQuery('swth', 'Switch Theme'), true);
  assert.equal(fuzzyMatchesQuery('xyz', 'Switch Theme'), false);
  const items = [
    { id: 'theme', label: 'Switch Theme', group: 'Appearance' },
    { id: 'toast', label: 'Push Edge Toast', group: 'Overlays' },
  ];
  assert.equal(filterCommandItems(items, 'ovly').length, 1);
  assert.equal(filterCommandItems(items, '').length, 2);
});

test('splitFuzzyMatchSpans isolates matched characters for highlighting', () => {
  assert.deepEqual(splitFuzzyMatchSpans('', 'Switch Theme'), [{ text: 'Switch Theme', match: false }]);
  assert.deepEqual(splitFuzzyMatchSpans('swth', 'Switch Theme'), [
    { text: 'Sw', match: true },
    { text: 'i', match: false },
    { text: 't', match: true },
    { text: 'c', match: false },
    { text: 'h', match: true },
    { text: ' Theme', match: false },
  ]);
  assert.deepEqual(splitFuzzyMatchSpans('xyz', 'Switch Theme'), [{ text: 'Switch Theme', match: false }]);
});

test('clampNotificationStack retains at most maxItems newest toasts', () => {
  assert.deepEqual(clampNotificationStack([1, 2, 3, 4, 5, 6], 5), [2, 3, 4, 5, 6]);
  assert.deepEqual(clampNotificationStack([1, 2], 5), [1, 2]);
});

test('cycleFocusIndex wraps forward and backward across trapped boundaries', () => {
  assert.equal(cycleFocusIndex(null, 4, false), 0);
  assert.equal(cycleFocusIndex(null, 4, true), 3);
  assert.equal(cycleFocusIndex(3, 4, false), 0);
  assert.equal(cycleFocusIndex(0, 4, true), 3);
  assert.equal(cycleFocusIndex(1, 4, false), 2);
});

