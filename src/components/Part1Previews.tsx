import { useEffect, useRef, useState } from 'react';
import {
  AlertTriangle,
  Check,
  ChevronDown,
  ChevronRight,
  Copy,
  Info,
  Minus,
  Moon,
  Plus,
  RotateCcw,
  Search,
  Settings,
  Sparkles,
  Sun,
  Terminal,
  User,
  X,
} from 'lucide-react';
import type { ThemePreset } from '../types/kit';
import { type KbdIconKind, parseKbdTokens } from '../utils/catalogLogic';

export type SizeOpt = 'Xs' | 'Sm' | 'Md' | 'Lg';

export interface PreviewProps {
  slug: string;
  activeTheme: ThemePreset;
  allThemes: ThemePreset[];
  onSelectTheme: (id: string) => void;
}

export const SIZE_METRICS: Record<SizeOpt, { h: string; px: string; text: string; icon: number; radius: string }> = {
  Xs: { h: '24px', px: '8px', text: '11px', icon: 12, radius: '4px' },
  Sm: { h: '28px', px: '10px', text: '12px', icon: 14, radius: '5px' },
  Md: { h: '34px', px: '14px', text: '13.5px', icon: 16, radius: '6px' },
  Lg: { h: '40px', px: '18px', text: '15px', icon: 18, radius: '8px' },
};

export function StageShell({
  title,
  themeName,
  onReset,
  children,
  controls,
}: {
  title: string;
  themeName: string;
  onReset?: () => void;
  children: React.ReactNode;
  controls?: React.ReactNode;
}) {
  return (
    <div className="grid grid-cols-1 lg:grid-cols-12 gap-6 items-stretch">
      <div
        className="lg:col-span-8 rounded-lg border flex flex-col"
        style={{
          backgroundColor: 'var(--background)',
          borderColor: 'var(--border)',
        }}
      >
        <div
          className="flex items-center justify-between gap-4 px-4 py-2.5 border-b text-xs rounded-t-[7px]"
          style={{
            backgroundColor: 'var(--titlebar)',
            borderColor: 'var(--border)',
            color: 'var(--muted-foreground)',
          }}
        >
          <span className="font-mono truncate">
            {title} · {themeName}
          </span>
          {onReset && (
            <button
              type="button"
              onClick={onReset}
              className="inline-flex items-center gap-1.5 px-2 py-1 rounded text-xs font-medium cursor-pointer transition-colors hover:opacity-80 whitespace-nowrap shrink-0"
              style={{ color: 'var(--foreground)' }}
            >
              <RotateCcw className="w-3.5 h-3.5" aria-hidden="true" />
              <span>Reset</span>
            </button>
          )}
        </div>
        <div className="flex-1 min-h-[260px] p-8 flex items-center justify-center relative">
          {children}
        </div>
      </div>

      <div
        className="lg:col-span-4 rounded-lg border p-5 flex flex-col gap-4"
        style={{
          backgroundColor: 'var(--card)',
          borderColor: 'var(--border)',
        }}
      >
        <div className="text-sm font-semibold" style={{ color: 'var(--card-foreground)' }}>
          Interactive Props
        </div>
        {controls}
      </div>
    </div>
  );
}

export function ControlField({ label, children }: { label: string; children: React.ReactNode }) {
  return (
    <div className="flex flex-col gap-1.5">
      <span className="text-xs font-medium" style={{ color: 'var(--muted-foreground)' }}>
        {label}
      </span>
      {children}
    </div>
  );
}

export function SegmentedButtons<T extends string>({
  options,
  value,
  onChange,
}: {
  options: readonly T[];
  value: T;
  onChange: (val: T) => void;
}) {
  return (
    <div
      className="flex flex-wrap gap-1 p-1 rounded-md border"
      style={{ backgroundColor: 'var(--muted)', borderColor: 'var(--border)' }}
    >
      {options.map((opt) => {
        const active = opt === value;
        return (
          <button
            key={opt}
            type="button"
            onClick={() => onChange(opt)}
            className="px-2 py-1 rounded text-xs font-medium transition-[background-color,color,box-shadow,transform] duration-150 active:scale-[0.97] cursor-pointer whitespace-nowrap shrink-0 focus:outline-none focus-visible:ring-2 focus-visible:ring-[var(--ring)]"
            style={{
              backgroundColor: active ? 'var(--card)' : 'transparent',
              color: active ? 'var(--foreground)' : 'var(--muted-foreground)',
              boxShadow: active ? '0 1px 2px rgba(0,0,0,0.08)' : 'none',
            }}
          >
            {opt}
          </button>
        );
      })}
    </div>
  );
}

export function ToggleCheckbox({
  label,
  checked,
  onChange,
}: {
  label: string;
  checked: boolean;
  onChange: (next: boolean) => void;
}) {
  return (
    <label className="group inline-flex items-center gap-2.5 text-sm cursor-pointer select-none">
      <input
        type="checkbox"
        checked={checked}
        onChange={(e) => onChange(e.target.checked)}
        className="sr-only peer"
      />
      <span
        aria-hidden="true"
        className="w-4 h-4 rounded-[4px] border inline-flex items-center justify-center shrink-0 transition-[background-color,border-color,box-shadow,transform] duration-150 group-active:scale-90 peer-focus-visible:ring-2 peer-focus-visible:ring-[var(--ring)] group-hover:border-[var(--primary)]"
        style={{
          backgroundColor: checked ? 'var(--primary)' : 'var(--background)',
          borderColor: checked ? 'var(--primary)' : 'var(--input)',
          color: 'var(--primary-foreground)',
        }}
      >
        {checked && <Check className="w-3 h-3 animate-egui-check-draw" strokeWidth={2.75} />}
      </span>
      <span
        className="transition-colors duration-150"
        style={{ color: checked ? 'var(--foreground)' : 'var(--muted-foreground)' }}
      >
        {label}
      </span>
    </label>
  );
}

export function Part1Preview({ slug, activeTheme, allThemes, onSelectTheme }: PreviewProps) {
  switch (slug) {
    case 'root':
      return <RootPreview activeTheme={activeTheme} />;
    case 'theme':
      return (
        <ThemePreview
          activeTheme={activeTheme}
          allThemes={allThemes}
          onSelectTheme={onSelectTheme}
        />
      );
    case 'button':
      return <ButtonPreview activeTheme={activeTheme} />;
    case 'dropdown_button':
      return <DropdownButtonPreview activeTheme={activeTheme} />;
    case 'icon':
      return <IconPreview activeTheme={activeTheme} />;
    case 'badge':
      return <BadgePreview activeTheme={activeTheme} />;
    case 'tag':
      return <TagPreview activeTheme={activeTheme} />;
    case 'label':
      return <LabelPreview activeTheme={activeTheme} />;
    case 'kbd':
      return <KbdPreview activeTheme={activeTheme} />;
    case 'avatar':
      return <AvatarPreview activeTheme={activeTheme} />;
    default:
      return null;
  }
}

