import { useEffect, useRef, useState } from 'react';
import {
  AlertTriangle,
  Calendar as CalendarIcon,
  Check,
  CheckCircle2,
  ChevronLeft,
  ChevronRight,
  Clock,
  Copy,
  FileCode2,
  FolderOpen,
  Image as ImageIcon,
  Info,
  Mic,
  Minus,
  Plus,
  X,
} from 'lucide-react';
import type { ThemePreset } from '../types/kit';
import {
  formatByteSize,
  getCalendarYearOptions,
  getMonthGrid,
  hsvToRgbHex,
  stepTimeSegment,
  type SimpleTimeValue,
} from '../utils/catalogLogic';
import {
  ControlField,
  SIZE_METRICS,
  SegmentedButtons,
  StageShell,
  ToggleCheckbox,
  type SizeOpt,
} from './Part1Previews';

interface Part4PreviewProps {
  slug: string;
  activeTheme: ThemePreset;
}

export function Part4Preview({ slug, activeTheme }: Part4PreviewProps) {
  switch (slug) {
    case 'alert':
      return <AlertPreview activeTheme={activeTheme} />;
    case 'empty':
      return <EmptyPreview activeTheme={activeTheme} />;
    case 'attachment':
      return <AttachmentPreview activeTheme={activeTheme} />;
    case 'image':
      return <ImagePreview activeTheme={activeTheme} />;
    case 'calendar':
      return <CalendarPreview activeTheme={activeTheme} />;
    case 'date-picker':
      return <DatePickerPreview activeTheme={activeTheme} />;
    case 'time-field':
      return <TimeFieldPreview activeTheme={activeTheme} />;
    case 'color-picker':
      return <ColorPickerPreview activeTheme={activeTheme} />;
    case 'marker':
      return <MarkerPreview activeTheme={activeTheme} />;
    case 'speech':
      return <SpeechPreview activeTheme={activeTheme} />;
    default:
      return null;
  }
}

function AlertPreview({ activeTheme }: { activeTheme: ThemePreset }) {
  const [tone, setTone] = useState<'Info' | 'Success' | 'Warning' | 'Destructive'>('Info');
  const [dismissible, setDismissible] = useState(true);
  const [visible, setVisible] = useState(true);

  const toneVar =
    tone === 'Info'
      ? 'var(--primary)'
      : tone === 'Success'
        ? 'var(--success)'
        : tone === 'Warning'
          ? 'var(--warning)'
          : 'var(--destructive)';

  const copyMap: Record<typeof tone, { title: string; desc: string }> = {
    Info: {
      title: 'Part 4 widgets ready',
      desc: 'Alert::info() renders crisp vector icons paired with high-contrast theme accents.',
    },
    Success: {
      title: 'Cargo check succeeded',
      desc: 'All 40 crate components compiled cleanly with zero external icon dependencies.',
    },
    Warning: {
      title: 'Deprecated builder alias',
      desc: 'Prefer ui.kit_alert() or Alert::warning() with explicit semantic tone.',
    },
    Destructive: {
      title: 'Workspace sync interrupted',
      desc: 'Reconnect your repository remote or inspect logs before retrying.',
    },
  };

  const ToneIcon = tone === 'Info' ? Info : tone === 'Success' ? CheckCircle2 : AlertTriangle;

  return (
    <StageShell
      title={`Alert::${tone.toLowerCase()}(...)`}
      themeName={activeTheme.name}
      onReset={() => {
        setTone('Info');
        setDismissible(true);
        setVisible(true);
      }}
      controls={
        <>
          <ControlField label="Tone">
            <SegmentedButtons
              options={['Info', 'Success', 'Warning', 'Destructive'] as const}
              value={tone}
              onChange={(next) => {
                setTone(next);
                setVisible(true);
              }}
            />
          </ControlField>
          <ToggleCheckbox checked={dismissible} onChange={setDismissible} label="Dismissible" />
        </>
      }
    >
      <div className="w-full max-w-md flex flex-col items-center gap-3">
        {visible ? (
          <div
            key={tone}
            role="status"
            className="animate-egui-menu-in w-full rounded-xl border p-3.5 flex items-start gap-3 shadow-xs transition-colors duration-150"
            style={{
              backgroundColor: `color-mix(in srgb, ${toneVar} 8%, var(--card))`,
              borderColor: `color-mix(in srgb, ${toneVar} 38%, var(--border))`,
            }}
          >
            <div
              className="w-7 h-7 rounded-lg flex items-center justify-center shrink-0 mt-0.5"
              style={{
                backgroundColor: `color-mix(in srgb, ${toneVar} 16%, var(--background))`,
                color: toneVar,
              }}
              aria-hidden="true"
            >
              <ToneIcon className="w-4 h-4" />
            </div>
            <div className="flex-1 min-w-0 space-y-1">
              <div className="text-sm font-semibold" style={{ color: 'var(--foreground)' }}>
                {copyMap[tone].title}
              </div>
              <p className="text-xs leading-relaxed" style={{ color: 'var(--muted-foreground)' }}>
                {copyMap[tone].desc}
              </p>
            </div>
            {dismissible && (
              <button
                type="button"
                aria-label="Dismiss alert"
                onClick={() => setVisible(false)}
                className="p-1 rounded-md hover:bg-black/5 dark:hover:bg-white/10 cursor-pointer shrink-0 transition-colors"
                style={{ color: 'var(--muted-foreground)' }}
              >
                <X className="w-3.5 h-3.5" aria-hidden="true" />
              </button>
            )}
          </div>
        ) : (
          <div className="text-center space-y-3 animate-egui-fade-in">
            <p className="text-xs font-mono" style={{ color: 'var(--muted-foreground)' }}>
              Alert dismissed (visible = false)
            </p>
            <button
              type="button"
              onClick={() => setVisible(true)}
              className="px-3.5 py-1.5 rounded-md text-xs font-semibold cursor-pointer active:scale-[0.97] transition-transform"
              style={{ backgroundColor: 'var(--primary)', color: 'var(--primary-foreground)' }}
            >
              Restore Alert
            </button>
          </div>
        )}
      </div>
    </StageShell>
  );
}

