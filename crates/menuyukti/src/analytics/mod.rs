//! Sales analytics over normalized [`LineItem`] rows.

mod basket_affinities;
mod heatmaps;
mod menu_engineering;
mod slot_demand;
mod summary;
mod weak_review;

pub use basket_affinities::{
    compute_menu_basket_affinities_from_orders, BasketScope, MenuBasketAffinitiesResult,
    MenuBasketPair, MAX_FOCUS_ITEMS, MAX_TOP_PAIRS, MIN_CO_OCCURRENCE,
};
pub use heatmaps::{
    apply_matrix_categories, calculate_menu_heatmaps, DailyHeatmapRow, MenuHeatmapPayload,
    WeeklyHeatmapRow,
};
pub use menu_engineering::{
    compute_menu_engineering_from_orders, MenuEngineeringCoverage, MenuEngineeringDistributionItem,
    MenuEngineeringMatrixItem, MenuEngineeringMatrixResult, MenuEngineeringThresholds,
};
pub use slot_demand::{
    compute_slot_demand_profile_from_orders, SlotDemandCell, HIGH_DEMAND_THRESHOLD,
    LOW_DEMAND_THRESHOLD,
};
pub use summary::{
    list_normalized_rows, summarize_sales, NormalizedSalesList, SalesSummary, TopItem,
};
pub use weak_review::{
    adapt_weak_review, adapt_weak_review_from_matrix, adapt_weak_review_from_orders,
    inclusive_calendar_days, period_from_line_items, WeakDisplayRow, WeakDistributionStats,
    WeakPortfolioStats, WeakReviewResult,
};
