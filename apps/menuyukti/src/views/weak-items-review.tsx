import { useMemo, useState } from "react";
import { useToolInfo } from "../helpers.js";
import { Empty, Loading, Panel } from "./shared.js";

type WeakRow = {
  menu?: string;
  units_sold?: number;
  avg_qty_per_day?: number | null;
  contribution_margin?: number;
  contribution_margin_share?: number;
};

type SortKey =
  | "menu"
  | "avg_qty_per_day"
  | "units_sold"
  | "contribution_margin"
  | "contribution_margin_share";

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
  const [sortKey, setSortKey] = useState<SortKey>("contribution_margin_share");
  const [sortDir, setSortDir] = useState<"asc" | "desc">("asc");

  const items = (output?.items ?? []) as WeakRow[];
  const stats = (output?.stats ?? {}) as Record<string, unknown>;
  const showAvgQty = items.some(
    (r) => r.avg_qty_per_day != null && Number.isFinite(Number(r.avg_qty_per_day)),
  );

  const sorted = useMemo(() => {
    const rows = [...items];
    rows.sort((a, b) => {
      if (sortKey === "menu") {
        const cmp = String(a.menu ?? "").localeCompare(String(b.menu ?? ""));
        return sortDir === "asc" ? cmp : -cmp;
      }
      let aVal = asNumber(a[sortKey]);
      let bVal = asNumber(b[sortKey]);
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
    if (sortKey === next) {
      setSortDir((d) => (d === "asc" ? "desc" : "asc"));
    } else {
      setSortKey(next);
      setSortDir("asc");
    }
  }

  function header(key: SortKey, label: string, numeric = true) {
    const active = sortKey === key;
    return (
      <th className={`sortable ${numeric ? "num" : "row-label"}`}>
        <button
          type="button"
          aria-sort={
            active
              ? sortDir === "asc"
                ? "ascending"
                : "descending"
              : "none"
          }
          onClick={() => toggleSort(key)}
        >
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
                  <td>
                    {formatMoney(Number(row.contribution_margin ?? 0))}
                  </td>
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
