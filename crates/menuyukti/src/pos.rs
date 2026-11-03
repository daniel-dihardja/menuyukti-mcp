//! POS detection and registry metadata.

use crate::excel::{read_a1_string, ExcelError};

/// Known POS systems and their Excel title / skip_rows configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PosConfig {
    pub key: &'static str,
    pub detection_pattern: &'static str,
    pub skip_rows: usize,
}

pub const POS_CONFIG: &[PosConfig] = &[
    PosConfig {
        key: "esb",
        detection_pattern: "sales recapitulation",
        skip_rows: 10,
    },
    PosConfig {
        key: "quino",
        detection_pattern: "transaction item detail report",
        skip_rows: 3,
    },
    PosConfig {
        key: "moka",
        detection_pattern: "item details",
        skip_rows: 3,
    },
];

pub fn detect_from_a1(cell_value: &str) -> Option<&'static str> {
    let v = cell_value.to_lowercase();
    POS_CONFIG
        .iter()
        .find(|cfg| v.contains(cfg.detection_pattern))
        .map(|cfg| cfg.key)
}

pub fn config_for(pos: &str) -> Option<&'static PosConfig> {
    POS_CONFIG.iter().find(|cfg| cfg.key == pos)
}

pub fn detect_pos_from_excel_bytes(data: &[u8]) -> Result<Option<&'static str>, ExcelError> {
    let a1 = read_a1_string(data)?;
    Ok(a1.as_deref().and_then(detect_from_a1))
}
