/**
 * @license
 * SPDX-License-Identifier: Apache-2.0
 */

import { useEffect, useState } from 'react';
import { CrateExplorerPage } from './components/CrateExplorerPage';
import { OverviewPage } from './components/OverviewPage';
import { Sidebar } from './components/Sidebar';
import { ThemesPage } from './components/ThemesPage';
import { TopBar, type ActivePage } from './components/TopBar';
import { WidgetWorkbench } from './components/WidgetWorkbench';
import { WIDGET_CATALOG } from './data/catalog';
import { CRATE_FILES } from './data/crateFiles';
import { applyThemeToDocument, DEFAULT_THEME_ID, getThemeById, THEME_PRESETS } from './data/themes';
import type { PartNumber, WidgetCategory } from './types/kit';
import { countByStatus } from './utils/catalogLogic';

const STORAGE_THEME_KEY = 'egui-kit-theme';

export default function App() {
  const [themeId, setThemeId] = useState<string>(() => {
    try {
      return localStorage.getItem(STORAGE_THEME_KEY) || DEFAULT_THEME_ID;
    } catch {
      return DEFAULT_THEME_ID;
    }
  });

  const [activePage, setActivePage] = useState<ActivePage>('overview');
  const [selectedSlug, setSelectedSlug] = useState<string>('button');
  const [searchQuery, setSearchQuery] = useState('');
  const [categoryFilter, setCategoryFilter] = useState<WidgetCategory | 'All'>('All');
  const [partFilter, setPartFilter] = useState<PartNumber | 'All'>('All');
  const [mobileSidebarOpen, setMobileSidebarOpen] = useState(false);

  const activeTheme = getThemeById(themeId);

  useEffect(() => {
    applyThemeToDocument(activeTheme);
    try {
      localStorage.setItem(STORAGE_THEME_KEY, activeTheme.id);
    } catch {
      // ignore storage failure in private frames
    }
  }, [activeTheme]);

  const handleSelectWidget = (slug: string) => {
    setSelectedSlug(slug);
    setActivePage('components');
    window.scrollTo({ top: 0, behavior: 'smooth' });
  };

  const handleNavigate = (page: ActivePage) => {
    setActivePage(page);
    window.scrollTo({ top: 0, behavior: 'smooth' });
  };

  const handleExportCrate = () => {
    const lines = [
      '#!/usr/bin/env bash',
      'set -euo pipefail',
      'mkdir -p egui-kit/src/components egui-kit/examples',
      ...CRATE_FILES.map(
        (file) =>
          `cat <<'EOF_EGUI_KIT' > "egui-kit/${file.path}"\n${file.content}\nEOF_EGUI_KIT`
      ),
      'echo "Created ./egui-kit crate. Run: cd egui-kit && cargo check"',
    ].join('\n\n');

    const blob = new Blob([lines], { type: 'text/x-shellscript;charset=utf-8' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = 'setup-egui-kit.sh';
    a.click();
    URL.revokeObjectURL(url);
    setActivePage('crate');
  };

  const selectedWidget =
    WIDGET_CATALOG.find((w) => w.slug === selectedSlug) ?? WIDGET_CATALOG[0];
  const counts = countByStatus(WIDGET_CATALOG);

  return (
    <div
      className="min-h-screen flex flex-col"
      style={{
        backgroundColor: 'var(--background)',
        color: 'var(--foreground)',
      }}
    >
      <TopBar
        activePage={activePage}
        onNavigate={handleNavigate}
        onExportCrate={handleExportCrate}
        onOpenMobileSidebar={() => setMobileSidebarOpen(true)}
      />

      <div className="max-w-[1440px] w-full mx-auto flex-1 flex">
        <Sidebar
          widgets={WIDGET_CATALOG}
          selectedSlug={activePage === 'components' ? selectedWidget.slug : null}
          searchQuery={searchQuery}
          onSearchChange={setSearchQuery}
          categoryFilter={categoryFilter}
          onCategoryChange={setCategoryFilter}
          partFilter={partFilter}
          onPartChange={setPartFilter}
          onSelectWidget={handleSelectWidget}
          mobileOpen={mobileSidebarOpen}
          onCloseMobile={() => setMobileSidebarOpen(false)}
        />

        <main className="flex-1 min-w-0 px-5 sm:px-8 lg:px-12 py-8 sm:py-10 flex flex-col">
          <div className="flex-1 max-w-5xl w-full">
            {activePage === 'overview' && (
              <OverviewPage
                widgets={WIDGET_CATALOG}
                activeTheme={activeTheme}
                onSelectWidget={handleSelectWidget}
                onSelectPartFilter={(part) => setPartFilter(part)}
                onOpenThemes={() => handleNavigate('themes')}
                onOpenCrate={() => handleNavigate('crate')}
              />
            )}

            {activePage === 'components' && (
              <WidgetWorkbench
                widget={selectedWidget}
                allWidgets={WIDGET_CATALOG}
                activeTheme={activeTheme}
                allThemes={THEME_PRESETS}
                onSelectTheme={setThemeId}
                onSelectWidget={handleSelectWidget}
              />
            )}

            {activePage === 'themes' && (
              <ThemesPage
                themes={THEME_PRESETS}
                activeTheme={activeTheme}
                onSelectTheme={setThemeId}
              />
            )}

            {activePage === 'crate' && <CrateExplorerPage />}
          </div>

          <footer
            className="mt-16 pt-6 border-t flex flex-wrap items-center justify-between gap-4 text-xs"
            style={{
              borderColor: 'var(--border)',
              color: 'var(--muted-foreground)',
            }}
          >
            <span>egui-kit · Idiomatic egui.rs port of gpui-kit.com components</span>
            <span>Parts 1–6 active ({counts.ready} / {counts.total} widgets) · Theme: {activeTheme.name}</span>
          </footer>
        </main>
      </div>
    </div>
  );
}