function RootPreview({ activeTheme }: { activeTheme: ThemePreset }) {
  const [toasts, setToasts] = useState<Array<{ id: number; title: string; tone: 'Info' | 'Success' | 'Warning' | 'Destructive' }>>([
    { id: 1, title: 'Workspace initialized', tone: 'Success' },
  ]);
  const [modalOpen, setModalOpen] = useState(false);
  const [tone, setTone] = useState<'Info' | 'Success' | 'Warning' | 'Destructive'>('Success');
  const nextIdRef = useRef(2);

  const pushToast = () => {
    const id = nextIdRef.current++;
    setToasts((prev) => [...prev.slice(-2), { id, title: `${tone} event #${id}`, tone }]);
  };

  const toneColor = (t: 'Info' | 'Success' | 'Warning' | 'Destructive') => {
    if (t === 'Success') return 'var(--success)';
    if (t === 'Warning') return 'var(--warning)';
    if (t === 'Destructive') return 'var(--destructive)';
    return 'var(--primary)';
  };

  return (
    <StageShell
      title="Root::show(ctx, ...)"
      themeName={activeTheme.name}
      onReset={() => {
        setToasts([{ id: 1, title: 'Workspace initialized', tone: 'Success' }]);
        setModalOpen(false);
      }}
      controls={
        <>
          <ControlField label="Toast Tone">
            <SegmentedButtons
              options={['Info', 'Success', 'Warning', 'Destructive'] as const}
              value={tone}
              onChange={setTone}
            />
          </ControlField>
          <ControlField label="Portal Triggers">
            <div className="flex flex-col gap-2">
              <button
                type="button"
                onClick={pushToast}
                className="px-3 py-2 rounded-md text-xs font-medium cursor-pointer transition-opacity hover:opacity-90"
                style={{ backgroundColor: 'var(--primary)', color: 'var(--primary-foreground)' }}
              >
                Push Toast to Overlay
              </button>
              <button
                type="button"
                onClick={() => setModalOpen(true)}
                className="px-3 py-2 rounded-md text-xs font-medium border cursor-pointer"
                style={{
                  backgroundColor: 'var(--secondary)',
                  color: 'var(--secondary-foreground)',
                  borderColor: 'var(--border)',
                }}
              >
                Open Root Modal Scrim
              </button>
            </div>
          </ControlField>
          <p className="text-xs leading-relaxed" style={{ color: 'var(--muted-foreground)' }}>
            Active overlays render in foreground layers above CentralPanel without clipping inside child scroll regions.
          </p>
        </>
      }
    >
      <div className="w-full h-full min-h-[240px] flex flex-col justify-between relative">
        <div className="flex flex-col items-start justify-start gap-3">
          <div className="text-sm font-semibold" style={{ color: 'var(--foreground)' }}>
            CentralPanel Host Area
          </div>
          <p className="text-xs max-w-xs" style={{ color: 'var(--muted-foreground)' }}>
            Trigger global overlays below; the bottom-right portal anchors cleanly over workspace panels.
          </p>
          <div className="flex flex-wrap gap-2">
            <button
              type="button"
              onClick={pushToast}
              className="px-3.5 py-2 rounded-md text-xs font-medium cursor-pointer"
              style={{ backgroundColor: 'var(--primary)', color: 'var(--primary-foreground)' }}
            >
              Push Toast
            </button>
            <button
              type="button"
              onClick={() => setModalOpen(true)}
              className="px-3.5 py-2 rounded-md text-xs font-medium border cursor-pointer"
              style={{
                backgroundColor: 'var(--card)',
                color: 'var(--foreground)',
                borderColor: 'var(--border)',
              }}
            >
              Open Modal
            </button>
          </div>
        </div>

        {/* Toast portal stack */}
        <div className="absolute right-0 bottom-0 flex flex-col gap-2 w-64 z-10">
          {toasts.map((t) => {
            const col = toneColor(t.tone);
            return (
              <div
                key={t.id}
                className="animate-egui-toast-in flex items-center justify-between gap-2 px-3 py-2.5 rounded-lg border text-xs shadow-lg"
                style={{
                  backgroundColor: 'var(--card)',
                  borderColor: `color-mix(in srgb, ${col} 40%, var(--border))`,
                  color: 'var(--card-foreground)',
                }}
              >
                <div className="flex items-center gap-2 min-w-0">
                  <span
                    className="w-5 h-5 rounded-md flex items-center justify-center shrink-0"
                    style={{ backgroundColor: `color-mix(in srgb, ${col} 15%, var(--muted))`, color: col }}
                    aria-hidden="true"
                  >
                    {t.tone === 'Success' ? (
                      <Check className="w-3 h-3" />
                    ) : t.tone === 'Info' ? (
                      <Info className="w-3 h-3" />
                    ) : (
                      <AlertTriangle className="w-3 h-3" />
                    )}
                  </span>
                  <span className="truncate font-medium">{t.title}</span>
                </div>
                <button
                  type="button"
                  aria-label={`Dismiss ${t.title}`}
                  onClick={() => setToasts((prev) => prev.filter((item) => item.id !== t.id))}
                  className="p-0.5 rounded cursor-pointer opacity-70 hover:opacity-100 transition-opacity"
                >
                  <X className="w-3.5 h-3.5" aria-hidden="true" />
                </button>
              </div>
            );
          })}
        </div>

        {/* Root Modal Scrim */}
        {modalOpen && (
          <div
            className="animate-egui-fade-in absolute inset-0 rounded-md flex items-center justify-center p-4 z-20 backdrop-blur-[2px]"
            style={{ backgroundColor: 'rgba(8, 12, 20, 0.48)' }}
          >
            <div
              role="dialog"
              aria-modal="true"
              aria-label="Root Modal Overlay"
              className="animate-egui-modal-in w-full max-w-xs rounded-xl border p-4 flex flex-col gap-3 shadow-2xl"
              style={{
                backgroundColor: 'var(--card)',
                borderColor: 'var(--border)',
                color: 'var(--card-foreground)',
              }}
            >
              <div className="text-sm font-semibold">Root Overlay Dialog</div>
              <p className="text-xs leading-relaxed" style={{ color: 'var(--muted-foreground)' }}>
                Hosted by Root::open_modal(ctx, title, body) above all workspace panels.
              </p>
              <div className="flex justify-end">
                <button
                  type="button"
                  onClick={() => setModalOpen(false)}
                  className="px-3 py-1.5 rounded-md text-xs font-medium cursor-pointer active:scale-[0.97] transition-transform"
                  style={{ backgroundColor: 'var(--primary)', color: 'var(--primary-foreground)' }}
                >
                  Close Modal
                </button>
              </div>
            </div>
          </div>
        )}
      </div>
    </StageShell>
  );
}

