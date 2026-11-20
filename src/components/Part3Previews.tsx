import { useEffect, useRef, useState } from 'react';
import {
  AlertCircle,
  Check,
  ChevronDown,
  Copy,
  Eye,
  EyeOff,
  Minus,
  Plus,
  Search,
  X,
} from 'lucide-react';
import type { ThemePreset } from '../types/kit';
import {
  clampNumberInput,
  distributeOtpInput,
  filterComboboxOptions,
  validateFormFields,
  type FormValidationErrors,
} from '../utils/catalogLogic';
import {
  ControlField,
  KbdChord,
  SIZE_METRICS,
  SegmentedButtons,
  StageShell,
  ToggleCheckbox,
  type SizeOpt,
} from './Part1Previews';

interface Part3PreviewProps {
  slug: string;
  activeTheme: ThemePreset;
}

export function Part3Preview({ slug, activeTheme }: Part3PreviewProps) {
  switch (slug) {
    case 'input':
      return <InputPreview activeTheme={activeTheme} />;
    case 'input-group':
      return <InputGroupPreview activeTheme={activeTheme} />;
    case 'textarea':
      return <TextAreaPreview activeTheme={activeTheme} />;
    case 'number-input':
      return <NumberInputPreview activeTheme={activeTheme} />;
    case 'otp-input':
      return <OtpInputPreview activeTheme={activeTheme} />;
    case 'select':
      return <SelectPreview activeTheme={activeTheme} />;
    case 'combobox':
      return <ComboboxPreview activeTheme={activeTheme} />;
    case 'form':
      return <FormPreview activeTheme={activeTheme} />;
    case 'questionnaire':
      return <QuestionnairePreview activeTheme={activeTheme} />;
    case 'clipboard':
      return <ClipboardPreview activeTheme={activeTheme} />;
    default:
      return null;
  }
}

function InputPreview({ activeTheme }: { activeTheme: ThemePreset }) {
  const [value, setValue] = useState('egui-kit');
  const [size, setSize] = useState<SizeOpt>('Md');
  const [prefixIcon, setPrefixIcon] = useState(true);
  const [clearable, setClearable] = useState(true);
  const [password, setPassword] = useState(false);
  const [revealPwd, setRevealPwd] = useState(false);
  const [error, setError] = useState(false);
  const [disabled, setDisabled] = useState(false);
  const [focused, setFocused] = useState(false);
  const inputRef = useRef<HTMLInputElement>(null);

  const metrics = SIZE_METRICS[size];
  const borderColor = error
    ? 'var(--destructive)'
    : focused
      ? 'var(--ring)'
      : 'var(--input)';

  return (
    <StageShell
      title="ui.kit_input(&mut value, ...)"
      themeName={activeTheme.name}
      onReset={() => {
        setValue('egui-kit');
        setSize('Md');
        setPrefixIcon(true);
        setClearable(true);
        setPassword(false);
        setRevealPwd(false);
        setError(false);
        setDisabled(false);
      }}
      controls={
        <>
          <ControlField label="Size">
            <SegmentedButtons options={['Xs', 'Sm', 'Md', 'Lg'] as const} value={size} onChange={setSize} />
          </ControlField>
          <div className="flex flex-col gap-2 pt-1">
            <ToggleCheckbox label="Prefix Search Icon" checked={prefixIcon} onChange={setPrefixIcon} />
            <ToggleCheckbox label="Clearable Button" checked={clearable} onChange={setClearable} />
            <ToggleCheckbox label="Password Mode" checked={password} onChange={setPassword} />
            <ToggleCheckbox label="Validation Error" checked={error} onChange={setError} />
            <ToggleCheckbox label="Disabled" checked={disabled} onChange={setDisabled} />
          </div>
          <div className="p-3 rounded border text-xs font-mono truncate" style={{ backgroundColor: 'var(--muted)', borderColor: 'var(--border)' }}>
            value: &quot;{value}&quot; ({value.length} chars)
          </div>
        </>
      }
    >
      <div className="w-full max-w-xs space-y-2">
        <div
          className="flex items-center gap-2 border transition-[border-color,box-shadow] duration-150"
          style={{
            height: metrics.h,
            paddingInline: '10px',
            borderRadius: metrics.radius,
            backgroundColor: disabled ? 'var(--muted)' : 'var(--background)',
            borderColor,
            boxShadow: focused && !disabled
              ? `0 0 0 3px color-mix(in srgb, ${error ? 'var(--destructive)' : 'var(--ring)'} 22%, transparent)`
              : 'none',
            opacity: disabled ? 0.65 : 1,
          }}
        >
          {prefixIcon && (
            <Search
              aria-hidden="true"
              className="shrink-0 transition-colors duration-150"
              style={{
                width: metrics.icon,
                height: metrics.icon,
                color: focused ? 'var(--foreground)' : 'var(--muted-foreground)',
              }}
            />
          )}
          <input
            ref={inputRef}
            type={password && !revealPwd ? 'password' : 'text'}
            value={value}
            disabled={disabled}
            placeholder="Filter components..."
            aria-label="Component filter input"
            aria-invalid={error}
            onFocus={() => setFocused(true)}
            onBlur={() => setFocused(false)}
            onChange={(e) => setValue(e.target.value)}
            className="w-full min-w-0 bg-transparent outline-none"
            style={{
              fontSize: metrics.text,
              color: disabled ? 'var(--muted-foreground)' : 'var(--foreground)',
            }}
          />
          {password && !disabled && (
            <button
              type="button"
              onClick={() => setRevealPwd((r) => !r)}
              aria-label={revealPwd ? 'Hide password' : 'Show password'}
              className="p-0.5 rounded cursor-pointer transition-opacity hover:opacity-80 shrink-0"
              style={{ color: 'var(--muted-foreground)' }}
            >
              {revealPwd ? <EyeOff className="w-3.5 h-3.5" /> : <Eye className="w-3.5 h-3.5" />}
            </button>
          )}
          {clearable && value.length > 0 && !disabled && (
            <button
              type="button"
              onClick={() => {
                setValue('');
                inputRef.current?.focus();
              }}
              aria-label="Clear input text"
              className="p-0.5 rounded cursor-pointer transition-transform duration-150 hover:scale-110 active:scale-95 shrink-0"
              style={{ color: 'var(--muted-foreground)' }}
            >
              <X className="w-3.5 h-3.5" />
            </button>
          )}
        </div>
        {error && (
          <p className="text-xs flex items-center gap-1.5 animate-egui-menu-in" style={{ color: 'var(--destructive)' }}>
            <AlertCircle className="w-3.5 h-3.5 shrink-0" aria-hidden="true" />
            <span>Crate identifier failed validation check.</span>
          </p>
        )}
      </div>
    </StageShell>
  );
}