function EmptyPreview({ activeTheme }: { activeTheme: ThemePreset }) {
  const [showAction, setShowAction] = useState(true);
  const [showDescription, setShowDescription] = useState(true);
  const [actionCount, setActionCount] = useState(0);

  return (
    <StageShell
      title="Empty::new(...).show(ui)"
      themeName={activeTheme.name}
      onReset={() => {
        setShowAction(true);
        setShowDescription(true);
        setActionCount(0);
      }}
      controls={
        <>
          <ToggleCheckbox checked={showDescription} onChange={setShowDescription} label="Show Description" />
          <ToggleCheckbox checked={showAction} onChange={setShowAction} label="Show Action CTA" />
        </>
      }
    >
      <div
        className="w-full max-w-sm rounded-xl border border-dashed p-7 flex flex-col items-center text-center gap-3.5"
        style={{ backgroundColor: 'var(--card)', borderColor: 'var(--border)' }}
      >
        <div className="relative w-20 h-16 flex items-center justify-center" aria-hidden="true">
          <div
            className="absolute left-2 top-2 w-12 h-10 rounded-lg border -rotate-6"
            style={{ backgroundColor: 'var(--muted)', borderColor: 'var(--border)' }}
          />
          <div
            className="absolute right-2 top-2 w-12 h-10 rounded-lg border rotate-6"
            style={{ backgroundColor: 'var(--muted)', borderColor: 'var(--border)' }}
          />
          <div
            className="relative w-12 h-12 rounded-xl border flex items-center justify-center shadow-xs"
            style={{
              backgroundColor: 'color-mix(in srgb, var(--primary) 12%, var(--background))',
              borderColor: 'color-mix(in srgb, var(--primary) 32%, var(--border))',
              color: 'var(--primary)',
            }}
          >
            <FolderOpen className="w-5 h-5" />
          </div>
        </div>
        <div className="space-y-1">
          <div className="text-sm font-semibold" style={{ color: 'var(--card-foreground)' }}>
            No matching widgets
          </div>
          {showDescription && (
            <p className="text-xs leading-relaxed" style={{ color: 'var(--muted-foreground)' }}>
              Clear active search filters or inspect all 80 gpui-kit ports across Parts 1–8.
            </p>
          )}
        </div>
        {showAction && (
          <button
            type="button"
            onClick={() => setActionCount((c) => c + 1)}
            className="mt-0.5 px-4 py-2 rounded-md text-xs font-semibold cursor-pointer transition-[opacity,transform] hover:opacity-90 active:scale-[0.97]"
            style={{ backgroundColor: 'var(--primary)', color: 'var(--primary-foreground)' }}
          >
            Reset Filters
          </button>
        )}
        {actionCount > 0 && (
          <span key={actionCount} className="animate-egui-pop text-[11px] font-mono" style={{ color: 'var(--success)' }}>
            Action triggered ×{actionCount}
          </span>
        )}
      </div>
    </StageShell>
  );
}

interface DemoFile {
  id: string;
  name: string;
  bytes: number;
  progress?: number;
}

const INITIAL_FILES: DemoFile[] = [
  { id: 'lock', name: 'Cargo.lock', bytes: 48_210, progress: 0.72 },
  { id: 'theme', name: 'theme_tokens.rs', bytes: 14_920 },
];

function AttachmentPreview({ activeTheme }: { activeTheme: ThemePreset }) {
  const [files, setFiles] = useState<DemoFile[]>(INITIAL_FILES);
  const [showProgress, setShowProgress] = useState(true);
  const [removable, setRemovable] = useState(true);
  const [uploadPct, setUploadPct] = useState(72);

  return (
    <StageShell
      title="Attachment::new(name, bytes).show(ui)"
      themeName={activeTheme.name}
      onReset={() => {
        setFiles(INITIAL_FILES);
        setShowProgress(true);
        setRemovable(true);
        setUploadPct(72);
      }}
      controls={
        <>
          <ToggleCheckbox checked={showProgress} onChange={setShowProgress} label="Show Upload Progress" />
          <ToggleCheckbox checked={removable} onChange={setRemovable} label="Removable" />
          {showProgress && (
            <ControlField label={`Upload Progress (${uploadPct}%)`}>
              <input
                type="range"
                min={0}
                max={100}
                value={uploadPct}
                onChange={(e) => setUploadPct(Number(e.target.value))}
                className="w-full accent-[var(--primary)] cursor-pointer"
              />
            </ControlField>
          )}
        </>
      }
    >
      <div className="w-full max-w-sm space-y-3">
        {files.length > 0 ? (
          files.map((file, idx) => {
            const pct = idx === 0 && showProgress ? uploadPct : undefined;
            return (
              <div
                key={file.id}
                className="rounded-lg border p-3 space-y-2.5 transition-colors"
                style={{ backgroundColor: 'var(--card)', borderColor: 'var(--border)' }}
              >
                <div className="flex items-center gap-3">
                  <div
                    className="w-8 h-8 rounded-md flex items-center justify-center shrink-0"
                    style={{ backgroundColor: 'var(--muted)', color: 'var(--primary)' }}
                  >
                    <FileCode2 className="w-4 h-4" aria-hidden="true" />
                  </div>
                  <div className="flex-1 min-w-0">
                    <div className="text-xs font-semibold truncate" style={{ color: 'var(--foreground)' }}>
                      {file.name}
                    </div>
                    <div className="text-[11px] font-mono tabular-nums" style={{ color: 'var(--muted-foreground)' }}>
                      {formatByteSize(file.bytes)}
                      {pct !== undefined ? ` · ${pct}%` : ''}
                    </div>
                  </div>
                  {removable && (
                    <button
                      type="button"
                      aria-label={`Remove ${file.name}`}
                      onClick={() => setFiles((prev) => prev.filter((f) => f.id !== file.id))}
                      className="p-1 rounded hover:opacity-80 cursor-pointer"
                      style={{ color: 'var(--muted-foreground)' }}
                    >
                      <X className="w-3.5 h-3.5" aria-hidden="true" />
                    </button>
                  )}
                </div>
                {pct !== undefined && (
                  <div className="h-1.5 rounded-full overflow-hidden" style={{ backgroundColor: 'var(--muted)' }}>
                    <div
                      className="h-full rounded-full transition-all duration-150 relative overflow-hidden"
                      style={{ width: `${pct}%`, backgroundColor: 'var(--primary)' }}
                    >
                      <div
                        className="absolute inset-y-0 w-1/3 animate-egui-shimmer"
                        style={{ background: 'linear-gradient(90deg, transparent, rgba(255,255,255,0.3), transparent)' }}
                      />
                    </div>
                  </div>
                )}
              </div>
            );
          })
        ) : (
          <div className="text-center space-y-2 py-4">
            <p className="text-xs font-mono" style={{ color: 'var(--muted-foreground)' }}>
              All attachments removed
            </p>
            <button
              type="button"
              onClick={() => setFiles(INITIAL_FILES)}
              className="px-3.5 py-1.5 rounded-md text-xs font-semibold cursor-pointer"
              style={{ backgroundColor: 'var(--primary)', color: 'var(--primary-foreground)' }}
            >
              Restore Attachments
            </button>
          </div>
        )}
      </div>
    </StageShell>
  );
}