function ThemePreview({
  activeTheme,
  allThemes,
  onSelectTheme,
}: {
  activeTheme: ThemePreset;
  allThemes: ThemePreset[];
  onSelectTheme: (id: string) => void;
}) {
  const [showSwatches, setShowSwatches] = useState(true);
  const [size, setSize] = useState<SizeOpt>('Md');

  return (
    <StageShell
      title="ThemePicker::new(...)"
      themeName={activeTheme.name}
      onReset={() => onSelectTheme('aurora-aurora-light')}
      controls={
        <>
          <ControlField label="Control Size">
            <SegmentedButtons options={['Xs', 'Sm', 'Md', 'Lg'] as const} value={size} onChange={setSize} />
          </ControlField>
          <ToggleCheckbox
            label="Show inline color swatches"
            checked={showSwatches}
            onChange={setShowSwatches}
          />
          <ControlField label="Quick Presets">
            <div className="grid grid-cols-2 gap-1.5">
              {['aurora-aurora-light', 'tokyonight-tokyo-night', 'catppuccin-catppuccin-mocha', 'gruvbox-gruvbox-dark'].map(
                (id) => {
                  const t = allThemes.find((item) => item.id === id);
                  if (!t) return null;
                  return (
                    <button
                      key={t.id}
                      type="button"
                      onClick={() => onSelectTheme(t.id)}
                      className="px-2.5 py-1.5 rounded border text-xs font-medium text-left truncate cursor-pointer transition-[background-color,color,transform] duration-150 active:scale-[0.97] flex items-center justify-between gap-1.5"
                      style={{
                        backgroundColor: activeTheme.id === t.id ? 'var(--primary)' : 'var(--muted)',
                        color: activeTheme.id === t.id ? 'var(--primary-foreground)' : 'var(--foreground)',
                        borderColor: activeTheme.id === t.id ? 'var(--primary)' : 'var(--border)',
                      }}
                    >
                      <span className="truncate">{t.name}</span>
                      {activeTheme.id === t.id && <Check className="w-3 h-3 shrink-0 animate-egui-check-draw" aria-hidden="true" />}
                    </button>
                  );
                }
              )}
            </div>
          </ControlField>
        </>
      }
    >
      <div className="w-full max-w-sm flex flex-col gap-4">
        <label className="text-xs font-medium" style={{ color: 'var(--muted-foreground)' }} htmlFor="theme-picker-select">
          Select Active gpui-kit Theme ({allThemes.length} presets)
        </label>
        <div className="flex items-center gap-3">
          {showSwatches && (
            <div
              key={activeTheme.id}
              className="animate-egui-pop flex items-center gap-1 px-2.5 rounded border shrink-0"
              style={{
                height: SIZE_METRICS[size].h,
                backgroundColor: 'var(--card)',
                borderColor: 'var(--border)',
              }}
              title="Background, Primary, Success, Warning, Destructive tokens"
            >
              <span className="w-2.5 h-2.5 rounded-sm border" style={{ backgroundColor: 'var(--background)', borderColor: 'var(--border)' }} />
              <span className="w-2.5 h-2.5 rounded-full" style={{ backgroundColor: 'var(--primary)' }} />
              <span className="w-2.5 h-2.5 rounded-full" style={{ backgroundColor: 'var(--success)' }} />
              <span className="w-2.5 h-2.5 rounded-full" style={{ backgroundColor: 'var(--warning)' }} />
              <span className="w-2.5 h-2.5 rounded-full" style={{ backgroundColor: 'var(--destructive)' }} />
            </div>
          )}
          <select
            id="theme-picker-select"
            value={activeTheme.id}
            onChange={(e) => onSelectTheme(e.target.value)}
            className="flex-1 rounded border px-3 font-medium cursor-pointer transition-colors focus:outline-none focus-visible:ring-2 focus-visible:ring-[var(--ring)]"
            style={{
              height: SIZE_METRICS[size].h,
              fontSize: SIZE_METRICS[size].text,
              backgroundColor: 'var(--card)',
              color: 'var(--foreground)',
              borderColor: 'var(--border)',
            }}
          >
            {allThemes.map((t) => (
              <option key={t.id} value={t.id}>
                {t.name} ({t.mode})
              </option>
            ))}
          </select>
        </div>
        <div
          className="p-3 rounded-md border flex items-center justify-between text-xs"
          style={{ backgroundColor: 'var(--card)', borderColor: 'var(--border)' }}
        >
          <span style={{ color: 'var(--muted-foreground)' }}>Mode polarity</span>
          <span className="font-mono font-medium inline-flex items-center gap-1.5" style={{ color: 'var(--foreground)' }}>
            {activeTheme.mode === 'dark' ? <Moon className="w-3.5 h-3.5" aria-hidden="true" /> : <Sun className="w-3.5 h-3.5" aria-hidden="true" />}
            ThemeMode::{activeTheme.mode === 'dark' ? 'Dark' : 'Light'}
          </span>
        </div>
      </div>
    </StageShell>
  );
}

function ButtonPreview({ activeTheme }: { activeTheme: ThemePreset }) {
  const [variant, setVariant] = useState<'Primary' | 'Secondary' | 'Outline' | 'Ghost' | 'Destructive' | 'Link'>('Primary');
  const [size, setSize] = useState<SizeOpt>('Md');
  const [loading, setLoading] = useState(false);
  const [disabled, setDisabled] = useState(false);
  const [withIcon, setWithIcon] = useState(true);
  const [label, setLabel] = useState('Deploy Crate');
  const [clicks, setClicks] = useState(0);

  const m = SIZE_METRICS[size];

  const getStyle = (): React.CSSProperties => {
    const base: React.CSSProperties = {
      height: m.h,
      paddingLeft: m.px,
      paddingRight: m.px,
      fontSize: m.text,
      borderRadius: m.radius,
      opacity: disabled || loading ? 0.6 : 1,
    };
    switch (variant) {
      case 'Primary':
        return { ...base, backgroundColor: 'var(--primary)', color: 'var(--primary-foreground)' };
      case 'Secondary':
        return { ...base, backgroundColor: 'var(--secondary)', color: 'var(--secondary-foreground)', border: '1px solid var(--border)' };
      case 'Outline':
        return { ...base, backgroundColor: 'transparent', color: 'var(--foreground)', border: '1px solid var(--border)' };
      case 'Ghost':
        return { ...base, backgroundColor: 'transparent', color: 'var(--foreground)' };
      case 'Destructive':
        return { ...base, backgroundColor: 'var(--destructive)', color: 'var(--primary-foreground)' };
      case 'Link':
        return { ...base, backgroundColor: 'transparent', color: 'var(--primary)', textDecoration: 'underline' };
    }
  };

  return (
    <StageShell
      title="Button::new(...)"
      themeName={activeTheme.name}
      onReset={() => {
        setVariant('Primary');
        setSize('Md');
        setLoading(false);
        setDisabled(false);
        setWithIcon(true);
        setLabel('Deploy Crate');
        setClicks(0);
      }}
      controls={
        <>
          <ControlField label="Variant">
            <SegmentedButtons
              options={['Primary', 'Secondary', 'Outline', 'Ghost', 'Destructive', 'Link'] as const}
              value={variant}
              onChange={setVariant}
            />
          </ControlField>
          <ControlField label="Size">
            <SegmentedButtons options={['Xs', 'Sm', 'Md', 'Lg'] as const} value={size} onChange={setSize} />
          </ControlField>
          <ControlField label="Label">
            <input
              type="text"
              value={label}
              onChange={(e) => setLabel(e.target.value)}
              className="px-2.5 py-1.5 rounded border text-sm"
              style={{ backgroundColor: 'var(--background)', color: 'var(--foreground)', borderColor: 'var(--border)' }}
            />
          </ControlField>
          <div className="flex flex-col gap-2 pt-1">
            <ToggleCheckbox label="Leading Icon" checked={withIcon} onChange={setWithIcon} />
            <ToggleCheckbox label="Loading Spinner" checked={loading} onChange={setLoading} />
            <ToggleCheckbox label="Disabled" checked={disabled} onChange={setDisabled} />
          </div>
        </>
      }
    >
      <div className="flex flex-col items-center gap-6">
        <button
          type="button"
          disabled={disabled || loading}
          onClick={() => setClicks((c) => c + 1)}
          className="inline-flex items-center justify-center gap-2 font-medium transition-[transform,filter,box-shadow,background-color] duration-150 cursor-pointer hover:brightness-95 active:translate-y-[1px] active:scale-[0.98] disabled:cursor-not-allowed whitespace-nowrap shrink-0 focus:outline-none focus-visible:ring-2 focus-visible:ring-[var(--ring)] focus-visible:ring-offset-2"
          style={{
            ...getStyle(),
            boxShadow:
              variant === 'Primary' || variant === 'Destructive'
                ? '0 1px 2px rgba(0,0,0,0.12), inset 0 1px 0 rgba(255,255,255,0.14)'
                : variant === 'Secondary' || variant === 'Outline'
                  ? '0 1px 2px rgba(0,0,0,0.05)'
                  : 'none',
          }}
        >
          {loading ? (
            <span
              className="inline-block rounded-full border-2 border-current border-t-transparent animate-spin"
              style={{ width: m.icon, height: m.icon }}
              aria-hidden="true"
            />
          ) : withIcon ? (
            <Sparkles key={clicks} className="animate-egui-pop" style={{ width: m.icon, height: m.icon }} aria-hidden="true" />
          ) : null}
          <span>{label || 'Button'}</span>
        </button>

        <div className="text-xs font-mono tabular-nums" style={{ color: 'var(--muted-foreground)' }}>
          response.clicked() count:{' '}
          <span key={clicks} className="inline-block animate-egui-pop font-semibold" style={{ color: 'var(--foreground)' }}>
            {clicks}
          </span>
        </div>
      </div>
    </StageShell>
  );
}