function InputGroupPreview({ activeTheme }: { activeTheme: ThemePreset }) {
  const [host, setHost] = useState('crates.io/crates/egui-kit');
  const [prefix, setPrefix] = useState('https://');
  const [suffix, setSuffix] = useState('.rs');
  const [size, setSize] = useState<SizeOpt>('Md');
  const [error, setError] = useState(false);
  const [disabled, setDisabled] = useState(false);
  const [focused, setFocused] = useState(false);

  const metrics = SIZE_METRICS[size];
  const borderColor = error
    ? 'var(--destructive)'
    : focused
      ? 'var(--ring)'
      : 'var(--input)';

  return (
    <StageShell
      title="InputGroup::new().prefix(...).suffix(...)"
      themeName={activeTheme.name}
      onReset={() => {
        setHost('crates.io/crates/egui-kit');
        setPrefix('https://');
        setSuffix('.rs');
        setSize('Md');
        setError(false);
        setDisabled(false);
      }}
      controls={
        <>
          <ControlField label="Size">
            <SegmentedButtons options={['Xs', 'Sm', 'Md', 'Lg'] as const} value={size} onChange={setSize} />
          </ControlField>
          <ControlField label="Prefix Addon">
            <SegmentedButtons options={['https://', 'git://', 'ssh://'] as const} value={prefix as 'https://'} onChange={setPrefix} />
          </ControlField>
          <ControlField label="Suffix Addon">
            <SegmentedButtons options={['.rs', '.dev', '/docs'] as const} value={suffix as '.rs'} onChange={setSuffix} />
          </ControlField>
          <div className="flex flex-col gap-2 pt-1">
            <ToggleCheckbox label="Error Highlight" checked={error} onChange={setError} />
            <ToggleCheckbox label="Disabled" checked={disabled} onChange={setDisabled} />
          </div>
          <div className="p-3 rounded border text-xs font-mono truncate" style={{ backgroundColor: 'var(--muted)', borderColor: 'var(--border)' }}>
            {prefix}{host}{suffix}
          </div>
        </>
      }
    >
      <div className="w-full max-w-md space-y-2">
        <div
          className="flex items-stretch border overflow-hidden transition-[border-color,box-shadow] duration-150"
          style={{
            height: metrics.h,
            borderRadius: metrics.radius,
            backgroundColor: disabled ? 'var(--muted)' : 'var(--background)',
            borderColor,
            boxShadow: focused && !disabled
              ? `0 0 0 3px color-mix(in srgb, ${error ? 'var(--destructive)' : 'var(--ring)'} 22%, transparent)`
              : 'none',
            opacity: disabled ? 0.65 : 1,
          }}
        >
          <span
            className="inline-flex items-center px-3 border-r font-mono text-xs select-none shrink-0"
            style={{
              backgroundColor: 'var(--muted)',
              borderColor: 'var(--border)',
              color: 'var(--muted-foreground)',
            }}
          >
            {prefix}
          </span>
          <input
            type="text"
            value={host}
            disabled={disabled}
            aria-label="Endpoint host path"
            placeholder="domain/path"
            onFocus={() => setFocused(true)}
            onBlur={() => setFocused(false)}
            onChange={(e) => setHost(e.target.value)}
            className="flex-1 min-w-0 px-3 bg-transparent outline-none"
            style={{
              fontSize: metrics.text,
              color: disabled ? 'var(--muted-foreground)' : 'var(--foreground)',
            }}
          />
          <span
            className="inline-flex items-center px-3 border-l font-mono text-xs select-none shrink-0"
            style={{
              backgroundColor: 'var(--muted)',
              borderColor: 'var(--border)',
              color: 'var(--muted-foreground)',
            }}
          >
            {suffix}
          </span>
        </div>
      </div>
    </StageShell>
  );
}

function TextAreaPreview({ activeTheme }: { activeTheme: ThemePreset }) {
  const [notes, setNotes] = useState('Zero-boilerplate egui widgets with smooth 60fps spring transitions and 36 synchronized presets.');
  const [rows, setRows] = useState<'3' | '4' | '6'>('4');
  const [maxChars, setMaxChars] = useState<'80' | '140' | '280'>('140');
  const [disabled, setDisabled] = useState(false);
  const [focused, setFocused] = useState(false);

  const limit = Number(maxChars);
  const count = notes.length;
  const overLimit = count > limit;
  const nearLimit = !overLimit && count >= Math.floor(limit * 0.85);

  const countColor = overLimit
    ? 'var(--destructive)'
    : nearLimit
      ? 'var(--warning)'
      : 'var(--muted-foreground)';

  return (
    <StageShell
      title="TextArea::new(&mut notes).max_chars(...)"
      themeName={activeTheme.name}
      onReset={() => {
        setNotes('Zero-boilerplate egui widgets with smooth 60fps spring transitions and 36 synchronized presets.');
        setRows('4');
        setMaxChars('140');
        setDisabled(false);
      }}
      controls={
        <>
          <ControlField label="Visible Rows">
            <SegmentedButtons options={['3', '4', '6'] as const} value={rows} onChange={setRows} />
          </ControlField>
          <ControlField label="Character Budget">
            <SegmentedButtons options={['80', '140', '280'] as const} value={maxChars} onChange={setMaxChars} />
          </ControlField>
          <ToggleCheckbox label="Disabled" checked={disabled} onChange={setDisabled} />
        </>
      }
    >
      <div className="w-full max-w-md space-y-1.5">
        <textarea
          rows={Number(rows)}
          value={notes}
          disabled={disabled}
          aria-label="Multi-line release notes"
          aria-invalid={overLimit}
          placeholder="Write release changelog..."
          onFocus={() => setFocused(true)}
          onBlur={() => setFocused(false)}
          onChange={(e) => setNotes(e.target.value)}
          className="w-full rounded-md border p-3 text-sm leading-relaxed outline-none resize-y transition-[border-color,box-shadow] duration-150"
          style={{
            backgroundColor: disabled ? 'var(--muted)' : 'var(--background)',
            color: disabled ? 'var(--muted-foreground)' : 'var(--foreground)',
            borderColor: overLimit
              ? 'var(--destructive)'
              : focused
                ? 'var(--ring)'
                : 'var(--input)',
            boxShadow: focused && !disabled
              ? `0 0 0 3px color-mix(in srgb, ${overLimit ? 'var(--destructive)' : 'var(--ring)'} 22%, transparent)`
              : 'none',
          }}
        />
        <div className="flex items-center justify-between text-xs">
          <span style={{ color: overLimit ? 'var(--destructive)' : 'var(--muted-foreground)' }}>
            {overLimit ? 'Character limit exceeded' : nearLimit ? 'Approaching budget limit' : 'Scrollable multiline buffer'}
          </span>
          <span className="font-mono tabular-nums font-medium transition-colors duration-150" style={{ color: countColor }}>
            {count} / {limit}
          </span>
        </div>
      </div>
    </StageShell>
  );
}