function ImagePreview({ activeTheme }: { activeTheme: ThemePreset }) {
  const [state, setState] = useState<'Loaded' | 'Loading' | 'Failed'>('Loaded');
  const [aspect, setAspect] = useState<'16:9' | '4:3' | '1:1'>('16:9');
  const [radius, setRadius] = useState<'0px' | '8px' | '16px'>('8px');
  const [showCaption, setShowCaption] = useState(true);

  const aspectClass =
    aspect === '16:9' ? 'aspect-video w-72' : aspect === '4:3' ? 'aspect-[4/3] w-64' : 'aspect-square w-48';

  return (
    <StageShell
      title="Image::new(texture).aspect_ratio(...)"
      themeName={activeTheme.name}
      onReset={() => {
        setState('Loaded');
        setAspect('16:9');
        setRadius('8px');
        setShowCaption(true);
      }}
      controls={
        <>
          <ControlField label="State">
            <SegmentedButtons options={['Loaded', 'Loading', 'Failed'] as const} value={state} onChange={setState} />
          </ControlField>
          <ControlField label="Aspect Ratio">
            <SegmentedButtons options={['16:9', '4:3', '1:1'] as const} value={aspect} onChange={setAspect} />
          </ControlField>
          <ControlField label="Rounding">
            <SegmentedButtons options={['0px', '8px', '16px'] as const} value={radius} onChange={setRadius} />
          </ControlField>
          <ToggleCheckbox checked={showCaption} onChange={setShowCaption} label="Show Caption" />
        </>
      }
    >
      <div className="flex flex-col items-start gap-2">
        <div
          className={`${aspectClass} group border overflow-hidden relative flex items-center justify-center transition-all duration-200 ${
            state === 'Failed' ? 'border-dashed' : ''
          }`}
          style={{
            borderRadius: radius,
            borderColor: state === 'Failed' ? 'var(--destructive)' : 'var(--border)',
            backgroundColor: 'var(--muted)',
          }}
        >
          {state === 'Loaded' && (
            <svg
              viewBox="0 0 320 180"
              className="animate-egui-fade-in w-full h-full object-cover transition-transform duration-300 group-hover:scale-[1.03]"
              aria-label="Vector mountain preview"
            >
              <defs>
                <linearGradient id="eguiSky" x1="0" y1="0" x2="0.4" y2="1">
                  <stop offset="0%" stopColor="var(--primary)" stopOpacity="0.34" />
                  <stop offset="55%" stopColor="var(--success)" stopOpacity="0.18" />
                  <stop offset="100%" stopColor="var(--card)" stopOpacity="0.05" />
                </linearGradient>
              </defs>
              <rect width="320" height="180" fill="var(--card)" />
              <rect width="320" height="180" fill="url(#eguiSky)" />
              <circle cx="246" cy="48" r="30" fill="var(--warning)" opacity="0.16" />
              <circle cx="246" cy="48" r="18" fill="var(--warning)" opacity="0.88" />
              <path d="M52 54 Q66 42 82 50 Q94 46 102 54 Z" fill="var(--card)" opacity="0.55" />
              <path
                d="M0 180 L78 96 L142 134 L214 68 L320 180 Z"
                fill="var(--primary)"
                opacity="0.72"
              />
              <path
                d="M0 180 L55 128 L112 152 L176 108 L254 148 L320 120 L320 180 Z"
                fill="var(--foreground)"
                opacity="0.22"
              />
            </svg>
          )}

          {state === 'Loading' && (
            <div className="absolute inset-0 overflow-hidden" style={{ backgroundColor: 'var(--muted)' }}>
              <div
                className="absolute inset-y-0 w-1/2 animate-egui-shimmer"
                style={{
                  background: 'linear-gradient(90deg, transparent, rgba(255,255,255,0.28), transparent)',
                }}
              />
            </div>
          )}

          {state === 'Failed' && (
            <div className="animate-egui-fade-in flex flex-col items-center gap-1.5 text-center px-4">
              <div
                className="w-10 h-10 rounded-lg flex items-center justify-center"
                style={{ backgroundColor: 'color-mix(in srgb, var(--destructive) 12%, var(--background))' }}
              >
                <ImageIcon className="w-5 h-5" style={{ color: 'var(--destructive)' }} aria-hidden="true" />
              </div>
              <span className="text-xs font-medium" style={{ color: 'var(--muted-foreground)' }}>
                Image failed to load
              </span>
            </div>
          )}
        </div>
        {showCaption && (
          <span className="text-xs font-mono" style={{ color: 'var(--muted-foreground)' }}>
            aspect {aspect} · rounding {radius} · {state.toLowerCase()}
          </span>
        )}
      </div>
    </StageShell>
  );
}

const MONTH_NAMES = [
  'January',
  'February',
  'March',
  'April',
  'May',
  'June',
  'July',
  'August',
  'September',
  'October',
  'November',
  'December',
];

const YEAR_OPTIONS = getCalendarYearOptions(1950, 2050);

function CalendarGrid({
  viewYear,
  viewMonth0,
  selectedStart,
  selectedEnd,
  hoverEnd,
  rangeMode,
  onPrevMonth,
  onNextMonth,
  onChangeMonthYear,
  onSelectDay,
  onHoverDay,
}: {
  viewYear: number;
  viewMonth0: number;
  selectedStart: string;
  selectedEnd: string;
  hoverEnd?: string | null;
  rangeMode: boolean;
  onPrevMonth: () => void;
  onNextMonth: () => void;
  onChangeMonthYear: (year: number, month0: number) => void;
  onSelectDay: (ymd: string) => void;
  onHoverDay?: (ymd: string | null) => void;
}) {
  const cells = getMonthGrid(viewYear, viewMonth0);
  const effEnd = rangeMode && hoverEnd ? hoverEnd : selectedEnd;
  const lo = selectedStart <= effEnd ? selectedStart : effEnd;
  const hi = selectedStart <= effEnd ? effEnd : selectedStart;
  const singleOrEqual = !rangeMode || lo === hi;

  return (
    <div
      className="w-[292px] rounded-xl border p-3.5 select-none shadow-sm"
      style={{ backgroundColor: 'var(--card)', borderColor: 'var(--border)' }}
    >
      <div className="flex items-center gap-1.5 mb-3">
        <button
          type="button"
          aria-label="Previous month"
          onClick={onPrevMonth}
          className="w-7 h-7 rounded-md border flex items-center justify-center cursor-pointer hover:bg-[var(--accent)] active:scale-95 transition-[background-color,transform] shrink-0"
          style={{ backgroundColor: 'var(--muted)', borderColor: 'var(--border)', color: 'var(--foreground)' }}
        >
          <ChevronLeft className="w-3.5 h-3.5" aria-hidden="true" />
        </button>

        <select
          aria-label="Select month"
          value={viewMonth0}
          onChange={(e) => onChangeMonthYear(viewYear, Number(e.target.value))}
          className="flex-1 min-w-0 h-7 px-2 rounded-md border text-xs font-medium cursor-pointer"
          style={{
            backgroundColor: 'var(--background)',
            borderColor: 'var(--border)',
            color: 'var(--foreground)',
          }}
        >
          {MONTH_NAMES.map((name, idx) => (
            <option key={name} value={idx}>
              {name}
            </option>
          ))}
        </select>

        <select
          aria-label="Select year"
          value={viewYear}
          onChange={(e) => onChangeMonthYear(Number(e.target.value), viewMonth0)}
          className="w-[70px] shrink-0 h-7 px-1.5 rounded-md border text-xs font-mono tabular-nums cursor-pointer"
          style={{
            backgroundColor: 'var(--background)',
            borderColor: 'var(--border)',
            color: 'var(--foreground)',
          }}
        >
          {YEAR_OPTIONS.map((yr) => (
            <option key={yr} value={yr}>
              {yr}
            </option>
          ))}
        </select>

        <button
          type="button"
          aria-label="Next month"
          onClick={onNextMonth}
          className="w-7 h-7 rounded-md border flex items-center justify-center cursor-pointer hover:bg-[var(--accent)] active:scale-95 transition-[background-color,transform] shrink-0"
          style={{ backgroundColor: 'var(--muted)', borderColor: 'var(--border)', color: 'var(--foreground)' }}
        >
          <ChevronRight className="w-3.5 h-3.5" aria-hidden="true" />
        </button>
      </div>

      <div className="grid grid-cols-7 text-center mb-1">
        {['Su', 'Mo', 'Tu', 'We', 'Th', 'Fr', 'Sa'].map((wd) => (
          <span key={wd} className="text-[11px] py-1 font-medium" style={{ color: 'var(--muted-foreground)' }}>
            {wd}
          </span>
        ))}
      </div>

      <div
        key={`${viewYear}-${viewMonth0}`}
        className="animate-egui-fade-in grid grid-cols-7 gap-y-1"
        onMouseLeave={() => onHoverDay?.(null)}
      >
        {cells.map((cell) => {
          const isLo = cell.ymd === lo;
          const isHi = cell.ymd === hi;
          const isEdge = rangeMode ? isLo || isHi : cell.ymd === selectedStart;
          const inRange = rangeMode && cell.ymd > lo && cell.ymd < hi;
          const radiusClass =
            singleOrEqual || (!isEdge && !inRange)
              ? 'rounded-md'
              : isLo
                ? 'rounded-l-md rounded-r-none'
                : isHi
                  ? 'rounded-r-md rounded-l-none'
                  : 'rounded-none';

          return (
            <button
              key={cell.ymd}
              type="button"
              disabled={!cell.inMonth}
              aria-label={cell.ymd}
              onMouseEnter={() => cell.inMonth && onHoverDay?.(cell.ymd)}
              onClick={() => cell.inMonth && onSelectDay(cell.ymd)}
              className={`relative h-8 ${radiusClass} text-xs font-mono tabular-nums transition-[background-color,color,transform] duration-100 flex items-center justify-center ${
                cell.inMonth && !isEdge && !inRange ? 'hover:bg-[var(--accent)]' : ''
              } ${cell.inMonth ? 'active:scale-95' : ''}`}
              style={{
                cursor: cell.inMonth ? 'pointer' : 'default',
                opacity: cell.inMonth ? 1 : 0.25,
                backgroundColor: isEdge
                  ? 'var(--primary)'
                  : inRange
                    ? 'color-mix(in srgb, var(--primary) 18%, var(--card))'
                    : 'transparent',
                color: isEdge ? 'var(--primary-foreground)' : 'var(--foreground)',
                fontWeight: isEdge ? 600 : 400,
              }}
            >
              {cell.day}
              {cell.ymd === '2025-03-15' && !isEdge && (
                <span
                  aria-hidden="true"
                  className="absolute bottom-1 w-1 h-1 rounded-full"
                  style={{ backgroundColor: 'var(--primary)' }}
                />
              )}
            </button>
          );
        })}
      </div>
    </div>
  );
}

