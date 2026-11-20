import { ArrowRight } from 'lucide-react';
import type { PartNumber, ThemePreset, WidgetSpec } from '../types/kit';
import { countByStatus, PARTS_ROADMAP } from '../utils/catalogLogic';
import { RustCodeBlock } from './RustCodeBlock';

interface OverviewPageProps {
  widgets: WidgetSpec[];
  activeTheme: ThemePreset;
  onSelectWidget: (slug: string) => void;
  onSelectPartFilter: (part: PartNumber) => void;
  onOpenThemes: () => void;
  onOpenCrate: () => void;
}

export function OverviewPage({
  widgets,
  activeTheme,
  onSelectWidget,
  onSelectPartFilter,
  onOpenThemes,
  onOpenCrate,
}: OverviewPageProps) {
  const counts = countByStatus(widgets);
  const readyWidgets = widgets.filter((w) => w.status === 'ready');

  const quickstartCode = `use eframe::egui;
use egui_kit::prelude::*;

fn update_ui(ctx: &egui::Context, date: &mut SimpleDate, time: &mut SimpleTime) {
    Root::show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.kit_label("egui-kit Workspace");
            ui.add(ThemePicker::new("theme_select"));
        });
        ui.kit_date_picker("release", date);
        ui.kit_time_field("window", time);
        if ui.kit_button("Build Crate").clicked() {
            ui.kit_notify(Notification::success("Ready").message("Parts 1–6 active"));
        }
    });
    NotificationManager::show(ctx, NotificationEdge::BottomRight);
}`;

  return (
    <div className="space-y-12">
      {/* Hero Section */}
      <section className="space-y-6">
        <div className="text-xs font-mono" style={{ color: 'var(--muted-foreground)' }}>
          Rust UI Library · egui 0.30 Port of gpui-kit · Parts 1–6 of 8 Complete
        </div>

        <h1
          className="text-balance text-3xl sm:text-5xl font-bold tracking-tight max-w-3xl"
          style={{ color: 'var(--foreground)' }}
        >
          Every gpui-kit component, rebuilt idiomatically for egui.rs
        </h1>

        <p className="text-base leading-relaxed max-w-2xl" style={{ color: 'var(--muted-foreground)' }}>
          egui-kit translates all 80 primitives, form controls, overlays, and data views from gpui-kit into
          lightweight immediate-mode <code className="font-mono text-sm">egui::Widget</code> builders backed by
          36 synchronized color themes.
        </p>

        <div className="flex flex-wrap items-center gap-3">
          <button
            type="button"
            onClick={() => onSelectWidget('dialog')}
            className="inline-flex items-center gap-2 px-5 py-2.5 rounded-md text-sm font-semibold cursor-pointer transition-opacity hover:opacity-90 whitespace-nowrap shrink-0"
            style={{ backgroundColor: 'var(--primary)', color: 'var(--primary-foreground)' }}
          >
            <span>Explore Part 6 Widgets</span>
            <ArrowRight className="w-4 h-4" aria-hidden="true" />
          </button>
          <button
            type="button"
            onClick={onOpenThemes}
            className="px-4 py-2.5 rounded-md border text-sm font-medium cursor-pointer transition-colors whitespace-nowrap shrink-0"
            style={{
              backgroundColor: 'var(--card)',
              color: 'var(--foreground)',
              borderColor: 'var(--border)',
            }}
          >
            Browse 36 Themes ({activeTheme.name})
          </button>
          <button
            type="button"
            onClick={onOpenCrate}
            className="px-4 py-2.5 rounded-md text-sm font-medium cursor-pointer underline-offset-4 hover:underline whitespace-nowrap shrink-0"
            style={{ color: 'var(--foreground)' }}
          >
            Inspect Rust Crate
          </button>
        </div>

        {/* Clean quantitative proof strip */}
        <div
          className="grid grid-cols-2 sm:grid-cols-4 gap-4 pt-6 border-t"
          style={{ borderColor: 'var(--border)' }}
        >
          <div>
            <div className="text-2xl font-bold font-mono tabular-nums" style={{ color: 'var(--foreground)' }}>
              {counts.ready} / {counts.total}
            </div>
            <div className="text-xs mt-1" style={{ color: 'var(--muted-foreground)' }}>
              Widgets ready (Parts 1–6)
            </div>
          </div>
          <div>
            <div className="text-2xl font-bold font-mono tabular-nums" style={{ color: 'var(--foreground)' }}>
              36
            </div>
            <div className="text-xs mt-1" style={{ color: 'var(--muted-foreground)' }}>
              Synchronized gpui-kit themes
            </div>
          </div>
          <div>
            <div className="text-2xl font-bold font-mono tabular-nums" style={{ color: 'var(--foreground)' }}>
              8 Parts
            </div>
            <div className="text-xs mt-1" style={{ color: 'var(--muted-foreground)' }}>
              10 components per milestone
            </div>
          </div>
          <div>
            <div className="text-2xl font-bold font-mono tabular-nums" style={{ color: 'var(--foreground)' }}>
              egui 0.30
            </div>
            <div className="text-xs mt-1" style={{ color: 'var(--muted-foreground)' }}>
              Zero external icon deps
            </div>
          </div>
        </div>
      </section>

      {/* Quickstart Code & GPUI Mapping */}
      <section className="grid grid-cols-1 xl:grid-cols-12 gap-6 items-start">
        <div className="xl:col-span-7 space-y-3">
          <h2 className="text-xl font-semibold" style={{ color: 'var(--foreground)' }}>
            01. Immediate-Mode Quick Start
          </h2>
          <p className="text-sm" style={{ color: 'var(--muted-foreground)' }}>
            Wrap your frame inside <code className="font-mono">Root::show(ctx, ...)</code> to register
            tokens and host overlays cleanly.
          </p>
          <RustCodeBlock code={quickstartCode} filename="examples/quickstart.rs" />
        </div>

        <div className="xl:col-span-5 space-y-3">
          <h2 className="text-xl font-semibold" style={{ color: 'var(--foreground)' }}>
            02. GPUI to egui Translation
          </h2>
          <p className="text-sm" style={{ color: 'var(--muted-foreground)' }}>
            Retained GPUI concepts map directly to deterministic immediate-mode primitives.
          </p>
          <div
            className="rounded-lg border overflow-hidden"
            style={{ backgroundColor: 'var(--card)', borderColor: 'var(--border)' }}
          >
            <table className="w-full text-left text-xs border-collapse">
              <thead>
                <tr className="border-b" style={{ backgroundColor: 'var(--muted)', borderColor: 'var(--border)' }}>
                  <th className="px-3.5 py-2.5 font-semibold">gpui-kit</th>
                  <th className="px-3.5 py-2.5 font-semibold">egui-kit</th>
                </tr>
              </thead>
              <tbody>
                {[
                  ['Entity<T> + cx.notify()', '&mut T / egui::Memory'],
                  ['RenderOnce + Styled', 'impl egui::Widget'],
                  ['Theme Global', 'ctx.data() + egui::Visuals'],
                  ['Window / Overlay Portal', 'Root + foreground Area'],
                  ['SVG Asset Icons', 'Painter vector IconName'],
                ].map(([gpui, egui]) => (
                  <tr key={gpui} className="border-b last:border-b-0" style={{ borderColor: 'var(--border)' }}>
                    <td className="px-3.5 py-2.5 font-mono" style={{ color: 'var(--muted-foreground)' }}>
                      {gpui}
                    </td>
                    <td className="px-3.5 py-2.5 font-mono font-medium" style={{ color: 'var(--foreground)' }}>
                      {egui}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>
      </section>

      {/* Ready Interactive Grid */}
      <section className="space-y-4">
        <div className="flex flex-wrap items-baseline justify-between gap-2">
          <h2 className="text-xl font-semibold" style={{ color: 'var(--foreground)' }}>
            03. Ready Components ({readyWidgets.length})
          </h2>
          <span className="text-xs font-mono" style={{ color: 'var(--muted-foreground)' }}>
            Click any widget to launch its interactive workbench
          </span>
        </div>

        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
          {readyWidgets.map((w) => (
            <button
              key={w.slug}
              type="button"
              onClick={() => onSelectWidget(w.slug)}
              className="text-left p-5 rounded-lg border transition-colors cursor-pointer hover:bg-[var(--accent)] flex flex-col gap-2"
              style={{
                backgroundColor: 'var(--card)',
                borderColor: 'var(--border)',
              }}
            >
              <div className="text-xs font-mono" style={{ color: 'var(--muted-foreground)' }}>
                {w.category} · Part {w.part} · Ready
              </div>
              <div className="text-base font-semibold" style={{ color: 'var(--card-foreground)' }}>
                {w.name}
              </div>
              <p className="text-xs leading-relaxed line-clamp-2" style={{ color: 'var(--muted-foreground)' }}>
                {w.summary}
              </p>
              <code
                className="text-[11px] font-mono pt-1 truncate"
                style={{ color: 'var(--primary)' }}
              >
                {w.eguiSignature}
              </code>
            </button>
          ))}
        </div>
      </section>

      {/* 8-Part Roadmap */}
      <section className="space-y-4">
        <div className="flex flex-wrap items-baseline justify-between gap-2">
          <h2 className="text-xl font-semibold" style={{ color: 'var(--foreground)' }}>
            04. Full 8-Part Porting Roadmap (80 Components)
          </h2>
          <span className="text-xs font-mono tabular-nums" style={{ color: 'var(--muted-foreground)' }}>
            10 widgets per milestone
          </span>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          {PARTS_ROADMAP.map((p) => {
            const partWidgets = widgets.filter((w) => w.part === p.part);
            return (
              <div
                key={p.part}
                className="p-5 rounded-lg border flex flex-col justify-between gap-3"
                style={{ backgroundColor: 'var(--card)', borderColor: 'var(--border)' }}
              >
                <div className="space-y-1.5">
                  <div className="flex items-center justify-between gap-2 text-xs font-mono">
                    <span style={{ color: p.status === 'completed' ? 'var(--success)' : 'var(--muted-foreground)' }}>
                      {p.status === 'completed' ? 'Completed' : 'Scheduled'} · 10 widgets
                    </span>
                    <button
                      type="button"
                      onClick={() => {
                        onSelectPartFilter(p.part);
                        if (partWidgets[0]) onSelectWidget(partWidgets[0].slug);
                      }}
                      className="underline-offset-4 hover:underline cursor-pointer"
                      style={{ color: 'var(--primary)' }}
                    >
                      Inspect Part {p.part}
                    </button>
                  </div>
                  <div className="text-base font-semibold" style={{ color: 'var(--card-foreground)' }}>
                    {p.title}
                  </div>
                  <p className="text-xs" style={{ color: 'var(--muted-foreground)' }}>
                    {p.focus}
                  </p>
                </div>
                <div className="text-xs font-mono leading-relaxed" style={{ color: 'var(--foreground)' }}>
                  {partWidgets.map((w) => w.name).join(' · ')}
                </div>
              </div>
            );
          })}
        </div>
      </section>
    </div>
  );
}
