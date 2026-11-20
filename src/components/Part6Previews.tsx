import { useEffect, useMemo, useRef, useState } from 'react';
import {
  AlertTriangle,
  Check,
  ChevronRight,
  Command as CommandIcon,
  FileCode,
  Info,
  Pencil,
  Search,
  Share2,
  Sparkles,
  X,
} from 'lucide-react';
import type { ThemePreset } from '../types/kit';
import {
  clampNotificationStack,
  cycleFocusIndex,
  filterCommandItems,
  splitFuzzyMatchSpans,
} from '../utils/catalogLogic';
import {
  ControlField,
  KbdChord,
  SegmentedButtons,
  StageShell,
  ToggleCheckbox,
} from './Part1Previews';

interface Part6PreviewProps {
  slug: string;
  activeTheme: ThemePreset;
}

export function Part6Preview({ slug, activeTheme }: Part6PreviewProps) {
  switch (slug) {
    case 'dialog':
      return <DialogPreview activeTheme={activeTheme} />;
    case 'alert-dialog':
      return <AlertDialogPreview activeTheme={activeTheme} />;
    case 'sheet':
      return <SheetPreview activeTheme={activeTheme} />;
    case 'popover':
      return <PopoverPreview activeTheme={activeTheme} />;
    case 'tooltip':
      return <TooltipPreview activeTheme={activeTheme} />;
    case 'hover-card':
      return <HoverCardPreview activeTheme={activeTheme} />;
    case 'menu':
      return <MenuPreview activeTheme={activeTheme} />;
    case 'notification':
      return <NotificationPreview activeTheme={activeTheme} />;
    case 'focus-trap':
      return <FocusTrapPreview activeTheme={activeTheme} />;
    case 'command':
      return <CommandPreview activeTheme={activeTheme} />;
    default:
      return null;
  }
}

function DialogPreview({ activeTheme }: { activeTheme: ThemePreset }) {
  const [open, setOpen] = useState(false);
  const [widthOpt, setWidthOpt] = useState<'360px' | '440px' | '520px'>('440px');
  const [dismissBackdrop, setDismissBackdrop] = useState(true);
  const [showClose, setShowClose] = useState(true);
  const [workspaceName, setWorkspaceName] = useState('egui-kit-studio');
  const [lastOutcome, setLastOutcome] = useState('Closed');

  useEffect(() => {
    if (!open) return;
    const onKey = (e: KeyboardEvent) => {
      if (e.key === 'Escape') {
        setOpen(false);
        setLastOutcome('Dismissed (Escape)');
      }
    };
    window.addEventListener('keydown', onKey);
    return () => window.removeEventListener('keydown', onKey);
  }, [open]);

  return (
    <StageShell
      title='Dialog::new("prefs", &mut open).show(ctx, |ui| ...)'
      themeName={activeTheme.name}
      onReset={() => {
        setOpen(false);
        setWidthOpt('440px');
        setDismissBackdrop(true);
        setShowClose(true);
        setWorkspaceName('egui-kit-studio');
        setLastOutcome('Closed');
      }}
      controls={
        <>
          <ControlField label="Modal Width">
            <SegmentedButtons options={['360px', '440px', '520px'] as const} value={widthOpt} onChange={setWidthOpt} />
          </ControlField>
          <ToggleCheckbox
            checked={dismissBackdrop}
            onChange={setDismissBackdrop}
            label="Dismiss on backdrop click"
          />
          <ToggleCheckbox
            checked={showClose}
            onChange={setShowClose}
            label="Show close button (×)"
          />
          <div className="p-3 rounded border text-xs font-mono" style={{ backgroundColor: 'var(--muted)', borderColor: 'var(--border)' }}>
            open: {String(open)} · {lastOutcome}
          </div>
        </>
      }
    >
      <div className="flex flex-col items-center gap-3">
        <button
          type="button"
          onClick={() => setOpen(true)}
          className="px-4 py-2 rounded-md text-xs font-semibold cursor-pointer shadow-sm transition-all hover:opacity-95 active:scale-[0.98]"
          style={{ backgroundColor: 'var(--primary)', color: 'var(--primary-foreground)' }}
        >
          Open Modal Dialog
        </button>
        <span className="text-xs font-mono" style={{ color: 'var(--muted-foreground)' }}>
          Workspace: {workspaceName}
        </span>
      </div>

      {open && (
        <div
          className="fixed inset-0 z-50 flex items-center justify-center p-4 animate-egui-fade-in backdrop-blur-[2px]"
          style={{ backgroundColor: 'rgba(8, 12, 20, 0.48)' }}
          onMouseDown={(e) => {
            if (e.target === e.currentTarget && dismissBackdrop) {
              setOpen(false);
              setLastOutcome('Dismissed (Backdrop)');
            }
          }}
        >
          <div
            role="dialog"
            aria-modal="true"
            aria-labelledby="dialog-preview-title"
            className="animate-egui-modal-in w-full rounded-xl border p-5 space-y-4 shadow-2xl"
            style={{
              maxWidth: widthOpt,
              backgroundColor: 'var(--card)',
              borderColor: 'var(--border)',
              color: 'var(--card-foreground)',
            }}
          >
            <div className="flex items-start justify-between gap-4 pb-3 border-b" style={{ borderColor: 'var(--border)' }}>
              <div>
                <h3 id="dialog-preview-title" className="text-base font-semibold">
                  Workspace Preferences
                </h3>
                <p className="text-xs mt-0.5" style={{ color: 'var(--muted-foreground)' }}>
                  Bound directly to &amp;mut bool with Escape &amp; backdrop support.
                </p>
              </div>
              {showClose && (
                <button
                  type="button"
                  aria-label="Close dialog"
                  onClick={() => {
                    setOpen(false);
                    setLastOutcome('Dismissed (Close ×)');
                  }}
                  className="p-1.5 rounded-md hover:opacity-80 active:scale-95 transition-all cursor-pointer"
                  style={{ color: 'var(--muted-foreground)', backgroundColor: 'var(--muted)' }}
                >
                  <X className="w-3.5 h-3.5" aria-hidden="true" />
                </button>
              )}
            </div>

            <div className="space-y-2">
              <label htmlFor="dialog-ws-name" className="block text-xs font-medium" style={{ color: 'var(--muted-foreground)' }}>
                Crate Workspace Name
              </label>
              <input
                id="dialog-ws-name"
                type="text"
                value={workspaceName}
                onChange={(e) => setWorkspaceName(e.target.value)}
                className="w-full px-3 py-2 rounded-md border text-xs font-mono outline-none transition-colors focus:border-[var(--primary)]"
                style={{
                  backgroundColor: 'var(--background)',
                  color: 'var(--foreground)',
                  borderColor: 'var(--border)',
                }}
              />
            </div>

            <div className="flex items-center justify-end gap-2 pt-2">
              <button
                type="button"
                onClick={() => {
                  setOpen(false);
                  setLastOutcome('Cancelled');
                }}
                className="px-3.5 py-1.5 rounded-md border text-xs font-medium cursor-pointer transition-colors hover:bg-[var(--accent)] active:scale-[0.98]"
                style={{
                  backgroundColor: 'transparent',
                  color: 'var(--foreground)',
                  borderColor: 'var(--border)',
                }}
              >
                Cancel
              </button>
              <button
                type="button"
                onClick={() => {
                  setOpen(false);
                  setLastOutcome('Saved');
                }}
                className="px-3.5 py-1.5 rounded-md text-xs font-semibold cursor-pointer shadow-sm transition-all hover:opacity-95 active:scale-[0.98]"
                style={{
                  backgroundColor: 'var(--primary)',
                  color: 'var(--primary-foreground)',
                }}
              >
                Save Changes
              </button>
            </div>
          </div>
        </div>
      )}
    </StageShell>
  );
}