function NumberInputPreview({ activeTheme }: { activeTheme: ThemePreset }) {
  const min = 0;
  const max = 100;
  const [value, setValue] = useState(24);
  const [draft, setDraft] = useState('24');
  const [stepOpt, setStepOpt] = useState<'1' | '5' | '0.5'>('1');
  const [size, setSize] = useState<SizeOpt>('Md');
  const [disabled, setDisabled] = useState(false);
  const [focused, setFocused] = useState(false);

  const step = Number(stepOpt);
  const metrics = SIZE_METRICS[size];
  const formatVal = (v: number) => (stepOpt === '0.5' ? v.toFixed(1) : String(Math.round(v)));

  const commit = (nextRaw: number) => {
    const next = clampNumberInput(nextRaw, min, max, step);
    setValue(next);
    setDraft(formatVal(next));
  };

  return (
    <StageShell
      title="NumberInput::new(&mut count, 0.0..=100.0)"
      themeName={activeTheme.name}
      onReset={() => {
        setStepOpt('1');
        setSize('Md');
        setDisabled(false);
        setValue(24);
        setDraft('24');
      }}
      controls={
        <>
          <ControlField label="Size">
            <SegmentedButtons options={['Xs', 'Sm', 'Md', 'Lg'] as const} value={size} onChange={setSize} />
          </ControlField>
          <ControlField label="Step Increment">
            <SegmentedButtons
              options={['1', '5', '0.5'] as const}
              value={stepOpt}
              onChange={(nextStep) => {
                setStepOpt(nextStep);
                const snapped = clampNumberInput(value, min, max, Number(nextStep));
                setValue(snapped);
                setDraft(nextStep === '0.5' ? snapped.toFixed(1) : String(Math.round(snapped)));
              }}
            />
          </ControlField>
          <ToggleCheckbox label="Disabled" checked={disabled} onChange={setDisabled} />
          <div className="p-3 rounded border text-xs font-mono tabular-nums" style={{ backgroundColor: 'var(--muted)', borderColor: 'var(--border)' }}>
            range: 0..=100 · value: {formatVal(value)}
          </div>
        </>
      }
    >
      <div
        className="inline-flex items-stretch border overflow-hidden transition-[border-color,box-shadow] duration-150"
        style={{
          height: metrics.h,
          width: '210px',
          borderRadius: metrics.radius,
          backgroundColor: disabled ? 'var(--muted)' : 'var(--background)',
          borderColor: focused ? 'var(--ring)' : 'var(--input)',
          boxShadow: focused && !disabled ? '0 0 0 3px color-mix(in srgb, var(--ring) 22%, transparent)' : 'none',
          opacity: disabled ? 0.65 : 1,
        }}
      >
        <button
          type="button"
          disabled={disabled || value <= min}
          onClick={() => commit(value - step)}
          aria-label="Decrement value"
          className="px-3 border-r flex items-center justify-center cursor-pointer disabled:cursor-not-allowed disabled:opacity-45 hover:bg-[var(--accent)] active:scale-95 transition-[background-color,transform] duration-100"
          style={{ backgroundColor: 'var(--muted)', borderColor: 'var(--border)', color: 'var(--foreground)' }}
        >
          <Minus className="w-3.5 h-3.5" aria-hidden="true" />
        </button>
        <input
          type="text"
          inputMode="decimal"
          value={draft}
          disabled={disabled}
          aria-label="Numeric input value"
          onFocus={() => setFocused(true)}
          onBlur={() => {
            setFocused(false);
            const parsed = Number.parseFloat(draft);
            commit(Number.isFinite(parsed) ? parsed : value);
          }}
          onKeyDown={(e) => {
            if (e.key === 'ArrowUp') {
              e.preventDefault();
              commit(value + step);
            } else if (e.key === 'ArrowDown') {
              e.preventDefault();
              commit(value - step);
            } else if (e.key === 'Enter') {
              const parsed = Number.parseFloat(draft);
              commit(Number.isFinite(parsed) ? parsed : value);
            }
          }}
          onChange={(e) => setDraft(e.target.value)}
          className="flex-1 min-w-0 text-center font-mono tabular-nums bg-transparent outline-none px-2"
          style={{ fontSize: metrics.text, color: 'var(--foreground)' }}
        />
        <button
          type="button"
          disabled={disabled || value >= max}
          onClick={() => commit(value + step)}
          aria-label="Increment value"
          className="px-3 border-l flex items-center justify-center cursor-pointer disabled:cursor-not-allowed disabled:opacity-45 hover:bg-[var(--accent)] active:scale-95 transition-[background-color,transform] duration-100"
          style={{ backgroundColor: 'var(--muted)', borderColor: 'var(--border)', color: 'var(--foreground)' }}
        >
          <Plus className="w-3.5 h-3.5" aria-hidden="true" />
        </button>
      </div>
    </StageShell>
  );
}