function CalendarPreview({ activeTheme }: { activeTheme: ThemePreset }) {
  const [mode, setMode] = useState<'Single' | 'Range'>('Single');
  const [viewYear, setViewYear] = useState(2025);
  const [viewMonth0, setViewMonth0] = useState(2); // March 2025
  const [selectedStart, setSelectedStart] = useState('2025-03-15');
  const [selectedEnd, setSelectedEnd] = useState('2025-03-21');
  const [pickingEnd, setPickingEnd] = useState(false);
  const [hoverDay, setHoverDay] = useState<string | null>(null);

  const shiftMonth = (delta: number) => {
    const dt = new Date(Date.UTC(viewYear, viewMonth0 + delta, 1));
    setViewYear(dt.getUTCFullYear());
    setViewMonth0(dt.getUTCMonth());
  };

  const handleSelectDay = (ymd: string) => {
    if (mode === 'Single') {
      setSelectedStart(ymd);
      setSelectedEnd(ymd);
      return;
    }
    if (!pickingEnd) {
      setSelectedStart(ymd);
      setSelectedEnd(ymd);
      setPickingEnd(true);
    } else {
      if (ymd < selectedStart) {
        setSelectedEnd(selectedStart);
        setSelectedStart(ymd);
      } else {
        setSelectedEnd(ymd);
      }
      setPickingEnd(false);
      setHoverDay(null);
    }
  };

  return (
    <StageShell
      title='Calendar::new("cal", &mut date)'
      themeName={activeTheme.name}
      onReset={() => {
        setMode('Single');
        setViewYear(2025);
        setViewMonth0(2);
        setSelectedStart('2025-03-15');
        setSelectedEnd('2025-03-21');
        setPickingEnd(false);
        setHoverDay(null);
      }}
      controls={
        <>
          <ControlField label="Selection Mode">
            <SegmentedButtons options={['Single', 'Range'] as const} value={mode} onChange={setMode} />
          </ControlField>
          <ControlField label="Bound Value">
            <div
              className="px-3 py-2 rounded border text-xs font-mono tabular-nums"
              style={{ backgroundColor: 'var(--muted)', borderColor: 'var(--border)', color: 'var(--foreground)' }}
            >
              {mode === 'Single' ? selectedStart : `${selectedStart} → ${selectedEnd}`}
            </div>
          </ControlField>
        </>
      }
    >
      <CalendarGrid
        viewYear={viewYear}
        viewMonth0={viewMonth0}
        selectedStart={selectedStart}
        selectedEnd={selectedEnd}
        hoverEnd={pickingEnd ? hoverDay : null}
        rangeMode={mode === 'Range'}
        onPrevMonth={() => shiftMonth(-1)}
        onNextMonth={() => shiftMonth(1)}
        onChangeMonthYear={(y, m0) => {
          setViewYear(y);
          setViewMonth0(m0);
        }}
        onSelectDay={handleSelectDay}
        onHoverDay={setHoverDay}
      />
    </StageShell>
  );
}

function DatePickerPreview({ activeTheme }: { activeTheme: ThemePreset }) {
  const [open, setOpen] = useState(false);
  const [size, setSize] = useState<SizeOpt>('Md');
  const [disabled, setDisabled] = useState(false);
  const [date, setDate] = useState('2025-03-15');
  const [viewYear, setViewYear] = useState(2025);
  const [viewMonth0, setViewMonth0] = useState(2);
  const wrapRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const onKey = (e: KeyboardEvent) => {
      if (e.key === 'Escape') setOpen(false);
    };
    const onDocClick = (e: MouseEvent) => {
      if (wrapRef.current && !wrapRef.current.contains(e.target as Node)) {
        setOpen(false);
      }
    };
    window.addEventListener('keydown', onKey);
    document.addEventListener('mousedown', onDocClick);
    return () => {
      window.removeEventListener('keydown', onKey);
      document.removeEventListener('mousedown', onDocClick);
    };
  }, []);

  const metrics = SIZE_METRICS[size];

  return (
    <StageShell
      title='ui.kit_date_picker("dob", &mut date)'
      themeName={activeTheme.name}
      onReset={() => {
        setOpen(false);
        setSize('Md');
        setDisabled(false);
        setDate('2025-03-15');
        setViewYear(2025);
        setViewMonth0(2);
      }}
      controls={
        <>
          <ControlField label="Size">
            <SegmentedButtons options={['Xs', 'Sm', 'Md', 'Lg'] as const} value={size} onChange={setSize} />
          </ControlField>
          <ToggleCheckbox checked={disabled} onChange={setDisabled} label="Disabled" />
          <ControlField label="Selected SimpleDate">
            <div
              className="px-3 py-2 rounded border text-xs font-mono tabular-nums"
              style={{ backgroundColor: 'var(--muted)', borderColor: 'var(--border)', color: 'var(--foreground)' }}
            >
              {date}
            </div>
          </ControlField>
        </>
      }
    >
      <div ref={wrapRef} className={`relative transition-all duration-150 ${open ? 'mb-64' : ''}`}>
        <button
          type="button"
          disabled={disabled}
          aria-expanded={open}
          aria-label="Open date picker"
          onClick={() => !disabled && setOpen((o) => !o)}
          className="w-56 rounded-md border flex items-center gap-2.5 transition-colors cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed"
          style={{
            height: metrics.h,
            paddingLeft: metrics.px,
            paddingRight: metrics.px,
            fontSize: metrics.text,
            backgroundColor: disabled ? 'var(--muted)' : 'var(--background)',
            borderColor: open ? 'var(--ring)' : 'var(--input)',
            color: disabled ? 'var(--muted-foreground)' : 'var(--foreground)',
          }}
        >
          <CalendarIcon
            className="w-4 h-4 shrink-0"
            style={{ color: open ? 'var(--primary)' : 'var(--muted-foreground)' }}
            aria-hidden="true"
          />
          <span className="font-mono tabular-nums">{date}</span>
        </button>

        {open && !disabled && (
          <div className="animate-egui-menu-in absolute left-0 top-full mt-2 z-30 shadow-xl rounded-xl">
            <CalendarGrid
              viewYear={viewYear}
              viewMonth0={viewMonth0}
              selectedStart={date}
              selectedEnd={date}
              rangeMode={false}
              onPrevMonth={() => {
                const dt = new Date(Date.UTC(viewYear, viewMonth0 - 1, 1));
                setViewYear(dt.getUTCFullYear());
                setViewMonth0(dt.getUTCMonth());
              }}
              onNextMonth={() => {
                const dt = new Date(Date.UTC(viewYear, viewMonth0 + 1, 1));
                setViewYear(dt.getUTCFullYear());
                setViewMonth0(dt.getUTCMonth());
              }}
              onChangeMonthYear={(y, m0) => {
                setViewYear(y);
                setViewMonth0(m0);
              }}
              onSelectDay={(ymd) => {
                setDate(ymd);
                setOpen(false);
              }}
            />
          </div>
        )}
      </div>
    </StageShell>
  );
}

