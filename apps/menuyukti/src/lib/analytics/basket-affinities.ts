import type { LineItem } from "../line-item.js";

export const MAX_FOCUS_ITEMS = 12;
export const MAX_TOP_PAIRS = 25;
export const MIN_CO_OCCURRENCE = 3;

export type BasketScope = "stars" | "top_by_presence";

export type MenuBasketPair = {
  menu_a: string;
  menu_b: string;
  co_order_count: number;
  support: number;
  confidence_a_to_b: number;
  confidence_b_to_a: number;
  lift: number;
  menu_a_category: string | null;
  menu_b_category: string | null;
};

export type MenuBasketAffinitiesResult = {
  total_orders: number;
  multi_item_order_count: number;
  avg_distinct_items_per_order: number;
  scope: BasketScope;
  focus_menus: string[];
  pairs: MenuBasketPair[];
  matrix_lift: Array<Array<number | null>>;
};

function emptyResult(scope: BasketScope): MenuBasketAffinitiesResult {
  return {
    total_orders: 0,
    multi_item_order_count: 0,
    avg_distinct_items_per_order: 0,
    scope,
    focus_menus: [],
    pairs: [],
    matrix_lift: [],
  };
}

function round6(v: number): number {
  return Math.round(v * 1_000_000) / 1_000_000;
}

function round4(v: number): number {
  return Math.round(v * 10_000) / 10_000;
}

function categoryLabel(raw: string): string | null {
  const trimmed = raw.trim();
  return trimmed ? trimmed : null;
}

function ordersWithMenus(rows: LineItem[]): Set<string>[] {
  const byBill = new Map<string, Set<string>>();
  for (const row of rows) {
    const bill = row.bill_number.trim();
    const menu = row.menu.trim();
    if (!bill || !menu) continue;
    const set = byBill.get(bill) ?? new Set<string>();
    set.add(menu);
    byBill.set(bill, set);
  }
  return [...byBill.values()].filter((s) => s.size > 0);
}

function menuCategoryByMenu(
  rows: LineItem[],
): Map<string, string | null> {
  const out = new Map<string, string | null>();
  for (const row of rows) {
    const menu = row.menu.trim();
    if (!menu || out.has(menu)) continue;
    out.set(menu, categoryLabel(row.menu_category));
  }
  return out;
}

function resolveFocusMenus(
  orders: Set<string>[],
  focusMenus?: string[],
): { focus: string[]; scope: BasketScope } {
  if (focusMenus) {
    const seen = new Set<string>();
    const unique: string[] = [];
    for (const menu of focusMenus) {
      const cleaned = menu.trim();
      if (!cleaned || seen.has(cleaned)) continue;
      seen.add(cleaned);
      unique.push(cleaned);
    }
    if (unique.length >= 2) {
      return {
        focus: unique.slice(0, MAX_FOCUS_ITEMS),
        scope: "stars",
      };
    }
  }

  const presence = new Map<string, number>();
  for (const orderMenus of orders) {
    for (const menu of orderMenus) {
      presence.set(menu, (presence.get(menu) ?? 0) + 1);
    }
  }
  const ranked = [...presence.entries()].sort(
    (a, b) => b[1] - a[1] || a[0].localeCompare(b[0]),
  );
  return {
    focus: ranked.slice(0, MAX_FOCUS_ITEMS).map(([menu]) => menu),
    scope: "top_by_presence",
  };
}

function canonicalPairKey(a: string, b: string): string {
  return a <= b ? `${a}\0${b}` : `${b}\0${a}`;
}

function buildMatrixLift(
  focus: string[],
  liftByPair: Map<string, number>,
): Array<Array<number | null>> {
  const n = focus.length;
  const grid: Array<Array<number | null>> = Array.from({ length: n }, () =>
    Array.from({ length: n }, () => null),
  );
  for (let i = 0; i < n; i += 1) {
    for (let j = 0; j < n; j += 1) {
      if (i === j) continue;
      const key = canonicalPairKey(focus[i]!, focus[j]!);
      const lift = liftByPair.get(key);
      if (lift !== undefined) grid[i]![j] = lift;
    }
  }
  return grid;
}