function DropdownButtonPreview({ activeTheme }: { activeTheme: ThemePreset }) {
  const [open, setOpen] = useState(false);
  const [selected, setSelected] = useState<string>('None yet');
  const [size, setSize] = useState<SizeOpt>('Md');
  const [disabled, setDisabled] = useState(false);
  const containerRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const onKey = (e: KeyboardEvent) => {
      if (e.key === 'Escape') setOpen(false);
    };
    const onClickOutside = (e: MouseEvent) => {
      if (containerRef.current && !containerRef.current.contains(e.target as Node)) {
        setOpen(false);
      }
    };
    window.addEventListener('keydown', onKey);
    window.addEventListener('mousedown', onClickOutside);
    return () => {
      window.removeEventListener('keydown', onKey);
      window.removeEventListener('mousedown', onClickOutside);
    };
  }, []);

  const items = [
    { id: 'new', label: 'New Module', shortcut: 'Cmd+N', Icon: Plus, destructive: false, disabled: false },
    { id: 'duplicate', label: 'Duplicate Widget', shortcut: 'Cmd+D', Icon: Copy, destructive: false, disabled: false },
    { id: 'archive', label: 'Archive (Locked)', shortcut: '', Icon: Settings, destructive: false, disabled: true },
    { id: 'delete', label: 'Delete Item', shortcut: 'Backspace', Icon: X, destructive: true, disabled: false },
  ];

  const m = SIZE_METRICS[size];

  return (
    <StageShell
      title="DropdownButton::new(...)"
      themeName={activeTheme.name}
      onReset={() => {
        setOpen(false);
        setSelected('None yet');
        setSize('Md');
        setDisabled(false);
      }}
      controls={
        <>
          <ControlField label="Size">
            <SegmentedButtons options={['Xs', 'Sm', 'Md', 'Lg'] as const} value={size} onChange={setSize} />
          </ControlField>
          <ToggleCheckbox label="Disabled" checked={disabled} onChange={setDisabled} />
          <div className="p-3 rounded border text-xs font-mono" style={{ backgroundColor: 'var(--muted)', borderColor: 'var(--border)' }}>
            selected_id: {selected}
          </div>
        </>
      }
    >
      <div ref={containerRef} className="relative inline-flex flex-col items-start gap-4 mb-24">
        <button
          type="button"
          disabled={disabled}
          aria-expanded={open}
          onClick={() => setOpen((o) => !o)}
          className="inline-flex items-center gap-2 font-medium border cursor-pointer transition-[background-color,border-color,box-shadow,transform] duration-150 active:scale-[0.98] disabled:opacity-50 disabled:cursor-not-allowed whitespace-nowrap shrink-0 focus:outline-none focus-visible:ring-2 focus-visible:ring-[var(--ring)]"
          style={{
            height: m.h,
            paddingLeft: m.px,
            paddingRight: m.px,
            fontSize: m.text,
            borderRadius: m.radius,
            backgroundColor: open ? 'var(--accent)' : 'var(--secondary)',
            color: 'var(--secondary-foreground)',
            borderColor: open ? 'var(--ring)' : 'var(--border)',
          }}
        >
          <span>Actions</span>
          <ChevronDown
            className={`w-4 h-4 transition-transform duration-150 ${open ? 'rotate-180' : ''}`}
            aria-hidden="true"
          />
        </button>

        {open && !disabled && (
          <div
            role="menu"
            className="animate-egui-menu-in origin-top-left absolute left-0 top-full mt-1.5 w-60 rounded-lg border p-1.5 z-20 space-y-0.5 shadow-xl"
            style={{
              backgroundColor: 'var(--card)',
              borderColor: 'var(--border)',
            }}
          >
            {items.map((item) => {
              const ItemIcon = item.Icon;
              return (
                <button
                  key={item.id}
                  type="button"
                  role="menuitem"
                  disabled={item.disabled}
                  onClick={() => {
                    setSelected(item.id);
                    setOpen(false);
                  }}
                  className="w-full flex items-center justify-between gap-2 px-2.5 py-1.5 rounded-md text-xs font-medium text-left cursor-pointer disabled:opacity-45 disabled:cursor-not-allowed hover:bg-[var(--accent)] transition-colors"
                  style={{
                    color: item.destructive ? 'var(--destructive)' : 'var(--card-foreground)',
                  }}
                >
                  <span className="flex items-center gap-2 min-w-0">
                    <ItemIcon className="w-3.5 h-3.5 shrink-0 opacity-75" aria-hidden="true" />
                    <span className="truncate">{item.label}</span>
                  </span>
                  {item.shortcut && <KbdChord shortcut={item.shortcut} macos size="Xs" />}
                </button>
              );
            })}
          </div>
        )}
      </div>
    </StageShell>
  );
}

const ICON_SET = [
  { name: 'Check', Icon: Check },
  { name: 'Close', Icon: X },
  { name: 'ChevronDown', Icon: ChevronDown },
  { name: 'ChevronRight', Icon: ChevronRight },
  { name: 'Plus', Icon: Plus },
  { name: 'Minus', Icon: Minus },
  { name: 'Search', Icon: Search },
  { name: 'Settings', Icon: Settings },
  { name: 'Copy', Icon: Copy },
  { name: 'Sparkles', Icon: Sparkles },
  { name: 'Sun', Icon: Sun },
  { name: 'Moon', Icon: Moon },
  { name: 'AlertTriangle', Icon: AlertTriangle },
  { name: 'Info', Icon: Info },
  { name: 'User', Icon: User },
  { name: 'Terminal', Icon: Terminal },
];