function OtpInputPreview({ activeTheme }: { activeTheme: ThemePreset }) {
  const [code, setCode] = useState('482');
  const [digitsCount, setDigitsCount] = useState<'4' | '6'>('6');
  const [masked, setMasked] = useState(false);
  const [error, setError] = useState(false);
  const [disabled, setDisabled] = useState(false);
  const [focusedIndex, setFocusedIndex] = useState<number | null>(null);
  const slotRefs = useRef<Array<HTMLInputElement | null>>([]);

  const length = Number(digitsCount);
  const normalized = distributeOtpInput(code, length);
  const isComplete = normalized.length === length;

  const setSlotDigit = (idx: number, raw: string) => {
    const incoming = raw.replace(/\D/g, '');
    if (!incoming) {
      const next = normalized.slice(0, idx) + normalized.slice(idx + 1);
      setCode(next);
      return;
    }
    if (incoming.length > 1) {
      const pasted = distributeOtpInput(incoming, length);
      setCode(pasted);
      slotRefs.current[Math.min(length - 1, pasted.length)]?.focus();
      return;
    }
    const chars = normalized.split('');
    chars[idx] = incoming[0];
    const next = chars.join('').slice(0, length);
    setCode(next);
    if (idx + 1 < length) {
      slotRefs.current[idx + 1]?.focus();
    }
  };

  return (
    <StageShell
      title="ui.kit_otp(&mut code, 6)"
      themeName={activeTheme.name}
      onReset={() => {
        setCode('482');
        setDigitsCount('6');
        setMasked(false);
        setError(false);
        setDisabled(false);
      }}
      controls={
        <>
          <ControlField label="Digits">
            <SegmentedButtons options={['4', '6'] as const} value={digitsCount} onChange={setDigitsCount} />
          </ControlField>
          <div className="flex flex-col gap-2 pt-1">
            <ToggleCheckbox label="Masked Dots" checked={masked} onChange={setMasked} />
            <ToggleCheckbox label="Error State" checked={error} onChange={setError} />
            <ToggleCheckbox label="Disabled" checked={disabled} onChange={setDisabled} />
          </div>
          <div className="flex items-center gap-2 pt-1">
            <button
              type="button"
              onClick={() => setCode(distributeOtpInput('482910', length))}
              className="px-2.5 py-1.5 rounded border text-xs font-medium cursor-pointer hover:bg-[var(--accent)]"
              style={{ borderColor: 'var(--border)', color: 'var(--foreground)' }}
            >
              Simulate Paste (482910)
            </button>
          </div>
        </>
      }
    >
      <div className="flex flex-col items-center gap-3">
        <div
          key={error ? 'err' : 'ok'}
          className={`flex items-center gap-2 ${error ? 'animate-egui-shake' : ''}`}
          role="group"
          aria-label="One-time verification code"
        >
          {Array.from({ length }).map((_, idx) => {
            const char = normalized[idx] ?? '';
            const isFocused = focusedIndex === idx && !disabled;
            return (
              <div key={idx} className="relative">
                <input
                  ref={(el) => {
                    slotRefs.current[idx] = el;
                  }}
                  type={masked ? 'password' : 'text'}
                  inputMode="numeric"
                  maxLength={length}
                  value={char}
                  disabled={disabled}
                  aria-label={`Verification digit ${idx + 1}`}
                  onFocus={(e) => {
                    setFocusedIndex(idx);
                    e.target.select();
                  }}
                  onBlur={() => setFocusedIndex(null)}
                  onChange={(e) => setSlotDigit(idx, e.target.value)}
                  onPaste={(e) => {
                    e.preventDefault();
                    const text = e.clipboardData.getData('text');
                    const pasted = distributeOtpInput(text, length);
                    setCode(pasted);
                    slotRefs.current[Math.min(length - 1, pasted.length)]?.focus();
                  }}
                  onKeyDown={(e) => {
                    if (e.key === 'Backspace' && !char && idx > 0) {
                      slotRefs.current[idx - 1]?.focus();
                    } else if (e.key === 'ArrowLeft' && idx > 0) {
                      slotRefs.current[idx - 1]?.focus();
                    } else if (e.key === 'ArrowRight' && idx + 1 < length) {
                      slotRefs.current[idx + 1]?.focus();
                    }
                  }}
                  className="w-11 h-12 rounded-md border text-center font-mono text-lg font-semibold outline-none transition-[border-color,box-shadow,transform] duration-150"
                  style={{
                    backgroundColor: disabled ? 'var(--muted)' : 'var(--background)',
                    color: 'var(--foreground)',
                    borderColor: error
                      ? 'var(--destructive)'
                      : isFocused
                        ? 'var(--ring)'
                        : char
                          ? 'var(--primary)'
                          : 'var(--input)',
                    boxShadow: isFocused
                      ? `0 0 0 3px color-mix(in srgb, ${error ? 'var(--destructive)' : 'var(--ring)'} 22%, transparent)`
                      : 'none',
                    transform: char && !disabled ? 'scale(1.03)' : 'scale(1)',
                  }}
                />
                {isFocused && !char && (
                  <span
                    aria-hidden="true"
                    className="animate-egui-caret pointer-events-none absolute left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2 w-0.5 h-5 rounded-full"
                    style={{ backgroundColor: 'var(--primary)' }}
                  />
                )}
              </div>
            );
          })}
        </div>
        <div
          className="text-xs font-mono"
          style={{
            color: error
              ? 'var(--destructive)'
              : isComplete
                ? 'var(--success)'
                : 'var(--muted-foreground)',
          }}
        >
          {error
            ? 'Code expired or invalid'
            : isComplete
              ? `Ready to verify (${normalized})`
              : `${normalized.length} / ${length} digits entered`}
        </div>
      </div>
    </StageShell>
  );
}

const REGIONS = [
  'us-east-1 (N. Virginia)',
  'eu-central-1 (Frankfurt)',
  'ap-northeast-1 (Tokyo)',
  'us-west-2 (Oregon)',
];

