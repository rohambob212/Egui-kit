import { useEffect, useLayoutEffect, useRef, useState } from 'react';
import {
  AlertTriangle,
  Check,
  ChevronLeft,
  ChevronRight,
  Plus,
  RotateCcw,
  X,
} from 'lucide-react';
import type { ThemePreset } from '../types/kit';
import {
  clampSplitRatio,
  computePaginationItems,
  getNextStepIndex,
  stepAccordionIndex,
} from '../utils/catalogLogic';
import {
  ControlField,
  SegmentedButtons,
  StageShell,
  ToggleCheckbox,
} from './Part1Previews';

interface Part5PreviewProps {
  slug: string;
  activeTheme: ThemePreset;
}

export function Part5Preview({ slug, activeTheme }: Part5PreviewProps) {
  switch (slug) {
    case 'group-box':
      return <GroupBoxPreview activeTheme={activeTheme} />;
    case 'description-list':
      return <DescriptionListPreview activeTheme={activeTheme} />;
    case 'grid-layout':
      return <GridLayoutPreview activeTheme={activeTheme} />;
    case 'scrollable':
      return <ScrollablePreview activeTheme={activeTheme} />;
    case 'resizable':
      return <ResizablePreview activeTheme={activeTheme} />;
    case 'accordion':
      return <AccordionPreview activeTheme={activeTheme} />;
    case 'collapsible':
      return <CollapsiblePreview activeTheme={activeTheme} />;
    case 'tabs':
      return <TabsPreview activeTheme={activeTheme} />;
    case 'stepper':
      return <StepperPreview activeTheme={activeTheme} />;
    case 'pagination':
      return <PaginationPreview activeTheme={activeTheme} />;
    default:
      return null;
  }
}

function GroupBoxPreview({ activeTheme }: { activeTheme: ThemePreset }) {
  const [surface, setSurface] = useState<'Card' | 'Muted'>('Card');
  const [bordered, setBordered] = useState(true);
  const [showSubtitle, setShowSubtitle] = useState(true);
  const [compact, setCompact] = useState(false);
  const [thinLto, setThinLto] = useState(true);
  const [stripSymbols, setStripSymbols] = useState(false);

  return (
    <StageShell
      title='ui.kit_group_box("Compiler Options", |ui| ...)'
      themeName={activeTheme.name}
      onReset={() => {
        setSurface('Card');
        setBordered(true);
        setShowSubtitle(true);
        setCompact(false);
        setThinLto(true);
        setStripSymbols(false);
      }}
      controls={
        <>
          <ControlField label="Surface">
            <SegmentedButtons options={['Card', 'Muted'] as const} value={surface} onChange={setSurface} />
          </ControlField>
          <ToggleCheckbox checked={bordered} onChange={setBordered} label="Show Hairline Border" />
          <ToggleCheckbox checked={showSubtitle} onChange={setShowSubtitle} label="Show Subtitle" />
          <ToggleCheckbox checked={compact} onChange={setCompact} label="Compact Padding" />
        </>
      }
    >
      <div
        className={`w-full max-w-sm rounded-xl shadow-xs transition-all duration-150 ${
          compact ? 'p-3 space-y-2.5' : 'p-4 space-y-3'
        }`}
        style={{
          backgroundColor: surface === 'Muted' ? 'var(--muted)' : 'var(--card)',
          border: bordered ? '1px solid var(--border)' : '1px solid transparent',
        }}
      >
        <div className={`${compact ? 'pb-2' : 'pb-2.5'} border-b`} style={{ borderColor: 'var(--border)' }}>
          <div
            className="text-sm font-semibold"
            style={{ color: surface === 'Muted' ? 'var(--foreground)' : 'var(--card-foreground)' }}
          >
            Compiler Options
          </div>
          {showSubtitle && (
            <p className="text-xs mt-0.5" style={{ color: 'var(--muted-foreground)' }}>
              Release build optimizations for egui-kit targets
            </p>
          )}
        </div>
        <div className="flex flex-col gap-2.5">
          <ToggleCheckbox checked={thinLto} onChange={setThinLto} label="Enable Thin LTO" />
          <ToggleCheckbox checked={stripSymbols} onChange={setStripSymbols} label="Strip Debug Symbols" />
        </div>
      </div>
    </StageShell>
  );
}

