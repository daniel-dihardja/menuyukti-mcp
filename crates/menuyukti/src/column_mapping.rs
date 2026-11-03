//! Owner column-mapping for custom / unrecognized POS Excel exports.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use regex::Regex;
use serde::Deserialize;
use thiserror::Error;

use crate::excel::{
    cell_as_datetime, cell_as_f64, cell_as_i64, cell_as_string, get_cell, normalize_header,
    read_table,
};
use crate::line_item::{LineItem, REQUIRED_COLUMNS};
use crate::normalize::NormalizeError;

pub const COLUMN_MAPPING_ENV: &str = "MENUYUKTI_COLUMN_MAPPING";
pub const DEFAULT_MAPPING_FILENAME: &str = "pos-column-mapping.json";
pub const DEFAULT_POS_NAME: &str = "custom";

#[derive(Debug, Clone, Deserialize)]
pub struct ColumnMappingConfig {
    #[serde(default = "default_pos_name")]
    pub name: String,
    #[serde(default)]
    pub skip_rows: usize,
    pub columns: HashMap<String, String>,
}

fn default_pos_name() -> String {
    DEFAULT_POS_NAME.to_string()
}

#[derive(Debug, Error)]
pub enum ColumnMappingError {
    #[error("{0}")]
    Message(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

impl From<ColumnMappingError> for NormalizeError {
    fn from(value: ColumnMappingError) -> Self {
        NormalizeError::Message(value.to_string())
    }
}

pub fn mapping_config_path(data_dir: &Path) -> Option<PathBuf> {
    if let Ok(raw) = std::env::var(COLUMN_MAPPING_ENV) {
        let trimmed = raw.trim();
        if !trimmed.is_empty() {
            return Some(expand_user(Path::new(trimmed)));
        }
    }
    let candidate = data_dir.join(DEFAULT_MAPPING_FILENAME);
    if candidate.is_file() {
        Some(candidate)
    } else {
        None
    }
}

pub fn try_load_mapping_for_data_dir(
    data_dir: &Path,
) -> Result<Option<ColumnMappingConfig>, ColumnMappingError> {
    let Some(path) = mapping_config_path(data_dir) else {
        return Ok(None);
    };
    if !path.is_file() {
        return Err(ColumnMappingError::Message(format!(
            "Column-mapping file from {COLUMN_MAPPING_ENV} was not found: {}",
            path.display()
        )));
    }
    Ok(Some(load_column_mapping(&path)?))
}

pub fn load_column_mapping(path: &Path) -> Result<ColumnMappingConfig, ColumnMappingError> {
    let text = fs::read_to_string(path)?;
    let mut config: ColumnMappingConfig = serde_json::from_str(&text)?;
    if config.columns.is_empty() {
        return Err(ColumnMappingError::Message(format!(
            "Column-mapping config needs a non-empty 'columns' object: {}",
            path.display()
        )));
    }

    let mut cleaned = HashMap::new();
    for (field, source) in &config.columns {
        let field_key = field.trim();
        let source_key = source.trim();
        if field_key.is_empty() || source_key.is_empty() {
            return Err(ColumnMappingError::Message(format!(
                "Column-mapping entries cannot be empty: {}",
                path.display()
            )));
        }
        if source_key.contains("<YOUR_") {
            return Err(ColumnMappingError::Message(format!(
                "Column-mapping still contains placeholder '{source_key}'. \
                 Replace every <YOUR_…_COLUMN> value with your spreadsheet header."
            )));
        }
        cleaned.insert(field_key.to_string(), source_key.to_string());
    }

    let missing: Vec<_> = REQUIRED_COLUMNS
        .iter()
        .copied()
        .filter(|field| !cleaned.contains_key(*field))
        .collect();
    if !missing.is_empty() {
        return Err(ColumnMappingError::Message(format!(
            "Column-mapping is missing required fields: {missing:?}. Required: {REQUIRED_COLUMNS:?}"
        )));
    }
    let unknown: Vec<_> = cleaned
        .keys()
        .filter(|k| !REQUIRED_COLUMNS.contains(&k.as_str()))
        .cloned()
        .collect();
    if !unknown.is_empty() {
        return Err(ColumnMappingError::Message(format!(
            "Column-mapping has unknown fields: {unknown:?}. Allowed: {REQUIRED_COLUMNS:?}"
        )));
    }

    config.name = safe_pos_name(&config.name);
    config.columns = cleaned;
    Ok(config)
}

pub fn normalize_mapped_excel(
    data: &[u8],
    config: &ColumnMappingConfig,
) -> Result<Vec<LineItem>, ColumnMappingError> {
    let (headers, rows) = read_table(data, config.skip_rows).map_err(|e| {
        ColumnMappingError::Message(e.to_string())
    })?;

    let mut rename: HashMap<String, String> = HashMap::new();
    let mut missing_headers = Vec::new();
    for (field, header) in &config.columns {
        let normalized_header = normalize_header(header);
        if !headers.iter().any(|h| h == &normalized_header) {
            missing_headers.push(header.clone());
            continue;
        }
        rename.insert(normalized_header, field.clone());
    }
    if !missing_headers.is_empty() {
        return Err(ColumnMappingError::Message(format!(
            "Mapped spreadsheet headers were not found after reading the export: {missing_headers:?}. \
             Available columns: {headers:?}"
        )));
    }

    let mut out = Vec::new();
    for row in rows {
        let mut remapped = HashMap::new();
        for (header, value) in row {
            if let Some(field) = rename.get(&header) {
                remapped.insert(field.clone(), value);
            }
        }
        if let Some(item) = try_mapped_line_item(&remapped) {
            out.push(item);
        }
    }
    if out.is_empty() {
        return Err(ColumnMappingError::Message(
            "Mapped export produced no valid rows after type conversion.".into(),
        ));
    }
    Ok(out)
}

fn try_mapped_line_item(row: &HashMap<String, calamine::Data>) -> Option<LineItem> {
    let bill_number = cell_as_string(get_cell(row, "bill_number")?)?.trim().to_string();
    let menu = cell_as_string(get_cell(row, "menu")?)?.trim().to_string();
    if bill_number.is_empty() || menu.is_empty() {
        return None;
    }
    let qty = cell_as_i64(get_cell(row, "qty")?)?;
    let price = cell_as_f64(get_cell(row, "price")?)?;
    let total = cell_as_f64(get_cell(row, "total_after_bill_discount")?)?;
    let order_time = cell_as_datetime(get_cell(row, "order_time")?)?;
    let menu_category = cell_as_string(get_cell(row, "menu_category")?)?
        .trim()
        .to_string();
    let menu_category_detail = cell_as_string(get_cell(row, "menu_category_detail")?)?
        .trim()
        .to_string();
    if menu_category.is_empty() || menu_category_detail.is_empty() {
        return None;
    }
    Some(LineItem {
        bill_number,
        menu,
        qty,
        price,
        total_after_bill_discount: total,
        order_time,
        menu_category,
        menu_category_detail,
    })
}

fn safe_pos_name(name: &str) -> String {
    let re = Regex::new(r"[^a-z0-9_-]+").unwrap();
    let lower = name.to_lowercase();
    let slug = re.replace_all(&lower, "-");
    let slug = slug.trim_matches('-');
    if slug.is_empty() {
        DEFAULT_POS_NAME.to_string()
    } else {
        slug.to_string()
    }
}

fn expand_user(path: &Path) -> PathBuf {
    let raw = path.to_string_lossy();
    if let Some(rest) = raw.strip_prefix("~/") {
        if let Some(home) = std::env::var_os("HOME") {
            return PathBuf::from(home).join(rest);
        }
    }
    path.to_path_buf()
}