function TimeFieldPreview({ activeTheme }: { activeTheme: ThemePreset }) {
  const [time, setTime] = useState<SimpleTimeValue>({ hours: 14, minutes: 30, seconds: 0 });
  const [activeSeg, setActiveSeg] = useState<'hours' | 'minutes' | 'seconds'>('hours');
  const [use24h, setUse24h] = useState(true);
  const [showSeconds, setShowSeconds] = useState(true);
  const [size, setSize] = useState<SizeOpt>('Md');
  const [disabled, setDisabled] = useState(false);

  const metrics = SIZE_METRICS[size];
  const hour12 = time.hours % 12 === 0 ? 12 : time.hours % 12;
  const displayHour = use24h ? time.hours : hour12;
  const ampm = time.hours < 12 ? 'AM' : 'PM';

  const step = (delta: number) => {
    if (disabled) return;
    setTime((prev) => stepTimeSegment(prev, activeSeg, delta));
  };

  const segments: Array<{ key: 'hours' | 'minutes' | 'seconds'; val: number }> = [
    { key: 'hours', val: displayHour },
    { key: 'minutes', val: time.minutes },
    ...(showSeconds ? [{ key: 'seconds' as const, val: time.seconds }] : []),
  ];

  return (
    <StageShell
      title='ui.kit_time_field("start", &mut time)'
      themeName={activeTheme.name}
      onReset={() => {
        setTime({ hours: 14, minutes: 30, seconds: 0 });
        setActiveSeg('hours');
        setUse24h(true);
        setShowSeconds(true);
        setSize('Md');
        setDisabled(false);
      }}
      controls={
        <>
          <ControlField label="Size">
            <SegmentedButtons options={['Xs', 'Sm', 'Md', 'Lg'] as const} value={size} onChange={setSize} />
          </ControlField>
          <ToggleCheckbox checked={use24h} onChange={setUse24h} label="24-Hour Format" />
          <ToggleCheckbox checked={showSeconds} onChange={setShowSeconds} label="Show Seconds" />
          <ToggleCheckbox checked={disabled} onChange={setDisabled} label="Disabled" />
        </>
      }
    >
      <div className="flex flex-col items-center gap-3">
        <div
          className="inline-flex items-center gap-1.5 rounded-md border px-2.5 transition-[border-color,box-shadow] duration-150 focus-within:border-[var(--ring)] focus-within:ring-2 focus-within:ring-[var(--ring)]/20"
          style={{
            height: metrics.h,
            backgroundColor: disabled ? 'var(--muted)' : 'var(--background)',
            borderColor: 'var(--input)',
            opacity: disabled ? 0.6 : 1,
          }}
        >
          <Clock className="w-3.5 h-3.5 mr-1 shrink-0" style={{ color: 'var(--muted-foreground)' }} aria-hidden="true" />
          {segments.map((seg, idx) => {
            const focused = activeSeg === seg.key && !disabled;
            return (
              <div key={seg.key} className="inline-flex items-center">
                <button
                  type="button"
                  disabled={disabled}
                  aria-label={`Edit ${seg.key}`}
                  onClick={() => setActiveSeg(seg.key)}
                  onKeyDown={(e) => {
                    if (e.key === 'ArrowUp') {
                      e.preventDefault();
                      step(1);
                    } else if (e.key === 'ArrowDown') {
                      e.preventDefault();
                      step(-1);
                    }
                  }}
                  className="px-1.5 py-0.5 rounded font-mono tabular-nums text-xs cursor-pointer transition-colors"
                  style={{
                    backgroundColor: focused ? 'color-mix(in srgb, var(--primary) 18%, transparent)' : 'transparent',
                    color: focused ? 'var(--primary)' : 'var(--foreground)',
                    fontWeight: focused ? 600 : 500,
                  }}
                >
                  {String(seg.val).padStart(2, '0')}
                </button>
                {idx + 1 < segments.length && (
                  <span className="font-mono text-xs px-0.5" style={{ color: 'var(--muted-foreground)' }}>
                    :
                  </span>
                )}
              </div>
            );
          })}

          {!use24h && (
            <button
              type="button"
              disabled={disabled}
              onClick={() =>
                setTime((prev) => ({
                  ...prev,
                  hours: (prev.hours + 12) % 24,
                }))
              }
              className="ml-1 px-1.5 py-0.5 rounded font-mono text-[11px] font-semibold cursor-pointer"
              style={{ backgroundColor: 'var(--muted)', color: 'var(--foreground)' }}
            >
              {ampm}
            </button>
          )}

          <div className="flex items-center gap-0.5 ml-2 pl-2 border-l" style={{ borderColor: 'var(--border)' }}>
            <button
              type="button"
              disabled={disabled}
              aria-label="Decrement active time segment"
              onClick={() => step(-1)}
              className="w-6 h-6 rounded flex items-center justify-center cursor-pointer hover:bg-[var(--muted)] disabled:cursor-not-allowed"
              style={{ color: 'var(--foreground)' }}
            >
              <Minus className="w-3 h-3" aria-hidden="true" />
            </button>
            <button
              type="button"
              disabled={disabled}
              aria-label="Increment active time segment"
              onClick={() => step(1)}
              className="w-6 h-6 rounded flex items-center justify-center cursor-pointer hover:bg-[var(--muted)] disabled:cursor-not-allowed"
              style={{ color: 'var(--foreground)' }}
            >
              <Plus className="w-3 h-3" aria-hidden="true" />
            </button>
          </div>
        </div>
        <span className="text-xs font-mono tabular-nums" style={{ color: 'var(--muted-foreground)' }}>
          24h: {String(time.hours).padStart(2, '0')}:{String(time.minutes).padStart(2, '0')}:
          {String(time.seconds).padStart(2, '0')} · active: {activeSeg}
        </span>
      </div>
    </StageShell>
  );
}

