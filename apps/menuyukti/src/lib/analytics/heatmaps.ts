import type { LineItem } from "../line-item.js";
import { parseOrderTime, WEEKDAY_ORDER } from "./time.js";

export type DailyHeatmapRow = { hour: number; quantity: number };
export type WeeklyHeatmapRow = { day: string; quantity: number };

export type MenuHeatmapPayload = {
  menu: string;
  menu_category: string | null;
  menu_category_detail: string | null;
  matrix_category: string | null;
  daily_heatmap: DailyHeatmapRow[];
  weekly_heatmap: WeeklyHeatmapRow[];
  reporting_period: string;
};

export function applyMatrixCategories(
  heatmaps: MenuHeatmapPayload[],
  categoriesByMenu: Map<string, string> | Record<string, string>,
): void {
  const lookup =
    categoriesByMenu instanceof Map
      ? categoriesByMenu
      : new Map(Object.entries(categoriesByMenu));
  for (const item of heatmaps) {
    item.matrix_category = lookup.get(item.menu) ?? null;
  }
}

export function calculateMenuHeatmaps(
  rows: LineItem[],
): MenuHeatmapPayload[] {
  if (rows.length === 0) return [];

  let reportingPeriod = "";
  let minTime: string | null = null;
  for (const row of rows) {
    if (minTime === null || row.order_time < minTime) minTime = row.order_time;
  }
  if (minTime) reportingPeriod = minTime.slice(0, 7);

  const categories = new Map<
    string,
    { menu_category: string; menu_category_detail: string }
  >();
  const hourly = new Map<string, number[]>();
  const weekly = new Map<string, number[]>();

  for (const row of rows) {
    const existing = categories.get(row.menu);
    if (!existing) {
      categories.set(row.menu, {
        menu_category: row.menu_category,
        menu_category_detail: row.menu_category_detail,
      });
    } else {
      if (existing.menu_category !== row.menu_category) {
        throw new Error(
          `Menu '${row.menu}' has multiple categories: [${existing.menu_category}, ${row.menu_category}]`,
        );
      }
      if (existing.menu_category_detail !== row.menu_category_detail) {
        throw new Error(
          `Menu '${row.menu}' has multiple category details: [${existing.menu_category_detail}, ${row.menu_category_detail}]`,
        );
      }
    }

    const parsed = parseOrderTime(row.order_time);
    if (!parsed) continue;
    const hours = hourly.get(row.menu) ?? Array.from({ length: 24 }, () => 0);
    hours[parsed.hour]! += row.qty;
    hourly.set(row.menu, hours);

    const days = weekly.get(row.menu) ?? Array.from({ length: 7 }, () => 0);
    days[parsed.weekdayFromMonday]! += row.qty;
    weekly.set(row.menu, days);
  }

  const payloads: MenuHeatmapPayload[] = [];
  for (const [menu, cats] of categories) {
    const hours = hourly.get(menu) ?? Array.from({ length: 24 }, () => 0);
    const days = weekly.get(menu) ?? Array.from({ length: 7 }, () => 0);
    payloads.push({
      menu,
      menu_category: cats.menu_category,
      menu_category_detail: cats.menu_category_detail,
      matrix_category: null,
      daily_heatmap: hours.map((quantity, hour) => ({ hour, quantity })),
      weekly_heatmap: WEEKDAY_ORDER.map((day, idx) => ({
        day,
        quantity: days[idx] ?? 0,
      })),
      reporting_period: reportingPeriod,
    });
  }

  payloads.sort((a, b) => {
    const aTotal = a.daily_heatmap.reduce((s, r) => s + r.quantity, 0);
    const bTotal = b.daily_heatmap.reduce((s, r) => s + r.quantity, 0);
    return bTotal - aTotal || a.menu.localeCompare(b.menu);
  });
  return payloads;
}