function DescriptionListPreview({ activeTheme }: { activeTheme: ThemePreset }) {
  const [columns, setColumns] = useState<'1' | '2' | '3'>('2');
  const [bordered, setBordered] = useState(true);
  const [compact, setCompact] = useState(false);

  const pairs = [
    { label: 'Crate', value: 'egui-kit 0.1.0' },
    { label: 'Runtime', value: 'egui 0.30 / eframe' },
    { label: 'Target Triple', value: 'wasm32-unknown' },
    { label: 'Active Theme', value: activeTheme.name },
    { label: 'Part Status', value: '60 / 80 ready' },
    { label: 'License', value: 'Apache-2.0' },
  ];

  const gridColsClass =
    columns === '1' ? 'grid-cols-1' : columns === '2' ? 'grid-cols-2' : 'grid-cols-3';

  return (
    <StageShell
      title="DescriptionList::new(&pairs).columns(...)"
      themeName={activeTheme.name}
      onReset={() => {
        setColumns('2');
        setBordered(true);
        setCompact(false);
      }}
      controls={
        <>
          <ControlField label="Columns">
            <SegmentedButtons options={['1', '2', '3'] as const} value={columns} onChange={setColumns} />
          </ControlField>
          <ToggleCheckbox checked={bordered} onChange={setBordered} label="Bordered Cells" />
          <ToggleCheckbox checked={compact} onChange={setCompact} label="Compact Padding" />
        </>
      }
    >
      <dl
        className={`w-full max-w-lg grid ${gridColsClass} rounded-xl overflow-hidden shadow-xs`}
        style={{
          backgroundColor: 'var(--card)',
          border: bordered ? '1px solid var(--border)' : 'none',
        }}
      >
        {pairs.map((item) => (
          <div
            key={item.label}
            className={`${compact ? 'px-3 py-2' : 'px-3.5 py-2.5'} transition-colors duration-150 hover:bg-[var(--muted)]/55`}
            style={{
              borderBottom: bordered ? '1px solid var(--border)' : 'none',
              borderRight: bordered ? '1px solid var(--border)' : 'none',
            }}
          >
            <dt className="text-[11px] font-medium" style={{ color: 'var(--muted-foreground)' }}>
              {item.label}
            </dt>
            <dd className="text-xs font-mono tabular-nums mt-0.5 truncate" style={{ color: 'var(--foreground)' }}>
              {item.value}
            </dd>
          </div>
        ))}
      </dl>
    </StageShell>
  );
}

function GridLayoutPreview({ activeTheme }: { activeTheme: ThemePreset }) {
  const [cols, setCols] = useState<'2' | '3' | '4'>('3');
  const [gap, setGap] = useState<'8px' | '12px' | '16px'>('12px');
  const [spanWide, setSpanWide] = useState(true);

  const colCount = Number(cols);
  const items = [
    { id: 'Track 1', span: 1 },
    { id: 'Track 2 (Wide)', span: spanWide ? Math.min(2, colCount) : 1 },
    { id: 'Track 3', span: 1 },
    { id: 'Track 4', span: 1 },
    { id: 'Track 5', span: spanWide ? Math.min(colCount, 2) : 1 },
  ];

  return (
    <StageShell
      title="GridLayout::columns(...).gap(...)"
      themeName={activeTheme.name}
      onReset={() => {
        setCols('3');
        setGap('12px');
        setSpanWide(true);
      }}
      controls={
        <>
          <ControlField label="Column Tracks">
            <SegmentedButtons options={['2', '3', '4'] as const} value={cols} onChange={setCols} />
          </ControlField>
          <ControlField label="Track Gap">
            <SegmentedButtons options={['8px', '12px', '16px'] as const} value={gap} onChange={setGap} />
          </ControlField>
          <ToggleCheckbox checked={spanWide} onChange={setSpanWide} label="Enable Multi-Column Spans" />
        </>
      }
    >
      <div
        className="w-full max-w-md rounded-xl border border-dashed p-3"
        style={{
          borderColor: 'var(--border)',
          backgroundImage: 'radial-gradient(color-mix(in srgb, var(--muted-foreground) 22%, transparent) 1px, transparent 1px)',
          backgroundSize: '14px 14px',
        }}
      >
        <div
          style={{
            display: 'grid',
            gridTemplateColumns: `repeat(${colCount}, minmax(0, 1fr))`,
            gap,
          }}
        >
          {items.map((item) => (
            <div
              key={item.id}
              className="rounded-lg border px-3 py-3 text-center transition-[transform,border-color,background-color] duration-150 hover:-translate-y-0.5 shadow-xs"
              style={{
                gridColumn: `span ${item.span} / span ${item.span}`,
                backgroundColor: item.span > 1 ? 'color-mix(in srgb, var(--primary) 12%, var(--card))' : 'var(--card)',
                borderColor: item.span > 1 ? 'var(--primary)' : 'var(--border)',
              }}
            >
              <div className="text-xs font-semibold" style={{ color: 'var(--foreground)' }}>
                {item.id}
              </div>
              <div className="text-[11px] font-mono mt-0.5" style={{ color: 'var(--muted-foreground)' }}>
                span {item.span}
              </div>
            </div>
          ))}
        </div>
      </div>
    </StageShell>
  );
}