function SelectPreview({ activeTheme }: { activeTheme: ThemePreset }) {
  const [selectedIdx, setSelectedIdx] = useState(0);
  const [open, setOpen] = useState(false);
  const [size, setSize] = useState<SizeOpt>('Md');
  const [disabled, setDisabled] = useState(false);
  const wrapRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (!open) return;
    const onPointerDown = (e: PointerEvent) => {
      if (wrapRef.current && !wrapRef.current.contains(e.target as Node)) setOpen(false);
    };
    const onKeyDown = (e: KeyboardEvent) => {
      if (e.key === 'Escape') setOpen(false);
    };
    window.addEventListener('pointerdown', onPointerDown);
    window.addEventListener('keydown', onKeyDown);
    return () => {
      window.removeEventListener('pointerdown', onPointerDown);
      window.removeEventListener('keydown', onKeyDown);
    };
  }, [open]);

  const metrics = SIZE_METRICS[size];

  return (
    <StageShell
      title="ui.kit_select(&quot;region&quot;, &mut idx, &options)"
      themeName={activeTheme.name}
      onReset={() => {
        setSelectedIdx(0);
        setOpen(false);
        setSize('Md');
        setDisabled(false);
      }}
      controls={
        <>
          <ControlField label="Size">
            <SegmentedButtons options={['Xs', 'Sm', 'Md', 'Lg'] as const} value={size} onChange={setSize} />
          </ControlField>
          <ToggleCheckbox label="Disabled" checked={disabled} onChange={(next) => {
            setDisabled(next);
            if (next) setOpen(false);
          }} />
          <div className="p-3 rounded border text-xs font-mono" style={{ backgroundColor: 'var(--muted)', borderColor: 'var(--border)' }}>
            selected[{selectedIdx}]: {REGIONS[selectedIdx]}
          </div>
        </>
      }
    >
      <div ref={wrapRef} className="relative w-64 mb-28">
        <button
          type="button"
          disabled={disabled}
          aria-haspopup="listbox"
          aria-expanded={open}
          onClick={() => setOpen((o) => !o)}
          className="w-full flex items-center justify-between gap-2 border cursor-pointer disabled:cursor-not-allowed disabled:opacity-60 transition-[border-color,box-shadow] duration-150"
          style={{
            height: metrics.h,
            paddingInline: metrics.px,
            fontSize: metrics.text,
            borderRadius: metrics.radius,
            backgroundColor: disabled ? 'var(--muted)' : 'var(--background)',
            borderColor: open ? 'var(--ring)' : 'var(--input)',
            color: 'var(--foreground)',
            boxShadow: open ? '0 0 0 3px color-mix(in srgb, var(--ring) 20%, transparent)' : 'none',
          }}
        >
          <span className="truncate">{REGIONS[selectedIdx]}</span>
          <ChevronDown
            className="w-4 h-4 shrink-0 transition-transform duration-150 ease-out"
            style={{
              color: 'var(--muted-foreground)',
              transform: open ? 'rotate(180deg)' : 'rotate(0deg)',
            }}
            aria-hidden="true"
          />
        </button>

        {open && (
          <div
            role="listbox"
            aria-label="Deployment region options"
            className="animate-egui-menu-in absolute left-0 right-0 top-full mt-1.5 rounded-md border p-1 z-30 shadow-lg"
            style={{ backgroundColor: 'var(--card)', borderColor: 'var(--border)' }}
          >
            {REGIONS.map((opt, idx) => {
              const active = idx === selectedIdx;
              return (
                <button
                  key={opt}
                  type="button"
                  role="option"
                  aria-selected={active}
                  onClick={() => {
                    setSelectedIdx(idx);
                    setOpen(false);
                  }}
                  className="w-full flex items-center justify-between gap-2 px-2.5 py-1.5 rounded text-xs cursor-pointer transition-colors hover:bg-[var(--muted)]"
                  style={{
                    backgroundColor: active ? 'var(--accent)' : 'transparent',
                    color: 'var(--card-foreground)',
                  }}
                >
                  <span className="truncate">{opt}</span>
                  {active && <Check className="w-3.5 h-3.5 shrink-0 animate-egui-check-draw" style={{ color: 'var(--primary)' }} aria-hidden="true" />}
                </button>
              );
            })}
          </div>
        )}
      </div>
    </StageShell>
  );
}

const CRATE_OPTIONS = [
  { id: 'egui', label: 'egui', meta: 'Immediate-mode GUI' },
  { id: 'eframe', label: 'eframe', meta: 'Cross-platform app runner' },
  { id: 'wgpu', label: 'wgpu', meta: 'Modern WebGPU backend' },
  { id: 'tokio', label: 'tokio', meta: 'Async runtime' },
  { id: 'serde', label: 'serde', meta: 'Serialization framework' },
];

function ComboboxPreview({ activeTheme }: { activeTheme: ThemePreset }) {
  const [selectedId, setSelectedId] = useState<string | null>('egui');
  const [open, setOpen] = useState(false);
  const [query, setQuery] = useState('');
  const [disabled, setDisabled] = useState(false);
  const wrapRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (!open) return;
    const onPointerDown = (e: PointerEvent) => {
      if (wrapRef.current && !wrapRef.current.contains(e.target as Node)) setOpen(false);
    };
    const onKeyDown = (e: KeyboardEvent) => {
      if (e.key === 'Escape') setOpen(false);
    };
    window.addEventListener('pointerdown', onPointerDown);
    window.addEventListener('keydown', onKeyDown);
    return () => {
      window.removeEventListener('pointerdown', onPointerDown);
      window.removeEventListener('keydown', onKeyDown);
    };
  }, [open]);

  const filtered = filterComboboxOptions(CRATE_OPTIONS, query);
  const current = CRATE_OPTIONS.find((o) => o.id === selectedId);

  return (
    <StageShell
      title="ui.kit_combobox(&quot;crate&quot;, &mut selected, &items)"
      themeName={activeTheme.name}
      onReset={() => {
        setSelectedId('egui');
        setOpen(false);
        setQuery('');
        setDisabled(false);
      }}
      controls={
        <>
          <ToggleCheckbox
            label="Disabled"
            checked={disabled}
            onChange={(next) => {
              setDisabled(next);
              if (next) setOpen(false);
            }}
          />
          <button
            type="button"
            onClick={() => {
              setSelectedId(null);
              setQuery('');
            }}
            className="px-3 py-1.5 rounded border text-xs font-medium cursor-pointer w-fit hover:bg-[var(--accent)]"
            style={{ borderColor: 'var(--border)', color: 'var(--foreground)' }}
          >
            Clear Selection
          </button>
          <div className="p-3 rounded border text-xs font-mono" style={{ backgroundColor: 'var(--muted)', borderColor: 'var(--border)' }}>
            selected: {selectedId ? `Some("${selectedId}")` : 'None'}
          </div>
        </>
      }
    >
      <div ref={wrapRef} className="relative w-72 mb-32">
        <button
          type="button"
          disabled={disabled}
          aria-haspopup="listbox"
          aria-expanded={open}
          onClick={() => setOpen((o) => !o)}
          className="w-full h-9 px-3 rounded-md border flex items-center justify-between gap-2 text-sm cursor-pointer disabled:cursor-not-allowed disabled:opacity-60 transition-[border-color,box-shadow] duration-150"
          style={{
            backgroundColor: disabled ? 'var(--muted)' : 'var(--background)',
            borderColor: open ? 'var(--ring)' : 'var(--input)',
            color: current ? 'var(--foreground)' : 'var(--muted-foreground)',
            boxShadow: open ? '0 0 0 3px color-mix(in srgb, var(--ring) 20%, transparent)' : 'none',
          }}
        >
          <span className="truncate">{current ? `${current.label} — ${current.meta}` : 'Search Rust crates...'}</span>
          <ChevronDown
            className="w-4 h-4 shrink-0 transition-transform duration-150"
            style={{
              color: 'var(--muted-foreground)',
              transform: open ? 'rotate(180deg)' : 'rotate(0deg)',
            }}
            aria-hidden="true"
          />
        </button>

        {open && (
          <div
            className="animate-egui-menu-in absolute left-0 right-0 top-full mt-1.5 rounded-md border p-1.5 z-30 shadow-lg space-y-1.5"
            style={{ backgroundColor: 'var(--card)', borderColor: 'var(--border)' }}
          >
            <div className="flex items-center gap-2 px-2 py-1 rounded border" style={{ backgroundColor: 'var(--background)', borderColor: 'var(--border)' }}>
              <Search className="w-3.5 h-3.5 shrink-0" style={{ color: 'var(--muted-foreground)' }} aria-hidden="true" />
              <input
                type="text"
                value={query}
                autoFocus
                placeholder="Filter crate options..."
                aria-label="Filter combobox options"
                onChange={(e) => setQuery(e.target.value)}
                className="w-full bg-transparent text-xs outline-none"
                style={{ color: 'var(--foreground)' }}
              />
              {query ? (
                <button
                  type="button"
                  onClick={() => setQuery('')}
                  aria-label="Clear filter query"
                  className="cursor-pointer"
                  style={{ color: 'var(--muted-foreground)' }}
                >
                  <X className="w-3 h-3" />
                </button>
              ) : (
                <KbdChord shortcut="Esc" macos size="Xs" />
              )}
            </div>

            <div role="listbox" aria-label="Filtered crates" className="max-h-40 overflow-y-auto space-y-0.5">
              {filtered.length === 0 ? (
                <div className="px-2.5 py-3 text-xs text-center" style={{ color: 'var(--muted-foreground)' }}>
                  No matching crates for &quot;{query}&quot;
                </div>
              ) : (
                filtered.map((opt) => {
                  const active = opt.id === selectedId;
                  return (
                    <button
                      key={opt.id}
                      type="button"
                      role="option"
                      aria-selected={active}
                      onClick={() => {
                        setSelectedId(opt.id);
                        setQuery('');
                        setOpen(false);
                      }}
                      className="w-full flex items-center justify-between gap-2 px-2.5 py-1.5 rounded text-xs cursor-pointer transition-colors hover:bg-[var(--muted)]"
                      style={{
                        backgroundColor: active ? 'var(--accent)' : 'transparent',
                        color: 'var(--card-foreground)',
                      }}
                    >
                      <span className="truncate">
                        <strong className="font-mono">{opt.label}</strong>
                        <span className="ml-1.5" style={{ color: 'var(--muted-foreground)' }}>
                          {opt.meta}
                        </span>
                      </span>
                      {active && <Check className="w-3.5 h-3.5 shrink-0 animate-egui-check-draw" style={{ color: 'var(--primary)' }} aria-hidden="true" />}
                    </button>
                  );
                })
              )}
            </div>
          </div>
        )}
      </div>
    </StageShell>
  );
}

