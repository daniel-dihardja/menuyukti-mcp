export {
  computeMenuBasketAffinitiesFromOrders,
  type MenuBasketAffinitiesResult,
  type MenuBasketPair,
} from "./basket-affinities.js";
export {
  enrichedMenuHeatmaps,
  optionalMatrixCategories,
  optionalStarFocus,
} from "./enrichment.js";
export {
  applyMatrixCategories,
  calculateMenuHeatmaps,
  type DailyHeatmapRow,
  DEFAULT_MENU_HEATMAP_TOP_N,
  type MenuHeatmapPayload,
  pruneMenuHeatmaps,
  type WeeklyHeatmapRow,
} from "./heatmaps.js";
export {
  computeMenuEngineeringFromOrders,
  type MenuEngineeringMatrixResult,
} from "./menu-engineering.js";
export {
  computeSlotDemandProfileFromOrders,
  type SlotDemandCell,
} from "./slot-demand.js";
export {
  adaptWeakReviewFromOrders,
  type WeakDisplayRow,
  type WeakPortfolioStats,
  type WeakReviewResult,
} from "./weak-review.js";
