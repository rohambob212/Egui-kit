import { useState } from 'react';
import { ArrowLeft, ArrowRight, Download } from 'lucide-react';
import type { ThemePreset, WidgetSpec } from '../types/kit';
import { getCrateFileByPath } from '../data/crateFiles';
import { Part1Preview } from './Part1Previews';
import { Part2Preview } from './Part2Previews';
import { Part3Preview } from './Part3Previews';
import { Part4Preview } from './Part4Previews';
import { Part5Preview } from './Part5Previews';
import { Part6Preview } from './Part6Previews';
import { RustCodeBlock } from './RustCodeBlock';

interface WidgetWorkbenchProps {
  widget: WidgetSpec;
  allWidgets: WidgetSpec[];
  activeTheme: ThemePreset;
  allThemes: ThemePreset[];
  onSelectTheme: (id: string) => void;
  onSelectWidget: (slug: string) => void;
}

export function WidgetWorkbench({
  widget,
  allWidgets,
  activeTheme,
  allThemes,
  onSelectTheme,
  onSelectWidget,
}: WidgetWorkbenchProps) {
  const [codeTab, setCodeTab] = useState<'usage' | 'source' | 'props'>('usage');
  const crateEntry = getCrateFileByPath(widget.rustFilePath);

  const currentIdx = allWidgets.findIndex((w) => w.slug === widget.slug);
  const prevWidget = currentIdx > 0 ? allWidgets[currentIdx - 1] : null;
  const nextWidget = currentIdx < allWidgets.length - 1 ? allWidgets[currentIdx + 1] : null;

  const downloadRustModule = () => {
    if (!crateEntry) return;
    const blob = new Blob([crateEntry.content], { type: 'text/plain;charset=utf-8' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `${widget.slug.replace(/-/g, '_')}.rs`;
    a.click();
    URL.revokeObjectURL(url);
  };

  return (
    <div className="space-y-8">
      {/* Header & clean unboxed metadata */}
      <div className="space-y-3 border-b pb-6" style={{ borderColor: 'var(--border)' }}>
        <div className="flex flex-wrap items-center gap-2 text-xs font-mono" style={{ color: 'var(--muted-foreground)' }}>
          <span>{widget.category}</span>
          <span aria-hidden="true">·</span>
          <span>Part {widget.part} of 8</span>
          <span aria-hidden="true">·</span>
          <span style={{ color: widget.status === 'ready' ? 'var(--success)' : 'var(--warning)' }}>
            {widget.status === 'ready' ? 'Ready' : 'Scheduled'}
          </span>
          <span aria-hidden="true">·</span>
          <span>gpui-kit::{widget.gpuiName}</span>
        </div>

        <div className="flex flex-wrap items-center justify-between gap-4">
          <h1 className="text-3xl font-bold tracking-tight" style={{ color: 'var(--foreground)' }}>
            {widget.name}
          </h1>
          {crateEntry && (
            <button
              type="button"
              onClick={downloadRustModule}
              className="inline-flex items-center gap-2 px-3.5 py-2 rounded-md border text-xs font-medium cursor-pointer whitespace-nowrap shrink-0"
              style={{
                backgroundColor: 'var(--card)',
                color: 'var(--foreground)',
                borderColor: 'var(--border)',
              }}
            >
              <Download className="w-3.5 h-3.5" aria-hidden="true" />
              <span>Download {widget.rustFilePath.split('/').pop()}</span>
            </button>
          )}
        </div>

        <p className="text-sm sm:text-base max-w-3xl leading-relaxed" style={{ color: 'var(--muted-foreground)' }}>
          {widget.summary}
        </p>

        <div
          className="p-3 rounded-md border text-xs leading-relaxed"
          style={{
            backgroundColor: 'var(--muted)',
            borderColor: 'var(--border)',
            color: 'var(--foreground)',
          }}
        >
          <span className="font-semibold">egui Architecture Note: </span>
          <span style={{ color: 'var(--muted-foreground)' }}>{widget.gpuiMappingNote}</span>
        </div>
      </div>

      {/* Live Preview Stage OR Planned Part Spec */}
      {widget.status === 'ready' ? (
        widget.part === 6 ? (
          <Part6Preview slug={widget.slug} activeTheme={activeTheme} />
        ) : widget.part === 5 ? (
          <Part5Preview slug={widget.slug} activeTheme={activeTheme} />
        ) : widget.part === 4 ? (
          <Part4Preview slug={widget.slug} activeTheme={activeTheme} />
        ) : widget.part === 3 ? (
          <Part3Preview slug={widget.slug} activeTheme={activeTheme} />
        ) : widget.part === 2 ? (
          <Part2Preview slug={widget.slug} activeTheme={activeTheme} />
        ) : (
          <Part1Preview
            slug={widget.slug}
            activeTheme={activeTheme}
            allThemes={allThemes}
            onSelectTheme={onSelectTheme}
          />
        )
      ) : (
        <div
          className="rounded-lg border p-6 space-y-4"
          style={{ backgroundColor: 'var(--card)', borderColor: 'var(--border)' }}
        >
          <div className="flex flex-wrap items-center justify-between gap-2">
            <h2 className="text-lg font-semibold" style={{ color: 'var(--card-foreground)' }}>
              Scheduled for Part {widget.part}
            </h2>
            <button
              type="button"
              onClick={() => onSelectWidget('dialog')}
              className="text-xs font-medium underline-offset-4 hover:underline cursor-pointer"
              style={{ color: 'var(--primary)' }}
            >
              Jump to live Parts 1–6 widgets
            </button>
          </div>
          <p className="text-sm leading-relaxed" style={{ color: 'var(--muted-foreground)' }}>
            This widget is specified in the 8-part porting schedule and will be implemented in Part {widget.part}.
            Below is its planned <code className="font-mono">egui-kit</code> builder signature and module target.
          </p>
          <div
            className="p-3.5 rounded-md border font-mono text-xs overflow-x-auto"
            style={{ backgroundColor: 'var(--code-bg)', borderColor: 'var(--border)', color: 'var(--code-fg)' }}
          >
            {widget.eguiSignature}
          </div>
        </div>
      )}

      {/* Tabbed Code & API Reference */}
      <section className="space-y-4">
        <div
          className="flex flex-wrap items-center gap-1 p-1 rounded-md border w-fit"
          style={{ backgroundColor: 'var(--muted)', borderColor: 'var(--border)' }}
          role="tablist"
          aria-label="Widget documentation tabs"
        >
          {[
            { id: 'usage' as const, label: 'Usage Example' },
            ...(crateEntry ? [{ id: 'source' as const, label: 'Rust Crate Source' }] : []),
            { id: 'props' as const, label: 'API Props' },
          ].map((tab) => {
            const active = codeTab === tab.id;
            return (
              <button
                key={tab.id}
                type="button"
                role="tab"
                aria-selected={active}
                onClick={() => setCodeTab(tab.id)}
                className="px-3.5 py-1.5 rounded text-xs font-medium cursor-pointer transition-colors whitespace-nowrap shrink-0"
                style={{
                  backgroundColor: active ? 'var(--card)' : 'transparent',
                  color: active ? 'var(--foreground)' : 'var(--muted-foreground)',
                }}
              >
                {tab.label}
              </button>
            );
          })}
        </div>

        {codeTab === 'usage' && (
          <RustCodeBlock code={widget.usageRust} filename={`examples/${widget.slug.replace(/-/g, '_')}.rs`} />
        )}

        {codeTab === 'source' && crateEntry && (
          <RustCodeBlock code={crateEntry.content} filename={crateEntry.path} />
        )}

        {codeTab === 'props' && (
          <div
            className="rounded-lg border overflow-x-auto"
            style={{ backgroundColor: 'var(--card)', borderColor: 'var(--border)' }}
          >
            <table className="w-full text-left text-xs border-collapse">
              <thead>
                <tr className="border-b" style={{ backgroundColor: 'var(--muted)', borderColor: 'var(--border)' }}>
                  <th className="px-4 py-3 font-semibold">Method / Prop</th>
                  <th className="px-4 py-3 font-semibold">Rust Type</th>
                  <th className="px-4 py-3 font-semibold">Default</th>
                  <th className="px-4 py-3 font-semibold">Description</th>
                </tr>
              </thead>
              <tbody>
                {widget.props.map((prop) => (
                  <tr key={prop.name} className="border-b last:border-b-0" style={{ borderColor: 'var(--border)' }}>
                    <td className="px-4 py-3 font-mono font-medium" style={{ color: 'var(--primary)' }}>
                      {prop.name}
                    </td>
                    <td className="px-4 py-3 font-mono" style={{ color: 'var(--code-type)' }}>
                      {prop.rustType}
                    </td>
                    <td className="px-4 py-3 font-mono tabular-nums" style={{ color: 'var(--muted-foreground)' }}>
                      {prop.defaultValue}
                    </td>
                    <td className="px-4 py-3" style={{ color: 'var(--foreground)' }}>
                      {prop.description}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        )}
      </section>

      {/* Sequential Prev / Next navigation */}
      <div
        className="pt-6 border-t flex items-center justify-between gap-4"
        style={{ borderColor: 'var(--border)' }}
      >
        {prevWidget ? (
          <button
            type="button"
            onClick={() => onSelectWidget(prevWidget.slug)}
            className="inline-flex items-center gap-2 text-xs font-medium cursor-pointer hover:underline"
            style={{ color: 'var(--foreground)' }}
          >
            <ArrowLeft className="w-3.5 h-3.5" aria-hidden="true" />
            <span>
              Previous: {prevWidget.name} (Part {prevWidget.part})
            </span>
          </button>
        ) : (
          <span />
        )}
        {nextWidget && (
          <button
            type="button"
            onClick={() => onSelectWidget(nextWidget.slug)}
            className="inline-flex items-center gap-2 text-xs font-medium cursor-pointer hover:underline ml-auto"
            style={{ color: 'var(--foreground)' }}
          >
            <span>
              Next: {nextWidget.name} (Part {nextWidget.part})
            </span>
            <ArrowRight className="w-3.5 h-3.5" aria-hidden="true" />
          </button>
        )}
      </div>
    </div>
  );
}
