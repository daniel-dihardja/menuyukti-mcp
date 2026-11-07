export {
  applyMatrixCategories,
  calculateMenuHeatmaps,
  DEFAULT_MENU_HEATMAP_TOP_N,
  pruneMenuHeatmaps,
  type DailyHeatmapRow,
  type MenuHeatmapPayload,
  type WeeklyHeatmapRow,
} from "./heatmaps.js";
export {
  computeSlotDemandProfileFromOrders,
  type SlotDemandCell,
} from "./slot-demand.js";
export {
  computeMenuEngineeringFromOrders,
  type MenuEngineeringMatrixResult,
} from "./menu-engineering.js";
export {
  computeMenuBasketAffinitiesFromOrders,
  type MenuBasketAffinitiesResult,
  type MenuBasketPair,
} from "./basket-affinities.js";
export {
  adaptWeakReviewFromOrders,
  type WeakDisplayRow,
  type WeakPortfolioStats,
  type WeakReviewResult,
} from "./weak-review.js";
export {
  enrichedMenuHeatmaps,
  optionalMatrixCategories,
  optionalStarFocus,
} from "./enrichment.js";
