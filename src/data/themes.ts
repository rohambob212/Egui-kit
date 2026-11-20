import themesRaw from './themes.json';
import type { ThemePreset } from '../types/kit';

export const THEME_PRESETS: ThemePreset[] = themesRaw as ThemePreset[];

export const DEFAULT_THEME_ID = 'aurora-aurora-light';

export function getThemeById(id: string): ThemePreset {
  return (
    THEME_PRESETS.find((t) => t.id === id) ??
    THEME_PRESETS.find((t) => t.id === DEFAULT_THEME_ID) ??
    THEME_PRESETS[0]
  );
}

export function applyThemeToDocument(theme: ThemePreset): void {
  if (typeof document === 'undefined') return;
  const root = document.documentElement;
  root.dataset.theme = theme.id;
  root.dataset.themeName = theme.name;
  root.classList.toggle('dark', theme.mode === 'dark');

  for (const [key, value] of Object.entries(theme.vars)) {
    root.style.setProperty(key, value);
  }
}
