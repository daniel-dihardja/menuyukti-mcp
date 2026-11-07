import { useMemo } from "react";
import { useToolInfo } from "../helpers.js";
import type {
  WeakDisplayRow,
  WeakPortfolioStats,
} from "../lib/analytics/index.js";
import {
  Empty,
  Loading,
  Panel,
  ToolError,
  toolErrorMessage,
  usePersistedJson,
} from "./shared.js";

type SortKey =
  | "menu"
  | "avg_qty_per_day"
  | "units_sold"
  | "contribution_margin"
  | "contribution_margin_share";

type SortState = { key: SortKey; dir: "asc" | "desc" };

const WEAK_SORT_KEYS: readonly SortKey[] = [
  "menu",
  "avg_qty_per_day",
  "units_sold",
  "contribution_margin",
  "contribution_margin_share",
];

function isWeakSortState(value: unknown): value is SortState {
  if (!value || typeof value !== "object") return false;
  const v = value as { key?: unknown; dir?: unknown };
  return (
    typeof v.key === "string" &&
    (WEAK_SORT_KEYS as readonly string[]).includes(v.key) &&
    (v.dir === "asc" || v.dir === "desc")
  );
}

function asNumber(v: unknown): number {
  const n = Number(v);
  return Number.isFinite(n) ? n : Number.NaN;
}

function formatShare(n: number): string {
  return `${(n * 100).toFixed(1)}%`;
}

function formatMoney(n: number): string {
  return n.toLocaleString(undefined, {
    maximumFractionDigits: 0,
  });
}

