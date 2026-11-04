//! Sales analytics over normalized [`LineItem`] rows.

mod heatmaps;
mod menu_engineering;
mod summary;

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