function ScrollablePreview({ activeTheme }: { activeTheme: ThemePreset }) {
  const [logs, setLogs] = useState<string[]>(() =>
    Array.from({ length: 14 }, (_, idx) => `[${String(idx + 1).padStart(2, '0')}] Compiled egui_kit::part5 module`)
  );
  const [stickBottom, setStickBottom] = useState(true);
  const [bordered, setBordered] = useState(true);
  const viewportRef = useRef<HTMLDivElement>(null);

  const appendLog = () => {
    setLogs((prev) => {
      const next = [...prev, `[${String(prev.length + 1).padStart(2, '0')}] Frame synced (${activeTheme.name})`];
      if (stickBottom) {
        requestAnimationFrame(() => {
          if (viewportRef.current) {
            viewportRef.current.scrollTop = viewportRef.current.scrollHeight;
          }
        });
      }
      return next;
    });
  };

  return (
    <StageShell
      title='ui.kit_scrollable("log", 200.0, |ui| ...)'
      themeName={activeTheme.name}
      onReset={() => {
        setLogs(
          Array.from({ length: 14 }, (_, idx) => `[${String(idx + 1).padStart(2, '0')}] Compiled egui_kit::part5 module`)
        );
        setStickBottom(true);
        setBordered(true);
      }}
      controls={
        <>
          <ToggleCheckbox checked={stickBottom} onChange={setStickBottom} label="Stick to Bottom on Append" />
          <ToggleCheckbox checked={bordered} onChange={setBordered} label="Outer Border" />
          <button
            type="button"
            onClick={appendLog}
            className="inline-flex items-center justify-center gap-1.5 px-3 py-2 rounded-md text-xs font-semibold cursor-pointer"
            style={{ backgroundColor: 'var(--primary)', color: 'var(--primary-foreground)' }}
          >
            <Plus className="w-3.5 h-3.5" aria-hidden="true" />
            <span>Append Log Line</span>
          </button>
        </>
      }
    >
      <div
        ref={viewportRef}
        tabIndex={0}
        aria-label="Scrollable log feed"
        className="egui-scroll w-full max-w-md h-48 overflow-y-auto rounded-xl p-3 space-y-1.5 shadow-xs"
        style={{
          backgroundColor: 'var(--card)',
          border: bordered ? '1px solid var(--border)' : 'none',
        }}
      >
        {logs.map((line, i) => (
          <div
            key={`${i}-${line}`}
            className={`text-xs font-mono px-2.5 py-1.5 rounded-md ${
              i === logs.length - 1 && logs.length > 14 ? 'animate-egui-menu-in' : ''
            }`}
            style={{
              backgroundColor: i === logs.length - 1 ? 'color-mix(in srgb, var(--primary) 12%, var(--muted))' : 'var(--muted)',
              color: 'var(--foreground)',
            }}
          >
            {line}
          </div>
        ))}
      </div>
    </StageShell>
  );
}

