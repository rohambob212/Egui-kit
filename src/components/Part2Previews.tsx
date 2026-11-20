import { useId, useRef, useState } from 'react';
import { Check, Minus, Search } from 'lucide-react';
import type { ThemePreset } from '../types/kit';
import { clampProgressFraction, snapSliderValue } from '../utils/catalogLogic';
import {
  ControlField,
  SIZE_METRICS,
  SegmentedButtons,
  StageShell,
  ToggleCheckbox,
  type SizeOpt,
} from './Part1Previews';

interface Part2PreviewProps {
  slug: string;
  activeTheme: ThemePreset;
}

export function Part2Preview({ slug, activeTheme }: Part2PreviewProps) {
  switch (slug) {
    case 'checkbox':
      return <CheckboxPreview activeTheme={activeTheme} />;
    case 'radio':
      return <RadioPreview activeTheme={activeTheme} />;
    case 'switch':
      return <SwitchPreview activeTheme={activeTheme} />;
    case 'toggle':
      return <TogglePreview activeTheme={activeTheme} />;
    case 'slider':
      return <SliderPreview activeTheme={activeTheme} />;
    case 'rating':
      return <RatingPreview activeTheme={activeTheme} />;
    case 'progress':
      return <ProgressPreview activeTheme={activeTheme} />;
    case 'spinner':
      return <SpinnerPreview activeTheme={activeTheme} />;
    case 'skeleton':
      return <SkeletonPreview activeTheme={activeTheme} />;
    case 'shimmer':
      return <ShimmerPreview activeTheme={activeTheme} />;
    default:
      return null;
  }
}

function CheckboxPreview({ activeTheme }: { activeTheme: ThemePreset }) {
  const [checked, setChecked] = useState(true);
  const [indeterminate, setIndeterminate] = useState(false);
  const [size, setSize] = useState<SizeOpt>('Md');
  const [disabled, setDisabled] = useState(false);
  const [label, setLabel] = useState('Enable crate telemetry');

  const edge = Math.max(14, SIZE_METRICS[size].icon + 2);
  const isActive = checked || indeterminate;

  return (
    <StageShell
      title="ui.kit_checkbox(&mut checked, ...)"
      themeName={activeTheme.name}
      onReset={() => {
        setChecked(true);
        setIndeterminate(false);
        setSize('Md');
        setDisabled(false);
        setLabel('Enable crate telemetry');
      }}
      controls={
        <>
          <ControlField label="Size">
            <SegmentedButtons options={['Xs', 'Sm', 'Md', 'Lg'] as const} value={size} onChange={setSize} />
          </ControlField>
          <ControlField label="Label">
            <input
              type="text"
              value={label}
              onChange={(e) => setLabel(e.target.value)}
              className="px-2.5 py-1.5 rounded border text-xs"
              style={{ backgroundColor: 'var(--background)', color: 'var(--foreground)', borderColor: 'var(--border)' }}
            />
          </ControlField>
          <div className="flex flex-col gap-2 pt-1">
            <ToggleCheckbox label="Checked" checked={checked} onChange={setChecked} />
            <ToggleCheckbox label="Indeterminate" checked={indeterminate} onChange={setIndeterminate} />
            <ToggleCheckbox label="Disabled" checked={disabled} onChange={setDisabled} />
          </div>
          <div className="p-3 rounded border text-xs font-mono" style={{ backgroundColor: 'var(--muted)', borderColor: 'var(--border)' }}>
            checked: {String(checked)} · indeterminate: {String(indeterminate)}
          </div>
        </>
      }
    >
      <button
        type="button"
        role="checkbox"
        aria-checked={indeterminate ? 'mixed' : checked}
        disabled={disabled}
        onClick={() => {
          setIndeterminate(false);
          setChecked((c) => !c);
        }}
        className="group inline-flex items-center gap-2.5 cursor-pointer disabled:cursor-not-allowed disabled:opacity-55 select-none active:scale-[0.98] transition-transform"
      >
        <span
          className="inline-flex items-center justify-center rounded border transition-colors duration-150 group-hover:ring-2 group-hover:ring-[var(--ring)]/30 shrink-0"
          style={{
            width: edge,
            height: edge,
            backgroundColor: isActive ? 'var(--primary)' : 'var(--background)',
            borderColor: isActive ? 'var(--primary)' : 'var(--input)',
            color: 'var(--primary-foreground)',
          }}
        >
          {indeterminate ? (
            <Minus className="animate-egui-check-draw" style={{ width: edge * 0.72, height: edge * 0.72 }} strokeWidth={2.5} aria-hidden="true" />
          ) : checked ? (
            <Check className="animate-egui-check-draw" style={{ width: edge * 0.72, height: edge * 0.72 }} strokeWidth={2.5} aria-hidden="true" />
          ) : null}
        </span>
        <span style={{ fontSize: SIZE_METRICS[size].text, color: 'var(--foreground)' }}>
          {label || 'Checkbox'}
        </span>
      </button>
    </StageShell>
  );
}