function IconPreview({ activeTheme }: { activeTheme: ThemePreset }) {
  const [selectedIcon, setSelectedIcon] = useState('Sparkles');
  const [size, setSize] = useState<SizeOpt>('Lg');
  const [tone, setTone] = useState<'Foreground' | 'Primary' | 'Success' | 'Warning' | 'Destructive'>('Foreground');

  const colorMap: Record<typeof tone, string> = {
    Foreground: 'var(--foreground)',
    Primary: 'var(--primary)',
    Success: 'var(--success)',
    Warning: 'var(--warning)',
    Destructive: 'var(--destructive)',
  };

  const ActiveGlyph = (ICON_SET.find((i) => i.name === selectedIcon) ?? ICON_SET[0]).Icon;

  return (
    <StageShell
      title={`Icon::new(IconName::${selectedIcon})`}
      themeName={activeTheme.name}
      onReset={() => {
        setSelectedIcon('Sparkles');
        setSize('Lg');
        setTone('Foreground');
      }}
      controls={
        <>
          <ControlField label="Size">
            <SegmentedButtons options={['Xs', 'Sm', 'Md', 'Lg'] as const} value={size} onChange={setSize} />
          </ControlField>
          <ControlField label="Semantic Color">
            <SegmentedButtons
              options={['Foreground', 'Primary', 'Success', 'Warning', 'Destructive'] as const}
              value={tone}
              onChange={setTone}
            />
          </ControlField>
          <div className="p-3 rounded border text-xs font-mono" style={{ backgroundColor: 'var(--muted)', borderColor: 'var(--border)' }}>
            IconName::{selectedIcon} ({SIZE_METRICS[size].icon}px)
          </div>
        </>
      }
    >
      <div className="w-full max-w-md flex flex-col gap-4">
        <div
          className="rounded-lg border px-4 py-3 flex items-center justify-between gap-4"
          style={{
            backgroundColor: 'var(--card)',
            borderColor: 'var(--border)',
          }}
        >
          <div className="flex items-center gap-3">
            <div
              key={`${selectedIcon}-${tone}`}
              className="animate-egui-pop w-11 h-11 rounded-lg border flex items-center justify-center shrink-0"
              style={{
                backgroundColor: `color-mix(in srgb, ${colorMap[tone]} 12%, var(--muted))`,
                borderColor: `color-mix(in srgb, ${colorMap[tone]} 32%, var(--border))`,
              }}
            >
              <ActiveGlyph
                style={{
                  width: SIZE_METRICS[size].icon + 6,
                  height: SIZE_METRICS[size].icon + 6,
                  color: colorMap[tone],
                }}
                aria-hidden="true"
              />
            </div>
            <div className="text-xs">
              <div className="font-mono font-semibold" style={{ color: 'var(--foreground)' }}>
                IconName::{selectedIcon}
              </div>
              <div className="mt-0.5" style={{ color: 'var(--muted-foreground)' }}>
                {SIZE_METRICS[size].icon}px · {tone} token
              </div>
            </div>
          </div>
          <span className="text-[11px] font-mono tabular-nums" style={{ color: 'var(--muted-foreground)' }}>
            2px stroke
          </span>
        </div>

        <div className="grid grid-cols-4 gap-2.5 w-full">
          {ICON_SET.map(({ name, Icon: Glyph }) => {
            const active = name === selectedIcon;
            return (
              <button
                key={name}
                type="button"
                onClick={() => setSelectedIcon(name)}
                className="group flex flex-col items-center justify-center gap-2 p-3 rounded-md border cursor-pointer transition-[background-color,border-color,transform] duration-150 hover:-translate-y-0.5 active:scale-[0.97]"
                style={{
                  backgroundColor: active ? 'var(--accent)' : 'var(--card)',
                  borderColor: active ? 'var(--primary)' : 'var(--border)',
                }}
              >
                <Glyph
                  className="transition-transform duration-150 group-hover:scale-110"
                  style={{
                    width: SIZE_METRICS[size].icon,
                    height: SIZE_METRICS[size].icon,
                    color: colorMap[tone],
                  }}
                  aria-hidden="true"
                />
                <span className="text-[11px] font-mono truncate max-w-full" style={{ color: active ? 'var(--foreground)' : 'var(--muted-foreground)' }}>
                  {name}
                </span>
              </button>
            );
          })}
        </div>
      </div>
    </StageShell>
  );
}

function BadgePreview({ activeTheme }: { activeTheme: ThemePreset }) {
  const [count, setCount] = useState(8);
  const [maxCount, setMaxCount] = useState(99);
  const [variant, setVariant] = useState<'Primary' | 'Secondary' | 'Success' | 'Warning' | 'Destructive'>('Destructive');
  const [mode, setMode] = useState<'Count' | 'Dot' | 'Text'>('Count');

  const bgMap: Record<typeof variant, { bg: string; fg: string }> = {
    Primary: { bg: 'var(--primary)', fg: 'var(--primary-foreground)' },
    Secondary: { bg: 'var(--secondary)', fg: 'var(--secondary-foreground)' },
    Success: { bg: 'var(--success)', fg: 'var(--primary-foreground)' },
    Warning: { bg: 'var(--warning)', fg: 'var(--primary-foreground)' },
    Destructive: { bg: 'var(--destructive)', fg: 'var(--primary-foreground)' },
  };

  const formattedCount = count > maxCount ? `${maxCount}+` : String(count);

  return (
    <StageShell
      title="Badge::count(...).attach_to(ui, &target)"
      themeName={activeTheme.name}
      onReset={() => {
        setCount(8);
        setMaxCount(99);
        setVariant('Destructive');
        setMode('Count');
      }}
      controls={
        <>
          <ControlField label="Mode">
            <SegmentedButtons options={['Count', 'Dot', 'Text'] as const} value={mode} onChange={setMode} />
          </ControlField>
          <ControlField label="Variant">
            <SegmentedButtons
              options={['Primary', 'Secondary', 'Success', 'Warning', 'Destructive'] as const}
              value={variant}
              onChange={setVariant}
            />
          </ControlField>
          <ControlField label="Count Value">
            <div className="flex items-center gap-2">
              <button
                type="button"
                onClick={() => setCount((c) => Math.max(0, c - 1))}
                className="px-2.5 py-1 rounded border text-xs font-mono cursor-pointer"
                style={{ borderColor: 'var(--border)', backgroundColor: 'var(--muted)' }}
              >
                -1
              </button>
              <span className="font-mono text-sm tabular-nums px-2">{count}</span>
              <button
                type="button"
                onClick={() => setCount((c) => c + 15)}
                className="px-2.5 py-1 rounded border text-xs font-mono cursor-pointer"
                style={{ borderColor: 'var(--border)', backgroundColor: 'var(--muted)' }}
              >
                +15
              </button>
              <button
                type="button"
                onClick={() => setMaxCount((m) => (m === 99 ? 9 : 99))}
                className="ml-auto px-2 py-1 rounded border text-xs font-mono cursor-pointer"
                style={{ borderColor: 'var(--border)' }}
              >
                max: {maxCount}
              </button>
            </div>
          </ControlField>
        </>
      }
    >
      <div className="flex flex-wrap items-center gap-10">
        {/* Anchored to Button */}
        <div className="relative inline-flex">
          <div
            className="px-4 py-2 rounded-md border text-sm font-medium shadow-xs"
            style={{
              backgroundColor: 'var(--secondary)',
              color: 'var(--secondary-foreground)',
              borderColor: 'var(--border)',
            }}
          >
            Inbox
          </div>
          {mode === 'Dot' ? (
            <span className="absolute -top-1 -right-1 flex h-3 w-3 items-center justify-center">
              <span
                className="animate-ping absolute inline-flex h-full w-full rounded-full opacity-60"
                style={{ backgroundColor: bgMap[variant].bg }}
              />
              <span
                className="relative w-3 h-3 rounded-full border-2"
                style={{ backgroundColor: bgMap[variant].bg, borderColor: 'var(--background)' }}
              />
            </span>
          ) : (
            <span
              key={`${formattedCount}-${mode}-${variant}`}
              className="animate-egui-pop absolute -top-2 -right-2 px-1.5 min-w-5 h-5 rounded-full inline-flex items-center justify-center text-[11px] font-mono tabular-nums font-semibold border-2 shadow-xs"
              style={{
                backgroundColor: bgMap[variant].bg,
                color: bgMap[variant].fg,
                borderColor: 'var(--background)',
              }}
            >
              {mode === 'Text' ? 'NEW' : formattedCount}
            </span>
          )}
        </div>

        {/* Standalone */}
        <div className="flex items-center gap-3 text-xs" style={{ color: 'var(--muted-foreground)' }}>
          <span>Standalone:</span>
          {mode === 'Dot' ? (
            <span className="w-2.5 h-2.5 rounded-full inline-block" style={{ backgroundColor: bgMap[variant].bg }} />
          ) : (
            <span
              key={`st-${formattedCount}-${mode}-${variant}`}
              className="animate-egui-pop px-2 h-5 rounded-full inline-flex items-center justify-center text-[11px] font-mono tabular-nums font-semibold"
              style={{ backgroundColor: bgMap[variant].bg, color: bgMap[variant].fg }}
            >
              {mode === 'Text' ? 'BETA' : formattedCount}
            </span>
          )}
        </div>
      </div>
    </StageShell>
  );
}

