//! First-run setup / health status for Menuyukti data directories.

use std::fs;
use std::path::Path;

use serde::Serialize;

use crate::paths::{get_data_dir, PathNotAllowedError};
use crate::workspace::WORKSPACE_CONFIG_FILENAME;
use crate::VERSION;

/// Default demo workspace folder name shipped with the repo.
pub const DEMO_WORKSPACE_NAME: &str = "demo-cafe-july-2026";

const EXCEL_SUFFIXES: &[&str] = &[".xlsx", ".xls"];

/// One immediate workspace folder under the data dir.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct SetupWorkspaceSummary {
    /// Relative folder name under the data dir.
    pub path: String,
    pub has_workspace_yaml: bool,
    pub excel_file_count: usize,
}

/// Local setup diagnosis for owners and MCP first-contact coaching.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct SetupStatus {
    pub data_dir: String,
    /// Library crate version (same as package version).
    pub version: String,
    pub workspace_count: usize,
    pub workspaces: Vec<SetupWorkspaceSummary>,
    pub folders_missing_workspace_yaml: Vec<String>,
    pub demo_workspace_present: bool,
}

/// Inspect `MENUYUKTI_DATA_DIR` (or `data_dir`) for first-run health.
///
/// Lists immediate subdirectories as candidate workspaces, notes which lack
/// `workspace.yaml`, and whether the default demo folder is present.
pub fn get_setup_status(data_dir: Option<&Path>) -> Result<SetupStatus, PathNotAllowedError> {
    let root = match data_dir {
        Some(p) => p.to_path_buf(),
        None => get_data_dir()?,
    };
    if !root.is_dir() {
        return Err(PathNotAllowedError::NotADirectory(root));
    }

    let mut workspaces = Vec::new();
    let entries =
        fs::read_dir(&root).map_err(|_| PathNotAllowedError::NotADirectory(root.clone()))?;
    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        if name.starts_with('.') {
            continue;
        }
        let has_workspace_yaml = path.join(WORKSPACE_CONFIG_FILENAME).is_file();
        let excel_file_count = count_excel_files(&path);
        workspaces.push(SetupWorkspaceSummary {
            path: name.to_string(),
            has_workspace_yaml,
            excel_file_count,
        });
    }
    workspaces.sort_by(|a, b| a.path.cmp(&b.path));

    let folders_missing_workspace_yaml: Vec<String> = workspaces
        .iter()
        .filter(|w| !w.has_workspace_yaml)
        .map(|w| w.path.clone())
        .collect();
    let demo_workspace_present = workspaces.iter().any(|w| w.path == DEMO_WORKSPACE_NAME);

    Ok(SetupStatus {
        data_dir: display_data_dir(&root),
        version: VERSION.to_string(),
        workspace_count: workspaces.len(),
        workspaces,
        folders_missing_workspace_yaml,
        demo_workspace_present,
    })
}

fn display_data_dir(root: &Path) -> String {
    root.canonicalize()
        .unwrap_or_else(|_| root.to_path_buf())
        .display()
        .to_string()
}

fn count_excel_files(dir: &Path) -> usize {
    let mut count = 0;
    count_excel_files_rec(dir, &mut count);
    count
}

fn count_excel_files_rec(dir: &Path, count: &mut usize) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            count_excel_files_rec(&path, count);
            continue;
        }
        if !path.is_file() {
            continue;
        }
        let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        if name.starts_with("~$") {
            continue;
        }
        let suffix = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| format!(".{}", e.to_ascii_lowercase()))
            .unwrap_or_default();
        if EXCEL_SUFFIXES.contains(&suffix.as_str()) {
            *count += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn setup_status_lists_missing_yaml_and_demo() {
        let root = tempfile::tempdir().unwrap();
        fs::create_dir_all(root.path().join(DEMO_WORKSPACE_NAME)).unwrap();
        fs::write(
            root.path()
                .join(DEMO_WORKSPACE_NAME)
                .join(WORKSPACE_CONFIG_FILENAME),
            "version: 1\n",
        )
        .unwrap();
        fs::write(
            root.path().join(DEMO_WORKSPACE_NAME).join("sales.xlsx"),
            b"x",
        )
        .unwrap();

        fs::create_dir_all(root.path().join("needs-setup")).unwrap();
        fs::write(root.path().join("needs-setup/sales.xlsx"), b"y").unwrap();

        let status = get_setup_status(Some(root.path())).unwrap();
        assert_eq!(status.workspace_count, 2);
        assert!(status.demo_workspace_present);
        assert_eq!(
            status.folders_missing_workspace_yaml,
            vec!["needs-setup".to_string()]
        );
        assert_eq!(status.version, VERSION);
        let demo = status
            .workspaces
            .iter()
            .find(|w| w.path == DEMO_WORKSPACE_NAME)
            .expect("demo workspace");
        assert!(demo.has_workspace_yaml);
        assert_eq!(demo.excel_file_count, 1);
    }

    #[test]
    fn setup_status_rejects_missing_dir() {
        let missing = Path::new("/tmp/menuyukti-setup-status-missing-dir-does-not-exist");
        let err = get_setup_status(Some(missing)).unwrap_err();
        assert!(matches!(err, PathNotAllowedError::NotADirectory(_)));
    }
}