function AlertDialogPreview({ activeTheme }: { activeTheme: ThemePreset }) {
  const [open, setOpen] = useState(false);
  const [destructive, setDestructive] = useState(true);
  const [outcome, setOutcome] = useState('Awaiting action');
  const [blockedHint, setBlockedHint] = useState(false);
  const [shakeKey, setShakeKey] = useState(0);

  useEffect(() => {
    if (!open) return;
    const onKey = (e: KeyboardEvent) => {
      if (e.key === 'Escape') {
        setOpen(false);
        setOutcome('Cancelled (Escape)');
      }
    };
    window.addEventListener('keydown', onKey);
    return () => window.removeEventListener('keydown', onKey);
  }, [open]);

  const toneColor = destructive ? 'var(--destructive)' : 'var(--warning)';

  return (
    <StageShell
      title='AlertDialog::confirm("Delete release tag?", &mut open)'
      themeName={activeTheme.name}
      onReset={() => {
        setOpen(false);
        setDestructive(true);
        setOutcome('Awaiting action');
        setBlockedHint(false);
        setShakeKey(0);
      }}
      controls={
        <>
          <ToggleCheckbox checked={destructive} onChange={setDestructive} label="Destructive CTA" />
          <div className="p-3 rounded border text-xs font-mono space-y-1" style={{ backgroundColor: 'var(--muted)', borderColor: 'var(--border)' }}>
            <div>open: {String(open)}</div>
            <div>result: {outcome}</div>
          </div>
        </>
      }
    >
      <div className="flex flex-col items-center gap-3 text-center">
        <button
          type="button"
          onClick={() => {
            setBlockedHint(false);
            setShakeKey(0);
            setOpen(true);
          }}
          className="px-4 py-2 rounded-md text-xs font-semibold cursor-pointer shadow-sm transition-all hover:opacity-95 active:scale-[0.98]"
          style={{
            backgroundColor: destructive ? 'var(--destructive)' : 'var(--primary)',
            color: 'var(--primary-foreground)',
          }}
        >
          {destructive ? 'Delete Release Tag' : 'Confirm Crate Publish'}
        </button>
        <p className="text-xs max-w-xs" style={{ color: 'var(--muted-foreground)' }}>
          Outside-click dismissal is intentionally disabled so users must pick Confirm or Cancel.
        </p>
      </div>

      {open && (
        <div
          className="fixed inset-0 z-50 flex items-center justify-center p-4 animate-egui-fade-in backdrop-blur-[2px]"
          style={{ backgroundColor: 'rgba(8, 12, 20, 0.56)' }}
          onMouseDown={(e) => {
            if (e.target === e.currentTarget) {
              setBlockedHint(true);
              setShakeKey((k) => k + 1);
            }
          }}
        >
          <div
            key={shakeKey}
            role="alertdialog"
            aria-modal="true"
            aria-labelledby="alert-dialog-title"
            className={`${shakeKey > 0 ? 'animate-egui-shake' : 'animate-egui-modal-in'} w-full max-w-[420px] rounded-xl border p-5 space-y-4 shadow-2xl transition-colors`}
            style={{
              backgroundColor: 'var(--card)',
              borderColor: blockedHint ? toneColor : 'var(--border)',
              color: 'var(--card-foreground)',
            }}
          >
            <div className="flex items-start gap-3">
              <div
                className="w-9 h-9 rounded-lg flex items-center justify-center shrink-0 border"
                style={{
                  backgroundColor: `color-mix(in srgb, ${toneColor} 14%, var(--muted))`,
                  borderColor: `color-mix(in srgb, ${toneColor} 28%, transparent)`,
                  color: toneColor,
                }}
              >
                <AlertTriangle className="w-4 h-4" aria-hidden="true" />
              </div>
              <div className="space-y-1">
                <h3 id="alert-dialog-title" className="text-sm font-semibold">
                  {destructive ? 'Delete release tag v0.6.0?' : 'Publish egui-kit v0.6.0?'}
                </h3>
                <p className="text-xs leading-relaxed" style={{ color: 'var(--muted-foreground)' }}>
                  {destructive
                    ? 'This permanently removes the signed git tag from the registry.'
                    : 'This publishes the crate artifacts and locks the version number.'}
                </p>
                {blockedHint && (
                  <p className="text-[11px] font-mono animate-egui-fade-in" style={{ color: toneColor }}>
                    Backdrop click blocked — choose Cancel or Confirm.
                  </p>
                )}
              </div>
            </div>

            <div className="flex items-center justify-end gap-2 pt-2">
              <button
                type="button"
                onClick={() => {
                  setOpen(false);
                  setOutcome('Cancelled');
                }}
                className="px-3.5 py-1.5 rounded-md border text-xs font-medium cursor-pointer transition-colors hover:bg-[var(--accent)] active:scale-[0.98]"
                style={{
                  backgroundColor: 'transparent',
                  color: 'var(--foreground)',
                  borderColor: 'var(--border)',
                }}
              >
                Cancel
              </button>
              <button
                type="button"
                onClick={() => {
                  setOpen(false);
                  setOutcome(destructive ? 'Confirmed (Destructive)' : 'Confirmed');
                }}
                className="px-3.5 py-1.5 rounded-md text-xs font-semibold cursor-pointer shadow-sm transition-all hover:opacity-95 active:scale-[0.98]"
                style={{
                  backgroundColor: destructive ? 'var(--destructive)' : 'var(--primary)',
                  color: 'var(--primary-foreground)',
                }}
              >
                {destructive ? 'Delete Tag' : 'Confirm Publish'}
              </button>
            </div>
          </div>
        </div>
      )}
    </StageShell>
  );
}