function TagPreview({ activeTheme }: { activeTheme: ThemePreset }) {
  const [tags, setTags] = useState<Array<{ id: string; label: string; selected: boolean }>>([
    { id: '1', label: 'egui', selected: true },
    { id: '2', label: 'gpui-kit', selected: false },
    { id: '3', label: 'immediate-mode', selected: false },
    { id: '4', label: 'rust', selected: false },
  ]);
  const [newTag, setNewTag] = useState('');
  const [variant, setVariant] = useState<'Neutral' | 'Primary' | 'Outline' | 'Success' | 'Warning' | 'Destructive'>('Neutral');
  const [closable, setClosable] = useState(true);

  const addTag = (e: React.FormEvent) => {
    e.preventDefault();
    const trimmed = newTag.trim();
    if (!trimmed) return;
    setTags((prev) => [...prev, { id: String(Date.now()), label: trimmed, selected: false }]);
    setNewTag('');
  };

  const getTagStyle = (selected: boolean): React.CSSProperties => {
    if (selected) {
      return { backgroundColor: 'var(--primary)', color: 'var(--primary-foreground)', borderColor: 'var(--primary)' };
    }
    switch (variant) {
      case 'Primary':
        return { backgroundColor: 'var(--accent)', color: 'var(--primary)', borderColor: 'var(--primary)' };
      case 'Outline':
        return { backgroundColor: 'transparent', color: 'var(--foreground)', borderColor: 'var(--border)' };
      case 'Success':
        return { backgroundColor: 'var(--muted)', color: 'var(--success)', borderColor: 'var(--success)' };
      case 'Warning':
        return { backgroundColor: 'var(--muted)', color: 'var(--warning)', borderColor: 'var(--warning)' };
      case 'Destructive':
        return { backgroundColor: 'var(--muted)', color: 'var(--destructive)', borderColor: 'var(--destructive)' };
      default:
        return { backgroundColor: 'var(--muted)', color: 'var(--foreground)', borderColor: 'var(--border)' };
    }
  };

  return (
    <StageShell
      title="Tag::new(...).closable(true)"
      themeName={activeTheme.name}
      onReset={() =>
        setTags([
          { id: '1', label: 'egui', selected: true },
          { id: '2', label: 'gpui-kit', selected: false },
          { id: '3', label: 'immediate-mode', selected: false },
          { id: '4', label: 'rust', selected: false },
        ])
      }
      controls={
        <>
          <ControlField label="Variant">
            <SegmentedButtons
              options={['Neutral', 'Primary', 'Outline', 'Success', 'Warning', 'Destructive'] as const}
              value={variant}
              onChange={setVariant}
            />
          </ControlField>
          <ToggleCheckbox label="Closable (×)" checked={closable} onChange={setClosable} />
          <form onSubmit={addTag} className="flex gap-2">
            <input
              type="text"
              value={newTag}
              onChange={(e) => setNewTag(e.target.value)}
              placeholder="Add tag..."
              className="flex-1 min-w-0 px-2.5 py-1.5 rounded border text-xs"
              style={{ backgroundColor: 'var(--background)', color: 'var(--foreground)', borderColor: 'var(--border)' }}
            />
            <button
              type="submit"
              className="px-3 py-1.5 rounded text-xs font-medium cursor-pointer shrink-0"
              style={{ backgroundColor: 'var(--primary)', color: 'var(--primary-foreground)' }}
            >
              Add
            </button>
          </form>
        </>
      }
    >
      <div className="flex flex-wrap gap-2 justify-center max-w-md">
        {tags.length === 0 ? (
          <span className="text-xs" style={{ color: 'var(--muted-foreground)' }}>
            All tags dismissed. Click Reset or add a new tag.
          </span>
        ) : (
          tags.map((tag) => (
            <div
              key={tag.id}
              className="animate-egui-pop inline-flex items-center gap-1.5 px-2.5 py-1 rounded-md border text-xs font-medium transition-[background-color,border-color,color,transform] duration-150 hover:-translate-y-0.5"
              style={getTagStyle(tag.selected)}
            >
              <button
                type="button"
                onClick={() =>
                  setTags((prev) =>
                    prev.map((item) => (item.id === tag.id ? { ...item, selected: !item.selected } : item))
                  )
                }
                className="inline-flex items-center gap-1 cursor-pointer"
              >
                {tag.selected && <Check className="w-3 h-3 animate-egui-check-draw shrink-0" aria-hidden="true" />}
                <span>{tag.label}</span>
              </button>
              {closable && (
                <button
                  type="button"
                  aria-label={`Remove ${tag.label}`}
                  onClick={() => setTags((prev) => prev.filter((item) => item.id !== tag.id))}
                  className="cursor-pointer rounded p-0.5 opacity-75 hover:opacity-100 hover:bg-black/10 transition-opacity"
                >
                  <X className="w-3 h-3" aria-hidden="true" />
                </button>
              )}
            </div>
          ))
        )}
      </div>
    </StageShell>
  );
}

function LabelPreview({ activeTheme }: { activeTheme: ThemePreset }) {
  const [text, setText] = useState('Crate Workspace Directory');
  const [tone, setTone] = useState<'Default' | 'Muted' | 'Primary' | 'Success' | 'Warning' | 'Destructive'>('Default');
  const [size, setSize] = useState<SizeOpt>('Md');
  const [required, setRequired] = useState(true);
  const [strong, setStrong] = useState(true);
  const [truncate, setTruncate] = useState(false);

  const toneColor: Record<typeof tone, string> = {
    Default: 'var(--foreground)',
    Muted: 'var(--muted-foreground)',
    Primary: 'var(--primary)',
    Success: 'var(--success)',
    Warning: 'var(--warning)',
    Destructive: 'var(--destructive)',
  };

  return (
    <StageShell
      title="Label::new(...)"
      themeName={activeTheme.name}
      onReset={() => {
        setText('Crate Workspace Directory');
        setTone('Default');
        setSize('Md');
        setRequired(true);
        setStrong(true);
        setTruncate(false);
      }}
      controls={
        <>
          <ControlField label="Tone">
            <SegmentedButtons
              options={['Default', 'Muted', 'Primary', 'Success', 'Warning', 'Destructive'] as const}
              value={tone}
              onChange={setTone}
            />
          </ControlField>
          <ControlField label="Size">
            <SegmentedButtons options={['Xs', 'Sm', 'Md', 'Lg'] as const} value={size} onChange={setSize} />
          </ControlField>
          <ControlField label="Text">
            <input
              type="text"
              value={text}
              onChange={(e) => setText(e.target.value)}
              className="px-2.5 py-1.5 rounded border text-xs"
              style={{ backgroundColor: 'var(--background)', color: 'var(--foreground)', borderColor: 'var(--border)' }}
            />
          </ControlField>
          <div className="flex flex-col gap-2 pt-1">
            <ToggleCheckbox label="Required (*)" checked={required} onChange={setRequired} />
            <ToggleCheckbox label="Strong weight" checked={strong} onChange={setStrong} />
            <ToggleCheckbox label="Truncate at 140px" checked={truncate} onChange={setTruncate} />
          </div>
        </>
      }
    >
      <div
        className="p-4 rounded-md border flex items-center gap-1"
        style={{
          backgroundColor: 'var(--card)',
          borderColor: 'var(--border)',
          maxWidth: truncate ? '160px' : '100%',
        }}
      >
        <span
          className={truncate ? 'truncate' : ''}
          style={{
            fontSize: SIZE_METRICS[size].text,
            fontWeight: strong ? 600 : 400,
            color: toneColor[tone],
          }}
        >
          {text || 'Label'}
        </span>
        {required && (
          <span className="shrink-0" style={{ color: 'var(--destructive)', fontSize: SIZE_METRICS[size].text }}>
            *
          </span>
        )}
      </div>
    </StageShell>
  );
}

