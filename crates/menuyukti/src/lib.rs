//! Menuyukti POS sales analytics library.
//!
//! Domain logic lives here; the MCP binary stays thin and calls into this crate.

pub mod analytics;
pub mod cogs;
pub mod column_mapping;
pub mod excel;
pub mod line_item;
pub mod load;
pub mod normalize;
pub mod paths;
pub mod pos;
pub mod workspace;

/// Library crate version (mirrors package version).
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub use analytics::{
    adapt_weak_review, adapt_weak_review_from_matrix, adapt_weak_review_from_orders,
    calculate_menu_heatmaps, compute_menu_basket_affinities_from_orders,
    compute_menu_engineering_from_orders, compute_slot_demand_profile_from_orders,
    inclusive_calendar_days, list_normalized_rows, period_from_line_items, summarize_sales,
    BasketScope, DailyHeatmapRow, MenuBasketAffinitiesResult, MenuBasketPair,
    MenuEngineeringCoverage, MenuEngineeringDistributionItem, MenuEngineeringMatrixItem,
    MenuEngineeringMatrixResult, MenuEngineeringThresholds, MenuHeatmapPayload,
    NormalizedSalesList, SalesSummary, SlotDemandCell, TopItem, WeakDisplayRow,
    WeakDistributionStats, WeakPortfolioStats, WeakReviewResult, WeeklyHeatmapRow,
    HIGH_DEMAND_THRESHOLD, LOW_DEMAND_THRESHOLD, MAX_FOCUS_ITEMS, MAX_TOP_PAIRS, MIN_CO_OCCURRENCE,
};
pub use cogs::{load_workspace_cogs, parse_cogs_excel, resolve_cogs_path, CogsError, CogsTable};
pub use column_mapping::{
    load_column_mapping, normalize_mapped_excel, try_load_mapping_for_data_dir,
    ColumnMappingConfig, ColumnMappingError, COLUMN_MAPPING_ENV,
};
pub use line_item::{LineItem, REQUIRED_COLUMNS};
pub use load::{load_normalized_sales, resolve_sales_path, LoadError, NormalizedSales};
pub use normalize::{detect_and_normalize, normalize_for_pos, NormalizeError};
pub use paths::{
    get_data_dir, list_workspace_files, list_workspaces, resolve_allowed_dir, resolve_allowed_path,
    PathNotAllowedError, ResolvePathError, WorkspaceEntry, WorkspaceFileEntry, WorkspaceFilesList,
    WorkspaceList, DATA_DIR_ENV,
};
pub use pos::{detect_from_a1, detect_pos_from_excel_bytes};
pub use workspace::{
    load_workspace_config, read_workspace_knowledge, resolve_workspace_cogs_path,
    resolve_workspace_sales_path, write_workspace_config, write_workspace_knowledge,
    WorkspaceConfig, WorkspaceConfigError, WorkspaceConfigLoad, WorkspaceConfigWriteResult,
    WorkspaceFileRole, WorkspaceFiles, WorkspaceKnowledgeRead, WorkspaceKnowledgeWriteResult,
    WorkspacePeriod, DEFAULT_KNOWLEDGE_FILENAME, WORKSPACE_CONFIG_FILENAME,
};