function SheetPreview({ activeTheme }: { activeTheme: ThemePreset }) {
  const [open, setOpen] = useState(false);
  const [side, setSide] = useState<'Right' | 'Left' | 'Top' | 'Bottom'>('Right');
  const [extent, setExtent] = useState<'300px' | '360px' | '440px'>('360px');

  useEffect(() => {
    if (!open) return;
    const onKey = (e: KeyboardEvent) => {
      if (e.key === 'Escape') setOpen(false);
    };
    window.addEventListener('keydown', onKey);
    return () => window.removeEventListener('keydown', onKey);
  }, [open]);

  const panelPositionStyle = (): React.CSSProperties => {
    switch (side) {
      case 'Right':
        return { top: 0, right: 0, bottom: 0, width: extent, maxWidth: '92vw' };
      case 'Left':
        return { top: 0, left: 0, bottom: 0, width: extent, maxWidth: '92vw' };
      case 'Top':
        return { top: 0, left: 0, right: 0, height: '260px', maxHeight: '75vh' };
      case 'Bottom':
        return { bottom: 0, left: 0, right: 0, height: '260px', maxHeight: '75vh' };
    }
  };

  const panelAnimationClass = (): string => {
    switch (side) {
      case 'Right':
        return 'animate-egui-sheet-right';
      case 'Left':
        return 'animate-egui-sheet-left';
      case 'Top':
        return 'animate-egui-sheet-top';
      case 'Bottom':
        return 'animate-egui-sheet-bottom';
    }
  };

  return (
    <StageShell
      title={`Sheet::${side.toLowerCase()}("inspector", &mut open)`}
      themeName={activeTheme.name}
      onReset={() => {
        setOpen(false);
        setSide('Right');
        setExtent('360px');
      }}
      controls={
        <>
          <ControlField label="Viewport Edge">
            <SegmentedButtons options={['Right', 'Left', 'Top', 'Bottom'] as const} value={side} onChange={setSide} />
          </ControlField>
          {(side === 'Left' || side === 'Right') && (
            <ControlField label="Drawer Extent">
              <SegmentedButtons options={['300px', '360px', '440px'] as const} value={extent} onChange={setExtent} />
            </ControlField>
          )}
          <div className="p-3 rounded border text-xs font-mono" style={{ backgroundColor: 'var(--muted)', borderColor: 'var(--border)' }}>
            side: SheetSide::{side} · open: {String(open)}
          </div>
        </>
      }
    >
      <button
        type="button"
        onClick={() => setOpen(true)}
        className="px-4 py-2 rounded-md text-xs font-semibold cursor-pointer shadow-sm transition-all hover:opacity-95 active:scale-[0.98]"
        style={{ backgroundColor: 'var(--primary)', color: 'var(--primary-foreground)' }}
      >
        Open {side} Sheet
      </button>

      {open && (
        <div
          className="fixed inset-0 z-50 animate-egui-fade-in backdrop-blur-[2px]"
          style={{ backgroundColor: 'rgba(8, 12, 20, 0.42)' }}
          onMouseDown={(e) => {
            if (e.target === e.currentTarget) setOpen(false);
          }}
        >
          <div
            role="dialog"
            aria-modal="true"
            aria-label={`${side} Inspector Sheet`}
            className={`fixed border p-5 flex flex-col justify-between shadow-2xl ${panelAnimationClass()}`}
            style={{
              ...panelPositionStyle(),
              backgroundColor: 'var(--card)',
              borderColor: 'var(--border)',
              color: 'var(--card-foreground)',
            }}
          >
            <div className="space-y-4">
              <div className="flex items-start justify-between gap-4 pb-3 border-b" style={{ borderColor: 'var(--border)' }}>
                <div>
                  <h3 className="text-base font-semibold">Theme Token Inspector</h3>
                  <p className="text-xs mt-0.5" style={{ color: 'var(--muted-foreground)' }}>
                    Anchored to SheetSide::{side}
                  </p>
                </div>
                <button
                  type="button"
                  aria-label="Close sheet"
                  onClick={() => setOpen(false)}
                  className="p-1.5 rounded-md cursor-pointer transition-all hover:opacity-80 active:scale-95"
                  style={{ backgroundColor: 'var(--muted)', color: 'var(--muted-foreground)' }}
                >
                  <X className="w-3.5 h-3.5" aria-hidden="true" />
                </button>
              </div>

              <div className="grid grid-cols-2 gap-2.5 text-xs">
                {[
                  ['Active Theme', activeTheme.name],
                  ['Mode', activeTheme.mode],
                  ['Primary', activeTheme.vars['--primary'] || '#0f172a'],
                  ['Border', activeTheme.vars['--border'] || '#e2e8f0'],
                ].map(([k, v]) => (
                  <div
                    key={k}
                    className="p-2.5 rounded border"
                    style={{ backgroundColor: 'var(--muted)', borderColor: 'var(--border)' }}
                  >
                    <div style={{ color: 'var(--muted-foreground)' }}>{k}</div>
                    <div className="font-mono font-medium mt-0.5">{v}</div>
                  </div>
                ))}
              </div>
            </div>

            <div className="flex justify-end pt-3 border-t" style={{ borderColor: 'var(--border)' }}>
              <button
                type="button"
                onClick={() => setOpen(false)}
                className="px-3.5 py-1.5 rounded-md text-xs font-semibold cursor-pointer shadow-sm transition-all hover:opacity-95 active:scale-[0.98]"
                style={{ backgroundColor: 'var(--primary)', color: 'var(--primary-foreground)' }}
              >
                Done
              </button>
            </div>
          </div>
        </div>
      )}
    </StageShell>
  );
}

function PopoverPreview({ activeTheme }: { activeTheme: ThemePreset }) {
  const [open, setOpen] = useState(false);
  const [side, setSide] = useState<'Bottom' | 'Top' | 'Right' | 'Left'>('Bottom');
  const [showIcons, setShowIcons] = useState(true);
  const [onlyReady, setOnlyReady] = useState(true);
  const anchorWrapRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (!open) return;
    const onKey = (e: KeyboardEvent) => {
      if (e.key === 'Escape') setOpen(false);
    };
    const onOutside = (e: MouseEvent) => {
      if (anchorWrapRef.current && !anchorWrapRef.current.contains(e.target as Node)) {
        setOpen(false);
      }
    };
    window.addEventListener('keydown', onKey);
    window.addEventListener('mousedown', onOutside);
    return () => {
      window.removeEventListener('keydown', onKey);
      window.removeEventListener('mousedown', onOutside);
    };
  }, [open]);

  const placementClass = () => {
    switch (side) {
      case 'Top':
        return 'bottom-full left-1/2 -translate-x-1/2 mb-2.5';
      case 'Right':
        return 'left-full top-1/2 -translate-y-1/2 ml-2.5';
      case 'Left':
        return 'right-full top-1/2 -translate-y-1/2 mr-2.5';
      default:
        return 'top-full left-1/2 -translate-x-1/2 mt-2.5';
    }
  };

  const caretPlacementClass = () => {
    switch (side) {
      case 'Top':
        return '-bottom-[5px] left-1/2 -translate-x-1/2 border-r border-b';
      case 'Right':
        return '-left-[5px] top-1/2 -translate-y-1/2 border-l border-b';
      case 'Left':
        return '-right-[5px] top-1/2 -translate-y-1/2 border-r border-t';
      default:
        return '-top-[5px] left-1/2 -translate-x-1/2 border-l border-t';
    }
  };

  return (
    <StageShell
      title={`Popover::new("filters").side(PopoverSide::${side})`}
      themeName={activeTheme.name}
      onReset={() => {
        setOpen(false);
        setSide('Bottom');
        setShowIcons(true);
        setOnlyReady(true);
      }}
      controls={
        <>
          <ControlField label="Preferred Side">
            <SegmentedButtons options={['Bottom', 'Top', 'Right', 'Left'] as const} value={side} onChange={setSide} />
          </ControlField>
          <div className="p-3 rounded border text-xs font-mono" style={{ backgroundColor: 'var(--muted)', borderColor: 'var(--border)' }}>
            open: {String(open)} · ready_only: {String(onlyReady)}
          </div>
        </>
      }
    >
      <div ref={anchorWrapRef} className="relative inline-flex">
        <button
          type="button"
          aria-expanded={open}
          onClick={() => setOpen((v) => !v)}
          className="px-4 py-2 rounded-md border text-xs font-medium cursor-pointer shadow-xs transition-all hover:bg-[var(--accent)] active:scale-[0.98]"
          style={{
            backgroundColor: open ? 'var(--accent)' : 'var(--card)',
            color: 'var(--foreground)',
            borderColor: open ? 'var(--ring)' : 'var(--border)',
          }}
        >
          Filter Options
        </button>

        {open && (
          <div
            role="dialog"
            aria-label="Filter Options Popover"
            className={`animate-egui-menu-in absolute z-30 w-64 rounded-lg border p-3.5 shadow-xl space-y-3 ${placementClass()}`}
            style={{
              backgroundColor: 'var(--card)',
              borderColor: 'var(--border)',
              color: 'var(--card-foreground)',
            }}
          >
            <span
              aria-hidden="true"
              className={`absolute w-2.5 h-2.5 rotate-45 ${caretPlacementClass()}`}
              style={{
                backgroundColor: 'var(--card)',
                borderColor: 'var(--border)',
              }}
            />
            <div>
              <div className="text-xs font-semibold">Target Filters</div>
              <p className="text-[11px] mt-0.5" style={{ color: 'var(--muted-foreground)' }}>
                Click outside or press Esc to close.
              </p>
            </div>
            <div className="flex flex-col gap-2">
              <ToggleCheckbox checked={onlyReady} onChange={setOnlyReady} label="Ready widgets only" />
              <ToggleCheckbox checked={showIcons} onChange={setShowIcons} label="Include Painter icons" />
            </div>
            <div className="flex justify-end pt-1">
              <button
                type="button"
                onClick={() => setOpen(false)}
                className="px-2.5 py-1 rounded text-[11px] font-semibold cursor-pointer transition-all hover:opacity-95 active:scale-[0.97]"
                style={{ backgroundColor: 'var(--primary)', color: 'var(--primary-foreground)' }}
              >
                Apply
              </button>
            </div>
          </div>
        )}
      </div>
    </StageShell>
  );
}