function ColorChannelSlider({
  label,
  value,
  min,
  max,
  step = 1,
  unit = '',
  trackStyle,
  thumbColor,
  checkerboard = false,
  onChange,
}: {
  label: string;
  value: number;
  min: number;
  max: number;
  step?: number;
  unit?: string;
  trackStyle: React.CSSProperties;
  thumbColor: string;
  checkerboard?: boolean;
  onChange: (next: number) => void;
}) {
  const trackRef = useRef<HTMLDivElement>(null);
  const [dragging, setDragging] = useState(false);
  const [hovered, setHovered] = useState(false);

  const frac = max === min ? 0 : Math.min(1, Math.max(0, (value - min) / (max - min)));
  const pct = frac * 100;

  const updateFromClientX = (clientX: number) => {
    const el = trackRef.current;
    if (!el) return;
    const rect = el.getBoundingClientRect();
    const nextFrac = Math.min(1, Math.max(0, (clientX - rect.left) / Math.max(rect.width, 1)));
    const raw = min + nextFrac * (max - min);
    const snapped = Math.round(raw / step) * step;
    onChange(Math.min(max, Math.max(min, snapped)));
  };

  const handleKeyDown = (e: React.KeyboardEvent<HTMLDivElement>) => {
    const bigStep = step * 10;
    let next: number | null = null;
    if (e.key === 'ArrowRight' || e.key === 'ArrowUp') next = value + step;
    else if (e.key === 'ArrowLeft' || e.key === 'ArrowDown') next = value - step;
    else if (e.key === 'PageUp') next = value + bigStep;
    else if (e.key === 'PageDown') next = value - bigStep;
    else if (e.key === 'Home') next = min;
    else if (e.key === 'End') next = max;
    if (next !== null) {
      e.preventDefault();
      onChange(Math.min(max, Math.max(min, next)));
    }
  };

  const thumbActive = hovered || dragging;

  return (
    <div
      role="slider"
      tabIndex={0}
      aria-label={label}
      aria-valuemin={min}
      aria-valuemax={max}
      aria-valuenow={value}
      aria-valuetext={`${value}${unit}`}
      onKeyDown={handleKeyDown}
      onMouseEnter={() => setHovered(true)}
      onMouseLeave={() => setHovered(false)}
      onPointerDown={(e) => {
        e.currentTarget.setPointerCapture(e.pointerId);
        setDragging(true);
        updateFromClientX(e.clientX);
      }}
      onPointerMove={(e) => {
        if (!dragging) return;
        updateFromClientX(e.clientX);
      }}
      onPointerUp={(e) => {
        if (dragging) {
          setDragging(false);
          if (e.currentTarget.hasPointerCapture(e.pointerId)) {
            e.currentTarget.releasePointerCapture(e.pointerId);
          }
        }
      }}
      onPointerCancel={() => setDragging(false)}
      className="relative py-2 px-1.5 select-none touch-none cursor-pointer rounded focus:outline-none focus-visible:ring-2 focus-visible:ring-[var(--ring)]"
    >
      <div
        ref={trackRef}
        className="relative h-3 w-full rounded-full border"
        style={{
          borderColor: 'var(--border)',
          ...(checkerboard
            ? {
                backgroundImage:
                  'conic-gradient(var(--muted) 90deg, var(--background) 90deg 180deg, var(--muted) 180deg 270deg, var(--background) 270deg)',
                backgroundSize: '8px 8px',
              }
            : {}),
        }}
      >
        <div className="absolute inset-0 rounded-full" style={trackStyle} />
        <span
          aria-hidden="true"
          className={`pointer-events-none absolute top-1/2 -translate-x-1/2 -translate-y-1/2 w-4 h-4 rounded-full border-2 border-white transition-transform duration-100 ${
            dragging ? 'scale-115' : thumbActive ? 'scale-105' : 'scale-100'
          }`}
          style={{
            left: `${pct}%`,
            backgroundColor: thumbColor,
            boxShadow: '0 0 0 1px rgba(0, 0, 0, 0.35), 0 1px 3px rgba(0, 0, 0, 0.25)',
          }}
        />
      </div>
    </div>
  );
}

