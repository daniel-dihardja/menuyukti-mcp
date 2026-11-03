//! Gated local POS Excel load → normalized line items.

use std::fs;
use std::path::Path;

use thiserror::Error;

use crate::column_mapping::{
    normalize_mapped_excel, try_load_mapping_for_data_dir, ColumnMappingError,
};
use crate::line_item::LineItem;
use crate::normalize::{normalize_for_pos, NormalizeError};
use crate::paths::{get_data_dir, resolve_allowed_path, ResolvePathError};
use crate::pos::{config_for, detect_pos_from_excel_bytes};
use crate::workspace::{resolve_workspace_sales_path, WorkspaceConfigError};

#[derive(Debug, Clone)]
pub struct NormalizedSales {
    pub pos: String,
    pub rows: Vec<LineItem>,
}

#[derive(Debug, Error)]
pub enum LoadError {
    #[error(transparent)]
    Path(#[from] ResolvePathError),
    #[error(transparent)]
    Workspace(#[from] WorkspaceConfigError),
    #[error(transparent)]
    Normalize(#[from] NormalizeError),
    #[error(transparent)]
    Mapping(#[from] ColumnMappingError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

/// Resolve a sales Excel path: either an explicit `.xlsx`/`.xls` file, or a
/// workspace folder that has `workspace.yaml` with `files.sales`.
pub fn resolve_sales_path(path: &str, data_dir: Option<&Path>) -> Result<String, LoadError> {
    let trimmed = path.trim();
    let lower = trimmed.to_ascii_lowercase();
    if lower.ends_with(".xlsx") || lower.ends_with(".xls") {
        return Ok(trimmed.to_string());
    }
    Ok(resolve_workspace_sales_path(trimmed, data_dir)?)
}

/// Gate `path`, detect POS, and return normalized line items.
///
/// `path` may be an Excel file under `MENUYUKTI_DATA_DIR`, or a workspace
/// folder whose `workspace.yaml` declares `files.sales`.
///
/// Falls back to owner column-mapping when the POS is unrecognized.
pub fn load_normalized_sales(
    path: &str,
    data_dir: Option<&Path>,
) -> Result<NormalizedSales, LoadError> {
    let root = match data_dir {
        Some(p) => p.to_path_buf(),
        None => get_data_dir().map_err(|e| LoadError::Path(e.into()))?,
    };
    let sales_path = resolve_sales_path(path, Some(&root))?;
    let allowed = resolve_allowed_path(&sales_path, Some(&root))?;
    let payload = fs::read(&allowed)?;

    let detected = detect_pos_from_excel_bytes(&payload)
        .map_err(NormalizeError::from)?
        .unwrap_or("unknown");
    if config_for(detected).is_some() {
        let rows = normalize_for_pos(detected, &payload)?;
        return Ok(NormalizedSales {
            pos: detected.to_string(),
            rows,
        });
    }

    match try_load_mapping_for_data_dir(&root)? {
        Some(mapping) => {
            let rows = normalize_mapped_excel(&payload, &mapping)?;
            Ok(NormalizedSales {
                pos: mapping.name,
                rows,
            })
        }
        None => Err(LoadError::Normalize(NormalizeError::Message(format!(
            "Unsupported POS system detected: {detected}. \
             Add a pos-column-mapping.json under MENUYUKTI_DATA_DIR \
             (or set MENUYUKTI_COLUMN_MAPPING) to map your export columns."
        )))),
    }
}