function TooltipPreview({ activeTheme }: { activeTheme: ThemePreset }) {
  const [placement, setPlacement] = useState<'Top' | 'Bottom'>('Top');
  const [showShortcut, setShowShortcut] = useState(true);
  const [macos, setMacos] = useState(true);
  const [activeId, setActiveId] = useState<string | null>(null);

  const items = [
    { id: 'save', label: 'Save Crate', tip: 'Save module changes', shortcut: 'Cmd+S' },
    { id: 'cmd', label: 'Palette', tip: 'Open command palette', shortcut: 'Cmd+K' },
    { id: 'check', label: 'Inspect', tip: 'Verify theme contrast', shortcut: 'Alt+I' },
  ];

  return (
    <StageShell
      title="response.kit_tooltip_shortcut(text, chord)"
      themeName={activeTheme.name}
      onReset={() => {
        setPlacement('Top');
        setShowShortcut(true);
        setMacos(true);
        setActiveId(null);
      }}
      controls={
        <>
          <ControlField label="Placement">
            <SegmentedButtons options={['Top', 'Bottom'] as const} value={placement} onChange={setPlacement} />
          </ControlField>
          <ToggleCheckbox checked={showShortcut} onChange={setShowShortcut} label="Show Kbd Shortcut" />
          <ToggleCheckbox checked={macos} onChange={setMacos} label="macOS Key Glyphs" />
          <div className="p-3 rounded border text-xs font-mono" style={{ backgroundColor: 'var(--muted)', borderColor: 'var(--border)' }}>
            hovered_or_focused: {activeId ?? 'none'}
          </div>
        </>
      }
    >
      <div className="flex flex-wrap items-center justify-center gap-4">
        {items.map((item) => {
          const visible = activeId === item.id;
          return (
            <div key={item.id} className="relative inline-flex">
              <button
                type="button"
                onMouseEnter={() => setActiveId(item.id)}
                onMouseLeave={() => setActiveId((prev) => (prev === item.id ? null : prev))}
                onFocus={() => setActiveId(item.id)}
                onBlur={() => setActiveId((prev) => (prev === item.id ? null : prev))}
                className="px-3.5 py-2 rounded-md border text-xs font-medium cursor-pointer shadow-xs transition-all hover:bg-[var(--accent)] active:scale-[0.98]"
                style={{
                  backgroundColor: visible ? 'var(--accent)' : 'var(--card)',
                  color: 'var(--foreground)',
                  borderColor: visible ? 'var(--ring)' : 'var(--border)',
                }}
              >
                {item.label}
              </button>

              {visible && (
                <div
                  role="tooltip"
                  className={`animate-egui-menu-in pointer-events-none absolute left-1/2 -translate-x-1/2 z-30 flex items-center gap-2 px-2.5 py-1.5 rounded-md border shadow-lg whitespace-nowrap ${
                    placement === 'Top' ? 'bottom-full mb-2.5' : 'top-full mt-2.5'
                  }`}
                  style={{
                    backgroundColor: 'var(--card)',
                    borderColor: 'var(--border)',
                    color: 'var(--card-foreground)',
                  }}
                >
                  <span
                    aria-hidden="true"
                    className={`absolute left-1/2 -translate-x-1/2 w-2 h-2 rotate-45 ${
                      placement === 'Top' ? '-bottom-[5px] border-r border-b' : '-top-[5px] border-l border-t'
                    }`}
                    style={{
                      backgroundColor: 'var(--card)',
                      borderColor: 'var(--border)',
                    }}
                  />
                  <span className="text-xs relative z-10">{item.tip}</span>
                  {showShortcut && (
                    <span className="relative z-10">
                      <KbdChord shortcut={item.shortcut} macos={macos} size="Xs" />
                    </span>
                  )}
                </div>
              )}
            </div>
          );
        })}
      </div>
    </StageShell>
  );
}

function HoverCardPreview({ activeTheme }: { activeTheme: ThemePreset }) {
  const [open, setOpen] = useState(false);
  const [delayMs, setDelayMs] = useState<'0ms' | '200ms' | '450ms'>('200ms');
  const openTimerRef = useRef<number | null>(null);
  const closeTimerRef = useRef<number | null>(null);

  const clearTimers = () => {
    if (openTimerRef.current !== null) window.clearTimeout(openTimerRef.current);
    if (closeTimerRef.current !== null) window.clearTimeout(closeTimerRef.current);
    openTimerRef.current = null;
    closeTimerRef.current = null;
  };

  useEffect(() => () => clearTimers(), []);

  const delayNum = delayMs === '0ms' ? 0 : delayMs === '200ms' ? 200 : 450;

  const handleEnter = () => {
    if (closeTimerRef.current !== null) {
      window.clearTimeout(closeTimerRef.current);
      closeTimerRef.current = null;
    }
    if (open) return;
    if (delayNum === 0) {
      setOpen(true);
    } else {
      openTimerRef.current = window.setTimeout(() => setOpen(true), delayNum);
    }
  };

  const handleLeave = () => {
    if (openTimerRef.current !== null) {
      window.clearTimeout(openTimerRef.current);
      openTimerRef.current = null;
    }
    closeTimerRef.current = window.setTimeout(() => setOpen(false), 160);
  };

  return (
    <StageShell
      title='HoverCard::new("crate_info").open_delay(0.2)'
      themeName={activeTheme.name}
      onReset={() => {
        clearTimers();
        setOpen(false);
        setDelayMs('200ms');
      }}
      controls={
        <>
          <ControlField label="Open Delay">
            <SegmentedButtons options={['0ms', '200ms', '450ms'] as const} value={delayMs} onChange={setDelayMs} />
          </ControlField>
          <div className="p-3 rounded border text-xs font-mono" style={{ backgroundColor: 'var(--muted)', borderColor: 'var(--border)' }}>
            card_open: {String(open)} · close_grace: 160ms
          </div>
        </>
      }
    >
      <div className="relative inline-flex">
        <button
          type="button"
          onMouseEnter={handleEnter}
          onMouseLeave={handleLeave}
          onFocus={handleEnter}
          onBlur={handleLeave}
          className="px-3 py-1.5 rounded-md border text-xs font-mono font-semibold cursor-pointer underline underline-offset-4 transition-all hover:bg-[var(--accent)]"
          style={{
            backgroundColor: open ? 'var(--accent)' : 'var(--muted)',
            borderColor: open ? 'var(--ring)' : 'var(--border)',
            color: 'var(--primary)',
          }}
        >
          @egui-kit/overlays
        </button>

        {open && (
          <div
            onMouseEnter={handleEnter}
            onMouseLeave={handleLeave}
            className="animate-egui-menu-in absolute top-full left-1/2 -translate-x-1/2 mt-2.5 w-72 rounded-lg border p-4 shadow-xl z-30 space-y-2.5"
            style={{
              backgroundColor: 'var(--card)',
              borderColor: 'var(--border)',
              color: 'var(--card-foreground)',
            }}
          >
            <span
              aria-hidden="true"
              className="absolute -top-[5px] left-1/2 -translate-x-1/2 w-2.5 h-2.5 rotate-45 border-l border-t"
              style={{ backgroundColor: 'var(--card)', borderColor: 'var(--border)' }}
            />
            <div className="flex items-center gap-2.5">
              <div
                className="w-8 h-8 rounded-full flex items-center justify-center text-xs font-bold shrink-0 ring-2 ring-[var(--border)]"
                style={{ backgroundColor: 'var(--primary)', color: 'var(--primary-foreground)' }}
              >
                EK
              </div>
              <div>
                <div className="text-xs font-semibold">egui-kit crate</div>
                <div className="text-[11px] font-mono" style={{ color: 'var(--muted-foreground)' }}>
                  v0.6.0 · egui 0.30
                </div>
              </div>
            </div>
            <p className="text-xs leading-relaxed" style={{ color: 'var(--muted-foreground)' }}>
              Immediate-mode overlays, portals, and 36 synchronized gpui-kit theme palettes.
            </p>
            <div className="flex items-center justify-between text-[11px] font-mono pt-1 border-t" style={{ borderColor: 'var(--border)' }}>
              <span>60 / 80 widgets</span>
              <span style={{ color: 'var(--success)' }}>MIT / Apache-2.0</span>
            </div>
          </div>
        )}
      </div>
    </StageShell>
  );
}