function ResizablePreview({ activeTheme }: { activeTheme: ThemePreset }) {
  const [orientation, setOrientation] = useState<'Horizontal' | 'Vertical'>('Horizontal');
  const [ratio, setRatio] = useState(0.42);
  const [dragging, setDragging] = useState(false);
  const boxRef = useRef<HTMLDivElement>(null);

  const updateFromPointer = (clientX: number, clientY: number) => {
    const el = boxRef.current;
    if (!el) return;
    const rect = el.getBoundingClientRect();
    const raw =
      orientation === 'Horizontal'
        ? (clientX - rect.left) / Math.max(rect.width, 1)
        : (clientY - rect.top) / Math.max(rect.height, 1);
    setRatio(clampSplitRatio(raw, 0.2, 0.8));
  };

  const pct = Math.round(ratio * 100);

  return (
    <StageShell
      title='Resizable::horizontal("split").ratio(&mut ratio)'
      themeName={activeTheme.name}
      onReset={() => {
        setOrientation('Horizontal');
        setRatio(0.42);
      }}
      controls={
        <>
          <ControlField label="Orientation">
            <SegmentedButtons options={['Horizontal', 'Vertical'] as const} value={orientation} onChange={setOrientation} />
          </ControlField>
          <ControlField label="Split Ratio">
            <div
              className="px-3 py-2 rounded border text-xs font-mono tabular-nums"
              style={{ backgroundColor: 'var(--muted)', borderColor: 'var(--border)', color: 'var(--foreground)' }}
            >
              {pct}% / {100 - pct}%
            </div>
          </ControlField>
        </>
      }
    >
      <div
        ref={boxRef}
        className={`w-full max-w-md h-52 rounded-lg border overflow-hidden flex ${
          orientation === 'Vertical' ? 'flex-col' : 'flex-row'
        }`}
        style={{ backgroundColor: 'var(--card)', borderColor: 'var(--border)' }}
      >
        <div
          className="p-3 overflow-hidden flex flex-col justify-between"
          style={
            orientation === 'Horizontal'
              ? { width: `${ratio * 100}%` }
              : { height: `${ratio * 100}%` }
          }
        >
          <div className="text-xs font-semibold" style={{ color: 'var(--card-foreground)' }}>
            {orientation === 'Horizontal' ? 'Explorer' : 'Editor'}
          </div>
          <span className="text-[11px] font-mono tabular-nums" style={{ color: 'var(--muted-foreground)' }}>
            {pct}% pane
          </span>
        </div>

        <div
          role="separator"
          tabIndex={0}
          aria-orientation={orientation === 'Horizontal' ? 'vertical' : 'horizontal'}
          aria-label="Resize split divider"
          aria-valuemin={20}
          aria-valuemax={80}
          aria-valuenow={pct}
          onDoubleClick={() => setRatio(0.42)}
          onKeyDown={(e) => {
            if (e.key === 'ArrowRight' || e.key === 'ArrowDown') {
              e.preventDefault();
              setRatio((r) => clampSplitRatio(r + 0.05, 0.2, 0.8));
            } else if (e.key === 'ArrowLeft' || e.key === 'ArrowUp') {
              e.preventDefault();
              setRatio((r) => clampSplitRatio(r - 0.05, 0.2, 0.8));
            }
          }}
          onPointerDown={(e) => {
            setDragging(true);
            e.currentTarget.setPointerCapture(e.pointerId);
            updateFromPointer(e.clientX, e.clientY);
          }}
          onPointerMove={(e) => {
            if (dragging) updateFromPointer(e.clientX, e.clientY);
          }}
          onPointerUp={(e) => {
            setDragging(false);
            if (e.currentTarget.hasPointerCapture(e.pointerId)) {
              e.currentTarget.releasePointerCapture(e.pointerId);
            }
          }}
          onPointerCancel={() => setDragging(false)}
          className={`group shrink-0 flex items-center justify-center touch-none select-none transition-colors focus:outline-none focus-visible:ring-2 focus-visible:ring-[var(--ring)] ${
            orientation === 'Horizontal' ? 'w-3.5 cursor-col-resize border-x' : 'h-3.5 cursor-row-resize border-y'
          }`}
          style={{
            borderColor: dragging ? 'var(--primary)' : 'var(--border)',
            backgroundColor: dragging ? 'color-mix(in srgb, var(--primary) 16%, var(--muted))' : 'var(--muted)',
          }}
        >
          <span
            className={`flex items-center justify-center gap-0.5 ${
              orientation === 'Horizontal' ? 'flex-col' : 'flex-row'
            }`}
          >
            {[0, 1, 2].map((dot) => (
              <span
                key={dot}
                className="w-1 h-1 rounded-full transition-transform duration-150 group-hover:scale-125"
                style={{ backgroundColor: dragging ? 'var(--primary)' : 'var(--muted-foreground)' }}
              />
            ))}
          </span>
        </div>

        <div className="flex-1 p-3 overflow-hidden flex flex-col justify-between">
          <div className="text-xs font-semibold" style={{ color: 'var(--card-foreground)' }}>
            {orientation === 'Horizontal' ? 'Preview Stage' : 'Output Console'}
          </div>
          <span className="text-[11px] font-mono tabular-nums" style={{ color: 'var(--muted-foreground)' }}>
            {100 - pct}% pane
          </span>
        </div>
      </div>
    </StageShell>
  );
}