function ColorPickerPreview({ activeTheme }: { activeTheme: ThemePreset }) {
  const [hue, setHue] = useState(234);
  const [sat, setSat] = useState(0.62);
  const [val, setVal] = useState(0.94);
  const [alpha, setAlpha] = useState(100);
  const [showAlpha, setShowAlpha] = useState(true);
  const [copiedHex, setCopiedHex] = useState(false);
  const svRef = useRef<HTMLDivElement>(null);
  const draggingSv = useRef(false);

  const updateSvFromPointer = (clientX: number, clientY: number) => {
    const el = svRef.current;
    if (!el) return;
    const rect = el.getBoundingClientRect();
    const s = Math.min(1, Math.max(0, (clientX - rect.left) / Math.max(rect.width, 1)));
    const v = Math.min(1, Math.max(0, 1 - (clientY - rect.top) / Math.max(rect.height, 1)));
    setSat(Number(s.toFixed(2)));
    setVal(Number(v.toFixed(2)));
  };

  const { r, g, b, hex } = hsvToRgbHex(hue, sat, val);
  const pureHueHex = hsvToRgbHex(hue, 1, 1).hex;
  const rgbaCss = `rgba(${r}, ${g}, ${b}, ${alpha / 100})`;

  const presets = [
    { label: 'Indigo', h: 234, s: 0.62, v: 0.94 },
    { label: 'Emerald', h: 156, s: 0.72, v: 0.78 },
    { label: 'Amber', h: 38, s: 0.85, v: 0.96 },
    { label: 'Crimson', h: 350, s: 0.78, v: 0.88 },
    { label: 'Sky', h: 198, s: 0.76, v: 0.95 },
    { label: 'Violet', h: 272, s: 0.65, v: 0.94 },
  ];

  const handleCopyHex = async () => {
    try {
      await navigator.clipboard?.writeText(hex);
    } catch {
      // Clipboard blocked in sandboxed frames
    }
    setCopiedHex(true);
    window.setTimeout(() => setCopiedHex(false), 1200);
  };

  return (
    <StageShell
      title='ui.kit_color_picker("accent", &mut color)'
      themeName={activeTheme.name}
      onReset={() => {
        setHue(234);
        setSat(0.62);
        setVal(0.94);
        setAlpha(100);
        setShowAlpha(true);
        setCopiedHex(false);
      }}
      controls={
        <>
          <ToggleCheckbox checked={showAlpha} onChange={setShowAlpha} label="Show Alpha Slider" />
          <ControlField label="Computed RGBA">
            <div
              className="px-3 py-2 rounded border text-xs font-mono tabular-nums"
              style={{ backgroundColor: 'var(--muted)', borderColor: 'var(--border)', color: 'var(--foreground)' }}
            >
              {hex} · alpha {alpha}%
            </div>
          </ControlField>
        </>
      }
    >
      <div
        className="w-64 rounded-xl border p-3.5 space-y-2.5 select-none shadow-xs"
        style={{ backgroundColor: 'var(--card)', borderColor: 'var(--border)' }}
      >
        <div
          ref={svRef}
          role="slider"
          tabIndex={0}
          aria-label="Saturation and Value picker"
          aria-valuenow={Math.round(sat * 100)}
          aria-valuemin={0}
          aria-valuemax={100}
          aria-valuetext={`${Math.round(sat * 100)}% saturation, ${Math.round(val * 100)}% brightness`}
          onKeyDown={(e) => {
            if (e.key === 'ArrowRight') {
              e.preventDefault();
              setSat((s) => Number(Math.min(1, s + 0.05).toFixed(2)));
            } else if (e.key === 'ArrowLeft') {
              e.preventDefault();
              setSat((s) => Number(Math.max(0, s - 0.05).toFixed(2)));
            } else if (e.key === 'ArrowUp') {
              e.preventDefault();
              setVal((v) => Number(Math.min(1, v + 0.05).toFixed(2)));
            } else if (e.key === 'ArrowDown') {
              e.preventDefault();
              setVal((v) => Number(Math.max(0, v - 0.05).toFixed(2)));
            }
          }}
          onPointerDown={(e) => {
            draggingSv.current = true;
            e.currentTarget.setPointerCapture(e.pointerId);
            updateSvFromPointer(e.clientX, e.clientY);
          }}
          onPointerMove={(e) => {
            if (draggingSv.current) updateSvFromPointer(e.clientX, e.clientY);
          }}
          onPointerUp={(e) => {
            draggingSv.current = false;
            if (e.currentTarget.hasPointerCapture(e.pointerId)) {
              e.currentTarget.releasePointerCapture(e.pointerId);
            }
          }}
          onPointerCancel={() => {
            draggingSv.current = false;
          }}
          className="relative h-36 rounded-lg border cursor-crosshair overflow-hidden touch-none focus:outline-none focus-visible:ring-2 focus-visible:ring-[var(--ring)]"
          style={{
            borderColor: 'var(--border)',
            background: `linear-gradient(to top, #000000, transparent), linear-gradient(to right, #ffffff, ${pureHueHex})`,
          }}
        >
          <div
            className="w-4 h-4 rounded-full border-2 border-white -translate-x-1/2 -translate-y-1/2 pointer-events-none transition-transform duration-75"
            style={{
              position: 'absolute',
              left: `${sat * 100}%`,
              top: `${(1 - val) * 100}%`,
              backgroundColor: hex,
              boxShadow: '0 0 0 1.5px rgba(0, 0, 0, 0.5), 0 2px 6px rgba(0, 0, 0, 0.3)',
            }}
          />
        </div>

        <div className="space-y-0.5">
          <ColorChannelSlider
            label="Hue"
            value={hue}
            min={0}
            max={359}
            unit="°"
            thumbColor={pureHueHex}
            trackStyle={{
              background:
                'linear-gradient(to right, #ff0000 0%, #ffff00 17%, #00ff00 33%, #00ffff 50%, #0000ff 67%, #ff00ff 83%, #ff0000 100%)',
            }}
            onChange={setHue}
          />

          {showAlpha && (
            <ColorChannelSlider
              label="Alpha opacity"
              value={alpha}
              min={0}
              max={100}
              unit="%"
              checkerboard
              thumbColor={rgbaCss}
              trackStyle={{
                background: `linear-gradient(to right, rgba(${r}, ${g}, ${b}, 0), ${hex})`,
              }}
              onChange={setAlpha}
            />
          )}
        </div>

        <div className="flex items-center justify-between gap-2 pt-0.5">
          <div className="flex items-center gap-2.5 min-w-0">
            <span
              className="relative w-6 h-6 rounded-md border shrink-0 overflow-hidden"
              style={{
                borderColor: 'var(--border)',
                backgroundImage:
                  'conic-gradient(var(--muted) 90deg, var(--background) 90deg 180deg, var(--muted) 180deg 270deg, var(--background) 270deg)',
                backgroundSize: '8px 8px',
              }}
            >
              <span className="absolute inset-0" style={{ backgroundColor: rgbaCss }} />
            </span>
            <span className="text-xs font-mono tabular-nums truncate" style={{ color: 'var(--foreground)' }}>
              {hex} · {alpha}%
            </span>
          </div>
          <button
            type="button"
            onClick={handleCopyHex}
            aria-label="Copy hex color"
            className="p-1.5 rounded-md border cursor-pointer hover:bg-[var(--accent)] transition-colors shrink-0"
            style={{ borderColor: 'var(--border)', color: copiedHex ? 'var(--success)' : 'var(--muted-foreground)' }}
          >
            {copiedHex ? (
              <Check className="w-3 h-3 animate-egui-pop" aria-hidden="true" />
            ) : (
              <Copy className="w-3 h-3" aria-hidden="true" />
            )}
          </button>
        </div>

        <div className="flex items-center gap-1.5 pt-1">
          {presets.map((p) => {
            const pHex = hsvToRgbHex(p.h, p.s, p.v).hex;
            const selected = pHex === hex;
            return (
              <button
                key={p.label}
                type="button"
                aria-label={`Select ${p.label} swatch`}
                onClick={() => {
                  setHue(p.h);
                  setSat(p.s);
                  setVal(p.v);
                }}
                className="w-6 h-6 rounded-md border flex items-center justify-center cursor-pointer transition-transform duration-150 hover:scale-110 active:scale-95"
                style={{
                  backgroundColor: pHex,
                  borderColor: selected ? 'var(--foreground)' : 'var(--border)',
                }}
              >
                {selected && <Check className="w-3 h-3 text-white animate-egui-check-draw" aria-hidden="true" />}
              </button>
            );
          })}
        </div>
      </div>
    </StageShell>
  );
}

function MarkerPreview({ activeTheme }: { activeTheme: ThemePreset }) {
  const [style, setStyle] = useState<'Pin' | 'Callout' | 'Timeline'>('Pin');
  const [tone, setTone] = useState<'Primary' | 'Success' | 'Warning' | 'Destructive'>('Primary');
  const [pulsing, setPulsing] = useState(true);

  const toneVar =
    tone === 'Primary'
      ? 'var(--primary)'
      : tone === 'Success'
        ? 'var(--success)'
        : tone === 'Warning'
          ? 'var(--warning)'
          : 'var(--destructive)';

  return (
    <StageShell
      title="Marker::new(...).style(...)"
      themeName={activeTheme.name}
      onReset={() => {
        setStyle('Pin');
        setTone('Primary');
        setPulsing(true);
      }}
      controls={
        <>
          <ControlField label="Marker Style">
            <SegmentedButtons options={['Pin', 'Callout', 'Timeline'] as const} value={style} onChange={setStyle} />
          </ControlField>
          <ControlField label="Tone">
            <SegmentedButtons options={['Primary', 'Success', 'Warning', 'Destructive'] as const} value={tone} onChange={setTone} />
          </ControlField>
          <ToggleCheckbox checked={pulsing} onChange={setPulsing} label="Pulsing Indicator" />
        </>
      }
    >
      <div className="flex flex-col items-center gap-4">
        {style === 'Pin' && (
          <div className={`flex flex-col items-center ${pulsing ? 'animate-egui-pin-float' : ''}`}>
            <span
              className="px-3 py-1 rounded-full text-xs font-mono font-semibold shadow-sm"
              style={{ backgroundColor: toneVar, color: 'var(--primary-foreground)' }}
            >
              v0.4 · Anchor
            </span>
            <div
              className="w-0 h-0 border-x-[6px] border-x-transparent border-t-[8px] -mt-[1px]"
              style={{ borderTopColor: toneVar }}
            />
          </div>
        )}

        {style === 'Callout' && (
          <div className="inline-flex items-center gap-3 animate-egui-fade-in">
            <span className="relative flex h-3.5 w-3.5 items-center justify-center">
              {pulsing && (
                <span
                  className="animate-egui-mic-ring absolute inset-0 rounded-full border-2 pointer-events-none"
                  style={{ borderColor: toneVar }}
                />
              )}
              <span className="relative inline-flex rounded-full h-2.5 w-2.5" style={{ backgroundColor: toneVar }} />
            </span>
            <span
              className="px-3 py-1.5 rounded-lg border text-xs font-mono font-medium shadow-xs"
              style={{ backgroundColor: 'var(--card)', borderColor: toneVar, color: 'var(--foreground)' }}
            >
              Live frame telemetry
            </span>
          </div>
        )}

        {style === 'Timeline' && (
          <div className="relative flex flex-col gap-3.5 pl-1 animate-egui-fade-in">
            <span
              aria-hidden="true"
              className="absolute left-[9px] top-2 bottom-2 w-0.5 rounded-full"
              style={{ backgroundColor: 'var(--border)' }}
            />
            {[
              { label: 'v0.1 — Core Primitives', active: false },
              { label: 'v0.4 — Alerts & Pickers', active: true },
              { label: 'v0.5 — Navigation Shell', active: false },
            ].map((item) => (
              <div key={item.label} className="relative flex items-center gap-3">
                <span className="relative flex h-3 w-3 items-center justify-center shrink-0">
                  {item.active && pulsing && (
                    <span
                      className="animate-egui-mic-ring absolute inset-0 rounded-full border"
                      style={{ borderColor: toneVar }}
                    />
                  )}
                  <span
                    className="relative w-3 h-3 rounded-full border-2 shrink-0"
                    style={{
                      backgroundColor: item.active ? toneVar : 'var(--muted)',
                      borderColor: 'var(--background)',
                      boxShadow: item.active ? `0 0 0 2px ${toneVar}` : 'none',
                    }}
                  />
                </span>
                <span
                  className="text-xs font-mono"
                  style={{
                    color: item.active ? 'var(--foreground)' : 'var(--muted-foreground)',
                    fontWeight: item.active ? 600 : 400,
                  }}
                >
                  {item.label}
                </span>
              </div>
            ))}
          </div>
        )}
      </div>
    </StageShell>
  );
}

