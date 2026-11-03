//! POS Excel normalizers → [`LineItem`] rows.

mod category;
mod esb;
mod moka;
mod quino;

use thiserror::Error;

use crate::excel::ExcelError;
use crate::line_item::LineItem;
use crate::pos::{config_for, detect_pos_from_excel_bytes};

pub use esb::normalize_esb_excel;
pub use moka::normalize_moka_excel;
pub use quino::normalize_quino_excel;

#[derive(Debug, Error)]
pub enum NormalizeError {
    #[error(transparent)]
    Excel(#[from] ExcelError),
    #[error("{0}")]
    Message(String),
}

/// Normalize bytes for a known POS key.
pub fn normalize_for_pos(pos: &str, data: &[u8]) -> Result<Vec<LineItem>, NormalizeError> {
    let skip = config_for(pos).map(|c| c.skip_rows).unwrap_or(0);
    match pos {
        "esb" => normalize_esb_excel(data, skip),
        "moka" => normalize_moka_excel(data, skip),
        "quino" => normalize_quino_excel(data, skip),
        other => Err(NormalizeError::Message(format!(
            "Unsupported POS system detected: {other}"
        ))),
    }
}

/// Detect POS then normalize. Returns `(pos_key, rows)`.
pub fn detect_and_normalize(data: &[u8]) -> Result<(String, Vec<LineItem>), NormalizeError> {
    let pos = detect_pos_from_excel_bytes(data)?.unwrap_or("unknown");
    if config_for(pos).is_none() {
        return Err(NormalizeError::Message(format!(
            "Unsupported POS system detected: {pos}. \
             Add a pos-column-mapping.json under MENUYUKTI_DATA_DIR \
             (or set MENUYUKTI_COLUMN_MAPPING) to map your export columns."
        )));
    }
    let rows = normalize_for_pos(pos, data)?;
    Ok((pos.to_string(), rows))
}