function AccordionPreview({ activeTheme }: { activeTheme: ThemePreset }) {
  const [openIdx, setOpenIdx] = useState<number | null>(0);
  const [allowCollapseAll, setAllowCollapseAll] = useState(true);

  const items = [
    {
      title: 'Zero-dependency vector Painter',
      body: 'Every chevron, divider, and indicator in egui-kit draws directly via egui::Painter without external font icon packs.',
    },
    {
      title: 'Immediate-mode &mut state',
      body: 'Accordion binds cleanly to &mut Option<usize> and animates section expansion at 60fps with animate_bool_with_time().',
    },
    {
      title: 'Synchronized 36 themes',
      body: 'Switching active ThemeId updates card fills, hairline borders, and active title accents across the entire workspace.',
    },
  ];

  return (
    <StageShell
      title='ui.kit_accordion("faq", &mut open_idx, &items)'
      themeName={activeTheme.name}
      onReset={() => {
        setOpenIdx(0);
        setAllowCollapseAll(true);
      }}
      controls={
        <>
          <ToggleCheckbox
            checked={allowCollapseAll}
            onChange={(next) => {
              setAllowCollapseAll(next);
              if (!next && openIdx === null) setOpenIdx(0);
            }}
            label="Allow Collapse All"
          />
          <ControlField label="Bound open_idx">
            <div
              className="px-3 py-2 rounded border text-xs font-mono"
              style={{ backgroundColor: 'var(--muted)', borderColor: 'var(--border)', color: 'var(--foreground)' }}
            >
              {openIdx === null ? 'None' : `Some(${openIdx})`}
            </div>
          </ControlField>
        </>
      }
    >
      <div
        className="w-full max-w-md rounded-xl border overflow-hidden divide-y shadow-xs"
        style={{ backgroundColor: 'var(--card)', borderColor: 'var(--border)' }}
      >
        {items.map((item, idx) => {
          const isOpen = openIdx === idx;
          return (
            <div key={item.title} style={{ borderColor: 'var(--border)' }}>
              <button
                type="button"
                aria-expanded={isOpen}
                onClick={() => setOpenIdx((prev) => stepAccordionIndex(prev, idx, allowCollapseAll))}
                className="w-full px-4 py-3 flex items-center justify-between gap-3 text-left cursor-pointer transition-colors hover:bg-[var(--muted)]"
              >
                <span
                  className="text-xs font-semibold transition-colors duration-150"
                  style={{ color: isOpen ? 'var(--primary)' : 'var(--card-foreground)' }}
                >
                  {item.title}
                </span>
                <ChevronRight
                  className={`w-4 h-4 shrink-0 transition-transform duration-200 ${isOpen ? 'rotate-90' : ''}`}
                  style={{ color: isOpen ? 'var(--primary)' : 'var(--muted-foreground)' }}
                  aria-hidden="true"
                />
              </button>
              <div
                className="grid transition-[grid-template-rows,opacity] duration-200 ease-out"
                style={{
                  gridTemplateRows: isOpen ? '1fr' : '0fr',
                  opacity: isOpen ? 1 : 0,
                }}
              >
                <div className="overflow-hidden">
                  <div className="px-4 pb-3.5 text-xs leading-relaxed" style={{ color: 'var(--muted-foreground)' }}>
                    {item.body}
                  </div>
                </div>
              </div>
            </div>
          );
        })}
      </div>
    </StageShell>
  );
}

function CollapsiblePreview({ activeTheme }: { activeTheme: ThemePreset }) {
  const [open, setOpen] = useState(true);
  const [enableCache, setEnableCache] = useState(true);

  return (
    <StageShell
      title='ui.kit_collapsible("advanced", "Advanced", |ui| ...)'
      themeName={activeTheme.name}
      onReset={() => {
        setOpen(true);
        setEnableCache(true);
      }}
      controls={<ToggleCheckbox checked={open} onChange={setOpen} label="Expanded State" />}
    >
      <div
        className="w-full max-w-sm rounded-xl border overflow-hidden shadow-xs"
        style={{ backgroundColor: 'var(--card)', borderColor: 'var(--border)' }}
      >
        <button
          type="button"
          aria-expanded={open}
          onClick={() => setOpen((o) => !o)}
          className="w-full px-4 py-3 flex items-center justify-between gap-3 text-left cursor-pointer transition-colors hover:bg-[var(--muted)]"
        >
          <span className="text-xs font-semibold" style={{ color: 'var(--card-foreground)' }}>
            Advanced Linker Flags
          </span>
          <ChevronRight
            className={`w-4 h-4 transition-transform duration-200 ${open ? 'rotate-90' : ''}`}
            style={{ color: open ? 'var(--primary)' : 'var(--muted-foreground)' }}
            aria-hidden="true"
          />
        </button>

        <div
          className="grid transition-[grid-template-rows,opacity] duration-200 ease-out"
          style={{
            gridTemplateRows: open ? '1fr' : '0fr',
            opacity: open ? 1 : 0,
          }}
        >
          <div className="overflow-hidden">
            <div className="p-4 pt-3 border-t space-y-2.5" style={{ borderColor: 'var(--border)' }}>
              <ToggleCheckbox checked={enableCache} onChange={setEnableCache} label="Reuse Incremental Artifact Cache" />
              <div
                className="p-2.5 rounded border text-[11px] font-mono"
                style={{ backgroundColor: 'var(--muted)', borderColor: 'var(--border)', color: 'var(--foreground)' }}
              >
                RUSTFLAGS=&quot;-C target-cpu=native&quot;
              </div>
            </div>
          </div>
        </div>
      </div>
    </StageShell>
  );
}