function MenuPreview({ activeTheme }: { activeTheme: ThemePreset }) {
  const [open, setOpen] = useState(false);
  const [subOpen, setSubOpen] = useState(false);
  const [formatOnSave, setFormatOnSave] = useState(true);
  const [tabSize, setTabSize] = useState<'2' | '4'>('4');
  const [lastTriggered, setLastTriggered] = useState('None yet');
  const wrapRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (!open) return;
    const onKey = (e: KeyboardEvent) => {
      if (e.key === 'Escape') {
        setOpen(false);
        setSubOpen(false);
      }
    };
    const onOutside = (e: MouseEvent) => {
      if (wrapRef.current && !wrapRef.current.contains(e.target as Node)) {
        setOpen(false);
        setSubOpen(false);
      }
    };
    window.addEventListener('keydown', onKey);
    window.addEventListener('mousedown', onOutside);
    return () => {
      window.removeEventListener('keydown', onKey);
      window.removeEventListener('mousedown', onOutside);
    };
  }, [open]);

  const triggerAction = (label: string) => {
    setLastTriggered(label);
    setOpen(false);
    setSubOpen(false);
  };

  return (
    <StageShell
      title="Menu::dropdown / Menu::context"
      themeName={activeTheme.name}
      onReset={() => {
        setOpen(false);
        setSubOpen(false);
        setFormatOnSave(true);
        setTabSize('4');
        setLastTriggered('None yet');
      }}
      controls={
        <div className="p-3 rounded border text-xs font-mono space-y-1.5" style={{ backgroundColor: 'var(--muted)', borderColor: 'var(--border)' }}>
          <div>last_action: {lastTriggered}</div>
          <div>format_on_save: {String(formatOnSave)}</div>
          <div>tab_width: {tabSize} spaces</div>
        </div>
      }
    >
      <div ref={wrapRef} className="relative flex flex-col items-center gap-3">
        <button
          type="button"
          aria-expanded={open}
          onClick={() => {
            setOpen((v) => !v);
            setSubOpen(false);
          }}
          onContextMenu={(e) => {
            e.preventDefault();
            setOpen(true);
          }}
          className="px-4 py-2.5 rounded-md border text-xs font-medium cursor-pointer shadow-xs transition-all hover:bg-[var(--accent)] active:scale-[0.98]"
          style={{
            backgroundColor: open ? 'var(--accent)' : 'var(--card)',
            borderColor: open ? 'var(--ring)' : 'var(--border)',
            color: 'var(--foreground)',
          }}
        >
          Open / Right-Click Context Menu
        </button>

        {open && (
          <div
            role="menu"
            className="animate-egui-menu-in absolute top-full left-1/2 -translate-x-1/2 mt-2 w-60 rounded-lg border p-1.5 shadow-xl z-30 space-y-0.5"
            style={{ backgroundColor: 'var(--card)', borderColor: 'var(--border)' }}
          >
            <button
              type="button"
              role="menuitem"
              onClick={() => triggerAction('Rename Symbol')}
              className="w-full flex items-center justify-between px-2.5 py-1.5 rounded text-xs text-left cursor-pointer transition-colors hover:bg-[var(--accent)]"
            >
              <span className="flex items-center gap-2">
                <Pencil className="w-3.5 h-3.5" style={{ color: 'var(--muted-foreground)' }} aria-hidden="true" />
                <span>Rename Symbol</span>
              </span>
              <KbdChord shortcut="F2" macos size="Xs" />
            </button>

            <button
              type="button"
              role="menuitemcheckbox"
              aria-checked={formatOnSave}
              onClick={() => {
                setFormatOnSave((v) => !v);
                setLastTriggered(`format_on_save=${!formatOnSave}`);
              }}
              className="w-full flex items-center gap-2 px-2.5 py-1.5 rounded text-xs text-left cursor-pointer transition-colors hover:bg-[var(--accent)]"
            >
              <span className="w-3.5 h-3.5 flex items-center justify-center" style={{ color: 'var(--primary)' }}>
                {formatOnSave && <Check className="w-3.5 h-3.5 animate-egui-check-draw" aria-hidden="true" />}
              </span>
              <span>Format on Save</span>
            </button>

            <div className="my-1 border-t" style={{ borderColor: 'var(--border)' }} />

            {(['2', '4'] as const).map((sz) => (
              <button
                key={sz}
                type="button"
                role="menuitemradio"
                aria-checked={tabSize === sz}
                onClick={() => {
                  setTabSize(sz);
                  setLastTriggered(`Tab width: ${sz}`);
                }}
                className="w-full flex items-center gap-2 px-2.5 py-1.5 rounded text-xs text-left cursor-pointer transition-colors hover:bg-[var(--accent)]"
              >
                <span className="w-3.5 h-3.5 flex items-center justify-center">
                  {tabSize === sz && (
                    <span className="w-2 h-2 rounded-full animate-egui-dot-pop" style={{ backgroundColor: 'var(--primary)' }} />
                  )}
                </span>
                <span>Tab Width: {sz} spaces</span>
              </button>
            ))}

            <div className="my-1 border-t" style={{ borderColor: 'var(--border)' }} />

            <div
              className="relative"
              onMouseEnter={() => setSubOpen(true)}
              onMouseLeave={() => setSubOpen(false)}
            >
              <button
                type="button"
                role="menuitem"
                aria-haspopup="menu"
                aria-expanded={subOpen}
                onClick={() => setSubOpen((v) => !v)}
                className="w-full flex items-center justify-between px-2.5 py-1.5 rounded text-xs text-left cursor-pointer transition-colors hover:bg-[var(--accent)]"
              >
                <span className="flex items-center gap-2">
                  <Share2 className="w-3.5 h-3.5" style={{ color: 'var(--muted-foreground)' }} aria-hidden="true" />
                  <span>Export As</span>
                </span>
                <ChevronRight className="w-3.5 h-3.5" style={{ color: 'var(--muted-foreground)' }} aria-hidden="true" />
              </button>

              {subOpen && (
                <div
                  role="menu"
                  className="animate-egui-menu-in absolute left-full top-0 ml-1.5 w-48 rounded-lg border p-1.5 shadow-xl z-40 space-y-0.5"
                  style={{ backgroundColor: 'var(--card)', borderColor: 'var(--border)' }}
                >
                  {['Rust Module (.rs)', 'Theme JSON', 'Markdown Spec'].map((fmt) => (
                    <button
                      key={fmt}
                      type="button"
                      role="menuitem"
                      onClick={() => triggerAction(`Export -> ${fmt}`)}
                      className="w-full flex items-center gap-2 px-2.5 py-1.5 rounded text-xs text-left cursor-pointer transition-colors hover:bg-[var(--accent)]"
                    >
                      <FileCode className="w-3.5 h-3.5 shrink-0" style={{ color: 'var(--muted-foreground)' }} aria-hidden="true" />
                      <span>{fmt}</span>
                    </button>
                  ))}
                </div>
              )}
            </div>
          </div>
        )}
      </div>
    </StageShell>
  );
}

