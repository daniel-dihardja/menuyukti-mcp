//! Path-gated workspace COGS Excel load → menu unit-cost map.

use std::collections::HashMap;
use std::fs;
use std::path::Path;

use thiserror::Error;

use crate::excel::{cell_as_f64, cell_as_string, read_table, read_table_sheet, ExcelError, RowMap};
use crate::paths::{get_data_dir, resolve_allowed_path, ResolvePathError};
use crate::workspace::{resolve_workspace_cogs_path, WorkspaceConfigError};

/// Preferred worksheet name in owner COGS workbooks (demo uses this).
pub const COGS_SHEET_NAME: &str = "COGS";

#[derive(Debug, Clone, PartialEq)]
pub struct CogsTable {
    /// Path relative to `MENUYUKTI_DATA_DIR`.
    pub path: String,
    /// Menu name → unit cost.
    pub unit_costs: HashMap<String, f64>,
}

#[derive(Debug, Error)]
pub enum CogsError {
    #[error(transparent)]
    Path(#[from] ResolvePathError),
    #[error(transparent)]
    Workspace(#[from] WorkspaceConfigError),
    #[error(transparent)]
    Excel(#[from] ExcelError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Invalid(String),
}

/// Resolve a workspace folder to its COGS Excel path (relative to the data dir).
pub fn resolve_cogs_path(path: &str, data_dir: Option<&Path>) -> Result<String, CogsError> {
    Ok(resolve_workspace_cogs_path(path.trim(), data_dir)?)
}

/// Gate `path`, read Menu + COGS columns, and return a unit-cost map.
///
/// `path` must be a workspace folder whose `workspace.yaml` declares `files.cogs`.
/// Prefers a worksheet named `COGS`; falls back to the first sheet when that is absent.
pub fn load_workspace_cogs(
    workspace: &str,
    data_dir: Option<&Path>,
) -> Result<CogsTable, CogsError> {
    let root = match data_dir {
        Some(p) => p.to_path_buf(),
        None => get_data_dir().map_err(|e| CogsError::Path(e.into()))?,
    };
    let cogs_rel = resolve_cogs_path(workspace, Some(&root))?;
    let allowed = resolve_allowed_path(&cogs_rel, Some(&root))?;
    if !allowed.is_file() {
        return Err(CogsError::Invalid(format!(
            "COGS file not found: {cogs_rel}. \
             Drop a COGS Excel into the workspace and update files.cogs via write_workspace_config."
        )));
    }
    let payload = fs::read(&allowed)?;
    let unit_costs = parse_cogs_excel(&payload)?;
    if unit_costs.is_empty() {
        return Err(CogsError::Invalid(format!(
            "COGS file has no usable Menu/COGS rows: {cogs_rel}"
        )));
    }
    Ok(CogsTable {
        path: cogs_rel,
        unit_costs,
    })
}

/// Parse COGS Excel bytes into menu → unit cost.
pub fn parse_cogs_excel(data: &[u8]) -> Result<HashMap<String, f64>, CogsError> {
    let (headers, rows) = match read_table_sheet(data, COGS_SHEET_NAME, 0) {
        Ok(table) => table,
        Err(_) => read_table(data, 0)?,
    };
    if !headers.iter().any(|h| h == "menu") || !headers.iter().any(|h| h == "cogs") {
        return Err(CogsError::Invalid(
            "COGS Excel must include Menu and COGS columns (found sheet headers without both)"
                .into(),
        ));
    }
    Ok(rows_to_unit_costs(&rows))
}

fn rows_to_unit_costs(rows: &[RowMap]) -> HashMap<String, f64> {
    let mut out = HashMap::new();
    for row in rows {
        let Some(menu) = row.get("menu").and_then(cell_as_string) else {
            continue;
        };
        let menu = menu.trim();
        if menu.is_empty() {
            continue;
        }
        let Some(cost) = row.get("cogs").and_then(cell_as_f64) else {
            continue;
        };
        if !cost.is_finite() || cost < 0.0 {
            continue;
        }
        out.insert(menu.to_string(), cost);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workspace::{
        write_workspace_config, WorkspaceConfig, WorkspaceFileRole, WorkspaceFiles, WorkspacePeriod,
    };
    use std::sync::{Mutex, OnceLock};

    fn env_lock() -> &'static Mutex<()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(()))
    }

    fn sales_reports() -> std::path::PathBuf {
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../workspaces")
            .canonicalize()
            .expect("workspaces")
    }

    fn sample_config(with_cogs: bool) -> WorkspaceConfig {
        WorkspaceConfig {
            version: 1,
            name: "Test".into(),
            venue: "Test".into(),
            period: Some(WorkspacePeriod {
                label: Some("July 2026".into()),
                start: None,
                end: None,
            }),
            currency: Some("IDR".into()),
            files: WorkspaceFiles {
                sales: WorkspaceFileRole {
                    path: "sales.xlsx".into(),
                    description: None,
                },
                cogs: with_cogs.then_some(WorkspaceFileRole {
                    path: "cogs.xlsx".into(),
                    description: None,
                }),
                knowledge: None,
            },
        }
    }

    #[test]
    fn loads_demo_cafe_cogs() {
        let loaded = load_workspace_cogs("demo-cafe-july-2026", Some(&sales_reports()))
            .expect("demo cogs load");
        assert_eq!(loaded.path, "demo-cafe-july-2026/cogs.xlsx");
        assert!(!loaded.unit_costs.is_empty());
        assert!(
            loaded.unit_costs.contains_key("Iced Milk Coffee Aren")
                || loaded.unit_costs.values().any(|&v| v > 0.0),
            "expected at least one positive unit cost; keys={:?}",
            loaded.unit_costs.keys().take(5).collect::<Vec<_>>()
        );
        let avocado = loaded
            .unit_costs
            .get("Avocado Chocolate")
            .copied()
            .expect("Avocado Chocolate in demo COGS");
        assert!((avocado - 8000.0).abs() < f64::EPSILON);
    }

    #[test]
    fn missing_cogs_role_errors() {
        let _guard = env_lock().lock().unwrap();
        let root = tempfile::tempdir().unwrap();
        fs::create_dir_all(root.path().join("cafe")).unwrap();
        write_workspace_config("cafe", &sample_config(false), Some(root.path())).unwrap();

        let err = load_workspace_cogs("cafe", Some(root.path())).unwrap_err();
        assert!(
            err.to_string().contains("files.cogs"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn missing_cogs_file_errors() {
        let _guard = env_lock().lock().unwrap();
        let root = tempfile::tempdir().unwrap();
        fs::create_dir_all(root.path().join("cafe")).unwrap();
        write_workspace_config("cafe", &sample_config(true), Some(root.path())).unwrap();

        let err = load_workspace_cogs("cafe", Some(root.path())).unwrap_err();
        let msg = err.to_string();
        assert!(
            msg.contains("not found") || msg.contains("COGS file"),
            "unexpected error: {msg}"
        );
    }

    #[test]
    fn path_outside_data_dir_errors() {
        let _guard = env_lock().lock().unwrap();
        let root = tempfile::tempdir().unwrap();
        fs::create_dir_all(root.path().join("cafe")).unwrap();

        let err = load_workspace_cogs("../outside", Some(root.path())).unwrap_err();
        assert!(
            matches!(err, CogsError::Path(_) | CogsError::Workspace(_)),
            "unexpected error: {err}"
        );
    }
}
