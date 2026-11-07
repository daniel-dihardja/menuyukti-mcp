/**
 * Shared structuredContent builders for analytics get_* and render_* tools.
 */
import {
  adaptWeakReviewFromOrders,
  computeMenuBasketAffinitiesFromOrders,
  computeMenuEngineeringFromOrders,
  computeSlotDemandProfileFromOrders,
  DEFAULT_MENU_HEATMAP_TOP_N,
  enrichedMenuHeatmaps,
  optionalStarFocus,
  pruneMenuHeatmaps,
} from "./analytics/index.js";
import { CogsError, loadWorkspaceCogs } from "./cogs.js";
import { loadNormalizedSales } from "./load.js";

export type MenuHeatmapsPayloadOptions = {
  /** Max menus by quantity rank. null/undefined = all. */
  topN?: number | null;
  /** When true, keep only daily hours with demand across the returned menus. */
  activeHoursOnly?: boolean;
};

export function buildMenuHeatmapsPayload(
  path: string,
  options: MenuHeatmapsPayloadOptions = {},
) {
  const loaded = loadNormalizedSales(path);
  const all = enrichedMenuHeatmaps(path, loaded.rows);
  const topN = options.topN === undefined ? null : options.topN;
  const activeHoursOnly = options.activeHoursOnly === true;
  const heatmaps = pruneMenuHeatmaps(all, { topN, activeHoursOnly });
  return {
    pos: loaded.pos,
    menu_count: heatmaps.length,
    menu_total: all.length,
    limit_applied:
      topN == null || !Number.isFinite(topN) || topN <= 0
        ? null
        : Math.min(Math.floor(topN), all.length),
    active_hours_only: activeHoursOnly,
    heatmaps,
  };
}

export { DEFAULT_MENU_HEATMAP_TOP_N };

export function buildVenueSlotPayload(path: string) {
  const loaded = loadNormalizedSales(path);
  const profile = computeSlotDemandProfileFromOrders(loaded.rows);
  return {
    pos: loaded.pos,
    slot_count: profile.length,
    slot_demand_profile: profile,
  };
}

export function buildPairLiftPayload(path: string) {
  const loaded = loadNormalizedSales(path);
  const starFocus = optionalStarFocus(path, loaded.rows);
  const affinities = computeMenuBasketAffinitiesFromOrders(
    loaded.rows,
    starFocus,
  );
  return {
    pos: loaded.pos,
    total_orders: affinities.total_orders,
    multi_item_order_count: affinities.multi_item_order_count,
    avg_distinct_items_per_order: affinities.avg_distinct_items_per_order,
    scope: affinities.scope,
    focus_menus: affinities.focus_menus,
    matrix_lift: affinities.matrix_lift,
    pairs: affinities.pairs,
  };
}

/** Thrown when weak-items cannot load COGS (message is owner-facing). */
export class WeakItemsPayloadError extends Error {
  override readonly name = "WeakItemsPayloadError";
}

export function buildWeakItemsPayload(path: string) {
  let cogs;
  try {
    cogs = loadWorkspaceCogs(path);
  } catch (err) {
    const message =
      err instanceof CogsError || err instanceof Error
        ? err.message
        : String(err);
    throw new WeakItemsPayloadError(
      `${message} Weak items review needs sales + a unit-cost (COGS) Excel: ask the owner to drop COGS into the folder, then list_workspace_files and write_workspace_config (or call get_setup_status).`,
    );
  }
  const loaded = loadNormalizedSales(path);
  try {
    const matrix = computeMenuEngineeringFromOrders(
      loaded.rows,
      cogs.unit_costs,
    );
    const review = adaptWeakReviewFromOrders(matrix, loaded.rows);
    return {
      pos: loaded.pos,
      workspace: path,
      cogs_path: cogs.path,
      period_start: review.period_start,
      period_end: review.period_end,
      stats: review.stats,
      distribution_stats: review.distribution_stats,
      items: review.rows,
      coverage: matrix.coverage,
    };
  } catch (err) {
    const message = err instanceof Error ? err.message : String(err);
    throw new WeakItemsPayloadError(
      `${message}. Fix COGS coverage (Menu + COGS columns matching sold menu names) or ask the owner to update unit costs.`,
    );
  }
}

export type MenuHeatmapsToolPayload = ReturnType<typeof buildMenuHeatmapsPayload>;
export type VenueSlotToolPayload = ReturnType<typeof buildVenueSlotPayload>;
export type PairLiftToolPayload = ReturnType<typeof buildPairLiftPayload>;
export type WeakItemsToolPayload = ReturnType<typeof buildWeakItemsPayload>;