function KbdGlyphSvg({ kind, size }: { kind: KbdIconKind; size: number }) {
  const common = {
    width: size,
    height: size,
    viewBox: '0 0 16 16',
    fill: 'none',
    stroke: 'currentColor',
    strokeWidth: 1.55,
    strokeLinecap: 'round' as const,
    strokeLinejoin: 'round' as const,
    'aria-hidden': true,
    className: 'shrink-0',
  };

  switch (kind) {
    case 'command':
      return (
        <svg {...common}>
          <path d="M6 6V4.25A1.75 1.75 0 1 0 4.25 6H6Zm4 0V4.25A1.75 1.75 0 1 1 11.75 6H10Zm-4 4v1.75A1.75 1.75 0 1 1 4.25 10H6Zm4 0v1.75a1.75 1.75 0 1 0 1.75-1.75H10ZM6 6h4v4H6V6Z" />
        </svg>
      );
    case 'shift':
      return (
        <svg {...common}>
          <path d="M8 2.5 2.75 8h3v5h4.5V8h3L8 2.5Z" />
        </svg>
      );
    case 'option':
      return (
        <svg {...common}>
          <path d="M2.5 4.25h3l4.2 7.5h3.8" />
          <path d="M9.5 4.25h4" />
        </svg>
      );
    case 'control':
      return (
        <svg {...common}>
          <path d="m3.5 10 4.5-4.75L12.5 10" />
        </svg>
      );
    case 'enter':
      return (
        <svg {...common}>
          <path d="M13 3.75v4.5a1 1 0 0 1-1 1H3.75" />
          <path d="M6.5 6.5 3.5 9.25 6.5 12" />
        </svg>
      );
    case 'escape':
      return (
        <svg {...common}>
          <path d="M10.5 3.4A5.25 5.25 0 1 0 12.9 9.5" />
          <path d="M8.5 7.5 13 3" />
          <path d="M9.75 3H13v3.25" />
        </svg>
      );
    case 'tab':
      return (
        <svg {...common}>
          <path d="M2.5 8h8.25" />
          <path d="M7.75 5 10.75 8l-3 3" />
          <path d="M13.25 3.75v8.5" />
        </svg>
      );
    case 'backspace':
      return (
        <svg {...common}>
          <path d="M6.25 3.75h6.5a1 1 0 0 1 1 1v6.5a1 1 0 0 1-1 1h-6.5L2.25 8l4-4.25Z" />
          <path d="m8.25 6.25 3.25 3.5" />
          <path d="m11.5 6.25-3.25 3.5" />
        </svg>
      );
    case 'space':
      return (
        <svg {...common}>
          <path d="M2.75 8.5v2.25h10.5V8.5" />
        </svg>
      );
    case 'arrowUp':
      return (
        <svg {...common}>
          <path d="M8 13V3.25" />
          <path d="M4.25 6.75 8 3l3.75 3.75" />
        </svg>
      );
    case 'arrowDown':
      return (
        <svg {...common}>
          <path d="M8 3v9.75" />
          <path d="M4.25 9.25 8 13l3.75-3.75" />
        </svg>
      );
    case 'arrowLeft':
      return (
        <svg {...common}>
          <path d="M13 8H3.25" />
          <path d="M6.75 4.25 3 8l3.75 3.75" />
        </svg>
      );
    case 'arrowRight':
      return (
        <svg {...common}>
          <path d="M3 8h9.75" />
          <path d="M9.25 4.25 13 8l-3.75 3.75" />
        </svg>
      );
  }
}

export function KbdChord({
  shortcut,
  macos,
  size,
}: {
  shortcut: string;
  macos: boolean;
  size: SizeOpt;
}) {
  const tokens = parseKbdTokens(shortcut, macos);
  const height = size === 'Xs' ? 20 : size === 'Sm' ? 23 : size === 'Md' ? 26 : 32;
  const iconSize = size === 'Xs' ? 11 : size === 'Sm' ? 12 : size === 'Md' ? 13.5 : 16;
  const fontSize = size === 'Xs' ? '10.5px' : size === 'Sm' ? '11.5px' : size === 'Md' ? '12.5px' : '14px';

  return (
    <span className="inline-flex items-center gap-1.5">
      {tokens.map((tok, idx) => (
        <kbd
          key={`${tok.raw}-${idx}`}
          title={tok.ariaLabel}
          aria-label={tok.ariaLabel}
          className="inline-flex items-center justify-center gap-1 rounded-[5px] border font-mono font-medium select-none"
          style={{
            minWidth: `${height}px`,
            height: `${height}px`,
            paddingLeft: tok.label ? '7px' : '5px',
            paddingRight: tok.label ? '7px' : '5px',
            fontSize,
            backgroundColor: 'var(--muted)',
            color: 'var(--foreground)',
            borderColor: 'var(--border)',
            boxShadow: '0 1.5px 0 0 var(--border), inset 0 1px 0 0 color-mix(in srgb, var(--background) 65%, transparent)',
          }}
        >
          {tok.icon && <KbdGlyphSvg kind={tok.icon} size={iconSize} />}
          {tok.label && <span>{tok.label}</span>}
        </kbd>
      ))}
    </span>
  );
}