function TabsPreview({ activeTheme }: { activeTheme: ThemePreset }) {
  const [variant, setVariant] = useState<'Underline' | 'Pill' | 'Card'>('Underline');
  const [closable, setClosable] = useState(false);
  const [tabs, setTabs] = useState(['Overview', 'Props', 'Crate Source']);
  const [active, setActive] = useState(0);
  const tabWrapRef = useRef<HTMLDivElement>(null);
  const itemRefs = useRef<Array<HTMLDivElement | null>>([]);
  const [indicator, setIndicator] = useState<{ left: number; width: number }>({ left: 0, width: 0 });

  const safeActive = Math.min(active, Math.max(0, tabs.length - 1));

  useLayoutEffect(() => {
    const el = itemRefs.current[safeActive];
    const parent = tabWrapRef.current;
    if (el && parent) {
      setIndicator({
        left: el.offsetLeft,
        width: el.offsetWidth,
      });
    }
  }, [safeActive, tabs, variant, closable]);

  useEffect(() => {
    const onResize = () => {
      const el = itemRefs.current[safeActive];
      if (el) setIndicator({ left: el.offsetLeft, width: el.offsetWidth });
    };
    window.addEventListener('resize', onResize);
    return () => window.removeEventListener('resize', onResize);
  }, [safeActive]);

  const closeTab = (idx: number) => {
    if (tabs.length <= 1) return;
    setTabs((prev) => prev.filter((_, i) => i !== idx));
    if (safeActive >= idx && safeActive > 0) {
      setActive(safeActive - 1);
    }
  };

  return (
    <StageShell
      title='Tabs::new("doc_tabs", &mut active, &tabs)'
      themeName={activeTheme.name}
      onReset={() => {
        setVariant('Underline');
        setClosable(false);
        setTabs(['Overview', 'Props', 'Crate Source']);
        setActive(0);
      }}
      controls={
        <>
          <ControlField label="Tab Variant">
            <SegmentedButtons options={['Underline', 'Pill', 'Card'] as const} value={variant} onChange={setVariant} />
          </ControlField>
          <ToggleCheckbox checked={closable} onChange={setClosable} label="Closable Tabs" />
          {tabs.length < 3 && (
            <button
              type="button"
              onClick={() => {
                setTabs(['Overview', 'Props', 'Crate Source']);
                setActive(0);
              }}
              className="inline-flex items-center justify-center gap-1.5 px-3 py-1.5 rounded border text-xs font-medium cursor-pointer"
              style={{ backgroundColor: 'var(--muted)', borderColor: 'var(--border)', color: 'var(--foreground)' }}
            >
              <RotateCcw className="w-3.5 h-3.5" aria-hidden="true" />
              <span>Restore Closed Tabs</span>
            </button>
          )}
        </>
      }
    >
      <div className="w-full max-w-md space-y-3">
        <div
          ref={tabWrapRef}
          role="tablist"
          aria-label="Document sections"
          className={`relative flex items-center gap-1 ${
            variant === 'Underline' ? 'border-b pb-0' : 'p-1 rounded-lg border'
          }`}
          style={{
            borderColor: 'var(--border)',
            backgroundColor: variant === 'Underline' ? 'transparent' : 'var(--muted)',
          }}
        >
          {indicator.width > 0 && variant === 'Underline' && (
            <span
              aria-hidden="true"
              className="pointer-events-none absolute bottom-0 h-0.5 rounded-full"
              style={{
                backgroundColor: 'var(--primary)',
                width: indicator.width,
                transform: `translateX(${indicator.left}px)`,
                transition: 'transform 220ms cubic-bezier(0.22, 1, 0.36, 1), width 220ms cubic-bezier(0.22, 1, 0.36, 1)',
              }}
            />
          )}
          {indicator.width > 0 && variant !== 'Underline' && (
            <span
              aria-hidden="true"
              className="pointer-events-none absolute top-1 bottom-1 left-0 rounded-md shadow-xs"
              style={{
                backgroundColor: variant === 'Pill' ? 'var(--primary)' : 'var(--card)',
                width: indicator.width,
                transform: `translateX(${indicator.left}px)`,
                transition: 'transform 220ms cubic-bezier(0.22, 1, 0.36, 1), width 220ms cubic-bezier(0.22, 1, 0.36, 1)',
              }}
            />
          )}

          {tabs.map((label, idx) => {
            const isActive = idx === safeActive;
            return (
              <div
                key={label}
                ref={(el) => {
                  itemRefs.current[idx] = el;
                }}
                className="relative z-10 flex items-center"
              >
                <button
                  type="button"
                  role="tab"
                  aria-selected={isActive}
                  onClick={() => setActive(idx)}
                  className={`px-3 py-2 text-xs font-medium transition-colors duration-150 cursor-pointer whitespace-nowrap ${
                    variant === 'Underline' ? 'rounded-t' : 'rounded-md'
                  } ${closable ? 'pr-7' : ''}`}
                  style={{
                    color:
                      variant === 'Pill' && isActive
                        ? 'var(--primary-foreground)'
                        : isActive
                          ? 'var(--foreground)'
                          : 'var(--muted-foreground)',
                  }}
                >
                  {label}
                </button>
                {closable && tabs.length > 1 && (
                  <button
                    type="button"
                    aria-label={`Close ${label} tab`}
                    onClick={() => closeTab(idx)}
                    className="absolute right-1.5 p-0.5 rounded hover:opacity-80 cursor-pointer"
                    style={{
                      color:
                        variant === 'Pill' && isActive
                          ? 'var(--primary-foreground)'
                          : 'var(--muted-foreground)',
                    }}
                  >
                    <X className="w-3 h-3" aria-hidden="true" />
                  </button>
                )}
              </div>
            );
          })}
        </div>

        <div
          key={`${tabs[safeActive]}-${safeActive}`}
          className="animate-egui-fade-in rounded-xl border p-4 text-xs leading-relaxed shadow-xs"
          style={{ backgroundColor: 'var(--card)', borderColor: 'var(--border)', color: 'var(--muted-foreground)' }}
        >
          Active Panel: <span className="font-semibold" style={{ color: 'var(--foreground)' }}>{tabs[safeActive]}</span> — bound directly to index <code className="font-mono">{safeActive}</code>.
        </div>
      </div>
    </StageShell>
  );
}

