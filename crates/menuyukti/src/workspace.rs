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

/// Default markdown file for location/venue notes when `files.knowledge` is unset.
pub const DEFAULT_KNOWLEDGE_FILENAME: &str = "knowledge.md";

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

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct WorkspaceKnowledgeRead {
    pub data_dir: String,
    pub workspace: String,
    /// Path to the knowledge file relative to the data dir.
    pub knowledge_path: String,
    /// False when the file is not present yet (content is empty).
    pub exists: bool,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct WorkspaceKnowledgeWriteResult {
    pub data_dir: String,
    pub workspace: String,
    /// Path to the knowledge file relative to the data dir.
    pub knowledge_path: String,
    pub content: String,
    /// True when `files.knowledge` was added to an existing `workspace.yaml`.
    pub config_updated: bool,
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
            "Workspace config not found: {} (no files.sales mapping yet). \
             Call list_workspace_files, confirm which Excel is sales (+ optional COGS), \
             then write_workspace_config — or ask the owner which file is sales.",
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
             Ask the owner to drop a unit-cost (COGS) Excel into the folder, then call \
             list_workspace_files and write_workspace_config to set files.cogs.",
            loaded.workspace
        )));
    };
    join_workspace_role_path("cogs", &loaded.workspace, &cogs.path)
}

/// Read location/venue knowledge markdown for a workspace folder.
///
/// Uses `files.knowledge` when set; otherwise defaults to [`DEFAULT_KNOWLEDGE_FILENAME`].
/// Missing files return `exists: false` with empty content (not an error).
pub fn read_workspace_knowledge(
    workspace: &str,
    data_dir: Option<&Path>,
) -> Result<WorkspaceKnowledgeRead, WorkspaceConfigError> {
    let ResolvedKnowledge {
        root,
        workspace_label,
        abs,
        knowledge_path,
        ..
    } = resolve_knowledge_file(workspace, data_dir)?;
    if !abs.is_file() {
        ensure_path_inside_root(&root, &abs, &knowledge_path)?;
        return Ok(WorkspaceKnowledgeRead {
            data_dir: root.display().to_string(),
            workspace: workspace_label,
            knowledge_path,
            exists: false,
            content: String::new(),
        });
    }
    let allowed = resolve_allowed_path(&knowledge_path, Some(&root))?;
    let content = fs::read_to_string(&allowed)?;
    Ok(WorkspaceKnowledgeRead {
        data_dir: root.display().to_string(),
        workspace: workspace_label,
        knowledge_path,
        exists: true,
        content,
    })
}

/// Create or fully replace knowledge markdown for a workspace folder.
///
/// When `workspace.yaml` exists and `files.knowledge` is unset, sets the role to
/// the resolved knowledge path via [`write_workspace_config`].
pub fn write_workspace_knowledge(
    workspace: &str,
    content: &str,
    data_dir: Option<&Path>,
) -> Result<WorkspaceKnowledgeWriteResult, WorkspaceConfigError> {
    let ResolvedKnowledge {
        root,
        workspace_label,
        dir,
        abs,
        knowledge_path,
        role_relative,
    } = resolve_knowledge_file(workspace, data_dir)?;
    ensure_path_inside_root(&root, &abs, &knowledge_path)?;
    if let Some(parent) = abs.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&abs, content)?;

    let mut config_updated = false;
    let config_abs = dir.join(WORKSPACE_CONFIG_FILENAME);
    if config_abs.is_file() {
        let loaded = load_workspace_config(workspace, data_dir)?;
        if loaded.config.files.knowledge.is_none() {
            let mut config = loaded.config;
            config.files.knowledge = Some(WorkspaceFileRole {
                path: role_relative,
                description: Some("Location and venue notes".into()),
            });
            write_workspace_config(workspace, &config, data_dir)?;
            config_updated = true;
        }
    }

    Ok(WorkspaceKnowledgeWriteResult {
        data_dir: root.display().to_string(),
        workspace: workspace_label,
        knowledge_path,
        content: content.to_string(),
        config_updated,
    })
}

struct ResolvedKnowledge {
    root: PathBuf,
    workspace_label: String,
    dir: PathBuf,
    abs: PathBuf,
    /// Path relative to the data dir.
    knowledge_path: String,
    /// Path relative to the workspace folder (role path).
    role_relative: String,
}

/// Resolve absolute knowledge path + data-dir-relative display path.
fn resolve_knowledge_file(
    workspace: &str,
    data_dir: Option<&Path>,
) -> Result<ResolvedKnowledge, WorkspaceConfigError> {
    let (root, workspace_label, dir) = resolve_workspace_dir(workspace, data_dir)?;
    let role_relative = knowledge_role_relative(&dir, workspace, data_dir)?;
    validate_role_path("knowledge", &role_relative)?;
    let abs = dir.join(&role_relative);
    let knowledge_path = join_workspace_role_path("knowledge", &workspace_label, &role_relative)?;
    Ok(ResolvedKnowledge {
        root,
        workspace_label,
        dir,
        abs,
        knowledge_path,
        role_relative,
    })
}

