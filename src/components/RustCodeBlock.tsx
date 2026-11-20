import { useState } from 'react';
import { Check, Copy } from 'lucide-react';

interface RustCodeBlockProps {
  code: string;
  filename?: string;
  language?: 'rust' | 'toml';
}

const RUST_KEYWORDS = new Set([
  'use', 'pub', 'fn', 'let', 'mut', 'struct', 'enum', 'impl', 'for', 'if', 'else',
  'match', 'return', 'mod', 'const', 'self', 'Self', 'true', 'false', 'in', 'while',
]);

function tokenizeLine(line: string): Array<{ text: string; kind: 'comment' | 'string' | 'keyword' | 'type' | 'number' | 'plain' }> {
  const trimmed = line.trimStart();
  if (trimmed.startsWith('//')) {
    return [{ text: line, kind: 'comment' }];
  }

  const tokens: Array<{ text: string; kind: 'comment' | 'string' | 'keyword' | 'type' | 'number' | 'plain' }> = [];
  const regex = /("([^"\\]|\\.)*")|(\/\/.*$)|\b([A-Z][A-Za-z0-9_]*)\b|\b([a-z_][a-z0-9_]*)\b|\b(\d+(?:\.\d+)?)\b/g;
  let lastIndex = 0;
  let match: RegExpExecArray | null;

  while ((match = regex.exec(line)) !== null) {
    if (match.index > lastIndex) {
      tokens.push({ text: line.slice(lastIndex, match.index), kind: 'plain' });
    }
    if (match[1]) {
      tokens.push({ text: match[1], kind: 'string' });
    } else if (match[3]) {
      tokens.push({ text: match[3], kind: 'comment' });
    } else if (match[4]) {
      tokens.push({ text: match[4], kind: 'type' });
    } else if (match[5]) {
      tokens.push({
        text: match[5],
        kind: RUST_KEYWORDS.has(match[5]) ? 'keyword' : 'plain',
      });
    } else if (match[6]) {
      tokens.push({ text: match[6], kind: 'number' });
    }
    lastIndex = regex.lastIndex;
  }

  if (lastIndex < line.length) {
    tokens.push({ text: line.slice(lastIndex), kind: 'plain' });
  }
  return tokens.length > 0 ? tokens : [{ text: '', kind: 'plain' }];
}

export function RustCodeBlock({ code, filename }: RustCodeBlockProps) {
  const [copied, setCopied] = useState(false);

  const handleCopy = async () => {
    try {
      await navigator.clipboard.writeText(code);
      setCopied(true);
      window.setTimeout(() => setCopied(false), 1800);
    } catch {
      setCopied(false);
    }
  };

  const lines = code.split('\n');

  const colorFor = (kind: string) => {
    switch (kind) {
      case 'comment':
        return 'var(--code-comment)';
      case 'string':
        return 'var(--code-string)';
      case 'keyword':
        return 'var(--code-keyword)';
      case 'type':
        return 'var(--code-type)';
      case 'number':
        return 'var(--code-number)';
      default:
        return 'var(--code-fg)';
    }
  };

  return (
    <div
      className="rounded-lg border overflow-hidden"
      style={{
        backgroundColor: 'var(--code-bg)',
        borderColor: 'var(--border)',
      }}
    >
      <div
        className="flex items-center justify-between gap-4 px-4 py-2.5 border-b text-xs"
        style={{
          backgroundColor: 'var(--muted)',
          borderColor: 'var(--border)',
          color: 'var(--muted-foreground)',
        }}
      >
        <span className="font-mono truncate">{filename ?? 'example.rs'}</span>
        <button
          type="button"
          onClick={handleCopy}
          aria-label={`Copy ${filename ?? 'code snippet'}`}
          className="inline-flex items-center gap-1.5 px-2.5 py-1 rounded text-xs font-medium transition-colors whitespace-nowrap shrink-0 cursor-pointer hover:opacity-90"
          style={{
            backgroundColor: 'var(--card)',
            color: 'var(--foreground)',
            border: '1px solid var(--border)',
          }}
        >
          {copied ? <Check className="w-3.5 h-3.5" aria-hidden="true" /> : <Copy className="w-3.5 h-3.5" aria-hidden="true" />}
          <span>{copied ? 'Copied' : 'Copy'}</span>
        </button>
      </div>
      <div className="overflow-x-auto p-4 text-[13px] leading-6 font-mono">
        <table className="w-full border-collapse">
          <tbody>
            {lines.map((line, idx) => (
              <tr key={idx}>
                <td
                  className="select-none pr-4 text-right align-top tabular-nums w-9 opacity-45"
                  style={{ color: 'var(--code-comment)' }}
                >
                  {idx + 1}
                </td>
                <td className="whitespace-pre align-top">
                  {tokenizeLine(line).map((tok, tIdx) => (
                    <span key={tIdx} style={{ color: colorFor(tok.kind) }}>
                      {tok.text}
                    </span>
                  ))}
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}
