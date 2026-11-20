import { useEffect, useRef } from 'react';
import { Search, X } from 'lucide-react';
import type { PartNumber, WidgetCategory, WidgetSpec } from '../types/kit';
import { filterWidgets, groupWidgetsByPart, PARTS_ROADMAP } from '../utils/catalogLogic';

interface SidebarProps {
  widgets: WidgetSpec[];
  selectedSlug: string | null;
  searchQuery: string;
  onSearchChange: (q: string) => void;
  categoryFilter: WidgetCategory | 'All';
  onCategoryChange: (cat: WidgetCategory | 'All') => void;
  partFilter: PartNumber | 'All';
  onPartChange: (part: PartNumber | 'All') => void;
  onSelectWidget: (slug: string) => void;
  mobileOpen: boolean;
  onCloseMobile: () => void;
}

export function Sidebar({
  widgets,
  selectedSlug,
  searchQuery,
  onSearchChange,
  categoryFilter,
  onCategoryChange,
  partFilter,
  onPartChange,
  onSelectWidget,
  mobileOpen,
  onCloseMobile,
}: SidebarProps) {
  const inputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    const onKeyDown = (e: KeyboardEvent) => {
      if ((e.metaKey || e.ctrlKey) && e.key.toLowerCase() === 'k') {
        e.preventDefault();
        inputRef.current?.focus();
      }
    };
    window.addEventListener('keydown', onKeyDown);
    return () => window.removeEventListener('keydown', onKeyDown);
  }, []);

  const filtered = filterWidgets(widgets, {
    query: searchQuery,
    category: categoryFilter,
    part: partFilter,
  });

  const grouped = groupWidgetsByPart(filtered);
  const categories: Array<WidgetCategory | 'All'> = ['All', 'Basic', 'Form', 'Layout', 'Advanced'];

  const body = (
    <div className="flex flex-col h-full">
      <div className="p-4 border-b flex flex-col gap-3" style={{ borderColor: 'var(--sidebar-border)' }}>
        <div className="flex items-center justify-between lg:hidden">
          <span className="text-sm font-semibold" style={{ color: 'var(--sidebar-foreground)' }}>
            Widgets Catalog
          </span>
          <button
            type="button"
            onClick={onCloseMobile}
            aria-label="Close component navigation"
            className="p-1.5 rounded cursor-pointer"
            style={{ color: 'var(--muted-foreground)' }}
          >
            <X className="w-4 h-4" aria-hidden="true" />
          </button>
        </div>

        <div className="relative">
          <Search
            className="w-3.5 h-3.5 absolute left-2.5 top-1/2 -translate-y-1/2 pointer-events-none"
            style={{ color: 'var(--muted-foreground)' }}
            aria-hidden="true"
          />
          <input
            ref={inputRef}
            type="search"
            value={searchQuery}
            onChange={(e) => onSearchChange(e.target.value)}
            placeholder="Filter 80 widgets..."
            aria-label="Filter widgets"
            className="w-full pl-8 pr-12 py-1.5 rounded-md border text-xs outline-none focus-visible:ring-2"
            style={{
              backgroundColor: 'var(--background)',
              color: 'var(--foreground)',
              borderColor: 'var(--border)',
            }}
          />
          <span
            className="absolute right-2.5 top-1/2 -translate-y-1/2 text-[10px] font-mono pointer-events-none"
            style={{ color: 'var(--muted-foreground)' }}
          >
            ⌘K
          </span>
        </div>

        {/* Category segmented buttons */}
        <div className="flex flex-wrap gap-1">
          {categories.map((cat) => {
            const active = categoryFilter === cat;
            return (
              <button
                key={cat}
                type="button"
                onClick={() => onCategoryChange(cat)}
                className="px-2 py-1 rounded text-[11px] font-medium cursor-pointer transition-colors whitespace-nowrap shrink-0"
                style={{
                  backgroundColor: active ? 'var(--primary)' : 'var(--muted)',
                  color: active ? 'var(--primary-foreground)' : 'var(--muted-foreground)',
                }}
              >
                {cat}
              </button>
            );
          })}
        </div>

        {/* Part quick selector */}
        <div className="flex items-center justify-between gap-2 text-xs">
          <label htmlFor="part-filter" style={{ color: 'var(--muted-foreground)' }}>
            Release Part
          </label>
          <select
            id="part-filter"
            value={String(partFilter)}
            onChange={(e) =>
              onPartChange(e.target.value === 'All' ? 'All' : (Number(e.target.value) as PartNumber))
            }
            className="px-2 py-1 rounded border text-xs font-mono cursor-pointer"
            style={{
              backgroundColor: 'var(--background)',
              color: 'var(--foreground)',
              borderColor: 'var(--border)',
            }}
          >
            <option value="All">All Parts (80)</option>
            {PARTS_ROADMAP.map((p) => (
              <option key={p.part} value={p.part}>
                Part {p.part} ({p.status === 'completed' ? 'Ready' : 'Planned'})
              </option>
            ))}
          </select>
        </div>
      </div>

      {/* Scrollable widget tree */}
      <nav aria-label="Widgets by part" className="flex-1 overflow-y-auto px-3 py-4 space-y-5">
        {filtered.length === 0 ? (
          <div className="px-2 py-6 text-xs text-center" style={{ color: 'var(--muted-foreground)' }}>
            No widgets match your filter.
          </div>
        ) : (
          PARTS_ROADMAP.map((partInfo) => {
            const items = grouped.get(partInfo.part) ?? [];
            if (items.length === 0) return null;
            return (
              <div key={partInfo.part} className="space-y-1">
                <div className="px-2 pb-1 flex items-center justify-between text-[11px] font-semibold">
                  <span style={{ color: 'var(--foreground)' }}>Part {partInfo.part}</span>
                  <span
                    className="font-mono tabular-nums"
                    style={{
                      color: partInfo.status === 'completed' ? 'var(--success)' : 'var(--muted-foreground)',
                    }}
                  >
                    {partInfo.status === 'completed' ? `${items.length} ready` : `${items.length} planned`}
                  </span>
                </div>
                <ul className="space-y-0.5">
                  {items.map((w) => {
                    const active = selectedSlug === w.slug;
                    return (
                      <li key={w.slug}>
                        <button
                          type="button"
                          onClick={() => {
                            onSelectWidget(w.slug);
                            onCloseMobile();
                          }}
                          className="w-full flex items-center justify-between gap-2 px-2.5 py-1.5 rounded-md text-xs text-left transition-colors cursor-pointer"
                          style={{
                            backgroundColor: active ? 'var(--sidebar-accent)' : 'transparent',
                            color: active
                              ? 'var(--sidebar-accent-foreground)'
                              : w.status === 'ready'
                              ? 'var(--sidebar-foreground)'
                              : 'var(--muted-foreground)',
                            fontWeight: active ? 600 : 400,
                          }}
                        >
                          <span className="truncate">{w.name}</span>
                          <span className="text-[10px] font-mono shrink-0 opacity-75">
                            {w.category}
                          </span>
                        </button>
                      </li>
                    );
                  })}
                </ul>
              </div>
            );
          })
        )}
      </nav>
    </div>
  );

  return (
    <>
      {/* Desktop persistent sidebar */}
      <aside
        className="hidden lg:block w-[264px] shrink-0 border-r sticky top-14 h-[calc(100vh-3.5rem)]"
        style={{
          backgroundColor: 'var(--sidebar)',
          borderColor: 'var(--sidebar-border)',
        }}
      >
        {body}
      </aside>

      {/* Mobile drawer */}
      {mobileOpen && (
        <div className="fixed inset-0 z-40 lg:hidden flex">
          <div
            className="fixed inset-0 bg-black/50"
            onClick={onCloseMobile}
            aria-hidden="true"
          />
          <aside
            className="relative w-72 max-w-[85vw] h-full border-r z-50"
            style={{
              backgroundColor: 'var(--sidebar)',
              borderColor: 'var(--sidebar-border)',
            }}
          >
            {body}
          </aside>
        </div>
      )}
    </>
  );
}