function FormPreview({ activeTheme }: { activeTheme: ThemePreset }) {
  const [crateName, setCrateName] = useState('egui-kit');
  const [email, setEmail] = useState('maintainer@egui.rs');
  const [orientation, setOrientation] = useState<'Vertical' | 'Horizontal'>('Vertical');
  const [errors, setErrors] = useState<FormValidationErrors>({});
  const [submitted, setSubmitted] = useState(false);

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    const nextErrors = validateFormFields({ crateName, email });
    setErrors(nextErrors);
    setSubmitted(Object.keys(nextErrors).length === 0);
  };

  return (
    <StageShell
      title="Form::new(&quot;publish&quot;).show(ui, |ui| ...)"
      themeName={activeTheme.name}
      onReset={() => {
        setCrateName('egui-kit');
        setEmail('maintainer@egui.rs');
        setOrientation('Vertical');
        setErrors({});
        setSubmitted(false);
      }}
      controls={
        <>
          <ControlField label="Field Layout">
            <SegmentedButtons options={['Vertical', 'Horizontal'] as const} value={orientation} onChange={setOrientation} />
          </ControlField>
          <button
            type="button"
            onClick={() => {
              setCrateName('');
              setEmail('bad-email');
              setErrors(validateFormFields({ crateName: '', email: 'bad-email' }));
              setSubmitted(false);
            }}
            className="px-3 py-1.5 rounded border text-xs font-medium cursor-pointer w-fit hover:bg-[var(--accent)]"
            style={{ borderColor: 'var(--border)', color: 'var(--foreground)' }}
          >
            Simulate Validation Errors
          </button>
        </>
      }
    >
      <form onSubmit={handleSubmit} className="w-full max-w-md space-y-4">
        <div className={orientation === 'Horizontal' ? 'grid grid-cols-1 sm:grid-cols-[120px_1fr] items-start gap-2' : 'space-y-1.5'}>
          <label htmlFor="form-crate-name" className="text-xs font-semibold pt-2" style={{ color: 'var(--foreground)' }}>
            Crate Name <span style={{ color: 'var(--destructive)' }}>*</span>
          </label>
          <div className="space-y-1">
            <input
              id="form-crate-name"
              type="text"
              value={crateName}
              onChange={(e) => {
                setCrateName(e.target.value);
                setSubmitted(false);
              }}
              aria-invalid={Boolean(errors.crateName)}
              className="w-full h-9 px-3 rounded-md border text-sm outline-none"
              style={{
                backgroundColor: 'var(--background)',
                color: 'var(--foreground)',
                borderColor: errors.crateName ? 'var(--destructive)' : 'var(--input)',
              }}
            />
            {errors.crateName ? (
              <p className="text-xs animate-egui-menu-in" style={{ color: 'var(--destructive)' }}>
                {errors.crateName}
              </p>
            ) : (
              <p className="text-xs" style={{ color: 'var(--muted-foreground)' }}>
                Lowercase identifier registered on crates.io
              </p>
            )}
          </div>
        </div>

        <div className={orientation === 'Horizontal' ? 'grid grid-cols-1 sm:grid-cols-[120px_1fr] items-start gap-2' : 'space-y-1.5'}>
          <label htmlFor="form-email" className="text-xs font-semibold pt-2" style={{ color: 'var(--foreground)' }}>
            Maintainer <span style={{ color: 'var(--destructive)' }}>*</span>
          </label>
          <div className="space-y-1">
            <input
              id="form-email"
              type="text"
              value={email}
              onChange={(e) => {
                setEmail(e.target.value);
                setSubmitted(false);
              }}
              aria-invalid={Boolean(errors.email)}
              className="w-full h-9 px-3 rounded-md border text-sm outline-none"
              style={{
                backgroundColor: 'var(--background)',
                color: 'var(--foreground)',
                borderColor: errors.email ? 'var(--destructive)' : 'var(--input)',
              }}
            />
            {errors.email && (
              <p className="text-xs animate-egui-menu-in" style={{ color: 'var(--destructive)' }}>
                {errors.email}
              </p>
            )}
          </div>
        </div>

        <div className="flex items-center justify-between gap-3 pt-1">
          <button
            type="submit"
            className="px-4 py-2 rounded-md text-xs font-semibold cursor-pointer transition-opacity hover:opacity-90"
            style={{ backgroundColor: 'var(--primary)', color: 'var(--primary-foreground)' }}
          >
            Validate &amp; Submit
          </button>
          {submitted && (
            <span className="text-xs font-medium inline-flex items-center gap-1 animate-egui-menu-in" style={{ color: 'var(--success)' }}>
              <Check className="w-3.5 h-3.5" aria-hidden="true" />
              <span>Ready to publish {crateName}</span>
            </span>
          )}
        </div>
      </form>
    </StageShell>
  );
}