function RadioPreview({ activeTheme }: { activeTheme: ThemePreset }) {
  const [selected, setSelected] = useState<'Debug' | 'Release' | 'Bench'>('Release');
  const [size, setSize] = useState<SizeOpt>('Md');
  const [disabled, setDisabled] = useState(false);

  const diameter = Math.max(14, SIZE_METRICS[size].icon + 2);
  const options: ReadonlyArray<'Debug' | 'Release' | 'Bench'> = ['Debug', 'Release', 'Bench'];

  return (
    <StageShell
      title="ui.kit_radio(&mut profile, ...)"
      themeName={activeTheme.name}
      onReset={() => {
        setSelected('Release');
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
            selected: BuildProfile::{selected}
          </div>
        </>
      }
    >
      <div role="radiogroup" aria-label="Build profile" className="flex flex-wrap items-center gap-6">
        {options.map((opt) => {
          const isSelected = selected === opt;
          return (
            <button
              key={opt}
              type="button"
              role="radio"
              aria-checked={isSelected}
              disabled={disabled}
              onClick={() => setSelected(opt)}
              className="inline-flex items-center gap-2 cursor-pointer disabled:cursor-not-allowed disabled:opacity-55 select-none"
            >
              <span
                className="inline-flex items-center justify-center rounded-full border-2 transition-colors duration-150 shrink-0"
                style={{
                  width: diameter,
                  height: diameter,
                  backgroundColor: 'var(--background)',
                  borderColor: isSelected ? 'var(--primary)' : 'var(--input)',
                }}
              >
                {isSelected && (
                  <span
                    className="animate-egui-dot-pop rounded-full"
                    style={{
                      width: diameter * 0.52,
                      height: diameter * 0.52,
                      backgroundColor: 'var(--primary)',
                    }}
                  />
                )}
              </span>
              <span style={{ fontSize: SIZE_METRICS[size].text, color: 'var(--foreground)' }}>
                {opt}
              </span>
            </button>
          );
        })}
      </div>
    </StageShell>
  );
}

function SwitchPreview({ activeTheme }: { activeTheme: ThemePreset }) {
  const [on, setOn] = useState(true);
  const [size, setSize] = useState<SizeOpt>('Md');
  const [disabled, setDisabled] = useState(false);
  const [label, setLabel] = useState('Auto-save buffers');

  const dims: Record<SizeOpt, { w: number; h: number }> = {
    Xs: { w: 28, h: 16 },
    Sm: { w: 34, h: 18 },
    Md: { w: 40, h: 22 },
    Lg: { w: 48, h: 26 },
  };
  const { w, h } = dims[size];
  const thumb = Math.round(h * 0.76);
  const pad = Math.round((h - thumb) / 2);

  return (
    <StageShell
      title="ui.kit_switch(&mut enabled, ...)"
      themeName={activeTheme.name}
      onReset={() => {
        setOn(true);
        setSize('Md');
        setDisabled(false);
        setLabel('Auto-save buffers');
      }}
      controls={
        <>
          <ControlField label="Size">
            <SegmentedButtons options={['Xs', 'Sm', 'Md', 'Lg'] as const} value={size} onChange={setSize} />
          </ControlField>
          <ControlField label="Label">
            <input
              type="text"
              value={label}
              onChange={(e) => setLabel(e.target.value)}
              className="px-2.5 py-1.5 rounded border text-xs"
              style={{ backgroundColor: 'var(--background)', color: 'var(--foreground)', borderColor: 'var(--border)' }}
            />
          </ControlField>
          <div className="flex flex-col gap-2 pt-1">
            <ToggleCheckbox label="Enabled (on)" checked={on} onChange={setOn} />
            <ToggleCheckbox label="Disabled" checked={disabled} onChange={setDisabled} />
          </div>
          <div className="p-3 rounded border text-xs font-mono" style={{ backgroundColor: 'var(--muted)', borderColor: 'var(--border)' }}>
            switch_state: {on ? 'true' : 'false'}
          </div>
        </>
      }
    >
      <button
        type="button"
        role="switch"
        aria-checked={on}
        disabled={disabled}
        onClick={() => setOn((v) => !v)}
        className="group inline-flex items-center gap-2.5 cursor-pointer disabled:cursor-not-allowed disabled:opacity-55 select-none focus:outline-none focus-visible:ring-2 focus-visible:ring-[var(--ring)] rounded-md p-0.5"
      >
        <span
          className="relative inline-flex items-center rounded-full border transition-colors duration-200 shrink-0 group-hover:ring-2 group-hover:ring-[var(--ring)]/20"
          style={{
            width: w,
            height: h,
            backgroundColor: on ? 'var(--primary)' : 'var(--muted)',
            borderColor: on ? 'var(--primary)' : 'var(--border)',
          }}
        >
          <span
            className="rounded-full shadow-sm"
            style={{
              width: thumb,
              height: thumb,
              backgroundColor: 'var(--background)',
              boxShadow: '0 1px 3px rgba(0,0,0,0.28)',
              transition: 'transform 220ms cubic-bezier(0.34, 1.56, 0.64, 1)',
              transform: `translateX(${on ? w - thumb - pad - 1 : pad}px)`,
            }}
          />
        </span>
        {label && (
          <span
            className="transition-colors duration-150"
            style={{ fontSize: SIZE_METRICS[size].text, color: on ? 'var(--foreground)' : 'var(--muted-foreground)' }}
          >
            {label}
          </span>
        )}
      </button>
    </StageShell>
  );
}

function TogglePreview({ activeTheme }: { activeTheme: ThemePreset }) {
  const [pressed, setPressed] = useState(true);
  const [withIcon, setWithIcon] = useState(true);
  const [size, setSize] = useState<SizeOpt>('Md');
  const [disabled, setDisabled] = useState(false);
  const [label, setLabel] = useState('Live Filter');

  const m = SIZE_METRICS[size];

  return (
    <StageShell
      title="ui.kit_toggle(&mut pressed, ...)"
      themeName={activeTheme.name}
      onReset={() => {
        setPressed(true);
        setWithIcon(true);
        setSize('Md');
        setDisabled(false);
        setLabel('Live Filter');
      }}
      controls={
        <>
          <ControlField label="Size">
            <SegmentedButtons options={['Xs', 'Sm', 'Md', 'Lg'] as const} value={size} onChange={setSize} />
          </ControlField>
          <ControlField label="Label">
            <input
              type="text"
              value={label}
              onChange={(e) => setLabel(e.target.value)}
              className="px-2.5 py-1.5 rounded border text-xs"
              style={{ backgroundColor: 'var(--background)', color: 'var(--foreground)', borderColor: 'var(--border)' }}
            />
          </ControlField>
          <div className="flex flex-col gap-2 pt-1">
            <ToggleCheckbox label="Pressed" checked={pressed} onChange={setPressed} />
            <ToggleCheckbox label="Leading Icon" checked={withIcon} onChange={setWithIcon} />
            <ToggleCheckbox label="Disabled" checked={disabled} onChange={setDisabled} />
          </div>
          <div className="p-3 rounded border text-xs font-mono" style={{ backgroundColor: 'var(--muted)', borderColor: 'var(--border)' }}>
            pressed: {String(pressed)}
          </div>
        </>
      }
    >
      <button
        type="button"
        aria-pressed={pressed}
        disabled={disabled}
        onClick={() => setPressed((p) => !p)}
        className="inline-flex items-center justify-center gap-2 font-medium border transition-[background-color,border-color,color,box-shadow,transform] duration-150 active:scale-[0.96] cursor-pointer disabled:opacity-55 disabled:cursor-not-allowed whitespace-nowrap shrink-0 focus:outline-none focus-visible:ring-2 focus-visible:ring-[var(--ring)]"
        style={{
          height: m.h,
          paddingLeft: m.px,
          paddingRight: m.px,
          fontSize: m.text,
          borderRadius: m.radius,
          backgroundColor: pressed ? 'var(--primary)' : 'var(--card)',
          color: pressed ? 'var(--primary-foreground)' : 'var(--foreground)',
          borderColor: pressed ? 'var(--primary)' : 'var(--border)',
          boxShadow: pressed
            ? '0 2px 8px color-mix(in srgb, var(--primary) 28%, transparent)'
            : '0 1px 2px rgba(0,0,0,0.04)',
        }}
      >
        {withIcon && (
          <Search key={String(pressed)} className="animate-egui-pop" style={{ width: m.icon, height: m.icon }} aria-hidden="true" />
        )}
        <span>{label || 'Toggle'}</span>
      </button>
    </StageShell>
  );
}

function SliderPreview({ activeTheme }: { activeTheme: ThemePreset }) {
  const [value, setValue] = useState(65);
  const [step, setStep] = useState<'Free' | '5' | '10'>('5');
  const [size, setSize] = useState<SizeOpt>('Md');
  const [showValue, setShowValue] = useState(true);
  const [disabled, setDisabled] = useState(false);
  const [dragging, setDragging] = useState(false);
  const [hovered, setHovered] = useState(false);
  const trackRef = useRef<HTMLDivElement>(null);

  const numericStep = step === 'Free' ? undefined : Number(step);
  const trackH = size === 'Xs' || size === 'Sm' ? 4 : size === 'Md' ? 6 : 8;
  const thumbR = trackH + 3;
  const pct = snapSliderValue(value, 0, 100, numericStep);

  const updateFromClientX = (clientX: number) => {
    if (disabled || !trackRef.current) return;
    const rect = trackRef.current.getBoundingClientRect();
    const frac = Math.min(1, Math.max(0, (clientX - rect.left) / Math.max(rect.width, 1)));
    setValue(snapSliderValue(frac * 100, 0, 100, numericStep));
  };

  const handleKeyDown = (e: React.KeyboardEvent<HTMLDivElement>) => {
    if (disabled) return;
    const delta = numericStep ?? 1;
    let next: number | null = null;
    if (e.key === 'ArrowRight' || e.key === 'ArrowUp') next = pct + delta;
    else if (e.key === 'ArrowLeft' || e.key === 'ArrowDown') next = pct - delta;
    else if (e.key === 'PageUp') next = pct + delta * 5;
    else if (e.key === 'PageDown') next = pct - delta * 5;
    else if (e.key === 'Home') next = 0;
    else if (e.key === 'End') next = 100;
    if (next !== null) {
      e.preventDefault();
      setValue(snapSliderValue(next, 0, 100, numericStep));
    }
  };

  const thumbActive = !disabled && (hovered || dragging);
  const motionClass = dragging ? '' : 'transition-[width,left] duration-150 ease-out';

  return (
    <StageShell
      title="Slider::new(&mut value, 0.0..=100.0)"
      themeName={activeTheme.name}
      onReset={() => {
        setValue(65);
        setStep('5');
        setSize('Md');
        setShowValue(true);
        setDisabled(false);
        setDragging(false);
      }}
      controls={
        <>
          <ControlField label="Size">
            <SegmentedButtons options={['Xs', 'Sm', 'Md', 'Lg'] as const} value={size} onChange={setSize} />
          </ControlField>
          <ControlField label="Step Snap">
            <SegmentedButtons
              options={['Free', '5', '10'] as const}
              value={step}
              onChange={(nextStep) => {
                setStep(nextStep);
                const snap = nextStep === 'Free' ? undefined : Number(nextStep);
                setValue((v) => snapSliderValue(v, 0, 100, snap));
              }}
            />
          </ControlField>
          <div className="flex flex-col gap-2 pt-1">
            <ToggleCheckbox label="Show tabular readout" checked={showValue} onChange={setShowValue} />
            <ToggleCheckbox label="Disabled" checked={disabled} onChange={setDisabled} />
          </div>
          <div className="p-3 rounded border text-xs font-mono tabular-nums" style={{ backgroundColor: 'var(--muted)', borderColor: 'var(--border)' }}>
            value: {pct.toFixed(1)}%
          </div>
        </>
      }
    >
      <div className="w-full max-w-xs flex flex-col gap-2">
        <div className="flex items-center justify-between text-xs">
          <span id="kit-slider-label" className="font-medium" style={{ color: 'var(--foreground)' }}>
            Audio Gain
          </span>
          {showValue && (
            <span className="font-mono tabular-nums" style={{ color: 'var(--muted-foreground)' }}>
              {pct.toFixed(1)}%
            </span>
          )}
        </div>
        <div
          id="kit-slider-input"
          role="slider"
          tabIndex={disabled ? -1 : 0}
          aria-labelledby="kit-slider-label"
          aria-valuemin={0}
          aria-valuemax={100}
          aria-valuenow={pct}
          aria-disabled={disabled}
          onKeyDown={handleKeyDown}
          onMouseEnter={() => setHovered(true)}
          onMouseLeave={() => setHovered(false)}
          onPointerDown={(e) => {
            if (disabled) return;
            e.currentTarget.setPointerCapture(e.pointerId);
            setDragging(true);
            updateFromClientX(e.clientX);
          }}
          onPointerMove={(e) => {
            if (!dragging || disabled) return;
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
          className={`relative select-none touch-none focus:outline-none focus-visible:ring-2 focus-visible:ring-[var(--ring)] rounded-md ${
            disabled ? 'cursor-not-allowed' : 'cursor-pointer'
          }`}
          style={{ paddingInline: thumbR, paddingBlock: Math.max(8, thumbR) }}
        >
          <div
            ref={trackRef}
            className="relative w-full rounded-full"
            style={{ height: trackH, backgroundColor: 'var(--muted)' }}
          >
            <div
              className={`h-full rounded-full ${motionClass}`}
              style={{
                width: `${pct}%`,
                backgroundColor: 'var(--primary)',
                opacity: disabled ? 0.45 : 1,
              }}
            />
            {numericStep && numericStep >= 5 && (
              <div className="pointer-events-none absolute inset-0 flex items-center justify-between px-[1px]">
                {Array.from({ length: Math.round(100 / numericStep) + 1 }).map((_, idx) => {
                  const tickPct = idx * numericStep;
                  const covered = tickPct <= pct;
                  return (
                    <span
                      key={tickPct}
                      className="w-1 h-1 rounded-full transition-colors duration-150"
                      style={{
                        backgroundColor: covered ? 'var(--primary-foreground)' : 'var(--muted-foreground)',
                        opacity: covered ? 0.55 : 0.35,
                      }}
                    />
                  );
                })}
              </div>
            )}
            {thumbActive && (
              <>
                <span
                  aria-hidden="true"
                  className={`pointer-events-none absolute top-1/2 -translate-x-1/2 -translate-y-1/2 rounded-full ${motionClass}`}
                  style={{
                    left: `${pct}%`,
                    width: (thumbR + 6) * 2,
                    height: (thumbR + 6) * 2,
                    backgroundColor: 'var(--ring)',
                    opacity: dragging ? 0.22 : 0.14,
                  }}
                />
                {showValue && (
                  <span
                    aria-hidden="true"
                    className={`animate-egui-menu-in pointer-events-none absolute -top-8 -translate-x-1/2 px-1.5 py-0.5 rounded border text-[10px] font-mono tabular-nums font-semibold shadow-sm ${motionClass}`}
                    style={{
                      left: `${pct}%`,
                      backgroundColor: 'var(--card)',
                      borderColor: 'var(--border)',
                      color: 'var(--foreground)',
                    }}
                  >
                    {pct.toFixed(0)}%
                  </span>
                )}
              </>
            )}
            <span
              aria-hidden="true"
              data-slider-thumb="true"
              className={`pointer-events-none absolute top-1/2 -translate-x-1/2 -translate-y-1/2 rounded-full border-2 flex items-center justify-center transition-transform duration-150 ${motionClass} ${
                disabled ? 'opacity-65 scale-100' : dragging ? 'scale-120' : hovered ? 'scale-110' : 'scale-100'
              }`}
              style={{
                width: thumbR * 2,
                height: thumbR * 2,
                left: `${pct}%`,
                backgroundColor: 'var(--background)',
                borderColor: disabled ? 'var(--muted-foreground)' : 'var(--primary)',
                boxShadow: '0 1px 4px rgba(0,0,0,0.2)',
              }}
            >
              <span
                className="rounded-full"
                style={{
                  width: Math.max(4, Math.round(thumbR * 0.55)),
                  height: Math.max(4, Math.round(thumbR * 0.55)),
                  backgroundColor: disabled ? 'var(--muted-foreground)' : 'var(--primary)',
                }}
              />
            </span>
          </div>
        </div>
      </div>
    </StageShell>
  );
}

const STAR_PATH =
  'M12 2.25l2.92 5.92 6.53.95-4.72 4.6 1.11 6.5L12 17.15l-5.84 3.07 1.11-6.5-4.72-4.6 6.53-.95L12 2.25z';

function RatingPreview({ activeTheme }: { activeTheme: ThemePreset }) {
  const [stars, setStars] = useState(4.5);
  const [hoverStars, setHoverStars] = useState<number | null>(null);
  const [suppressedRating, setSuppressedRating] = useState<number | null>(null);
  const [allowHalf, setAllowHalf] = useState(true);
  const [size, setSize] = useState<SizeOpt>('Lg');
  const [disabled, setDisabled] = useState(false);
  const [commitPulse, setCommitPulse] = useState(0);
  const rowRef = useRef<HTMLDivElement>(null);
  const downXRef = useRef<number | null>(null);
  const draggedRef = useRef(false);
  const baseId = useId();

  const starEdge = SIZE_METRICS[size].icon + 6;
  const gap = 6;
  const shown = hoverStars ?? stars;

  const valueFromClientX = (clientX: number): number => {
    if (!rowRef.current) return stars;
    const rect = rowRef.current.getBoundingClientRect();
    const totalW = 5 * starEdge + 4 * gap;
    const relX = Math.max(0, Math.min(totalW, clientX - rect.left));
    const stepW = starEdge + gap;
    const idx = Math.min(4, Math.floor(relX / stepW));
    const localFrac = Math.max(0, Math.min(1, (relX - idx * stepW) / starEdge));
    const val = allowHalf && localFrac <= 0.5 ? idx + 0.5 : idx + 1.0;
    return Math.max(0.5, Math.min(5, val));
  };

  const commitRating = (next: number, allowToggleClear: boolean) => {
    if (disabled) return;
    setCommitPulse((c) => c + 1);
    if (allowToggleClear && Math.abs(stars - next) < 0.01) {
      setStars(0);
      setHoverStars(null);
      setSuppressedRating(next);
    } else {
      setStars(next);
      setSuppressedRating(null);
    }
  };

  return (
    <StageShell
      title="Rating::new(&mut stars)"
      themeName={activeTheme.name}
      onReset={() => {
        setStars(4.5);
        setHoverStars(null);
        setSuppressedRating(null);
        setAllowHalf(true);
        setSize('Lg');
        setDisabled(false);
        setCommitPulse(0);
        downXRef.current = null;
        draggedRef.current = false;
      }}
      controls={
        <>
          <ControlField label="Size">
            <SegmentedButtons options={['Xs', 'Sm', 'Md', 'Lg'] as const} value={size} onChange={setSize} />
          </ControlField>
          <div className="flex flex-col gap-2 pt-1">
            <ToggleCheckbox
              label="Allow half-stars (0.5)"
              checked={allowHalf}
              onChange={(checked) => {
                setAllowHalf(checked);
                if (!checked) {
                  setStars((s) => Math.ceil(s));
                  setHoverStars((h) => (h === null ? null : Math.ceil(h)));
                }
              }}
            />
            <ToggleCheckbox label="Disabled (Read-only)" checked={disabled} onChange={setDisabled} />
          </div>
          <div className="p-3 rounded border text-xs font-mono tabular-nums" style={{ backgroundColor: 'var(--muted)', borderColor: 'var(--border)' }}>
            rating: {stars.toFixed(1)} / 5.0
          </div>
        </>
      }
    >
      <div className="flex items-center gap-4">
        <div
          ref={rowRef}
          className={`inline-flex items-center select-none touch-none py-1 ${
            disabled ? 'cursor-default' : 'cursor-pointer'
          }`}
          style={{ gap }}
          onPointerMove={(e) => {
            if (disabled) return;
            const next = valueFromClientX(e.clientX);
            if (downXRef.current !== null && (e.buttons & 1) === 1) {
              if (!draggedRef.current && Math.abs(e.clientX - downXRef.current) > 3) {
                draggedRef.current = true;
                e.currentTarget.setPointerCapture(e.pointerId);
              }
              if (draggedRef.current) {
                setSuppressedRating(null);
                setHoverStars(next);
                setStars(next);
                return;
              }
            }
            if (suppressedRating !== null) {
              if (Math.abs(next - suppressedRating) < 0.01) return;
              setSuppressedRating(null);
            }
            setHoverStars(next);
          }}
          onPointerDown={(e) => {
            if (disabled) return;
            downXRef.current = e.clientX;
            draggedRef.current = false;
            setHoverStars(valueFromClientX(e.clientX));
          }}
          onPointerUp={(e) => {
            if (draggedRef.current && e.currentTarget.hasPointerCapture(e.pointerId)) {
              e.currentTarget.releasePointerCapture(e.pointerId);
            }
            downXRef.current = null;
            setTimeout(() => {
              draggedRef.current = false;
            }, 0);
          }}
          onPointerLeave={() => {
            if (!draggedRef.current) {
              setHoverStars(null);
              setSuppressedRating(null);
            }
          }}
          role="group"
          aria-label="Star rating"
        >
          {[1, 2, 3, 4, 5].map((index) => {
            const fillRatio = Math.max(0, Math.min(1, shown - (index - 1)));
            const isHoveredStar = !disabled && hoverStars !== null && Math.ceil(hoverStars) === index;
            const justCommitted = commitPulse > 0 && fillRatio > 0;
            const clipId = `${baseId}-star-clip-${index}`;
            return (
              <div
                key={justCommitted ? `${index}-${commitPulse}` : index}
                className={`relative shrink-0 ${justCommitted ? 'animate-egui-pop' : ''}`}
                style={{
                  width: starEdge,
                  height: starEdge,
                  animationDelay: justCommitted ? `${(index - 1) * 28}ms` : undefined,
                }}
              >
                <svg
                  width={starEdge}
                  height={starEdge}
                  viewBox="0 0 24 24"
                  className={`block pointer-events-none transition-transform duration-150 ${
                    isHoveredStar ? 'scale-120' : 'scale-100'
                  }`}
                  style={{
                    filter:
                      fillRatio > 0 && !disabled
                        ? 'drop-shadow(0 1px 4px color-mix(in srgb, var(--warning) 35%, transparent))'
                        : 'none',
                  }}
                  aria-hidden="true"
                >
                  <defs>
                    <clipPath id={clipId}>
                      <rect x="0" y="0" width={24 * fillRatio} height="24" />
                    </clipPath>
                  </defs>
                  <path
                    d={STAR_PATH}
                    fill="var(--muted)"
                    stroke="color-mix(in srgb, var(--muted-foreground) 45%, transparent)"
                    strokeWidth="1.5"
                    strokeLinejoin="round"
                  />
                  {fillRatio > 0 && (
                    <path
                      d={STAR_PATH}
                      clipPath={`url(#${clipId})`}
                      fill="var(--warning)"
                      stroke="var(--warning)"
                      strokeWidth="1.5"
                      strokeLinejoin="round"
                      opacity={disabled ? 0.5 : 1}
                    />
                  )}
                </svg>
                {allowHalf && !disabled && (
                  <button
                    type="button"
                    aria-label={`Rate ${index - 0.5} stars`}
                    onClick={() => {
                      if (!draggedRef.current) commitRating(index - 0.5, true);
                    }}
                    className="absolute inset-y-0 left-0 w-1/2 cursor-pointer focus:outline-none focus-visible:ring-1 focus-visible:ring-[var(--ring)] rounded-l"
                  />
                )}
                {!disabled && (
                  <button
                    type="button"
                    aria-label={`Rate ${index} stars`}
                    onClick={() => {
                      if (!draggedRef.current) commitRating(index, true);
                    }}
                    className={`absolute inset-y-0 right-0 cursor-pointer focus:outline-none focus-visible:ring-1 focus-visible:ring-[var(--ring)] ${
                      allowHalf ? 'w-1/2 rounded-r' : 'w-full rounded'
                    }`}
                  />
                )}
              </div>
            );
          })}
        </div>
        <span className="text-xs font-mono tabular-nums" style={{ color: 'var(--muted-foreground)' }}>
          {shown.toFixed(1)}
        </span>
      </div>
    </StageShell>
  );
}

function ProgressPreview({ activeTheme }: { activeTheme: ThemePreset }) {
  const [fraction, setFraction] = useState(0.72);
  const [indeterminate, setIndeterminate] = useState(false);
  const [showLabel, setShowLabel] = useState(true);
  const [tone, setTone] = useState<'Primary' | 'Success' | 'Warning' | 'Destructive'>('Primary');
  const [size, setSize] = useState<SizeOpt>('Md');

  const height = size === 'Xs' ? 4 : size === 'Sm' ? 6 : size === 'Md' ? 8 : 12;
  const fillCol =
    tone === 'Success'
      ? 'var(--success)'
      : tone === 'Warning'
      ? 'var(--warning)'
      : tone === 'Destructive'
      ? 'var(--destructive)'
      : 'var(--primary)';
  const safeFrac = clampProgressFraction(fraction);

  return (
    <StageShell
      title={indeterminate ? 'Progress::indeterminate()' : `Progress::new(${safeFrac.toFixed(2)})`}
      themeName={activeTheme.name}
      onReset={() => {
        setFraction(0.72);
        setIndeterminate(false);
        setShowLabel(true);
        setTone('Primary');
        setSize('Md');
      }}
      controls={
        <>
          <ControlField label="Tone">
            <SegmentedButtons
              options={['Primary', 'Success', 'Warning', 'Destructive'] as const}
              value={tone}
              onChange={setTone}
            />
          </ControlField>
          <ControlField label="Size">
            <SegmentedButtons options={['Xs', 'Sm', 'Md', 'Lg'] as const} value={size} onChange={setSize} />
          </ControlField>
          <ControlField label={`Value: ${Math.round(safeFrac * 100)}%`}>
            <input
              type="range"
              min={0}
              max={100}
              value={Math.round(safeFrac * 100)}
              disabled={indeterminate}
              onChange={(e) => setFraction(Number(e.target.value) / 100)}
              className="accent-[var(--primary)] cursor-pointer disabled:opacity-45"
            />
          </ControlField>
          <div className="flex flex-col gap-2 pt-1">
            <ToggleCheckbox label="Indeterminate sweep" checked={indeterminate} onChange={setIndeterminate} />
            <ToggleCheckbox label="Show % label" checked={showLabel} onChange={setShowLabel} />
          </div>
        </>
      }
    >
      <div className="w-full max-w-sm flex items-center gap-3">
        <div
          role="progressbar"
          aria-valuemin={0}
          aria-valuemax={100}
          aria-valuenow={indeterminate ? undefined : Math.round(safeFrac * 100)}
          className="flex-1 rounded-full overflow-hidden relative"
          style={{ height, backgroundColor: 'var(--muted)' }}
        >
          {indeterminate ? (
            <div
              className="h-full w-1/3 rounded-full animate-egui-progress"
              style={{ backgroundColor: fillCol }}
            />
          ) : (
            <div
              className="h-full rounded-full transition-all duration-200 ease-out relative overflow-hidden"
              style={{ width: `${safeFrac * 100}%`, backgroundColor: fillCol }}
            >
              <div
                className="absolute inset-y-0 w-1/3 animate-egui-shimmer"
                style={{ background: 'linear-gradient(90deg, transparent, rgba(255,255,255,0.28), transparent)' }}
              />
            </div>
          )}
        </div>
        {showLabel && !indeterminate && (
          <span className="font-mono text-xs tabular-nums w-10 text-right" style={{ color: 'var(--muted-foreground)' }}>
            {Math.round(safeFrac * 100)}%
          </span>
        )}
      </div>
    </StageShell>
  );
}

function SpinnerPreview({ activeTheme }: { activeTheme: ThemePreset }) {
  const [size, setSize] = useState<SizeOpt>('Md');
  const [tone, setTone] = useState<'Primary' | 'Success' | 'Warning' | 'Destructive'>('Primary');
  const [label, setLabel] = useState('Compiling egui-kit...');

  const edge = SIZE_METRICS[size].icon + 6;
  const strokeW = size === 'Xs' ? 2 : size === 'Lg' ? 3 : 2.5;
  const color =
    tone === 'Success'
      ? 'var(--success)'
      : tone === 'Warning'
      ? 'var(--warning)'
      : tone === 'Destructive'
      ? 'var(--destructive)'
      : 'var(--primary)';

  return (
    <StageShell
      title="Spinner::new()"
      themeName={activeTheme.name}
      onReset={() => {
        setSize('Md');
        setTone('Primary');
        setLabel('Compiling egui-kit...');
      }}
      controls={
        <>
          <ControlField label="Size">
            <SegmentedButtons options={['Xs', 'Sm', 'Md', 'Lg'] as const} value={size} onChange={setSize} />
          </ControlField>
          <ControlField label="Color">
            <SegmentedButtons
              options={['Primary', 'Success', 'Warning', 'Destructive'] as const}
              value={tone}
              onChange={setTone}
            />
          </ControlField>
          <ControlField label="Status Label">
            <input
              type="text"
              value={label}
              onChange={(e) => setLabel(e.target.value)}
              className="px-2.5 py-1.5 rounded border text-xs"
              style={{ backgroundColor: 'var(--background)', color: 'var(--foreground)', borderColor: 'var(--border)' }}
            />
          </ControlField>
        </>
      }
    >
      <div role="status" className="inline-flex items-center gap-2.5">
        <svg
          width={edge}
          height={edge}
          viewBox="0 0 28 28"
          className="animate-spin shrink-0"
          aria-hidden="true"
        >
          <circle
            cx="14"
            cy="14"
            r="10.5"
            fill="none"
            stroke="var(--muted)"
            strokeWidth={strokeW}
          />
          <circle
            cx="14"
            cy="14"
            r="10.5"
            fill="none"
            stroke={color}
            strokeWidth={strokeW}
            strokeLinecap="round"
            className="animate-egui-spinner-arc"
          />
        </svg>
        {label && (
          <span style={{ fontSize: SIZE_METRICS[size].text, color: 'var(--muted-foreground)' }}>
            {label}
          </span>
        )}
      </div>
    </StageShell>
  );
}

function SkeletonPreview({ activeTheme }: { activeTheme: ThemePreset }) {
  const [preset, setPreset] = useState<'Card' | 'Lines' | 'Circle'>('Card');
  const [animated, setAnimated] = useState(true);
  const [lineCount, setLineCount] = useState(3);

  const Sheen = () =>
    animated ? (
      <div
        className="pointer-events-none absolute inset-y-0 w-1/2 animate-egui-shimmer"
        style={{ background: 'linear-gradient(90deg, transparent, color-mix(in srgb, var(--background) 55%, transparent), transparent)' }}
      />
    ) : null;

  return (
    <StageShell
      title={`Skeleton::${preset.toLowerCase()}(...)`}
      themeName={activeTheme.name}
      onReset={() => {
        setPreset('Card');
        setAnimated(true);
        setLineCount(3);
      }}
      controls={
        <>
          <ControlField label="Shape Preset">
            <SegmentedButtons options={['Card', 'Lines', 'Circle'] as const} value={preset} onChange={setPreset} />
          </ControlField>
          {preset === 'Lines' && (
            <ControlField label={`Line Count: ${lineCount}`}>
              <input
                type="range"
                min={1}
                max={5}
                value={lineCount}
                onChange={(e) => setLineCount(Number(e.target.value))}
                className="accent-[var(--primary)] cursor-pointer"
              />
            </ControlField>
          )}
          <ToggleCheckbox label="Animated shimmer" checked={animated} onChange={setAnimated} />
        </>
      }
    >
      {preset === 'Circle' && (
        <div
          className="relative overflow-hidden w-14 h-14 rounded-full"
          style={{ backgroundColor: 'var(--muted)' }}
          aria-label="Loading avatar skeleton"
        >
          <Sheen />
        </div>
      )}

      {preset === 'Lines' && (
        <div className="w-full max-w-xs flex flex-col gap-2.5">
          {Array.from({ length: lineCount }).map((_, idx) => (
            <div
              key={idx}
              className="relative overflow-hidden h-3 rounded"
              style={{
                width: idx === lineCount - 1 && lineCount > 1 ? '68%' : '100%',
                backgroundColor: 'var(--muted)',
              }}
            >
              <Sheen />
            </div>
          ))}
        </div>
      )}

      {preset === 'Card' && (
        <div
          className="w-full max-w-xs p-4 rounded-lg border flex items-start gap-3"
          style={{ backgroundColor: 'var(--card)', borderColor: 'var(--border)' }}
        >
          <div className="relative overflow-hidden w-10 h-10 rounded-full shrink-0" style={{ backgroundColor: 'var(--muted)' }}>
            <Sheen />
          </div>
          <div className="flex-1 flex flex-col gap-2">
            <div className="relative overflow-hidden h-3.5 w-3/4 rounded" style={{ backgroundColor: 'var(--muted)' }}>
              <Sheen />
            </div>
            <div className="relative overflow-hidden h-3 w-full rounded" style={{ backgroundColor: 'var(--muted)' }}>
              <Sheen />
            </div>
            <div className="relative overflow-hidden h-3 w-2/3 rounded" style={{ backgroundColor: 'var(--muted)' }}>
              <Sheen />
            </div>
          </div>
        </div>
      )}
    </StageShell>
  );
}

function ShimmerPreview({ activeTheme }: { activeTheme: ThemePreset }) {
  const [mode, setMode] = useState<'WrapCard' | 'Block'>('WrapCard');
  const [active, setActive] = useState(true);

  return (
    <StageShell
      title={mode === 'WrapCard' ? 'Shimmer::wrap(ui, ...)' : 'Shimmer::rect(240.0, 28.0)'}
      themeName={activeTheme.name}
      onReset={() => {
        setMode('WrapCard');
        setActive(true);
      }}
      controls={
        <>
          <ControlField label="Mode">
            <SegmentedButtons options={['WrapCard', 'Block'] as const} value={mode} onChange={setMode} />
          </ControlField>
          <ToggleCheckbox label="Sweep animation active" checked={active} onChange={setActive} />
        </>
      }
    >
      {mode === 'Block' ? (
        <div
          className="w-64 h-8 rounded-md relative overflow-hidden"
          style={{ backgroundColor: 'var(--muted)' }}
        >
          {active && (
            <div
              className="absolute inset-y-0 w-1/3 animate-egui-shimmer"
              style={{
                background: 'linear-gradient(90deg, transparent, var(--card), transparent)',
                opacity: 0.7,
              }}
            />
          )}
        </div>
      ) : (
        <div
          className="w-full max-w-xs p-4 rounded-lg border relative overflow-hidden space-y-3"
          style={{ backgroundColor: 'var(--card)', borderColor: 'var(--border)' }}
        >
          <div className="flex items-center gap-3">
            <div className="w-9 h-9 rounded-full shrink-0" style={{ backgroundColor: 'var(--muted)' }} />
            <div className="flex-1 space-y-1.5">
              <div className="h-3 w-2/3 rounded" style={{ backgroundColor: 'var(--muted)' }} />
              <div className="h-2.5 w-1/2 rounded" style={{ backgroundColor: 'var(--muted)' }} />
            </div>
          </div>
          <div className="h-16 rounded" style={{ backgroundColor: 'var(--muted)' }} />
          {active && (
            <div
              className="absolute inset-y-0 w-1/2 pointer-events-none animate-egui-shimmer"
              style={{
                background: 'linear-gradient(90deg, transparent, var(--background), transparent)',
                opacity: 0.45,
              }}
            />
          )}
        </div>
      )}
    </StageShell>
  );
}