export default function WeakItemsReview() {
  const { output, isPending } = useToolInfo<"render_weak_items_review">();
  const [sort, setSort] = usePersistedJson<SortState>(
    "menuyukti-weak-items-sort",
    { key: "contribution_margin_share", dir: "asc" },
    isWeakSortState,
  );
  const sortKey = sort.key;
  const sortDir = sort.dir;

  const items: WeakDisplayRow[] = Array.isArray(output?.items)
    ? output.items
    : [];
  const stats: WeakPortfolioStats | undefined =
    output?.stats && typeof output.stats === "object"
      ? (output.stats as WeakPortfolioStats)
      : undefined;
  const showAvgQty = items.some(
    (r) =>
      r.avg_qty_per_day != null && Number.isFinite(Number(r.avg_qty_per_day)),
  );

  const sorted = useMemo(() => {
    const rows = [...items];
    rows.sort((a, b) => {
      if (sortKey === "menu") {
        const cmp = String(a.menu ?? "").localeCompare(String(b.menu ?? ""));
        return sortDir === "asc" ? cmp : -cmp;
      }
      const aVal = asNumber(a[sortKey]);
      const bVal = asNumber(b[sortKey]);
      if (!Number.isFinite(aVal) && !Number.isFinite(bVal)) {
        return String(a.menu ?? "").localeCompare(String(b.menu ?? ""));
      }
      if (!Number.isFinite(aVal)) return 1;
      if (!Number.isFinite(bVal)) return -1;
      const cmp = aVal - bVal;
      if (cmp !== 0) return sortDir === "asc" ? cmp : -cmp;
      return String(a.menu ?? "").localeCompare(String(b.menu ?? ""));
    });
    return rows;
  }, [items, sortKey, sortDir]);

  function toggleSort(next: SortKey) {
    setSort((prev) => {
      if (prev.key === next) {
        return { key: next, dir: prev.dir === "asc" ? "desc" : "asc" };
      }
      return { key: next, dir: "asc" };
    });
  }

  function header(key: SortKey, label: string, numeric = true) {
    const active = sortKey === key;
    return (
      <th
        className={`sortable ${numeric ? "num" : "row-label"}`}
        aria-sort={
          active ? (sortDir === "asc" ? "ascending" : "descending") : "none"
        }
      >
        <button type="button" onClick={() => toggleSort(key)}>
          <span className="sort-label">{label}</span>
          <span className="sort-ind">
            {active ? (sortDir === "asc" ? "↑" : "↓") : "↕"}
          </span>
        </button>
      </th>
    );
  }

  if (isPending || !output) {
    return <Loading label="Weak items review" />;
  }

  const toolError = toolErrorMessage(output);
  if (toolError) {
    return (
      <ToolError
        title="Weak items review"
        description="Low popularity and low contribution-margin dishes."
        message={toolError}
      />
    );
  }

  if (!Array.isArray(output.items)) {
    return (
      <Panel
        title="Weak items review"
        description="Low popularity and low contribution-margin dishes."
      >
        <Empty>Missing weak items payload.</Empty>
      </Panel>
    );
  }

  if (items.length === 0) {
    return (
      <Panel
        title="Weak items review"
        description="Low popularity and low contribution-margin dishes."
      >
        <Empty>No weak-quadrant items in this workspace period.</Empty>
      </Panel>
    );
  }

  if (!stats) {
    return (
      <Panel
        title="Weak items review"
        description="Low popularity and low contribution-margin dishes."
      >
        <Empty>Missing weak items portfolio stats.</Empty>
      </Panel>
    );
  }

  const periodStart = output.period_start ? String(output.period_start) : null;
  const periodEnd = output.period_end ? String(output.period_end) : null;
  const periodLabel = [periodStart, periodEnd].filter(Boolean).join("–");
  const pos = output.pos ? String(output.pos) : "";
  const meta = [
    pos ? `POS: ${pos}` : null,
    periodLabel || null,
    stats.period_days != null ? `${stats.period_days} days` : null,
  ]
    .filter(Boolean)
    .join(" · ");

  const itemShare = asNumber(stats.item_share);
  const marginShare = asNumber(stats.margin_share);
  const marginAmount = asNumber(stats.margin_amount);

  return (
    <Panel
      title="Weak items review"
      description="Low popularity and low contribution-margin dishes — candidates to reprice, improve, or remove. No keep/remove action column; the owner decides."
    >
      <p className="myk-meta">{meta}</p>
      <div className="myk-kpis">
        <div className="myk-kpi">
          <span className="label">Weak items</span>
          <span className="value">{items.length}</span>
        </div>
        <div className="myk-kpi">
          <span className="label">Item share</span>
          <span className="value">
            {Number.isFinite(itemShare) ? formatShare(itemShare) : "—"}
          </span>
        </div>
        <div className="myk-kpi">
          <span className="label">Margin share</span>
          <span className="value">
            {Number.isFinite(marginShare) ? formatShare(marginShare) : "—"}
          </span>
        </div>
        <div className="myk-kpi">
          <span className="label">Margin amount</span>
          <span className="value">
            {Number.isFinite(marginAmount) ? formatMoney(marginAmount) : "—"}
          </span>
        </div>
      </div>
      <p className="myk-sort-hint">
        Default sort = margin share ascending. Click a column header to re-sort;
        click again to toggle direction.
      </p>
      <div className="myk-scroll">
        <table className="myk-table bordered">
          <thead>
            <tr>
              {header("menu", "Menu", false)}
              {showAvgQty ? header("avg_qty_per_day", "Avg qty/day") : null}
              {header("units_sold", "Units")}
              {header("contribution_margin", "Margin")}
              {header("contribution_margin_share", "Margin share")}
            </tr>
          </thead>
          <tbody>
            {sorted.map((row) => {
              const menu = String(row.menu ?? "Unknown");
              const avg = row.avg_qty_per_day;
              return (
                <tr key={menu}>
                  <td className="row-label">{menu}</td>
                  {showAvgQty ? (
                    <td>
                      {avg == null || !Number.isFinite(Number(avg))
                        ? "—"
                        : Number(avg).toFixed(2)}
                    </td>
                  ) : null}
                  <td>{Math.round(Number(row.units_sold ?? 0))}</td>
                  <td>{formatMoney(Number(row.contribution_margin ?? 0))}</td>
                  <td>
                    {formatShare(Number(row.contribution_margin_share ?? 0))}
                  </td>
                </tr>
              );
            })}
          </tbody>
        </table>
      </div>
      <p className="myk-caption">
        Data from get_weak_items_review (local workspace sales + COGS).
      </p>
    </Panel>
  );
}