const SURVEY_QUESTIONS = [
  {
    prompt: 'Primary target environment?',
    subtitle: 'Determines default DPI scaling and surface chrome.',
    options: ['Native Desktop (eframe)', 'WebAssembly (Canvas)', 'Both Desktop & Web'],
  },
  {
    prompt: 'Preferred graphics backend?',
    subtitle: 'Both backends render egui-kit vector painters cleanly.',
    options: ['wgpu (WebGPU / Vulkan / Metal)', 'glow (OpenGL / WebGL2)'],
  },
  {
    prompt: 'Default workspace palette?',
    subtitle: 'Synced automatically via set_theme(ctx, ThemeId).',
    options: ['Dark Theme (Tokyo Night)', 'Light Theme (Aurora Light)'],
  },
];

function QuestionnairePreview({ activeTheme }: { activeTheme: ThemePreset }) {
  const [step, setStep] = useState(0);
  const [answers, setAnswers] = useState<Array<number | null>>([0, null, null]);
  const [completed, setCompleted] = useState(false);

  const answeredCount = answers.filter((a) => a !== null).length;
  const pct = Math.round((answeredCount / SURVEY_QUESTIONS.length) * 100);
  const currentQ = SURVEY_QUESTIONS[step];

  return (
    <StageShell
      title="Questionnaire::new(&quot;setup&quot;, &questions, &mut answers)"
      themeName={activeTheme.name}
      onReset={() => {
        setStep(0);
        setAnswers([0, null, null]);
        setCompleted(false);
      }}
      controls={
        <>
          <div className="p-3 rounded border text-xs font-mono tabular-nums space-y-1" style={{ backgroundColor: 'var(--muted)', borderColor: 'var(--border)' }}>
            <div>step: {step + 1} / {SURVEY_QUESTIONS.length}</div>
            <div>answered: {answeredCount} / {SURVEY_QUESTIONS.length} ({pct}%)</div>
          </div>
          <p className="text-xs leading-relaxed" style={{ color: 'var(--muted-foreground)' }}>
            Binds directly to <code className="font-mono">&amp;mut Vec&lt;Option&lt;usize&gt;&gt;</code> with animated progress and step navigation.
          </p>
        </>
      }
    >
      <div
        className="w-full max-w-md rounded-lg border p-5 space-y-4"
        style={{ backgroundColor: 'var(--card)', borderColor: 'var(--border)' }}
      >
        <div className="space-y-2">
          <div className="flex items-center justify-between text-xs font-mono tabular-nums">
            <span style={{ color: 'var(--muted-foreground)' }}>
              {completed ? 'Survey Complete' : `Question ${step + 1} of ${SURVEY_QUESTIONS.length}`}
            </span>
            <span style={{ color: 'var(--primary)' }}>{answeredCount}/{SURVEY_QUESTIONS.length} answered</span>
          </div>
          <div className="w-full h-2 rounded-full overflow-hidden" style={{ backgroundColor: 'var(--muted)' }}>
            <div
              className="h-full rounded-full transition-[width] duration-200 ease-out"
              style={{ width: `${pct}%`, backgroundColor: 'var(--primary)' }}
            />
          </div>
        </div>

        {completed ? (
          <div className="space-y-3 py-2 animate-egui-menu-in">
            <div className="text-sm font-semibold" style={{ color: 'var(--card-foreground)' }}>
              Crate Configuration Summary
            </div>
            <ul className="space-y-1.5 text-xs">
              {SURVEY_QUESTIONS.map((q, idx) => (
                <li key={q.prompt} className="flex justify-between gap-2 py-1 border-b last:border-b-0" style={{ borderColor: 'var(--border)' }}>
                  <span style={{ color: 'var(--muted-foreground)' }}>{q.prompt}</span>
                  <span className="font-medium text-right" style={{ color: 'var(--foreground)' }}>
                    {answers[idx] !== null ? q.options[answers[idx] as number] : 'Skipped'}
                  </span>
                </li>
              ))}
            </ul>
            <button
              type="button"
              onClick={() => {
                setCompleted(false);
                setStep(0);
              }}
              className="px-3 py-1.5 rounded border text-xs font-medium cursor-pointer"
              style={{ borderColor: 'var(--border)', color: 'var(--foreground)' }}
            >
              Review Answers
            </button>
          </div>
        ) : (
          <div key={step} className="space-y-4 animate-egui-menu-in">
            <div className="space-y-1">
              <div className="text-base font-semibold" style={{ color: 'var(--card-foreground)' }}>
                {currentQ.prompt}
              </div>
              <div className="text-xs" style={{ color: 'var(--muted-foreground)' }}>
                {currentQ.subtitle}
              </div>
            </div>

            <div className="space-y-2">
              {currentQ.options.map((opt, idx) => {
                const selected = answers[step] === idx;
                return (
                  <button
                    key={opt}
                    type="button"
                    onClick={() => {
                      const next = [...answers];
                      next[step] = idx;
                      setAnswers(next);
                    }}
                    className="w-full px-3.5 py-2.5 rounded-md border flex items-center justify-between gap-2 text-xs font-medium cursor-pointer transition-[background-color,border-color,transform] duration-150 active:scale-[0.99]"
                    style={{
                      backgroundColor: selected ? 'var(--accent)' : 'var(--background)',
                      borderColor: selected ? 'var(--primary)' : 'var(--border)',
                      color: 'var(--foreground)',
                    }}
                  >
                    <span>{opt}</span>
                    {selected && <Check className="w-3.5 h-3.5 shrink-0 animate-egui-check-draw" style={{ color: 'var(--primary)' }} aria-hidden="true" />}
                  </button>
                );
              })}
            </div>

            <div className="flex items-center justify-between pt-2">
              <button
                type="button"
                disabled={step === 0}
                onClick={() => setStep((s) => Math.max(0, s - 1))}
                className="px-3 py-1.5 rounded border text-xs font-medium cursor-pointer disabled:opacity-45 disabled:cursor-not-allowed active:scale-[0.97] transition-transform"
                style={{ borderColor: 'var(--border)', color: 'var(--foreground)' }}
              >
                Back
              </button>
              <div className="flex items-center gap-1.5" aria-hidden="true">
                {SURVEY_QUESTIONS.map((_, idx) => (
                  <span
                    key={idx}
                    className="h-1.5 rounded-full transition-all duration-200"
                    style={{
                      width: idx === step ? '16px' : '6px',
                      backgroundColor: idx === step ? 'var(--primary)' : answers[idx] !== null ? 'color-mix(in srgb, var(--primary) 45%, transparent)' : 'var(--border)',
                    }}
                  />
                ))}
              </div>
              {step + 1 < SURVEY_QUESTIONS.length ? (
                <button
                  type="button"
                  onClick={() => setStep((s) => s + 1)}
                  className="px-3.5 py-1.5 rounded text-xs font-semibold cursor-pointer active:scale-[0.97] transition-transform"
                  style={{ backgroundColor: 'var(--primary)', color: 'var(--primary-foreground)' }}
                >
                  Next
                </button>
              ) : (
                <button
                  type="button"
                  onClick={() => setCompleted(true)}
                  className="px-3.5 py-1.5 rounded text-xs font-semibold cursor-pointer active:scale-[0.97] transition-transform"
                  style={{ backgroundColor: 'var(--primary)', color: 'var(--primary-foreground)' }}
                >
                  Finish
                </button>
              )}
            </div>
          </div>
        )}
      </div>
    </StageShell>
  );
}