function KbdPreview({ activeTheme }: { activeTheme: ThemePreset }) {
  const [shortcut, setShortcut] = useState('Cmd+Shift+P');
  const [macos, setMacos] = useState(true);
  const [size, setSize] = useState<SizeOpt>('Md');

  return (
    <StageShell
      title="Kbd::new(...)"
      themeName={activeTheme.name}
      onReset={() => {
        setShortcut('Cmd+Shift+P');
        setMacos(true);
        setSize('Md');
      }}
      controls={
        <>
          <ControlField label="Shortcut Chord">
            <input
              type="text"
              value={shortcut}
              onChange={(e) => setShortcut(e.target.value)}
              className="px-2.5 py-1.5 rounded border text-xs font-mono"
              style={{ backgroundColor: 'var(--background)', color: 'var(--foreground)', borderColor: 'var(--border)' }}
            />
          </ControlField>
          <ControlField label="Presets">
            <div className="flex flex-wrap gap-1.5">
              {['Cmd+K', 'Cmd+Shift+P', 'Ctrl+Alt+Enter', 'Alt+Tab', 'Cmd+Backspace', 'Esc'].map((p) => (
                <button
                  key={p}
                  type="button"
                  onClick={() => setShortcut(p)}
                  className="px-2 py-1 rounded border text-xs font-mono cursor-pointer transition-colors"
                  style={{
                    backgroundColor: shortcut === p ? 'var(--primary)' : 'var(--muted)',
                    color: shortcut === p ? 'var(--primary-foreground)' : 'var(--foreground)',
                    borderColor: shortcut === p ? 'var(--primary)' : 'var(--border)',
                  }}
                >
                  {p}
                </button>
              ))}
            </div>
          </ControlField>
          <ControlField label="Size">
            <SegmentedButtons options={['Xs', 'Sm', 'Md', 'Lg'] as const} value={size} onChange={setSize} />
          </ControlField>
          <ToggleCheckbox label="macOS vector glyphs (compact)" checked={macos} onChange={setMacos} />
        </>
      }
    >
      <div className="w-full max-w-sm flex flex-col items-center gap-6">
        <div className="py-2">
          <KbdChord shortcut={shortcut} macos={macos} size={size} />
        </div>

        <div
          className="w-full rounded-lg border divide-y text-xs"
          style={{ backgroundColor: 'var(--card)', borderColor: 'var(--border)' }}
        >
          {[
            { label: 'Command Palette', chord: 'Cmd+Shift+P' },
            { label: 'Cycle Windows', chord: 'Alt+Tab' },
            { label: 'Confirm & Submit', chord: 'Cmd+Enter' },
            { label: 'Dismiss Overlay', chord: 'Esc' },
          ].map((row) => (
            <div
              key={row.label}
              className="px-3.5 py-2 flex items-center justify-between gap-4"
              style={{ borderColor: 'var(--border)' }}
            >
              <span style={{ color: 'var(--card-foreground)' }}>{row.label}</span>
              <KbdChord shortcut={row.chord} macos={macos} size="Sm" />
            </div>
          ))}
        </div>
      </div>
    </StageShell>
  );
}

function AvatarPreview({ activeTheme }: { activeTheme: ThemePreset }) {
  const [name, setName] = useState('Ada Lovelace');
  const [size, setSize] = useState<SizeOpt>('Lg');
  const [shape, setShape] = useState<'Circle' | 'RoundedSquare'>('Circle');
  const [status, setStatus] = useState<'Online' | 'Away' | 'Busy' | 'Offline' | 'None'>('Online');
  const [maxVisible, setMaxVisible] = useState(3);

  const initials =
    name
      .trim()
      .split(/\s+/)
      .filter(Boolean)
      .slice(0, 2)
      .map((s) => s[0].toUpperCase())
      .join('') || 'EK';

  const diameter = size === 'Xs' ? 24 : size === 'Sm' ? 32 : size === 'Md' ? 40 : 52;
  const statusColor =
    status === 'Online'
      ? 'var(--success)'
      : status === 'Away'
      ? 'var(--warning)'
      : status === 'Busy'
      ? 'var(--destructive)'
      : 'var(--muted-foreground)';

  const groupMembers = ['Ada Lovelace', 'Graydon Hoare', 'Niko Matsakis', 'Raph Levien', 'Emil Ernerfeldt'];

  return (
    <StageShell
      title="Avatar::new(...) & AvatarGroup"
      themeName={activeTheme.name}
      onReset={() => {
        setName('Ada Lovelace');
        setSize('Lg');
        setShape('Circle');
        setStatus('Online');
        setMaxVisible(3);
      }}
      controls={
        <>
          <ControlField label="Display Name">
            <input
              type="text"
              value={name}
              onChange={(e) => setName(e.target.value)}
              className="px-2.5 py-1.5 rounded border text-xs"
              style={{ backgroundColor: 'var(--background)', color: 'var(--foreground)', borderColor: 'var(--border)' }}
            />
          </ControlField>
          <ControlField label="Shape">
            <SegmentedButtons options={['Circle', 'RoundedSquare'] as const} value={shape} onChange={setShape} />
          </ControlField>
          <ControlField label="Size">
            <SegmentedButtons options={['Xs', 'Sm', 'Md', 'Lg'] as const} value={size} onChange={setSize} />
          </ControlField>
          <ControlField label="Presence Status">
            <SegmentedButtons options={['Online', 'Away', 'Busy', 'Offline', 'None'] as const} value={status} onChange={setStatus} />
          </ControlField>
          <ControlField label={`AvatarGroup max_visible: ${maxVisible}`}>
            <input
              type="range"
              min={2}
              max={5}
              value={maxVisible}
              onChange={(e) => setMaxVisible(Number(e.target.value))}
              className="accent-[var(--primary)]"
            />
          </ControlField>
        </>
      }
    >
      <div className="flex flex-col items-center gap-8">
        <div className="flex items-center gap-4">
          <div className="relative inline-flex">
            <div
              key={shape}
              className="animate-egui-pop flex items-center justify-center font-semibold border select-none shadow-xs transition-all duration-200"
              style={{
                width: diameter,
                height: diameter,
                borderRadius: shape === 'Circle' ? '9999px' : '10px',
                fontSize: Math.round(diameter * 0.36),
                backgroundColor: 'var(--primary)',
                color: 'var(--primary-foreground)',
                borderColor: 'var(--border)',
                boxShadow: 'inset 0 1px 0 rgba(255,255,255,0.2)',
              }}
            >
              {initials}
            </div>
            {status !== 'None' && (
              <span
                className="absolute bottom-0 right-0 flex items-center justify-center"
                style={{
                  width: Math.max(10, Math.round(diameter * 0.28)),
                  height: Math.max(10, Math.round(diameter * 0.28)),
                }}
              >
                {status === 'Online' && (
                  <span
                    className="animate-egui-mic-ring absolute inset-0 rounded-full border pointer-events-none"
                    style={{ borderColor: statusColor }}
                  />
                )}
                <span
                  className="relative w-full h-full rounded-full border-2"
                  style={{
                    backgroundColor: statusColor,
                    borderColor: 'var(--background)',
                  }}
                />
              </span>
            )}
          </div>
          <div className="text-xs">
            <div className="font-medium" style={{ color: 'var(--foreground)' }}>
              {name || 'Unnamed'}
            </div>
            <div style={{ color: 'var(--muted-foreground)' }}>
              {status === 'None' ? 'No presence ring' : `Status: ${status}`}
            </div>
          </div>
        </div>

        <div className="flex flex-col items-center gap-2">
          <span className="text-xs" style={{ color: 'var(--muted-foreground)' }}>
            AvatarGroup ({groupMembers.length} total)
          </span>
          <div className="flex items-center -space-x-2">
            {groupMembers.slice(0, maxVisible).map((member) => {
              const ini = member
                .split(' ')
                .map((p) => p[0])
                .join('');
              return (
                <div
                  key={member}
                  title={member}
                  className="animate-egui-pop w-9 h-9 rounded-full flex items-center justify-center text-xs font-semibold border-2 transition-transform duration-150 hover:-translate-y-1 hover:z-10 cursor-default shadow-xs"
                  style={{
                    backgroundColor: 'var(--primary)',
                    color: 'var(--primary-foreground)',
                    borderColor: 'var(--background)',
                  }}
                >
                  {ini}
                </div>
              );
            })}
            {groupMembers.length > maxVisible && (
              <div
                className="w-9 h-9 rounded-full flex items-center justify-center text-xs font-mono font-semibold border-2 transition-transform duration-150 hover:-translate-y-1 hover:z-10"
                style={{
                  backgroundColor: 'var(--muted)',
                  color: 'var(--foreground)',
                  borderColor: 'var(--background)',
                }}
              >
                +{groupMembers.length - maxVisible}
              </div>
            )}
          </div>
        </div>
      </div>
    </StageShell>
  );
}