export function computeMenuBasketAffinitiesFromOrders(
  rows: LineItem[],
  focusMenus?: string[],
): MenuBasketAffinitiesResult {
  if (rows.length === 0) return emptyResult("top_by_presence");

  const orders = ordersWithMenus(rows);
  const totalOrders = orders.length;
  if (totalOrders === 0) return emptyResult("top_by_presence");

  const distinctSum = orders.reduce((s, o) => s + o.size, 0);
  const avgDistinct = distinctSum / totalOrders;
  const multiItemOrderCount = orders.filter((o) => o.size >= 2).length;

  const { focus, scope } = resolveFocusMenus(orders, focusMenus);
  if (focus.length < 2) {
    return {
      total_orders: totalOrders,
      multi_item_order_count: multiItemOrderCount,
      avg_distinct_items_per_order: round4(avgDistinct),
      scope,
      focus_menus: focus,
      pairs: [],
      matrix_lift: [],
    };
  }

  const focusSet = new Set(focus);
  const menuCategory = menuCategoryByMenu(rows);
  const ordersWithMenu = new Map<string, number>();
  const pairCounts = new Map<string, number>();

  for (const orderMenus of orders) {
    const inFocus = [...orderMenus].filter((m) => focusSet.has(m)).sort();
    for (const menu of inFocus) {
      ordersWithMenu.set(menu, (ordersWithMenu.get(menu) ?? 0) + 1);
    }
    if (inFocus.length < 2) continue;
    for (let i = 0; i < inFocus.length; i += 1) {
      for (let j = i + 1; j < inFocus.length; j += 1) {
        const key = `${inFocus[i]}\0${inFocus[j]}`;
        pairCounts.set(key, (pairCounts.get(key) ?? 0) + 1);
      }
    }
  }

  if (multiItemOrderCount === 0) {
    return {
      total_orders: totalOrders,
      multi_item_order_count: 0,
      avg_distinct_items_per_order: round4(avgDistinct),
      scope,
      focus_menus: focus,
      pairs: [],
      matrix_lift: buildMatrixLift(focus, new Map()),
    };
  }

  const pairs: MenuBasketPair[] = [];
  const liftByPair = new Map<string, number>();

  for (const [key, coCount] of pairCounts) {
    if (coCount < MIN_CO_OCCURRENCE) continue;
    const [menuA, menuB] = key.split("\0") as [string, string];
    const ordersA = ordersWithMenu.get(menuA) ?? 0;
    const ordersB = ordersWithMenu.get(menuB) ?? 0;
    if (ordersA === 0 || ordersB === 0) continue;
    const support = coCount / multiItemOrderCount;
    const pA = ordersA / totalOrders;
    const pB = ordersB / totalOrders;
    const denominator = pA * pB;
    const lift = round6(denominator > 0 ? support / denominator : 0);
    liftByPair.set(key, lift);
    pairs.push({
      menu_a: menuA,
      menu_b: menuB,
      co_order_count: coCount,
      support: round6(support),
      confidence_a_to_b: round6(coCount / ordersA),
      confidence_b_to_a: round6(coCount / ordersB),
      lift,
      menu_a_category: menuCategory.get(menuA) ?? null,
      menu_b_category: menuCategory.get(menuB) ?? null,
    });
  }

  pairs.sort(
    (a, b) =>
      b.lift - a.lift ||
      b.co_order_count - a.co_order_count ||
      a.menu_a.localeCompare(b.menu_a) ||
      a.menu_b.localeCompare(b.menu_b),
  );
  pairs.splice(MAX_TOP_PAIRS);

  return {
    total_orders: totalOrders,
    multi_item_order_count: multiItemOrderCount,
    avg_distinct_items_per_order: round4(avgDistinct),
    scope,
    focus_menus: focus,
    pairs,
    matrix_lift: buildMatrixLift(focus, liftByPair),
  };
}