function StepperPreview({ activeTheme }: { activeTheme: ThemePreset }) {
  const steps = ['Select Crate', 'Configure', 'Verify', 'Publish'];
  const [activeStep, setActiveStep] = useState(1);
  const [showError, setShowError] = useState(false);

  return (
    <StageShell
      title="Stepper::new(&steps, active_step)"
      themeName={activeTheme.name}
      onReset={() => {
        setActiveStep(1);
        setShowError(false);
      }}
      controls={
        <>
          <ToggleCheckbox checked={showError} onChange={setShowError} label="Mark Active Step as Error" />
          <div className="flex items-center gap-2">
            <button
              type="button"
              disabled={activeStep === 0}
              onClick={() => setActiveStep((s) => getNextStepIndex(s, steps.length, -1))}
              className="flex-1 px-3 py-1.5 rounded border text-xs font-medium cursor-pointer disabled:opacity-45"
              style={{ backgroundColor: 'var(--card)', borderColor: 'var(--border)', color: 'var(--foreground)' }}
            >
              Previous Step
            </button>
            <button
              type="button"
              disabled={activeStep === steps.length - 1}
              onClick={() => setActiveStep((s) => getNextStepIndex(s, steps.length, 1))}
              className="flex-1 px-3 py-1.5 rounded text-xs font-semibold cursor-pointer disabled:opacity-45"
              style={{ backgroundColor: 'var(--primary)', color: 'var(--primary-foreground)' }}
            >
              Next Step
            </button>
          </div>
        </>
      }
    >
      <div className="w-full max-w-md flex items-start justify-between gap-2">
        {steps.map((label, idx) => {
          const isErr = showError && idx === activeStep;
          const isDone = idx < activeStep && !isErr;
          const isCurr = idx === activeStep && !isErr;

          return (
            <div key={label} className="flex-1 flex flex-col items-center relative">
              {idx < steps.length - 1 && (
                <div
                  aria-hidden="true"
                  className="absolute top-3.5 left-1/2 w-full h-0.5 overflow-hidden rounded-full"
                  style={{ backgroundColor: 'var(--border)' }}
                >
                  <div
                    className="h-full transition-[width] duration-300 ease-out"
                    style={{
                      width: idx < activeStep ? '100%' : '0%',
                      backgroundColor: 'var(--primary)',
                    }}
                  />
                </div>
              )}
              <div className="relative z-10 flex items-center justify-center">
                {isCurr && (
                  <span
                    aria-hidden="true"
                    className="animate-egui-mic-ring absolute inset-0 rounded-full border pointer-events-none"
                    style={{ borderColor: 'var(--primary)' }}
                  />
                )}
                <button
                  type="button"
                  onClick={() => setActiveStep(idx)}
                  aria-current={idx === activeStep ? 'step' : undefined}
                  aria-label={`Step ${idx + 1}: ${label}`}
                  className="relative w-7 h-7 rounded-full border flex items-center justify-center text-xs font-mono font-semibold cursor-pointer transition-[background-color,border-color,color,transform] duration-200 active:scale-90"
                  style={{
                    backgroundColor: isErr
                      ? 'var(--destructive)'
                      : isDone || isCurr
                        ? 'var(--primary)'
                        : 'var(--muted)',
                    borderColor: isErr ? 'var(--destructive)' : isDone || isCurr ? 'var(--primary)' : 'var(--border)',
                    color: isErr || isDone || isCurr ? 'var(--primary-foreground)' : 'var(--muted-foreground)',
                  }}
                >
                  {isErr ? (
                    <AlertTriangle className="w-3.5 h-3.5 animate-egui-pop" aria-hidden="true" />
                  ) : isDone ? (
                    <Check className="w-3.5 h-3.5 animate-egui-check-draw" aria-hidden="true" />
                  ) : (
                    idx + 1
                  )}
                </button>
              </div>
              <span
                className="text-[11px] font-medium mt-2 text-center transition-colors duration-150"
                style={{
                  color: isErr
                    ? 'var(--destructive)'
                    : isCurr
                      ? 'var(--foreground)'
                      : 'var(--muted-foreground)',
                }}
              >
                {label}
              </span>
            </div>
          );
        })}
      </div>
    </StageShell>
  );
}

