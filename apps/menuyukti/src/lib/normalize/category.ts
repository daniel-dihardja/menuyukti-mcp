import type { LineItem } from "../line-item.js";

export function normalizeCategory(
  value: string | null | undefined,
  fallback: string,
): string {
  if (value == null) return fallback;
  const text = value.trim().toUpperCase();
  if (text === "" || text === "NAN") return fallback;
  return text;
}

/** Harmonize category fields to the mode value per menu name. */
export function harmonizeCategories(rows: LineItem[]): LineItem[] {
  const catCounts = new Map<string, Map<string, number>>();
  const detailCounts = new Map<string, Map<string, number>>();
  for (const row of rows) {
    bump(catCounts, row.menu, row.menu_category);
    bump(detailCounts, row.menu, row.menu_category_detail);
  }
  return rows.map((row) => ({
    ...row,
    menu_category: mode(catCounts, row.menu) ?? row.menu_category,
    menu_category_detail:
      mode(detailCounts, row.menu) ?? row.menu_category_detail,
  }));
}

function bump(
  map: Map<string, Map<string, number>>,
  menu: string,
  value: string,
) {
  const inner = map.get(menu) ?? new Map<string, number>();
  inner.set(value, (inner.get(value) ?? 0) + 1);
  map.set(menu, inner);
}

function mode(
  map: Map<string, Map<string, number>>,
  menu: string,
): string | undefined {
  const inner = map.get(menu);
  if (!inner) return undefined;
  let best: string | undefined;
  let bestCount = -1;
  for (const [k, c] of inner) {
    if (c > bestCount) {
      best = k;
      bestCount = c;
    }
  }
  return best;
}
