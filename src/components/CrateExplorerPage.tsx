import { useState } from 'react';
import { Download, FileCode2 } from 'lucide-react';
import { CRATE_FILES } from '../data/crateFiles';
import { RustCodeBlock } from './RustCodeBlock';

export function CrateExplorerPage() {
  const [selectedPath, setSelectedPath] = useState(CRATE_FILES[0].path);
  const currentFile = CRATE_FILES.find((f) => f.path === selectedPath) ?? CRATE_FILES[0];

  const downloadSingleFile = () => {
    const blob = new Blob([currentFile.content], { type: 'text/plain;charset=utf-8' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = currentFile.path.split('/').pop() || 'file.rs';
    a.click();
    URL.revokeObjectURL(url);
  };

  const downloadShellBundle = () => {
    const lines = [
      '#!/usr/bin/env bash',
      'set -euo pipefail',
      'mkdir -p egui-kit/src/components egui-kit/examples',
      ...CRATE_FILES.map(
        (file) =>
          `cat <<'EOF_EGUI_KIT' > "egui-kit/${file.path}"\n${file.content}\nEOF_EGUI_KIT`
      ),
      'echo "Created ./egui-kit crate. Run: cd egui-kit && cargo check"',
    ].join('\n\n');

    const blob = new Blob([lines], { type: 'text/x-shellscript;charset=utf-8' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = 'setup-egui-kit.sh';
    a.click();
    URL.revokeObjectURL(url);
  };

  const groups: Array<'Workspace' | 'Core' | 'Components' | 'Examples'> = [
    'Workspace',
    'Core',
    'Components',
    'Examples',
  ];

  return (
    <div className="space-y-8">
      <div className="flex flex-wrap items-end justify-between gap-4 border-b pb-6" style={{ borderColor: 'var(--border)' }}>
        <div className="space-y-2">
          <div className="text-xs font-mono" style={{ color: 'var(--muted-foreground)' }}>
            Rust Workspace · {CRATE_FILES.length} Source Files · Parts 1–6 Complete
          </div>
          <h1 className="text-3xl font-bold tracking-tight" style={{ color: 'var(--foreground)' }}>
            egui-kit Crate Explorer
          </h1>
          <p className="text-sm max-w-2xl" style={{ color: 'var(--muted-foreground)' }}>
            Inspect every module of the <code className="font-mono">egui-kit</code> Cargo crate or download the
            scaffold script to run <code className="font-mono">cargo run --example part6_showcase</code> locally.
          </p>
        </div>

        <button
          type="button"
          onClick={downloadShellBundle}
          className="inline-flex items-center gap-2 px-4 py-2 rounded-md text-xs font-semibold cursor-pointer transition-opacity hover:opacity-90 whitespace-nowrap shrink-0"
          style={{ backgroundColor: 'var(--primary)', color: 'var(--primary-foreground)' }}
        >
          <Download className="w-4 h-4" aria-hidden="true" />
          <span>Download Crate Script (.sh)</span>
        </button>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-12 gap-6 items-start">
        {/* Compact mobile file switcher */}
        <div className="lg:hidden flex flex-col gap-1.5">
          <label htmlFor="mobile-crate-file" className="text-xs font-medium" style={{ color: 'var(--muted-foreground)' }}>
            Crate Source File
          </label>
          <select
            id="mobile-crate-file"
            value={currentFile.path}
            onChange={(e) => setSelectedPath(e.target.value)}
            className="w-full px-3 py-2 rounded-md border text-xs font-mono"
            style={{
              backgroundColor: 'var(--card)',
              color: 'var(--foreground)',
              borderColor: 'var(--border)',
            }}
          >
            {CRATE_FILES.map((entry) => (
              <option key={entry.path} value={entry.path}>
                {entry.path}
              </option>
            ))}
          </select>
        </div>

        <div
          className="hidden lg:block lg:col-span-4 rounded-lg border p-3 space-y-4"
          style={{ backgroundColor: 'var(--card)', borderColor: 'var(--border)' }}
        >
          {groups.map((group) => {
            const entries = CRATE_FILES.filter((f) => f.group === group);
            return (
              <div key={group} className="space-y-1">
                <div className="px-2 text-[11px] font-semibold" style={{ color: 'var(--muted-foreground)' }}>
                  {group}
                </div>
                {entries.map((entry) => {
                  const active = entry.path === currentFile.path;
                  return (
                    <button
                      key={entry.path}
                      type="button"
                      onClick={() => setSelectedPath(entry.path)}
                      className="w-full flex items-center gap-2 px-2.5 py-1.5 rounded text-xs font-mono text-left cursor-pointer transition-colors"
                      style={{
                        backgroundColor: active ? 'var(--accent)' : 'transparent',
                        color: active ? 'var(--foreground)' : 'var(--muted-foreground)',
                        fontWeight: active ? 600 : 400,
                      }}
                    >
                      <FileCode2 className="w-3.5 h-3.5 shrink-0" aria-hidden="true" />
                      <span className="truncate">{entry.path}</span>
                    </button>
                  );
                })}
              </div>
            );
          })}
        </div>

        <div className="lg:col-span-8 space-y-3">
          <div className="flex items-center justify-between gap-4">
            <span className="text-sm font-mono font-semibold" style={{ color: 'var(--foreground)' }}>
              rust/egui-kit/{currentFile.path}
            </span>
            <button
              type="button"
              onClick={downloadSingleFile}
              className="inline-flex items-center gap-1.5 px-3 py-1.5 rounded border text-xs font-medium cursor-pointer whitespace-nowrap shrink-0"
              style={{
                backgroundColor: 'var(--card)',
                color: 'var(--foreground)',
                borderColor: 'var(--border)',
              }}
            >
              <Download className="w-3.5 h-3.5" aria-hidden="true" />
              <span>Save File</span>
            </button>
          </div>
          <RustCodeBlock code={currentFile.content} filename={currentFile.path} />
        </div>
      </div>
    </div>
  );
}
