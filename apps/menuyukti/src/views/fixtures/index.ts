/**
 * Mock structuredContent for analytics widget preview + React tests.
 * No Excel / MCP required.
 */
import type {
  MenuBasketPair,
  MenuHeatmapPayload,
  SlotDemandCell,
  WeakDisplayRow,
  WeakPortfolioStats,
} from "../../lib/analytics/index.js";

export type FixtureKind = "populated" | "empty" | "error";

export const venueSlotPopulated = {
  pos: "esb",
  slot_count: 35,
  slot_demand_profile: [
    {
      day: "mon",
      meal_period: "lunch",
      meal_period_label: "Lunch",
      meal_period_hours_label: "11:00–14:59",
      order_count: 12,
      traffic_share: 0.08,
      demand_index: 1.4,
      relative_demand: "high",
    },
    {
      day: "tue",
      meal_period: "lunch",
      meal_period_label: "Lunch",
      meal_period_hours_label: "11:00–14:59",
      order_count: 9,
      traffic_share: 0.06,
      demand_index: 1.05,
      relative_demand: "average",
    },
    {
      day: "wed",
      meal_period: "dinner",
      meal_period_label: "Dinner",
      meal_period_hours_label: "17:00–21:59",
      order_count: 15,
      traffic_share: 0.1,
      demand_index: 1.75,
      relative_demand: "high",
    },
  ] satisfies SlotDemandCell[],
};

export const venueSlotEmpty = {
  pos: "esb",
  slot_count: 0,
  slot_demand_profile: [] as SlotDemandCell[],
};

export const venueSlotError = { error: "Sales file not found under MENUYUKTI_DATA_DIR." };

export const pairLiftPopulated = {
  pos: "esb",
  total_orders: 40,
  multi_item_order_count: 22,
  avg_distinct_items_per_order: 2.1,
  scope: "top_by_presence" as const,
  focus_menus: ["Latte", "Croissant", "Americano"],
  matrix_lift: [
    [null, 1.8, 1.1],
    [1.8, null, 0.9],
    [1.1, 0.9, null],
  ],
  pairs: [
    {
      menu_a: "Latte",
      menu_b: "Croissant",
      co_order_count: 10,
      support: 0.25,
      confidence_a_to_b: 0.5,
      confidence_b_to_a: 0.6,
      lift: 1.8,
      menu_a_category: "Drinks",
      menu_b_category: "Bakery",
    },
  ] satisfies MenuBasketPair[],
};

export const pairLiftEmpty = {
  pos: "esb",
  total_orders: 0,
  multi_item_order_count: 0,
  avg_distinct_items_per_order: 0,
  scope: "top_by_presence" as const,
  focus_menus: [] as string[],
  matrix_lift: [] as Array<Array<number | null>>,
  pairs: [] as MenuBasketPair[],
};

export const pairLiftError = {
  error: "Path outside MENUYUKTI_DATA_DIR was rejected.",
};

function heatItem(
  menu: string,
  matrix: string | null,
  daily: Array<[number, number]>,
): MenuHeatmapPayload {
  return {
    menu,
    menu_category: "Food",
    menu_category_detail: null,
    matrix_category: matrix,
    daily_heatmap: daily.map(([hour, quantity]) => ({ hour, quantity })),
    weekly_heatmap: [
      { day: "mon", quantity: 4 },
      { day: "tue", quantity: 6 },
      { day: "wed", quantity: 5 },
      { day: "thu", quantity: 7 },
      { day: "fri", quantity: 9 },
      { day: "sat", quantity: 3 },
      { day: "sun", quantity: 2 },
    ],
    reporting_period: "2026-07",
  };
}

export const menuHeatmapsPopulated = {
  pos: "esb",
  menu_count: 3,
  menu_total: 98,
  limit_applied: 3,
  active_hours_only: true,
  heatmaps: [
    heatItem("Latte", "star", [
      [9, 2],
      [10, 5],
      [11, 8],
    ]),
    heatItem("Croissant", "plow_horse", [
      [9, 3],
      [10, 4],
      [11, 2],
    ]),
    heatItem("Mystery Soup", "weak", [
      [12, 1],
      [13, 1],
    ]),
  ],
};

export const menuHeatmapsEmpty = {
  pos: "esb",
  menu_count: 0,
  menu_total: 0,
  limit_applied: null,
  active_hours_only: true,
  heatmaps: [] as MenuHeatmapPayload[],
};

export const menuHeatmapsError = {
  error: "Could not normalize sales Excel for this path.",
};

const weakStats: WeakPortfolioStats = {
  item_count: 2,
  item_share: 0.12,
  margin_share: 0.04,
  margin_amount: 120,
  total_menu_items: 40,
  period_days: 31,
};

export const weakItemsPopulated = {
  pos: "esb",
  workspace: "demo-cafe-july-2026",
  cogs_path: "demo-cafe-july-2026/cogs.xlsx",
  period_start: "2026-07-01",
  period_end: "2026-07-31",
  stats: weakStats,
  distribution_stats: {
    item_count: 2,
    item_share: 0.12,
    margin_share: 0.04,
  },
  items: [
    {
      menu: "Mystery Soup",
      units_sold: 8,
      avg_qty_per_day: 0.26,
      contribution_margin: 40,
      contribution_margin_share: 0.01,
    },
    {
      menu: "Stale Muffin",
      units_sold: 5,
      avg_qty_per_day: 0.16,
      contribution_margin: 20,
      contribution_margin_share: 0.005,
    },
  ] satisfies WeakDisplayRow[],
  coverage: { matched: 40, unmatched: 0 },
};

export const weakItemsEmpty = {
  ...weakItemsPopulated,
  stats: { ...weakStats, item_count: 0, item_share: 0, margin_share: 0, margin_amount: 0 },
  distribution_stats: { item_count: 0, item_share: 0, margin_share: 0 },
  items: [] as WeakDisplayRow[],
};

export const weakItemsError = {
  error:
    "COGS file missing. Weak items review needs sales + a unit-cost (COGS) Excel.",
};

export const widgetFixtures = {
  "venue-slot-strength": {
    populated: venueSlotPopulated,
    empty: venueSlotEmpty,
    error: venueSlotError,
  },
  "pair-lift-matrix": {
    populated: pairLiftPopulated,
    empty: pairLiftEmpty,
    error: pairLiftError,
  },
  "menu-heatmaps": {
    populated: menuHeatmapsPopulated,
    empty: menuHeatmapsEmpty,
    error: menuHeatmapsError,
  },
  "weak-items-review": {
    populated: weakItemsPopulated,
    empty: weakItemsEmpty,
    error: weakItemsError,
  },
} as const;

export type WidgetFixtureId = keyof typeof widgetFixtures;