const SNIPPETS = [
  'cargo add egui-kit',
  'ui.kit_input(&mut query, "Filter...");',
  'Root::show(ctx, |ui| { ui.kit_button("Run"); });',
];

function ClipboardPreview({ activeTheme }: { activeTheme: ThemePreset }) {
  const [snippet, setSnippet] = useState(SNIPPETS[0]);
  const [size, setSize] = useState<SizeOpt>('Md');
  const [showText, setShowText] = useState(true);
  const [copied, setCopied] = useState(false);
  const timerRef = useRef<number | null>(null);

  useEffect(() => {
    return () => {
      if (timerRef.current !== null) window.clearTimeout(timerRef.current);
    };
  }, []);

  const triggerCopy = async () => {
    try {
      await navigator.clipboard?.writeText(snippet);
    } catch {
      // Clipboard API may be blocked in headless frames; UI feedback timer still fires cleanly
    }
    setCopied(true);
    if (timerRef.current !== null) window.clearTimeout(timerRef.current);
    timerRef.current = window.setTimeout(() => setCopied(false), 1600);
  };

  const metrics = SIZE_METRICS[size];

  return (
    <StageShell
      title="ui.kit_clipboard(&quot;install&quot;, &snippet)"
      themeName={activeTheme.name}
      onReset={() => {
        setSnippet(SNIPPETS[0]);
        setSize('Md');
        setShowText(true);
        setCopied(false);
      }}
      controls={
        <>
          <ControlField label="Size">
            <SegmentedButtons options={['Xs', 'Sm', 'Md', 'Lg'] as const} value={size} onChange={setSize} />
          </ControlField>
          <ControlField label="Payload Snippet">
            <div className="space-y-1.5">
              {SNIPPETS.map((s) => (
                <button
                  key={s}
                  type="button"
                  onClick={() => {
                    setSnippet(s);
                    setCopied(false);
                  }}
                  className="w-full text-left px-2.5 py-1.5 rounded border text-[11px] font-mono truncate cursor-pointer"
                  style={{
                    backgroundColor: snippet === s ? 'var(--accent)' : 'var(--background)',
                    borderColor: snippet === s ? 'var(--primary)' : 'var(--border)',
                    color: 'var(--foreground)',
                  }}
                >
                  {s}
                </button>
              ))}
            </div>
          </ControlField>
          <ToggleCheckbox label="Show Button Label" checked={showText} onChange={setShowText} />
        </>
      }
    >
      <div
        className="flex items-center gap-3 px-3.5 py-2.5 rounded-lg border max-w-full shadow-xs transition-[border-color,box-shadow] duration-200"
        style={{
          backgroundColor: 'var(--card)',
          borderColor: copied ? 'var(--success)' : 'var(--border)',
          boxShadow: copied ? '0 0 0 3px color-mix(in srgb, var(--success) 18%, transparent)' : 'none',
        }}
      >
        <code className="text-xs font-mono truncate max-w-[260px]" style={{ color: 'var(--foreground)' }}>
          {snippet}
        </code>
        <button
          type="button"
          onClick={triggerCopy}
          aria-label={copied ? 'Copied to clipboard' : 'Copy snippet to clipboard'}
          className="inline-flex items-center gap-1.5 border font-medium cursor-pointer transition-[background-color,border-color,color,transform] duration-150 active:scale-95 shrink-0"
          style={{
            height: metrics.h,
            paddingInline: showText ? metrics.px : '10px',
            fontSize: metrics.text,
            borderRadius: metrics.radius,
            backgroundColor: copied
              ? 'color-mix(in srgb, var(--success) 16%, var(--card))'
              : 'var(--background)',
            borderColor: copied ? 'var(--success)' : 'var(--border)',
            color: copied ? 'var(--success)' : 'var(--foreground)',
          }}
        >
          {copied ? (
            <Check key="copied" className="w-3.5 h-3.5 animate-egui-pop shrink-0" aria-hidden="true" />
          ) : (
            <Copy key="copy" className="w-3.5 h-3.5 animate-egui-fade-in shrink-0" aria-hidden="true" />
          )}
          {showText && <span>{copied ? 'Copied!' : 'Copy'}</span>}
        </button>
      </div>
    </StageShell>
  );
}
