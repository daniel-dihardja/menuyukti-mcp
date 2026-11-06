import type { LineItem } from "../line-item.js";
import type {
  MenuEngineeringDistributionItem,
  MenuEngineeringMatrixItem,
  MenuEngineeringMatrixResult,
} from "./menu-engineering.js";
import { parseOrderTime } from "./time.js";

export type WeakDisplayRow = {
  menu: string;
  units_sold: number;
  avg_qty_per_day: number | null;
  contribution_margin: number;
  contribution_margin_share: number;
};

export type WeakPortfolioStats = {
  item_count: number;
  item_share: number;
  margin_share: number;
  margin_amount: number;
  total_menu_items: number;
  period_days: number | null;
};

export type WeakDistributionStats = {
  item_count: number;
  item_share: number;
  margin_share: number;
};

export type WeakReviewResult = {
  rows: WeakDisplayRow[];
  stats: WeakPortfolioStats;
  distribution_stats: WeakDistributionStats;
  period_start: string | null;
  period_end: string | null;
};

export function inclusiveCalendarDays(
  periodStart?: string | null,
  periodEnd?: string | null,
): number | null {
  if (!periodStart?.trim() || !periodEnd?.trim()) return null;
  const start = Date.parse(`${periodStart.trim()}T00:00:00Z`);
  const end = Date.parse(`${periodEnd.trim()}T00:00:00Z`);
  if (Number.isNaN(start) || Number.isNaN(end)) return null;
  const diff = Math.round((end - start) / 86400000);
  if (diff < 0) return null;
  return diff + 1;
}

export function periodFromLineItems(
  rows: LineItem[],
): { start: string | null; end: string | null } {
  let min: string | null = null;
  let max: string | null = null;
  for (const row of rows) {
    const parsed = parseOrderTime(row.order_time);
    if (!parsed) continue;
    const key = parsed.dateKey;
    if (!min || key < min) min = key;
    if (!max || key > max) max = key;
  }
  return { start: min, end: max };
}

function weakDistribution(
  distribution: MenuEngineeringDistributionItem[],
): { count: number; itemShare: number; marginShare: number } {
  const d = distribution.find(
    (x) => x.category.toLowerCase() === "weak",
  );
  return d
    ? {
        count: d.item_count,
        itemShare: d.item_share,
        marginShare: d.margin_share,
      }
    : { count: 0, itemShare: 0, marginShare: 0 };
}

function toDisplayRow(
  item: MenuEngineeringMatrixItem,
  periodDays: number | null,
): WeakDisplayRow {
  const contributionMarginShare =
    Number.isFinite(item.contribution_margin_percentage) &&
    item.contribution_margin_percentage >= 0 &&
    item.contribution_margin_percentage <= 1
      ? item.contribution_margin_percentage
      : 0;
  const contributionMargin = Number.isFinite(item.contribution_margin)
    ? item.contribution_margin
    : 0;
  const avgQtyPerDay =
    periodDays && periodDays > 0 ? item.quantity / periodDays : null;
  const trimmed = item.menu.trim();
  return {
    menu: trimmed || "Unknown",
    units_sold: item.quantity,
    avg_qty_per_day: avgQtyPerDay,
    contribution_margin: contributionMargin,
    contribution_margin_share: contributionMarginShare,
  };
}

export function adaptWeakReview(
  items: MenuEngineeringMatrixItem[],
  distribution: MenuEngineeringDistributionItem[],
  periodStart?: string | null,
  periodEnd?: string | null,
): WeakReviewResult {
  const periodDays = inclusiveCalendarDays(periodStart, periodEnd);
  const weakDist = weakDistribution(distribution);
  const totalMenuItems = items.length;
  const rows = items
    .filter((item) => item.category.toLowerCase() === "weak")
    .map((item) => toDisplayRow(item, periodDays));

  const marginAmount = rows.reduce((s, r) => s + r.contribution_margin, 0);
  const totalMargin = items.reduce(
    (s, i) =>
      s + (Number.isFinite(i.contribution_margin) ? i.contribution_margin : 0),
    0,
  );
  const itemCount = rows.length;
  const itemShare =
    weakDist.count > 0
      ? weakDist.itemShare
      : totalMenuItems > 0
        ? itemCount / totalMenuItems
        : 0;
  const marginShare =
    weakDist.count > 0 || weakDist.marginShare > 0
      ? weakDist.marginShare
      : totalMargin > 0
        ? marginAmount / totalMargin
        : 0;

  return {
    rows,
    stats: {
      item_count: itemCount,
      item_share: itemShare,
      margin_share: marginShare,
      margin_amount: marginAmount,
      total_menu_items: totalMenuItems,
      period_days: periodDays,
    },
    distribution_stats: {
      item_count: weakDist.count > 0 ? weakDist.count : itemCount,
      item_share: itemShare,
      margin_share: marginShare,
    },
    period_start: periodStart?.trim() || null,
    period_end: periodEnd?.trim() || null,
  };
}

export function adaptWeakReviewFromOrders(
  matrix: MenuEngineeringMatrixResult,
  rows: LineItem[],
): WeakReviewResult {
  const { start, end } = periodFromLineItems(rows);
  return adaptWeakReview(matrix.items, matrix.distribution, start, end);
}
