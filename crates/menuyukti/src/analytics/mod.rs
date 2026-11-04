//! Sales analytics over normalized [`LineItem`] rows.

mod basket_affinities;
mod heatmaps;
mod menu_engineering;
mod summary;

pub use basket_affinities::{
    compute_menu_basket_affinities_from_orders, BasketScope, MenuBasketAffinitiesResult,
    MenuBasketPair, MAX_FOCUS_ITEMS, MAX_TOP_PAIRS, MIN_CO_OCCURRENCE,
};
pub use heatmaps::{
    calculate_menu_heatmaps, DailyHeatmapRow, MenuHeatmapPayload, WeeklyHeatmapRow,
};
pub use menu_engineering::{
    compute_menu_engineering_from_orders, MenuEngineeringCoverage, MenuEngineeringDistributionItem,
    MenuEngineeringMatrixItem, MenuEngineeringMatrixResult, MenuEngineeringThresholds,
};
pub use summary::{
    list_normalized_rows, summarize_sales, NormalizedSalesList, SalesSummary, TopItem,
};
