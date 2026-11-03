//! Menuyukti POS sales analytics library.
//!
//! Domain logic lives here; the MCP binary stays thin and calls into this crate.

pub mod analytics;
pub mod column_mapping;
pub mod excel;
pub mod line_item;
pub mod load;
pub mod normalize;
pub mod paths;
pub mod pos;

/// Library crate version (mirrors package version).
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub use analytics::{
    DailyHeatmapRow, MenuHeatmapPayload, NormalizedSalesList, SalesSummary, TopItem,
    WeeklyHeatmapRow, calculate_menu_heatmaps, list_normalized_rows, summarize_sales,
};
pub use column_mapping::{
    COLUMN_MAPPING_ENV, ColumnMappingConfig, ColumnMappingError, load_column_mapping,
    normalize_mapped_excel, try_load_mapping_for_data_dir,
};
pub use line_item::{LineItem, REQUIRED_COLUMNS};
pub use load::{LoadError, NormalizedSales, load_normalized_sales};
pub use normalize::{NormalizeError, detect_and_normalize, normalize_for_pos};
pub use paths::{
    DATA_DIR_ENV, PathNotAllowedError, ResolvePathError, SalesReportEntry, SalesReportList,
    get_data_dir, list_sales_reports, resolve_allowed_path,
};
pub use pos::{detect_pos_from_excel_bytes, detect_from_a1};
