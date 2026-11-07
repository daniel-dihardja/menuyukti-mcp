import type { LineItem } from "./line-item.js";

export const TOP_ITEMS_LIMIT = 10;

export type TopItem = {
  menu: string;
  popularity: number;
  quantity: number;
};

export type SalesSummary = {
  pos: string;
  period_start: string | null;
  period_end: string | null;
  total_revenue: number;
  total_items_sold: number;
  top_items: TopItem[];
};

export type NormalizedSalesList = {
  pos: string;
  rows: Array<Record<string, string | number>>;
  row_count: number;
};

export function summarizeSales(pos: string, rows: LineItem[]): SalesSummary {
  if (rows.length === 0) {
    throw new Error("DataFrame is empty. Cannot calculate analytics.");
  }
  const totalItemsSold = rows.reduce((sum, r) => sum + r.qty, 0);
  if (totalItemsSold === 0) {
    throw new Error("Total quantity is zero. Cannot calculate analytics.");
  }
  const totalRevenue = rows.reduce(
    (sum, r) => sum + r.total_after_bill_discount,
    0,
  );
  const topItems = popularityIndex(rows).slice(0, TOP_ITEMS_LIMIT);

  let periodStart: string | null = null;
  let periodEnd: string | null = null;
  const allEpoch = rows.every((r) =>
    r.order_time.startsWith("1970-01-01T00:00:00"),
  );
  if (!allEpoch) {
    const dates = rows.map((r) => r.order_time.slice(0, 10)).sort();
    periodStart = dates[0] ?? null;
    periodEnd = dates[dates.length - 1] ?? null;
  }

  return {
    pos,
    period_start: periodStart,
    period_end: periodEnd,
    total_revenue: totalRevenue,
    total_items_sold: totalItemsSold,
    top_items: topItems,
  };
}

export function listNormalizedRows(
  pos: string,
  rows: LineItem[],
): NormalizedSalesList {
  return {
    pos,
    row_count: rows.length,
    rows: rows.map((r) => ({
      bill_number: r.bill_number,
      menu: r.menu,
      qty: r.qty,
      price: r.price,
      total_after_bill_discount: r.total_after_bill_discount,
      order_time: r.order_time,
      menu_category: r.menu_category,
      menu_category_detail: r.menu_category_detail,
    })),
  };
}

function popularityIndex(rows: LineItem[]): TopItem[] {
  const totalQty = rows.reduce((sum, r) => sum + r.qty, 0);
  if (totalQty === 0) {
    throw new Error(
      "Total quantity is zero. Cannot calculate popularity index.",
    );
  }
  const byMenu = new Map<string, number>();
  for (const row of rows) {
    byMenu.set(row.menu, (byMenu.get(row.menu) ?? 0) + row.qty);
  }
  const items: TopItem[] = [...byMenu.entries()].map(([menu, quantity]) => ({
    menu,
    quantity,
    popularity: Math.round((quantity / totalQty) * 1_000_000) / 1_000_000,
  }));
  items.sort((a, b) => b.quantity - a.quantity || a.menu.localeCompare(b.menu));
  return items;
}
