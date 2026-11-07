import {
  type CSSProperties,
  type KeyboardEvent,
  type ReactNode,
  useRef,
  useState,
} from "react";
import "./widget.css";

export function Panel({
  title,
  description,
  children,
}: {
  title: string;
  description?: string;
  children: ReactNode;
}) {
  return (
    <main className="myk-panel">
      <h2>{title}</h2>
      {description ? <p className="myk-desc">{description}</p> : null}
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

/** Read errJson structuredContent.error from a render_* tool result. */
export function toolErrorMessage(output: unknown): string | null {
  if (!output || typeof output !== "object") return null;
  const err = (output as { error?: unknown }).error;
  if (typeof err === "string" && err.trim()) return err.trim();
  return null;
}

export function ToolError({
  title,
  description,
  message,
}: {
  title: string;
  description: string;
  message: string;
}) {
  return (
    <Panel title={title} description={description}>
      <Empty>{message}</Empty>
    </Panel>
  );
}

export function Segmented<T extends string>({
  label,
  value,
  options,
  onChange,
}: {
  label: string;
  value: T;
  options: Array<{ id: T; label: string; ariaLabel?: string }>;
  onChange: (id: T) => void;
}) {
  const labelId = `${label.replace(/\s+/g, "-").toLowerCase()}-label`;
  const buttonRefs = useRef<Array<HTMLButtonElement | null>>([]);

  function focusIndex(index: number) {
    const clamped = Math.max(0, Math.min(options.length - 1, index));
    const next = options[clamped];
    if (!next) return;
    onChange(next.id);
    queueMicrotask(() => buttonRefs.current[clamped]?.focus());
  }

  function onKeyDown(event: KeyboardEvent<HTMLDivElement>) {
    if (options.length === 0) return;
    const current = Math.max(
      0,
      options.findIndex((opt) => opt.id === value),
    );
    switch (event.key) {
      case "ArrowRight":
      case "ArrowDown":
        event.preventDefault();
        focusIndex((current + 1) % options.length);
        break;
      case "ArrowLeft":
      case "ArrowUp":
        event.preventDefault();
        focusIndex((current - 1 + options.length) % options.length);
        break;
      case "Home":
        event.preventDefault();
        focusIndex(0);
        break;
      case "End":
        event.preventDefault();
        focusIndex(options.length - 1);
        break;
      default:
        break;
    }
  }

  return (
    <div className="myk-metric-bar">
      <span className="myk-metric-label" id={labelId}>
        {label}
      </span>
      <div
        className="myk-seg"
        role="radiogroup"
        aria-labelledby={labelId}
        onKeyDown={onKeyDown}
      >
        {options.map((opt, index) => {
          const selected = opt.id === value;
          return (
            <button
              key={opt.id}
              ref={(el) => {
                buttonRefs.current[index] = el;
              }}
              type="button"
              role="radio"
              aria-checked={selected}
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

/** Persist a small JSON object (e.g. sort key + direction) in localStorage. */
export function usePersistedJson<T>(
  key: string,
  fallback: T,
  isValid: (value: unknown) => value is T,
): [T, (next: T | ((prev: T) => T)) => void] {
  const [value, setValue] = useState<T>(() => {
    try {
      const raw = localStorage.getItem(key);
      if (!raw) return fallback;
      const parsed: unknown = JSON.parse(raw);
      return isValid(parsed) ? parsed : fallback;
    } catch {
      return fallback;
    }
  });
  return [
    value,
    (next) => {
      setValue((prev) => {
        const resolved =
          typeof next === "function" ? (next as (prev: T) => T)(prev) : next;
        try {
          localStorage.setItem(key, JSON.stringify(resolved));
        } catch {
          /* ignore */
        }
        return resolved;
      });
    },
  ];
}
