/**
 * Zod output shapes for analytics get_* / render_* structuredContent.
 * Keep in sync with builders in analytics-payloads.ts.
 */
import { z } from "zod";

export const slotDemandCellSchema = z.object({
  day: z.string(),
  meal_period: z.string(),
  meal_period_label: z.string(),
  meal_period_hours_label: z.string(),
  order_count: z.number(),
  traffic_share: z.number(),
  demand_index: z.number(),
  relative_demand: z.string(),
});

export const venueSlotPayloadSchema = {
  pos: z.string(),
  slot_count: z.number(),
  slot_demand_profile: z.array(slotDemandCellSchema),
};

export const menuBasketPairSchema = z.object({
  menu_a: z.string(),
  menu_b: z.string(),
  co_order_count: z.number(),
  support: z.number(),
  confidence_a_to_b: z.number(),
  confidence_b_to_a: z.number(),
  lift: z.number(),
  menu_a_category: z.string().nullable(),
  menu_b_category: z.string().nullable(),
});

export const pairLiftPayloadSchema = {
  pos: z.string(),
  total_orders: z.number(),
  multi_item_order_count: z.number(),
  avg_distinct_items_per_order: z.number(),
  scope: z.enum(["stars", "top_by_presence"]),
  focus_menus: z.array(z.string()),
  matrix_lift: z.array(z.array(z.number().nullable())),
  pairs: z.array(menuBasketPairSchema),
};

export const dailyHeatmapRowSchema = z.object({
  hour: z.number(),
  quantity: z.number(),
});

export const weeklyHeatmapRowSchema = z.object({
  day: z.string(),
  quantity: z.number(),
});

export const menuHeatmapItemSchema = z.object({
  menu: z.string(),
  menu_category: z.string().nullable(),
  menu_category_detail: z.string().nullable(),
  matrix_category: z.string().nullable(),
  daily_heatmap: z.array(dailyHeatmapRowSchema),
  weekly_heatmap: z.array(weeklyHeatmapRowSchema),
  reporting_period: z.string(),
});

export const menuHeatmapsPayloadSchema = {
  pos: z.string(),
  menu_count: z.number(),
  menu_total: z.number(),
  limit_applied: z.number().nullable(),
  active_hours_only: z.boolean(),
  heatmaps: z.array(menuHeatmapItemSchema),
};

export const weakDisplayRowSchema = z.object({
  menu: z.string(),
  units_sold: z.number(),
  avg_qty_per_day: z.number().nullable(),
  contribution_margin: z.number(),
  contribution_margin_share: z.number(),
});

export const weakPortfolioStatsSchema = z.object({
  item_count: z.number(),
  item_share: z.number(),
  margin_share: z.number(),
  margin_amount: z.number(),
  total_menu_items: z.number(),
  period_days: z.number().nullable(),
});

export const weakDistributionStatsSchema = z.object({
  item_count: z.number(),
  item_share: z.number(),
  margin_share: z.number(),
});

export const weakItemsPayloadSchema = {
  pos: z.string(),
  workspace: z.string(),
  cogs_path: z.string(),
  period_start: z.string().nullable(),
  period_end: z.string().nullable(),
  stats: weakPortfolioStatsSchema,
  distribution_stats: weakDistributionStatsSchema,
  items: z.array(weakDisplayRowSchema),
  coverage: z.unknown(),
};