function SpeechPreview({ activeTheme }: { activeTheme: ThemePreset }) {
  const [listening, setListening] = useState(false);
  const [disabled, setDisabled] = useState(false);
  const [elapsedSec, setElapsedSec] = useState(0);

  useEffect(() => {
    if (!listening || disabled) {
      setElapsedSec(0);
      return;
    }
    const id = window.setInterval(() => {
      setElapsedSec((s) => s + 1);
    }, 1000);
    return () => window.clearInterval(id);
  }, [listening, disabled]);

  const active = listening && !disabled;
  const barProfile = [0.36, 0.62, 0.85, 1.0, 0.85, 0.62, 0.36];
  const mm = String(Math.floor(elapsedSec / 60)).padStart(2, '0');
  const ss = String(elapsedSec % 60).padStart(2, '0');

  const samplePhrases = [
    'cargo add egui-kit',
    'cargo add egui-kit --features all-themes',
    'ui.kit_speech("mic", &mut self.listening);',
  ];
  const activeTranscript = samplePhrases[Math.min(samplePhrases.length - 1, Math.floor(elapsedSec / 3))];

  return (
    <StageShell
      title='ui.kit_speech("mic", &mut listening)'
      themeName={activeTheme.name}
      onReset={() => {
        setListening(false);
        setDisabled(false);
        setElapsedSec(0);
      }}
      controls={
        <>
          <ToggleCheckbox checked={listening} onChange={setListening} label="Listening Active" />
          <ToggleCheckbox checked={disabled} onChange={setDisabled} label="Disabled" />
        </>
      }
    >
      <div className="flex flex-col items-center gap-3.5 w-full max-w-xs">
        <button
          type="button"
          disabled={disabled}
          aria-pressed={listening}
          aria-label={listening ? 'Stop speech dictation' : 'Start speech dictation'}
          onClick={() => !disabled && setListening((v) => !v)}
          className="w-80 rounded-2xl border p-2.5 pr-3 flex items-center justify-between gap-3 transition-colors duration-150 cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed text-left"
          style={{
            backgroundColor: active ? 'color-mix(in srgb, var(--primary) 10%, var(--card))' : 'var(--card)',
            borderColor: active ? 'var(--primary)' : 'var(--border)',
          }}
        >
          <div className="flex items-center gap-3 min-w-0">
            <span className="relative flex h-10 w-10 items-center justify-center rounded-full shrink-0">
              {active && (
                <>
                  <span
                    className="animate-egui-mic-ring absolute inset-0 rounded-full border-2 pointer-events-none"
                    style={{ borderColor: 'var(--primary)' }}
                  />
                  <span
                    className="animate-egui-mic-ring absolute inset-0 rounded-full border-2 pointer-events-none"
                    style={{ borderColor: 'var(--primary)', animationDelay: '800ms' }}
                  />
                </>
              )}
              <span
                className="relative w-10 h-10 rounded-full flex items-center justify-center transition-colors duration-150"
                style={{
                  backgroundColor: active ? 'var(--primary)' : 'var(--muted)',
                  color: active ? 'var(--primary-foreground)' : 'var(--foreground)',
                }}
              >
                <Mic className="w-4 h-4" aria-hidden="true" />
              </span>
            </span>

            <div className="min-w-0 flex flex-col">
              <span
                className="text-[13px] font-semibold leading-tight truncate"
                style={{ color: active ? 'var(--foreground)' : 'var(--card-foreground)' }}
              >
                {active ? 'Listening...' : 'Tap to speak'}
              </span>
              <span
                className="text-[11px] font-mono tabular-nums mt-0.5 truncate"
                style={{ color: active ? 'var(--primary)' : 'var(--muted-foreground)' }}
              >
                {active ? `${mm}:${ss} · Capturing` : disabled ? 'Mic unavailable' : 'Voice dictation ready'}
              </span>
            </div>
          </div>

          <div
            className="h-9 px-2.5 rounded-xl flex items-center gap-1 shrink-0 transition-colors duration-150"
            style={{
              backgroundColor: active ? 'color-mix(in srgb, var(--primary) 14%, var(--muted))' : 'var(--muted)',
            }}
            aria-hidden="true"
          >
            {barProfile.map((mult, idx) => (
              <span
                key={idx}
                className={`w-[3px] rounded-full origin-center ${active ? 'animate-egui-wave-bar' : ''}`}
                style={{
                  height: active ? `${Math.round(20 * mult)}px` : `${Math.max(4, Math.round(12 * mult))}px`,
                  backgroundColor: active ? 'var(--primary)' : 'var(--muted-foreground)',
                  opacity: active ? 1 : 0.65,
                  animationDelay: `${idx * 95}ms`,
                }}
              />
            ))}
          </div>
        </button>

        <div
          className="w-80 rounded-xl border px-3.5 py-2.5 space-y-1 transition-colors duration-150"
          style={{
            backgroundColor: 'var(--card)',
            borderColor: active ? 'color-mix(in srgb, var(--primary) 40%, var(--border))' : 'var(--border)',
          }}
        >
          <div className="flex items-center justify-between text-[11px] font-mono">
            <span style={{ color: 'var(--muted-foreground)' }}>Transcript</span>
            <span className="tabular-nums" style={{ color: active ? 'var(--primary)' : 'var(--muted-foreground)' }}>
              {active ? `${mm}:${ss}` : 'idle'}
            </span>
          </div>
          <p
            key={active ? activeTranscript : 'idle'}
            className="animate-egui-fade-in text-xs font-mono leading-relaxed truncate"
            style={{ color: active ? 'var(--foreground)' : 'var(--muted-foreground)' }}
          >
            {active ? `"${activeTranscript}"` : 'Microphone idle (listening = false)'}
          </p>
        </div>
      </div>
    </StageShell>
  );
}