function PaginationPreview({ activeTheme }: { activeTheme: ThemePreset }) {
  const [page, setPage] = useState(5);
  const [totalPages, setTotalPages] = useState(14);
  const [siblingCount, setSiblingCount] = useState<'0' | '1' | '2'>('1');

  const items = computePaginationItems(page, totalPages, Number(siblingCount));

  return (
    <StageShell
      title="ui.kit_pagination(&mut page, total_pages)"
      themeName={activeTheme.name}
      onReset={() => {
        setPage(5);
        setTotalPages(14);
        setSiblingCount('1');
      }}
      controls={
        <>
          <ControlField label="Sibling Count">
            <SegmentedButtons options={['0', '1', '2'] as const} value={siblingCount} onChange={setSiblingCount} />
          </ControlField>
          <ControlField label="Total Pages">
            <SegmentedButtons
              options={['6', '14', '24'] as const}
              value={String(totalPages) as '6' | '14' | '24'}
              onChange={(v) => {
                const n = Number(v);
                setTotalPages(n);
                setPage((p) => Math.min(p, n));
              }}
            />
          </ControlField>
          <ControlField label="Current Page">
            <div
              className="px-3 py-2 rounded border text-xs font-mono tabular-nums"
              style={{ backgroundColor: 'var(--muted)', borderColor: 'var(--border)', color: 'var(--foreground)' }}
            >
              Page {page} of {totalPages}
            </div>
          </ControlField>
        </>
      }
    >
      <nav aria-label="Demo pagination" className="flex flex-wrap items-center gap-1">
        <button
          type="button"
          disabled={page <= 1}
          aria-label="Previous page"
          onClick={() => setPage((p) => Math.max(1, p - 1))}
          className="w-8 h-8 rounded-md border flex items-center justify-center cursor-pointer disabled:opacity-40 disabled:cursor-not-allowed hover:bg-[var(--accent)] active:scale-95 transition-[background-color,transform]"
          style={{ backgroundColor: 'var(--card)', borderColor: 'var(--border)', color: 'var(--foreground)' }}
        >
          <ChevronLeft className="w-4 h-4" aria-hidden="true" />
        </button>

        {items.map((token, idx) =>
          token === 'ellipsis' ? (
            <span
              key={`el-${idx}`}
              className="w-7 h-8 flex items-center justify-center text-xs font-mono"
              style={{ color: 'var(--muted-foreground)' }}
            >
              …
            </span>
          ) : (
            <button
              key={token === page ? `active-${token}` : token}
              type="button"
              aria-current={token === page ? 'page' : undefined}
              onClick={() => setPage(token)}
              className={`min-w-8 h-8 px-2 rounded-md border text-xs font-mono tabular-nums cursor-pointer transition-[background-color,border-color,color,transform] duration-150 hover:bg-[var(--accent)] active:scale-95 ${
                token === page ? 'animate-egui-pop' : ''
              }`}
              style={{
                backgroundColor: token === page ? 'var(--primary)' : 'var(--card)',
                borderColor: token === page ? 'var(--primary)' : 'var(--border)',
                color: token === page ? 'var(--primary-foreground)' : 'var(--foreground)',
                fontWeight: token === page ? 600 : 400,
              }}
            >
              {token}
            </button>
          )
        )}

        <button
          type="button"
          disabled={page >= totalPages}
          aria-label="Next page"
          onClick={() => setPage((p) => Math.min(totalPages, p + 1))}
          className="w-8 h-8 rounded-md border flex items-center justify-center cursor-pointer disabled:opacity-40 disabled:cursor-not-allowed"
          style={{ backgroundColor: 'var(--card)', borderColor: 'var(--border)', color: 'var(--foreground)' }}
        >
          <ChevronRight className="w-4 h-4" aria-hidden="true" />
        </button>
      </nav>
    </StageShell>
  );
}
