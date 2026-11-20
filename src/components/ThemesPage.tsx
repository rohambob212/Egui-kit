import { useState } from 'react';
import { Check, Search } from 'lucide-react';
import type { ThemePreset } from '../types/kit';
import { buildRustThemeSnippet, filterThemes } from '../utils/catalogLogic';
import { RustCodeBlock } from './RustCodeBlock';

interface ThemesPageProps {
  themes: ThemePreset[];
  activeTheme: ThemePreset;
  onSelectTheme: (id: string) => void;
}

export function ThemesPage({ themes, activeTheme, onSelectTheme }: ThemesPageProps) {
  const [modeFilter, setModeFilter] = useState<'all' | 'light' | 'dark'>('all');
  const [query, setQuery] = useState('');

  const filtered = filterThemes(themes, modeFilter, query);
  const rustSnippet = buildRustThemeSnippet(activeTheme);

  return (
    <div className="space-y-8">
      <div className="space-y-2 border-b pb-6" style={{ borderColor: 'var(--border)' }}>
        <div className="text-xs font-mono" style={{ color: 'var(--muted-foreground)' }}>
          Theme Engine · {themes.length} gpui-kit Presets
        </div>
        <h1 className="text-3xl font-bold tracking-tight" style={{ color: 'var(--foreground)' }}>
          Theme Gallery & Rust Token Generator
        </h1>
        <p className="text-sm sm:text-base max-w-2xl" style={{ color: 'var(--muted-foreground)' }}>
          Select any preset below to re-theme both the documentation site and all live Part 1 egui-kit previews
          instantly, then copy the generated <code className="font-mono">ThemeTokens</code> Rust code.
        </p>
      </div>

      {/* Filter Toolbar */}
      <div className="flex flex-wrap items-center justify-between gap-4">
        <div
          className="inline-flex items-center gap-1 p-1 rounded-md border"
          style={{ backgroundColor: 'var(--muted)', borderColor: 'var(--border)' }}
        >
          {(['all', 'dark', 'light'] as const).map((mode) => {
            const active = modeFilter === mode;
            return (
              <button
                key={mode}
                type="button"
                onClick={() => setModeFilter(mode)}
                className="px-3 py-1.5 rounded text-xs font-medium capitalize cursor-pointer whitespace-nowrap shrink-0"
                style={{
                  backgroundColor: active ? 'var(--card)' : 'transparent',
                  color: active ? 'var(--foreground)' : 'var(--muted-foreground)',
                }}
              >
                {mode === 'all' ? `All (${themes.length})` : mode}
              </button>
            );
          })}
        </div>

        <div className="relative w-full sm:w-64">
          <Search
            className="w-3.5 h-3.5 absolute left-3 top-1/2 -translate-y-1/2 pointer-events-none"
            style={{ color: 'var(--muted-foreground)' }}
            aria-hidden="true"
          />
          <input
            type="search"
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            placeholder="Search themes..."
            aria-label="Search themes"
            className="w-full pl-8 pr-3 py-1.5 rounded-md border text-xs"
            style={{
              backgroundColor: 'var(--card)',
              color: 'var(--foreground)',
              borderColor: 'var(--border)',
            }}
          />
        </div>
      </div>

      {/* Swatch Grid */}
      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
        {filtered.map((t) => {
          const isCurrent = t.id === activeTheme.id;
          const swatches = [
            t.vars['--background'],
            t.vars['--card'],
            t.vars['--primary'],
            t.vars['--success'],
            t.vars['--warning'],
            t.vars['--destructive'],
          ].filter(Boolean);

          return (
            <button
              key={t.id}
              type="button"
              onClick={() => onSelectTheme(t.id)}
              className="p-4 rounded-lg border text-left transition-colors cursor-pointer flex flex-col gap-3"
              style={{
                backgroundColor: t.vars['--background'] || 'var(--card)',
                color: t.vars['--foreground'] || 'var(--foreground)',
                borderColor: isCurrent ? 'var(--primary)' : t.vars['--border'] || 'var(--border)',
                outline: isCurrent ? '2px solid var(--primary)' : 'none',
              }}
            >
              <div className="flex items-center justify-between gap-2">
                <span className="text-sm font-semibold truncate">{t.name}</span>
                {isCurrent ? (
                  <span className="inline-flex items-center gap-1 text-xs font-mono shrink-0">
                    <Check className="w-3.5 h-3.5" aria-hidden="true" />
                    Active
                  </span>
                ) : (
                  <span className="text-[11px] font-mono opacity-70 capitalize shrink-0">{t.mode}</span>
                )}
              </div>

              <div className="flex items-center gap-1.5">
                {swatches.map((hex, i) => (
                  <span
                    key={`${t.id}-sw-${i}`}
                    className="w-5 h-5 rounded border"
                    style={{ backgroundColor: hex, borderColor: 'rgba(128,128,128,0.28)' }}
                  />
                ))}
              </div>
            </button>
          );
        })}
      </div>

      {/* Generated Rust ThemeTokens */}
      <section className="space-y-3 pt-4">
        <h2 className="text-lg font-semibold" style={{ color: 'var(--foreground)' }}>
          Rust ThemeTokens Export — {activeTheme.name}
        </h2>
        <RustCodeBlock code={rustSnippet} filename={`themes/${activeTheme.id}.rs`} />
      </section>
    </div>
  );
}
