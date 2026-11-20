import { Menu } from 'lucide-react';

export type ActivePage = 'overview' | 'components' | 'themes' | 'crate';

interface TopBarProps {
  activePage: ActivePage;
  onNavigate: (page: ActivePage) => void;
  onExportCrate: () => void;
  onOpenMobileSidebar: () => void;
}

export function TopBar({
  activePage,
  onNavigate,
  onExportCrate,
  onOpenMobileSidebar,
}: TopBarProps) {
  const navItems: Array<{ id: ActivePage; label: string }> = [
    { id: 'overview', label: 'Overview' },
    { id: 'components', label: 'Components' },
    { id: 'themes', label: 'Themes' },
    { id: 'crate', label: 'Rust Crate' },
  ];

  return (
    <header
      className="sticky top-0 z-30 border-b"
      style={{
        backgroundColor: 'var(--titlebar)',
        borderColor: 'var(--border)',
      }}
    >
      <div className="max-w-[1440px] mx-auto h-14 px-4 sm:px-6 flex items-center justify-between gap-8">
        {/* Zone 1: Brand Wordmark (+ mobile drawer trigger on small screens) */}
        <div className="flex items-center gap-3 shrink-0">
          <button
            type="button"
            onClick={onOpenMobileSidebar}
            aria-label="Open component navigation"
            className="lg:hidden p-2 -ml-2 rounded-md cursor-pointer"
            style={{ color: 'var(--foreground)' }}
          >
            <Menu className="w-5 h-5" aria-hidden="true" />
          </button>
          <a
            href="#overview"
            onClick={(e) => {
              e.preventDefault();
              onNavigate('overview');
            }}
            className="text-lg font-bold tracking-tight whitespace-nowrap shrink-0"
            style={{ color: 'var(--foreground)', fontFamily: 'var(--font-display)' }}
          >
            egui-kit
          </a>
        </div>

        {/* Zone 2: 4 concise single-line navigation links */}
        <nav aria-label="Primary" className="hidden md:flex items-center gap-6">
          {navItems.map((item) => {
            const active = activePage === item.id;
            return (
              <button
                key={item.id}
                type="button"
                onClick={() => onNavigate(item.id)}
                className="text-sm font-medium py-1 border-b-2 transition-colors cursor-pointer whitespace-nowrap shrink-0"
                style={{
                  color: active ? 'var(--foreground)' : 'var(--muted-foreground)',
                  borderColor: active ? 'var(--primary)' : 'transparent',
                }}
              >
                {item.label}
              </button>
            );
          })}
        </nav>

        {/* Zone 3: 1 primary action */}
        <div className="shrink-0">
          <button
            type="button"
            onClick={onExportCrate}
            className="px-4 py-2 rounded-md text-xs font-semibold transition-opacity hover:opacity-90 cursor-pointer whitespace-nowrap shrink-0"
            style={{
              backgroundColor: 'var(--primary)',
              color: 'var(--primary-foreground)',
            }}
          >
            Download Crate
          </button>
        </div>
      </div>

      {/* Compact mobile nav row */}
      <nav
        aria-label="Mobile sections"
        className="flex md:hidden items-center gap-4 px-4 pb-2 overflow-x-auto border-t"
        style={{ borderColor: 'var(--border)' }}
      >
        {navItems.map((item) => {
          const active = activePage === item.id;
          return (
            <button
              key={item.id}
              type="button"
              onClick={() => onNavigate(item.id)}
              className="text-xs font-medium py-1.5 border-b-2 whitespace-nowrap shrink-0 cursor-pointer"
              style={{
                color: active ? 'var(--foreground)' : 'var(--muted-foreground)',
                borderColor: active ? 'var(--primary)' : 'transparent',
              }}
            >
              {item.label}
            </button>
          );
        })}
      </nav>
    </header>
  );
}
