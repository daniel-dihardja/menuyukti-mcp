import {
  useState,
  type CSSProperties,
  type ReactNode,
} from "react";
import "./widget.css";

export function Panel({
  title,
  description,
  children,
}: {
  title: string;
  description: string;
  children: ReactNode;
}) {
  return (
    <main className="myk-panel">
      <h2>{title}</h2>
      <p className="myk-desc">{description}</p>
      {children}
    </main>
  );
}

export function Empty({ children }: { children: ReactNode }) {
  return <div className="myk-empty">{children}</div>;
}

export function Loading({ label }: { label: string }) {
  return (
    <Panel title={label} description="Loading analytics…">
      <p className="myk-meta">Waiting for tool result…</p>
    </Panel>
  );
}

export function Segmented<T extends string>({
  label,
  value,
  options,
  onChange,
  role = "radiogroup",
}: {
  label: string;
  value: T;
  options: Array<{ id: T; label: string; ariaLabel?: string }>;
  onChange: (id: T) => void;
  role?: "radiogroup" | "group";
}) {
  const labelId = `${label.replace(/\s+/g, "-").toLowerCase()}-label`;
  return (
    <div className="myk-metric-bar">
      <span className="myk-metric-label" id={labelId}>
        {label}
      </span>
      <div className="myk-seg" role={role} aria-labelledby={labelId}>
        {options.map((opt) => {
          const selected = opt.id === value;
          return (
            <button
              key={opt.id}
              type="button"
              role={role === "radiogroup" ? "radio" : "button"}
              aria-checked={role === "radiogroup" ? selected : undefined}
              aria-pressed={role === "group" ? selected : undefined}
              aria-label={opt.ariaLabel ?? opt.label}
              tabIndex={selected ? 0 : -1}
              onClick={() => onChange(opt.id)}
            >
              {opt.label}
            </button>
          );
        })}
      </div>
    </div>
  );
}

export function Legend({ caption }: { caption: string }) {
  return (
    <div className="myk-legend">
      <span>Low</span>
      <div className="myk-legend-bar" aria-hidden="true" />
      <span>High</span>
      <span className="myk-showing">{caption}</span>
    </div>
  );
}

export function intensity(value: number, min: number, max: number): number {
  const range = max - min || 1;
  return Math.min(1, Math.max(0, (value - min) / range));
}

export function cellStyle(t: number): CSSProperties {
  const pct = Math.round(Math.min(1, Math.max(0, t)) * 100);
  return {
    background: `color-mix(in srgb, var(--heatmap-high) ${pct}%, var(--heatmap-low))`,
  };
}

export function lightTextClass(t: number): string {
  return t >= 0.55 ? "light-text" : "";
}

export function usePersistedState<T extends string>(
  key: string,
  fallback: T,
  allowed: readonly T[],
): [T, (next: T) => void] {
  const [value, setValue] = useState<T>(() => {
    try {
      const stored = localStorage.getItem(key);
      if (stored && (allowed as readonly string[]).includes(stored)) {
        return stored as T;
      }
    } catch {
      /* ignore */
    }
    return fallback;
  });
  return [
    value,
    (next: T) => {
      setValue(next);
      try {
        localStorage.setItem(key, next);
      } catch {
        /* ignore */
      }
    },
  ];
}
