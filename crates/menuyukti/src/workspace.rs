//! Per-workspace config (`workspace.yaml`) under `MENUYUKTI_DATA_DIR`.

use std::fs;
use std::path::{Component, Path, PathBuf};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::paths::{
    get_data_dir, resolve_allowed_dir, resolve_allowed_path, PathNotAllowedError, ResolvePathError,
};

/// Filename written and read for workspace role metadata.
pub const WORKSPACE_CONFIG_FILENAME: &str = "workspace.yaml";

const SUPPORTED_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorkspaceConfig {
    pub version: u32,
    pub name: String,
    pub venue: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub period: Option<WorkspacePeriod>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    pub files: WorkspaceFiles,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorkspacePeriod {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorkspaceFiles {
    pub sales: WorkspaceFileRole,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cogs: Option<WorkspaceFileRole>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub knowledge: Option<WorkspaceFileRole>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorkspaceFileRole {
    pub path: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct WorkspaceConfigLoad {
    pub data_dir: String,
    pub workspace: String,
    /// Path to `workspace.yaml` relative to the data dir.
    pub config_path: String,
    pub config: WorkspaceConfig,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct WorkspaceConfigWriteResult {
    pub data_dir: String,
    pub workspace: String,
    /// Path to `workspace.yaml` relative to the data dir.
    pub config_path: String,
    pub config: WorkspaceConfig,
}

#[derive(Debug, Error)]
pub enum WorkspaceConfigError {
    #[error(transparent)]
    Path(#[from] ResolvePathError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Yaml(#[from] serde_yaml::Error),
    #[error("{0}")]
    Invalid(String),
}

/// Load `workspace.yaml` from a workspace folder under the data dir.
pub fn load_workspace_config(
    workspace: &str,
    data_dir: Option<&Path>,
) -> Result<WorkspaceConfigLoad, WorkspaceConfigError> {
    let (root, workspace_label, dir) = resolve_workspace_dir(workspace, data_dir)?;
    let config_abs = dir.join(WORKSPACE_CONFIG_FILENAME);
    if !config_abs.is_file() {
        return Err(WorkspaceConfigError::Invalid(format!(
            "Workspace config not found: {}",
            relative_to_root(&root, &config_abs)
        )));
    }
    // Re-gate through the file resolver so traversal tricks in the joined path fail closed.
    let config_rel = relative_to_root(&root, &config_abs);
    let allowed = resolve_allowed_path(&config_rel, Some(&root))?;
    let raw = fs::read_to_string(&allowed)?;
    let config: WorkspaceConfig = serde_yaml::from_str(&raw)?;
    validate_config(&config)?;
    Ok(WorkspaceConfigLoad {
        data_dir: root.display().to_string(),
        workspace: workspace_label,
        config_path: config_rel,
        config,
    })
}

/// Write `workspace.yaml` into a workspace folder under the data dir.
///
/// Overwrites an existing config. Role paths must be relative (no `..`).
pub fn write_workspace_config(
    workspace: &str,
    config: &WorkspaceConfig,
    data_dir: Option<&Path>,
) -> Result<WorkspaceConfigWriteResult, WorkspaceConfigError> {
    validate_config(config)?;
    let (root, workspace_label, dir) = resolve_workspace_dir(workspace, data_dir)?;
    let config_abs = dir.join(WORKSPACE_CONFIG_FILENAME);
    let config_rel = relative_to_root(&root, &config_abs);
    ensure_path_inside_root(&root, &config_abs, &config_rel)?;

    let yaml = serde_yaml::to_string(config)?;
    fs::write(&config_abs, yaml)?;

    Ok(WorkspaceConfigWriteResult {
        data_dir: root.display().to_string(),
        workspace: workspace_label,
        config_path: config_rel,
        config: config.clone(),
    })
}

/// Resolve the sales Excel path for a workspace (relative to the data dir).
pub fn resolve_workspace_sales_path(
    workspace: &str,
    data_dir: Option<&Path>,
) -> Result<String, WorkspaceConfigError> {
    let loaded = load_workspace_config(workspace, data_dir)?;
    join_workspace_role_path("sales", &loaded.workspace, &loaded.config.files.sales.path)
}

/// Resolve the COGS Excel path for a workspace (relative to the data dir).
///
/// Requires `files.cogs` in `workspace.yaml`.
pub fn resolve_workspace_cogs_path(
    workspace: &str,
    data_dir: Option<&Path>,
) -> Result<String, WorkspaceConfigError> {
    let loaded = load_workspace_config(workspace, data_dir)?;
    let Some(cogs) = &loaded.config.files.cogs else {
        return Err(WorkspaceConfigError::Invalid(format!(
            "Workspace config missing files.cogs role: {}/{WORKSPACE_CONFIG_FILENAME}. \
             Add a COGS Excel file, then call write_workspace_config (or ask the owner for unit costs).",
            loaded.workspace
        )));
    };
    join_workspace_role_path("cogs", &loaded.workspace, &cogs.path)
}

fn resolve_workspace_dir(
    workspace: &str,
    data_dir: Option<&Path>,
) -> Result<(PathBuf, String, PathBuf), WorkspaceConfigError> {
    let root = match data_dir {
        Some(p) => p.to_path_buf(),
        None => get_data_dir().map_err(|e| WorkspaceConfigError::Path(e.into()))?,
    };
    let root = root
        .canonicalize()
        .unwrap_or_else(|_| normalize_lexical(&root));
    if !root.is_dir() {
        return Err(WorkspaceConfigError::Path(
            PathNotAllowedError::NotADirectory(root).into(),
        ));
    }
    let dir = resolve_allowed_dir(workspace, Some(&root))?;
    let trimmed = workspace.trim();
    let workspace_label = if trimmed.is_empty() || trimmed == "." {
        ".".to_string()
    } else {
        trimmed.replace('\\', "/")
    };
    Ok((root, workspace_label, dir))
}

fn validate_config(config: &WorkspaceConfig) -> Result<(), WorkspaceConfigError> {
    if config.version != SUPPORTED_VERSION {
        return Err(WorkspaceConfigError::Invalid(format!(
            "Unsupported workspace config version: {} (expected {SUPPORTED_VERSION})",
            config.version
        )));
    }
    if config.name.trim().is_empty() {
        return Err(WorkspaceConfigError::Invalid(
            "Workspace config name must not be empty".into(),
        ));
    }
    if config.venue.trim().is_empty() {
        return Err(WorkspaceConfigError::Invalid(
            "Workspace config venue must not be empty".into(),
        ));
    }
    validate_role_path("sales", &config.files.sales.path)?;
    if let Some(cogs) = &config.files.cogs {
        validate_role_path("cogs", &cogs.path)?;
    }
    if let Some(knowledge) = &config.files.knowledge {
        validate_role_path("knowledge", &knowledge.path)?;
    }
    Ok(())
}

fn validate_role_path(role: &str, path: &str) -> Result<(), WorkspaceConfigError> {
    let trimmed = path.trim();
    if trimmed.is_empty() {
        return Err(WorkspaceConfigError::Invalid(format!(
            "Workspace config files.{role}.path must not be empty"
        )));
    }
    let p = Path::new(trimmed);
    if p.is_absolute() {
        return Err(WorkspaceConfigError::Invalid(format!(
            "Workspace config files.{role}.path must be relative to the workspace folder: {trimmed}"
        )));
    }
    for comp in p.components() {
        match comp {
            Component::ParentDir => {
                return Err(WorkspaceConfigError::Invalid(format!(
                    "Workspace config files.{role}.path must not contain '..': {trimmed}"
                )));
            }
            Component::RootDir | Component::Prefix(_) => {
                return Err(WorkspaceConfigError::Invalid(format!(
                    "Workspace config files.{role}.path must be relative: {trimmed}"
                )));
            }
            Component::CurDir | Component::Normal(_) => {}
        }
    }
    Ok(())
}

fn join_workspace_role_path(
    role: &str,
    workspace: &str,
    role_path: &str,
) -> Result<String, WorkspaceConfigError> {
    validate_role_path(role, role_path)?;
    let relative = role_path.trim().replace('\\', "/");
    if workspace == "." {
        Ok(relative)
    } else {
        Ok(format!("{workspace}/{relative}"))
    }
}

fn relative_to_root(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

fn ensure_path_inside_root(
    root: &Path,
    absolute: &Path,
    display_path: &str,
) -> Result<(), WorkspaceConfigError> {
    let normalized = if absolute.exists() {
        absolute
            .canonicalize()
            .unwrap_or_else(|_| normalize_lexical(absolute))
    } else {
        normalize_lexical(absolute)
    };
    if !normalized.starts_with(root) {
        return Err(WorkspaceConfigError::Path(
            PathNotAllowedError::OutsideDataDir {
                root: root.to_path_buf(),
                path: display_path.to_string(),
            }
            .into(),
        ));
    }
    Ok(())
}

fn normalize_lexical(path: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    for comp in path.components() {
        match comp {
            Component::ParentDir => {
                out.pop();
            }
            Component::CurDir => {}
            other => out.push(other.as_os_str()),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Mutex, OnceLock};

    fn env_lock() -> &'static Mutex<()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(()))
    }

    fn sample_config() -> WorkspaceConfig {
        WorkspaceConfig {
            version: 1,
            name: "Demo Cafe — July 2026".into(),
            venue: "Demo Cafe".into(),
            period: Some(WorkspacePeriod {
                label: Some("July 2026".into()),
                start: Some("2026-07-01".into()),
                end: Some("2026-07-31".into()),
            }),
            currency: Some("IDR".into()),
            files: WorkspaceFiles {
                sales: WorkspaceFileRole {
                    path: "sales.xlsx".into(),
                    description: Some("sales export".into()),
                },
                cogs: Some(WorkspaceFileRole {
                    path: "cogs.xlsx".into(),
                    description: Some("menu costs".into()),
                }),
                knowledge: None,
            },
        }
    }

    #[test]
    fn writes_and_loads_workspace_yaml() {
        let _guard = env_lock().lock().unwrap();
        let root = tempfile::tempdir().unwrap();
        fs::create_dir_all(root.path().join("cafe")).unwrap();

        let written = write_workspace_config("cafe", &sample_config(), Some(root.path())).unwrap();
        assert_eq!(written.config_path, "cafe/workspace.yaml");
        assert!(root.path().join("cafe/workspace.yaml").is_file());

        let loaded = load_workspace_config("cafe", Some(root.path())).unwrap();
        assert_eq!(loaded.config, sample_config());
        assert_eq!(
            resolve_workspace_sales_path("cafe", Some(root.path())).unwrap(),
            "cafe/sales.xlsx"
        );
        assert_eq!(
            resolve_workspace_cogs_path("cafe", Some(root.path())).unwrap(),
            "cafe/cogs.xlsx"
        );
    }

    #[test]
    fn missing_cogs_role_is_clear_error() {
        let _guard = env_lock().lock().unwrap();
        let root = tempfile::tempdir().unwrap();
        fs::create_dir_all(root.path().join("cafe")).unwrap();

        let mut config = sample_config();
        config.files.cogs = None;
        write_workspace_config("cafe", &config, Some(root.path())).unwrap();

        let err = resolve_workspace_cogs_path("cafe", Some(root.path())).unwrap_err();
        let msg = err.to_string();
        assert!(
            msg.contains("files.cogs"),
            "expected missing cogs role message, got {msg}"
        );
    }

    #[test]
    fn rejects_role_path_traversal() {
        let _guard = env_lock().lock().unwrap();
        let root = tempfile::tempdir().unwrap();
        fs::create_dir_all(root.path().join("cafe")).unwrap();

        let mut bad = sample_config();
        bad.files.sales.path = "../secret.xlsx".into();
        let err = write_workspace_config("cafe", &bad, Some(root.path())).unwrap_err();
        assert!(matches!(err, WorkspaceConfigError::Invalid(_)));
    }

    #[test]
    fn rejects_write_outside_data_dir() {
        let _guard = env_lock().lock().unwrap();
        let root = tempfile::tempdir().unwrap();
        fs::create_dir_all(root.path().join("cafe")).unwrap();

        let err =
            write_workspace_config("../outside", &sample_config(), Some(root.path())).unwrap_err();
        assert!(matches!(err, WorkspaceConfigError::Path(_)));
    }

    #[test]
    fn missing_config_is_clear_error() {
        let _guard = env_lock().lock().unwrap();
        let root = tempfile::tempdir().unwrap();
        fs::create_dir_all(root.path().join("cafe")).unwrap();

        let err = load_workspace_config("cafe", Some(root.path())).unwrap_err();
        assert!(matches!(err, WorkspaceConfigError::Invalid(_)));
    }
}
