//! Sales analytics over normalized [`LineItem`] rows.

mod heatmaps;
mod summary;

pub use heatmaps::{
    DailyHeatmapRow, MenuHeatmapPayload, WeeklyHeatmapRow, calculate_menu_heatmaps,
};
pub use summary::{
    NormalizedSalesList, SalesSummary, TopItem, list_normalized_rows, summarize_sales,
};
