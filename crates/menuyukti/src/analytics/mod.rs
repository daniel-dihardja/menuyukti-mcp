//! Sales analytics over normalized [`LineItem`] rows.

mod heatmaps;
mod summary;

pub use heatmaps::{
    calculate_menu_heatmaps, DailyHeatmapRow, MenuHeatmapPayload, WeeklyHeatmapRow,
};
pub use summary::{
    list_normalized_rows, summarize_sales, NormalizedSalesList, SalesSummary, TopItem,
};
