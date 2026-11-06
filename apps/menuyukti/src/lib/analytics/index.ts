export {
  applyMatrixCategories,
  calculateMenuHeatmaps,
  type MenuHeatmapPayload,
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
} from "./basket-affinities.js";
export {
  adaptWeakReviewFromOrders,
  type WeakReviewResult,
} from "./weak-review.js";
export {
  enrichedMenuHeatmaps,
  optionalMatrixCategories,
  optionalStarFocus,
} from "./enrichment.js";