type ToastTone = 'Info' | 'Success' | 'Warning' | 'Destructive';
type ToastEdge = 'TopRight' | 'BottomRight' | 'TopLeft' | 'BottomLeft';

interface ActiveToast {
  id: number;
  tone: ToastTone;
  title: string;
  message: string;
  actionLabel?: string;
  createdAt: number;
  durationMs: number;
}

function NotificationPreview({ activeTheme }: { activeTheme: ThemePreset }) {
  const [edge, setEdge] = useState<ToastEdge>('BottomRight');
  const [autoDismiss, setAutoDismiss] = useState(true);
  const [toasts, setToasts] = useState<ActiveToast[]>([]);
  const [lastEvent, setLastEvent] = useState('Push a toast to slide from viewport edge');
  const [nowMs, setNowMs] = useState(() => Date.now());
  const seqRef = useRef(1);

  const pushToast = (tone: ToastTone) => {
    const id = seqRef.current;
    seqRef.current += 1;
    const copy: Record<ToastTone, { title: string; message: string; actionLabel?: string }> = {
      Info: { title: 'Theme Synced', message: `Applied ${activeTheme.name} tokens.`, actionLabel: 'Inspect' },
      Success: { title: 'Crate Compiled', message: '60 / 80 egui-kit widgets verified.', actionLabel: 'View' },
      Warning: { title: 'Deprecated Prop', message: 'Prefer .kit_notify() over legacy call.' },
      Destructive: { title: 'Link Failed', message: 'Check wasm32 target toolchain.' },
    };
    const next: ActiveToast = {
      id,
      tone,
      ...copy[tone],
      createdAt: Date.now(),
      durationMs: 5500,
    };
    setToasts((prev) => clampNotificationStack([...prev, next], 5));
    setLastEvent(`Pushed ${tone} toast #${id} (${edge})`);
  };

  useEffect(() => {
    pushToast('Success');
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  useEffect(() => {
    if (toasts.length === 0 || !autoDismiss) return;
    let rafId = 0;
    const tick = () => {
      const t = Date.now();
      setNowMs(t);
      setToasts((prev) => {
        const next = prev.filter((item) => t - item.createdAt < item.durationMs);
        return next.length === prev.length ? prev : next;
      });
      rafId = window.requestAnimationFrame(tick);
    };
    rafId = window.requestAnimationFrame(tick);
    return () => window.cancelAnimationFrame(rafId);
  }, [toasts.length, autoDismiss]);

  const edgePositionClass = (): string => {
    switch (edge) {
      case 'TopRight':
        return 'top-5 right-5 items-end flex-col';
      case 'TopLeft':
        return 'top-5 left-5 items-start flex-col';
      case 'BottomLeft':
        return 'bottom-5 left-5 items-start flex-col-reverse';
      default:
        return 'bottom-5 right-5 items-end flex-col-reverse';
    }
  };

  const toneColor = (tone: ToastTone): string => {
    switch (tone) {
      case 'Success':
        return 'var(--success)';
      case 'Warning':
        return 'var(--warning)';
      case 'Destructive':
        return 'var(--destructive)';
      default:
        return 'var(--primary)';
    }
  };

  const isLeftEdge = edge === 'TopLeft' || edge === 'BottomLeft';

  return (
    <StageShell
      title={`NotificationManager::show(ctx, NotificationEdge::${edge})`}
      themeName={activeTheme.name}
      onReset={() => {
        setEdge('BottomRight');
        setAutoDismiss(true);
        setToasts([]);
        setLastEvent('Cleared toast stack');
      }}
      controls={
        <>
          <ControlField label="Screen Edge">
            <SegmentedButtons
              options={['TopRight', 'BottomRight', 'TopLeft', 'BottomLeft'] as const}
              value={edge}
              onChange={setEdge}
            />
          </ControlField>
          <ToggleCheckbox checked={autoDismiss} onChange={setAutoDismiss} label="Auto-dismiss countdown (5.5s)" />
          <div className="p-3 rounded border text-xs font-mono space-y-1" style={{ backgroundColor: 'var(--muted)', borderColor: 'var(--border)' }}>
            <div>active_toasts: {toasts.length} / 5</div>
            <div className="truncate">{lastEvent}</div>
          </div>
        </>
      }
    >
      <div className="flex flex-col items-center gap-4 text-center">
        <div className="flex flex-wrap items-center justify-center gap-2">
          {(['Info', 'Success', 'Warning', 'Destructive'] as const).map((tone) => (
            <button
              key={tone}
              type="button"
              onClick={() => pushToast(tone)}
              className="px-3 py-1.5 rounded-md border text-xs font-semibold cursor-pointer shadow-xs transition-all hover:bg-[var(--accent)] active:scale-[0.97]"
              style={{
                backgroundColor: 'var(--card)',
                borderColor: 'var(--border)',
                color: toneColor(tone),
              }}
            >
              Push {tone}
            </button>
          ))}
          <button
            type="button"
            onClick={() => {
              setToasts([]);
              setLastEvent('Cleared all toasts');
            }}
            className="px-3 py-1.5 rounded-md text-xs font-medium cursor-pointer transition-opacity hover:opacity-80 active:scale-[0.97]"
            style={{ backgroundColor: 'var(--muted)', color: 'var(--muted-foreground)' }}
          >
            Clear All
          </button>
        </div>
        <p className="text-xs max-w-sm" style={{ color: 'var(--muted-foreground)' }}>
          Toasts slide directly out from the selected <code className="font-mono">{edge}</code> screen corner with
          stacked spacing and smooth 60fps countdown bars.
        </p>
      </div>

      {toasts.length > 0 && (
        <div
          aria-live="polite"
          className={`fixed z-50 flex gap-2.5 pointer-events-none ${edgePositionClass()}`}
        >
          {toasts.map((toast) => {
            const color = toneColor(toast.tone);
            const remainingFrac = autoDismiss
              ? Math.max(0, Math.min(1, 1 - (nowMs - toast.createdAt) / toast.durationMs))
              : 1;
            return (
              <div
                key={toast.id}
                className={`pointer-events-auto relative w-80 max-w-[90vw] rounded-xl border overflow-hidden shadow-2xl ${
                  isLeftEdge ? 'animate-egui-toast-left-in' : 'animate-egui-toast-in'
                }`}
                style={{
                  backgroundColor: 'var(--card)',
                  borderColor: `color-mix(in srgb, ${color} 34%, var(--border))`,
                  color: 'var(--card-foreground)',
                }}
              >
                <div className="p-3.5 flex items-start gap-2.5">
                  <div
                    className="w-7 h-7 rounded-lg flex items-center justify-center shrink-0 mt-0.5 border"
                    style={{
                      backgroundColor: `color-mix(in srgb, ${color} 15%, var(--muted))`,
                      borderColor: `color-mix(in srgb, ${color} 28%, transparent)`,
                      color,
                    }}
                  >
                    {toast.tone === 'Success' ? (
                      <Check className="w-3.5 h-3.5 animate-egui-check-draw" aria-hidden="true" />
                    ) : toast.tone === 'Info' ? (
                      <Info className="w-3.5 h-3.5" aria-hidden="true" />
                    ) : (
                      <AlertTriangle className="w-3.5 h-3.5" aria-hidden="true" />
                    )}
                  </div>
                  <div className="flex-1 min-w-0">
                    <div className="text-xs font-semibold">{toast.title}</div>
                    <p className="text-[11px] mt-0.5 leading-relaxed" style={{ color: 'var(--muted-foreground)' }}>
                      {toast.message}
                    </p>
                    {toast.actionLabel && (
                      <button
                        type="button"
                        onClick={() => {
                          setLastEvent(`Clicked "${toast.actionLabel}" on toast #${toast.id}`);
                          setToasts((prev) => prev.filter((item) => item.id !== toast.id));
                        }}
                        className="mt-2 px-2 py-0.5 rounded border text-[11px] font-medium cursor-pointer transition-colors hover:bg-[var(--accent)]"
                        style={{
                          backgroundColor: 'var(--background)',
                          borderColor: 'var(--border)',
                          color: 'var(--foreground)',
                        }}
                      >
                        {toast.actionLabel}
                      </button>
                    )}
                  </div>
                  <button
                    type="button"
                    aria-label="Dismiss notification"
                    onClick={() => setToasts((prev) => prev.filter((item) => item.id !== toast.id))}
                    className="p-1 rounded-md cursor-pointer transition-opacity hover:opacity-80 active:scale-95"
                    style={{ color: 'var(--muted-foreground)' }}
                  >
                    <X className="w-3.5 h-3.5" aria-hidden="true" />
                  </button>
                </div>
                <div className="h-[3px] w-full" style={{ backgroundColor: 'var(--muted)' }}>
                  <div
                    className="h-full"
                    style={{ width: `${(remainingFrac * 100).toFixed(2)}%`, backgroundColor: color }}
                  />
                </div>
              </div>
            );
          })}
        </div>
      )}
    </StageShell>
  );
}

function FocusTrapPreview({ activeTheme }: { activeTheme: ThemePreset }) {
  const [active, setActive] = useState(true);
  const [showRing, setShowRing] = useState(true);
  const [focusedIdx, setFocusedIdx] = useState<number>(0);
  const [crateInput, setCrateInput] = useState('egui-kit');
  const [lto, setLto] = useState(true);
  const refs = useRef<Array<HTMLElement | null>>([null, null, null, null]);
  const labels = ['Crate Input', 'Thin LTO Toggle', 'Cancel Button', 'Apply Button'];

  const focusAt = (nextIdx: number) => {
    setFocusedIdx(nextIdx);
    refs.current[nextIdx]?.focus();
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (!active || e.key !== 'Tab') return;
    e.preventDefault();
    const nextIdx = cycleFocusIndex(focusedIdx, labels.length, e.shiftKey);
    focusAt(nextIdx);
  };

  return (
    <StageShell
      title='FocusTrap::new("scope").active(true).show_ring(true)'
      themeName={activeTheme.name}
      onReset={() => {
        setActive(true);
        setShowRing(true);
        setFocusedIdx(0);
        setCrateInput('egui-kit');
        setLto(true);
      }}
      controls={
        <>
          <ToggleCheckbox checked={active} onChange={setActive} label="Trap Active (Lock Tab Loop)" />
          <ToggleCheckbox checked={showRing} onChange={setShowRing} label="Show Focus Ring" />
          <div className="flex flex-wrap gap-2">
            <button
              type="button"
              onClick={() => focusAt(cycleFocusIndex(focusedIdx, labels.length, true))}
              className="inline-flex items-center gap-1.5 px-2.5 py-1 rounded border text-xs cursor-pointer transition-colors hover:bg-[var(--accent)] active:scale-[0.97]"
              style={{ borderColor: 'var(--border)', backgroundColor: 'var(--background)' }}
            >
              <span>Prev</span>
              <KbdChord shortcut="Shift+Tab" macos size="Xs" />
            </button>
            <button
              type="button"
              onClick={() => focusAt(cycleFocusIndex(focusedIdx, labels.length, false))}
              className="inline-flex items-center gap-1.5 px-2.5 py-1 rounded border text-xs cursor-pointer transition-colors hover:bg-[var(--accent)] active:scale-[0.97]"
              style={{ borderColor: 'var(--border)', backgroundColor: 'var(--background)' }}
            >
              <span>Step</span>
              <KbdChord shortcut="Tab" macos size="Xs" />
            </button>
          </div>
          <div className="p-3 rounded border text-xs font-mono" style={{ backgroundColor: 'var(--muted)', borderColor: 'var(--border)' }}>
            focused: [{focusedIdx}] {labels[focusedIdx]}
          </div>
        </>
      }
    >
      <div
        onKeyDown={handleKeyDown}
        className="w-full max-w-sm rounded-lg border p-4 space-y-3 transition-shadow duration-200"
        style={{
          backgroundColor: 'var(--card)',
          borderColor: active && showRing ? 'var(--ring)' : 'var(--border)',
          boxShadow: active && showRing ? '0 0 0 2px var(--ring)' : 'none',
        }}
      >
        <div className="flex items-center justify-between text-xs">
          <span className="font-semibold">Scoped Overlay Form</span>
          <span
            key={String(active)}
            className="animate-egui-pop font-mono text-[11px] font-semibold px-2 py-0.5 rounded border"
            style={{
              color: active ? 'var(--success)' : 'var(--muted-foreground)',
              borderColor: active ? 'color-mix(in srgb, var(--success) 35%, var(--border))' : 'var(--border)',
              backgroundColor: active ? 'color-mix(in srgb, var(--success) 10%, var(--muted))' : 'var(--muted)',
            }}
          >
            {active ? 'TRAPPED' : 'UNLOCKED'}
          </span>
        </div>

        <input
          ref={(el) => {
            refs.current[0] = el;
          }}
          type="text"
          value={crateInput}
          onFocus={() => setFocusedIdx(0)}
          onChange={(e) => setCrateInput(e.target.value)}
          aria-label="Target Crate"
          className="w-full px-3 py-1.5 rounded-md border text-xs font-mono outline-none transition-colors"
          style={{
            backgroundColor: 'var(--background)',
            color: 'var(--foreground)',
            borderColor: focusedIdx === 0 ? 'var(--primary)' : 'var(--border)',
          }}
        />

        <label className="inline-flex items-center gap-2.5 text-xs cursor-pointer select-none">
          <span className="relative inline-flex items-center justify-center">
            <input
              ref={(el) => {
                refs.current[1] = el;
              }}
              type="checkbox"
              checked={lto}
              onFocus={() => setFocusedIdx(1)}
              onChange={(e) => setLto(e.target.checked)}
              className="sr-only peer"
            />
            <span
              aria-hidden="true"
              className="w-4 h-4 rounded-[4px] border flex items-center justify-center transition-all peer-focus-visible:ring-2 peer-focus-visible:ring-[var(--ring)]"
              style={{
                backgroundColor: lto ? 'var(--primary)' : 'var(--background)',
                borderColor: focusedIdx === 1 ? 'var(--ring)' : lto ? 'var(--primary)' : 'var(--border)',
                color: 'var(--primary-foreground)',
              }}
            >
              {lto && <Check className="w-3 h-3 stroke-[2.5] animate-egui-check-draw" />}
            </span>
          </span>
          <span>Enable Thin LTO Optimization</span>
        </label>

        <div className="flex items-center justify-end gap-2 pt-1">
          <button
            ref={(el) => {
              refs.current[2] = el;
            }}
            type="button"
            onFocus={() => setFocusedIdx(2)}
            onClick={() => setFocusedIdx(2)}
            className="px-3 py-1.5 rounded border text-xs font-medium cursor-pointer transition-colors hover:bg-[var(--accent)]"
            style={{
              borderColor: focusedIdx === 2 ? 'var(--primary)' : 'var(--border)',
              backgroundColor: 'transparent',
            }}
          >
            Cancel
          </button>
          <button
            ref={(el) => {
              refs.current[3] = el;
            }}
            type="button"
            onFocus={() => setFocusedIdx(3)}
            onClick={() => setFocusedIdx(3)}
            className="px-3 py-1.5 rounded text-xs font-semibold cursor-pointer shadow-xs transition-all hover:opacity-95 active:scale-[0.98]"
            style={{
              backgroundColor: 'var(--primary)',
              color: 'var(--primary-foreground)',
              outline: focusedIdx === 3 ? '2px solid var(--ring)' : 'none',
              outlineOffset: '2px',
            }}
          >
            Apply
          </button>
        </div>
      </div>
    </StageShell>
  );
}

interface PaletteCommand {
  id: string;
  label: string;
  group: string;
  shortcut?: string;
}

const PALETTE_COMMANDS: PaletteCommand[] = [
  { id: 'theme', label: 'Switch Theme Palette', group: 'Appearance', shortcut: 'Cmd+T' },
  { id: 'build', label: 'Run Cargo Check', group: 'Workspace', shortcut: 'Cmd+B' },
  { id: 'toast', label: 'Push Edge Notification', group: 'Overlays', shortcut: 'Cmd+N' },
  { id: 'dialog', label: 'Open Preferences Dialog', group: 'Overlays', shortcut: 'Cmd+,' },
  { id: 'docs', label: 'Inspect Rust Crate Source', group: 'Navigation', shortcut: 'Cmd+Shift+S' },
];

function CommandPreview({ activeTheme }: { activeTheme: ThemePreset }) {
  const [open, setOpen] = useState(false);
  const [query, setQuery] = useState('');
  const [selectedIdx, setSelectedIdx] = useState(0);
  const [lastExecuted, setLastExecuted] = useState('None yet');
  const inputRef = useRef<HTMLInputElement>(null);
  const itemRefs = useRef<Array<HTMLButtonElement | null>>([]);

  const filtered = useMemo(() => filterCommandItems(PALETTE_COMMANDS, query), [query]);

  useEffect(() => {
    const onGlobalKey = (e: KeyboardEvent) => {
      if ((e.metaKey || e.ctrlKey) && e.key.toLowerCase() === 'k') {
        e.preventDefault();
        setOpen((v) => !v);
      } else if (e.key === 'Escape' && open) {
        setOpen(false);
      }
    };
    window.addEventListener('keydown', onGlobalKey);
    return () => window.removeEventListener('keydown', onGlobalKey);
  }, [open]);

  useEffect(() => {
    if (open) {
      window.setTimeout(() => inputRef.current?.focus(), 10);
    }
  }, [open]);

  useEffect(() => {
    if (open) {
      itemRefs.current[selectedIdx]?.scrollIntoView({ block: 'nearest' });
    }
  }, [open, selectedIdx]);

  const executeItem = (item: PaletteCommand) => {
    setLastExecuted(`${item.label} (${item.id})`);
    setOpen(false);
    setQuery('');
    setSelectedIdx(0);
  };

  return (
    <StageShell
      title='Command::new("cmd", &mut open, &items).show(ctx)'
      themeName={activeTheme.name}
      onReset={() => {
        setOpen(false);
        setQuery('');
        setSelectedIdx(0);
        setLastExecuted('None yet');
      }}
      controls={
        <div className="p-3 rounded border text-xs font-mono space-y-1.5" style={{ backgroundColor: 'var(--muted)', borderColor: 'var(--border)' }}>
          <div>open: {String(open)}</div>
          <div>executed: {lastExecuted}</div>
        </div>
      }
    >
      <div className="flex flex-col items-center gap-3">
        <button
          type="button"
          onClick={() => setOpen(true)}
          className="inline-flex items-center gap-3 px-4 py-2 rounded-md border text-xs font-medium cursor-pointer shadow-xs transition-all hover:bg-[var(--accent)] active:scale-[0.98]"
          style={{
            backgroundColor: 'var(--card)',
            borderColor: 'var(--border)',
            color: 'var(--foreground)',
          }}
        >
          <CommandIcon className="w-3.5 h-3.5" style={{ color: 'var(--muted-foreground)' }} aria-hidden="true" />
          <span>Open Command Palette</span>
          <KbdChord shortcut="Cmd+K" macos size="Xs" />
        </button>
        <span className="text-xs font-mono" style={{ color: 'var(--muted-foreground)' }}>
          Supports subsequence fuzzy search, ArrowUp/Down, Enter &amp; Esc
        </span>
      </div>

      {open && (
        <div
          className="fixed inset-0 z-50 flex items-start justify-center pt-24 px-4 animate-egui-fade-in backdrop-blur-[2px]"
          style={{ backgroundColor: 'rgba(8, 12, 20, 0.48)' }}
          onMouseDown={(e) => {
            if (e.target === e.currentTarget) setOpen(false);
          }}
        >
          <div
            role="dialog"
            aria-modal="true"
            aria-label="Command Palette"
            className="animate-egui-modal-in w-full max-w-lg rounded-xl border p-2.5 shadow-2xl space-y-2"
            style={{
              backgroundColor: 'var(--card)',
              borderColor: 'var(--border)',
              color: 'var(--card-foreground)',
            }}
          >
            <div className="flex items-center gap-2 px-2 pb-2 border-b" style={{ borderColor: 'var(--border)' }}>
              <Search className="w-4 h-4 shrink-0" style={{ color: 'var(--muted-foreground)' }} aria-hidden="true" />
              <input
                ref={inputRef}
                type="text"
                value={query}
                onChange={(e) => {
                  setQuery(e.target.value);
                  setSelectedIdx(0);
                }}
                onKeyDown={(e) => {
                  if (e.key === 'ArrowDown' && filtered.length > 0) {
                    e.preventDefault();
                    setSelectedIdx((i) => (i + 1) % filtered.length);
                  } else if (e.key === 'ArrowUp' && filtered.length > 0) {
                    e.preventDefault();
                    setSelectedIdx((i) => (i === 0 ? filtered.length - 1 : i - 1));
                  } else if (e.key === 'Enter' && filtered[selectedIdx]) {
                    e.preventDefault();
                    executeItem(filtered[selectedIdx]);
                  }
                }}
                placeholder="Type a command or fuzzy query (e.g. 'swth' or 'theme')..."
                className="flex-1 bg-transparent text-xs outline-none"
                style={{ color: 'var(--foreground)' }}
              />
              <KbdChord shortcut="Esc" macos size="Xs" />
            </div>

            <div className="max-h-64 overflow-y-auto egui-scroll space-y-0.5">
              {filtered.length === 0 ? (
                <div className="py-6 text-center text-xs" style={{ color: 'var(--muted-foreground)' }}>
                  No matching commands
                </div>
              ) : (
                filtered.map((item, idx) => {
                  const active = idx === selectedIdx;
                  const spans = splitFuzzyMatchSpans(query, item.label);
                  return (
                    <button
                      key={item.id}
                      ref={(el) => {
                        itemRefs.current[idx] = el;
                      }}
                      type="button"
                      onMouseEnter={() => setSelectedIdx(idx)}
                      onClick={() => executeItem(item)}
                      className="w-full flex items-center justify-between gap-3 px-2.5 py-2 rounded-md text-xs text-left cursor-pointer transition-colors"
                      style={{
                        backgroundColor: active ? 'var(--accent)' : 'transparent',
                        color: active ? 'var(--accent-foreground)' : 'var(--foreground)',
                      }}
                    >
                      <span className="flex items-center gap-2 font-medium">
                        <Sparkles className="w-3.5 h-3.5" style={{ color: 'var(--primary)' }} aria-hidden="true" />
                        <span>
                          {spans.map((span, sIdx) =>
                            span.match ? (
                              <mark
                                key={sIdx}
                                className="bg-transparent font-semibold underline decoration-[var(--primary)] underline-offset-2"
                                style={{ color: 'var(--primary)' }}
                              >
                                {span.text}
                              </mark>
                            ) : (
                              <span key={sIdx}>{span.text}</span>
                            )
                          )}
                        </span>
                      </span>
                      <span className="flex items-center gap-2.5 shrink-0">
                        <span className="text-[11px]" style={{ color: 'var(--muted-foreground)' }}>
                          {item.group}
                        </span>
                        {item.shortcut && <KbdChord shortcut={item.shortcut} macos size="Xs" />}
                      </span>
                    </button>
                  );
                })
              )}
            </div>
          </div>
        </div>
      )}
    </StageShell>
  );
}