fn knowledge_role_relative(
    dir: &Path,
    workspace: &str,
    data_dir: Option<&Path>,
) -> Result<String, WorkspaceConfigError> {
    let config_abs = dir.join(WORKSPACE_CONFIG_FILENAME);
    if !config_abs.is_file() {
        return Ok(DEFAULT_KNOWLEDGE_FILENAME.to_string());
    }
    let loaded = load_workspace_config(workspace, data_dir)?;
    Ok(loaded
        .config
        .files
        .knowledge
        .map(|k| k.path)
        .unwrap_or_else(|| DEFAULT_KNOWLEDGE_FILENAME.to_string()))
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

    #[test]
    fn knowledge_read_missing_is_empty_not_error() {
        let _guard = env_lock().lock().unwrap();
        let root = tempfile::tempdir().unwrap();
        fs::create_dir_all(root.path().join("cafe")).unwrap();
        write_workspace_config("cafe", &sample_config(), Some(root.path())).unwrap();

        let read = read_workspace_knowledge("cafe", Some(root.path())).unwrap();
        assert!(!read.exists);
        assert!(read.content.is_empty());
        assert_eq!(read.knowledge_path, "cafe/knowledge.md");
    }

    #[test]
    fn knowledge_write_read_roundtrip_and_sets_role() {
        let _guard = env_lock().lock().unwrap();
        let root = tempfile::tempdir().unwrap();
        fs::create_dir_all(root.path().join("cafe")).unwrap();
        write_workspace_config("cafe", &sample_config(), Some(root.path())).unwrap();

        let body = "# Demo Cafe\n\nOpens 08:00–22:00.\n";
        let written = write_workspace_knowledge("cafe", body, Some(root.path())).unwrap();
        assert_eq!(written.knowledge_path, "cafe/knowledge.md");
        assert!(written.config_updated);
        assert_eq!(written.content, body);

        let loaded = load_workspace_config("cafe", Some(root.path())).unwrap();
        let knowledge = loaded.config.files.knowledge.expect("knowledge role");
        assert_eq!(knowledge.path, DEFAULT_KNOWLEDGE_FILENAME);

        let read = read_workspace_knowledge("cafe", Some(root.path())).unwrap();
        assert!(read.exists);
        assert_eq!(read.content, body);

        // Second write should not rewrite the role.
        let again = write_workspace_knowledge("cafe", "# updated\n", Some(root.path())).unwrap();
        assert!(!again.config_updated);
    }

    #[test]
    fn knowledge_uses_configured_role_path() {
        let _guard = env_lock().lock().unwrap();
        let root = tempfile::tempdir().unwrap();
        fs::create_dir_all(root.path().join("cafe")).unwrap();
        let mut config = sample_config();
        config.files.knowledge = Some(WorkspaceFileRole {
            path: "notes/venue.md".into(),
            description: Some("venue notes".into()),
        });
        write_workspace_config("cafe", &config, Some(root.path())).unwrap();

        let written =
            write_workspace_knowledge("cafe", "custom path notes\n", Some(root.path())).unwrap();
        assert_eq!(written.knowledge_path, "cafe/notes/venue.md");
        assert!(!written.config_updated);
        assert!(root.path().join("cafe/notes/venue.md").is_file());

        let read = read_workspace_knowledge("cafe", Some(root.path())).unwrap();
        assert!(read.exists);
        assert_eq!(read.content, "custom path notes\n");
    }

    #[test]
    fn knowledge_write_without_config_does_not_create_yaml() {
        let _guard = env_lock().lock().unwrap();
        let root = tempfile::tempdir().unwrap();
        fs::create_dir_all(root.path().join("cafe")).unwrap();

        let written = write_workspace_knowledge("cafe", "notes only\n", Some(root.path())).unwrap();
        assert!(!written.config_updated);
        assert!(root.path().join("cafe/knowledge.md").is_file());
        assert!(!root.path().join("cafe/workspace.yaml").is_file());
    }

    #[test]
    fn knowledge_rejects_outside_data_dir() {
        let _guard = env_lock().lock().unwrap();
        let root = tempfile::tempdir().unwrap();
        fs::create_dir_all(root.path().join("cafe")).unwrap();

        let err = write_workspace_knowledge("../outside", "x", Some(root.path())).unwrap_err();
        assert!(matches!(err, WorkspaceConfigError::Path(_)));

        let err = read_workspace_knowledge("../outside", Some(root.path())).unwrap_err();
        assert!(matches!(err, WorkspaceConfigError::Path(_)));
    }
}
