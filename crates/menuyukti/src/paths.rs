//! Path gating for local workspace files under `MENUYUKTI_DATA_DIR`.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde::Serialize;
use thiserror::Error;

/// Environment variable naming the sandboxed workspace directory.
pub const DATA_DIR_ENV: &str = "MENUYUKTI_DATA_DIR";

const EXCEL_SUFFIXES: &[&str] = &[".xlsx", ".xls"];

/// Raised when a path is outside the configured data directory, or the
/// data directory itself is missing/invalid.
#[derive(Debug, Error)]
pub enum PathNotAllowedError {
    #[error("{DATA_DIR_ENV} is not set. Set it to the directory that holds analysis workspaces.")]
    MissingDataDir,

    #[error("{DATA_DIR_ENV} is not a directory: {0}")]
    NotADirectory(PathBuf),

    #[error(
        "Path is outside {DATA_DIR_ENV} ({root}): {path}. \
         Use a folder under the data directory; call list_workspaces or get_setup_status to see available paths."
    )]
    OutsideDataDir { root: PathBuf, path: String },
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct WorkspaceFileEntry {
    pub path: String,
    pub name: String,
    pub size_bytes: u64,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct WorkspaceEntry {
    /// Relative workspace folder under the data dir (`"."` for files at the root).
    pub path: String,
    pub name: String,
    pub file_count: usize,
    pub files: Vec<WorkspaceFileEntry>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct WorkspaceList {
    pub data_dir: String,
    pub workspace_count: usize,
    pub workspaces: Vec<WorkspaceEntry>,
}

/// All regular files under one workspace directory (not Excel-only).
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct WorkspaceFilesList {
    pub data_dir: String,
    /// Relative workspace folder under the data dir (`"."` for the data dir itself).
    pub workspace: String,
    pub file_count: usize,
    pub files: Vec<WorkspaceFileEntry>,
}

/// Resolve `MENUYUKTI_DATA_DIR` to an absolute path.
pub fn get_data_dir() -> Result<PathBuf, PathNotAllowedError> {
    let raw = std::env::var(DATA_DIR_ENV).unwrap_or_default();
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Err(PathNotAllowedError::MissingDataDir);
    }
    Ok(expand_user(Path::new(trimmed)))
}

/// Resolve `path` and require it to stay inside the configured data directory.
///
/// Does not open the file for reading. Rejects missing env, traversal, and
/// paths outside the data directory. Returns [`std::io::Error`] with kind
/// [`NotFound`](std::io::ErrorKind::NotFound) when the path is inside the
/// sandbox but is not a file.
pub fn resolve_allowed_path(
    path: &str,
    data_dir: Option<&Path>,
) -> Result<PathBuf, ResolvePathError> {
    let root = match data_dir {
        Some(p) => expand_user(p),
        None => get_data_dir()?,
    };
    let root = root
        .canonicalize()
        .unwrap_or_else(|_| normalize_lexical(&root));

    let candidate = {
        let p = expand_user(Path::new(path));
        if p.is_absolute() {
            p
        } else {
            root.join(p)
        }
    };
    // Prefer real canonicalize when the path exists; otherwise lexical normalize
    // so traversal can still be rejected before any read.
    let resolved = candidate
        .canonicalize()
        .unwrap_or_else(|_| normalize_lexical(&candidate));

    if !resolved.starts_with(&root) {
        return Err(ResolvePathError::NotAllowed(
            PathNotAllowedError::OutsideDataDir {
                root,
                path: path.to_string(),
            },
        ));
    }
    if !resolved.is_file() {
        return Err(ResolvePathError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!(
                "Workspace file not found: {}. \
                 Verify the path is under {DATA_DIR_ENV}; call list_workspaces or get_setup_status.",
                resolved.display()
            ),
        )));
    }
    Ok(resolved)
}

/// Resolve `path` to a directory inside the configured data directory.
///
/// Empty path and `"."` resolve to the data directory itself. Rejects traversal
/// and paths outside the sandbox. Returns [`std::io::Error`] with kind
/// [`NotFound`](std::io::ErrorKind::NotFound) when the path is inside the
/// sandbox but is not a directory.
pub fn resolve_allowed_dir(
    path: &str,
    data_dir: Option<&Path>,
) -> Result<PathBuf, ResolvePathError> {
    let root = match data_dir {
        Some(p) => expand_user(p),
        None => get_data_dir()?,
    };
    let root = root
        .canonicalize()
        .unwrap_or_else(|_| normalize_lexical(&root));

    let trimmed = path.trim();
    let candidate = if trimmed.is_empty() || trimmed == "." {
        root.clone()
    } else {
        let p = expand_user(Path::new(trimmed));
        if p.is_absolute() {
            p
        } else {
            root.join(p)
        }
    };
    let resolved = candidate
        .canonicalize()
        .unwrap_or_else(|_| normalize_lexical(&candidate));

    if !resolved.starts_with(&root) {
        return Err(ResolvePathError::NotAllowed(
            PathNotAllowedError::OutsideDataDir {
                root,
                path: path.to_string(),
            },
        ));
    }
    if !resolved.is_dir() {
        return Err(ResolvePathError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!(
                "Workspace directory not found: {}. \
                 Verify the folder is under {DATA_DIR_ENV}; call list_workspaces or get_setup_status.",
                resolved.display()
            ),
        )));
    }
    Ok(resolved)
}

/// Errors from [`resolve_allowed_path`] / [`resolve_allowed_dir`].
#[derive(Debug, Error)]
pub enum ResolvePathError {
    #[error(transparent)]
    NotAllowed(#[from] PathNotAllowedError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

/// List analysis workspaces under the data directory, grouped by folder.
///
/// Each immediate subdirectory that contains Excel files becomes a workspace.
/// Excel files placed directly in the data directory are grouped under `"."`.
pub fn list_workspaces(data_dir: Option<&Path>) -> Result<WorkspaceList, PathNotAllowedError> {
    let root = match data_dir {
        Some(p) => p.to_path_buf(),
        None => get_data_dir()?,
    };
    if !root.is_dir() {
        return Err(PathNotAllowedError::NotADirectory(root));
    }

    let mut files = Vec::new();
    collect_excel_files(&root, &root, &mut files)?;
    files.sort_by(|a, b| a.path.cmp(&b.path));

    let mut grouped: BTreeMap<String, Vec<WorkspaceFileEntry>> = BTreeMap::new();
    for file in files {
        let workspace_path = match file.path.split_once('/') {
            Some((dir, _)) => dir.to_string(),
            None => ".".to_string(),
        };
        grouped.entry(workspace_path).or_default().push(file);
    }

    let workspaces: Vec<WorkspaceEntry> = grouped
        .into_iter()
        .map(|(path, files)| {
            let name = if path == "." {
                ".".to_string()
            } else {
                path.clone()
            };
            WorkspaceEntry {
                file_count: files.len(),
                path,
                name,
                files,
            }
        })
        .collect();

    Ok(WorkspaceList {
        data_dir: root.display().to_string(),
        workspace_count: workspaces.len(),
        workspaces,
    })
}

/// List all regular files under a workspace directory inside the data dir.
///
/// Unlike [`list_workspaces`], this includes non-Excel files (for example
/// `workspace.yaml` and notes). Excel lock files (`~$*`) are skipped.
/// Paths in the result are relative to the data directory.
pub fn list_workspace_files(
    workspace: &str,
    data_dir: Option<&Path>,
) -> Result<WorkspaceFilesList, ResolvePathError> {
    let root = match data_dir {
        Some(p) => expand_user(p),
        None => get_data_dir()?,
    };
    let root = root
        .canonicalize()
        .unwrap_or_else(|_| normalize_lexical(&root));
    if !root.is_dir() {
        return Err(PathNotAllowedError::NotADirectory(root).into());
    }

    let dir = resolve_allowed_dir(workspace, Some(&root))?;
    let mut files = Vec::new();
    collect_files(&root, &dir, false, &mut files)?;
    files.sort_by(|a, b| a.path.cmp(&b.path));

    let trimmed = workspace.trim();
    let workspace_label = if trimmed.is_empty() || trimmed == "." {
        ".".to_string()
    } else {
        trimmed.replace('\\', "/")
    };

    Ok(WorkspaceFilesList {
        data_dir: root.display().to_string(),
        workspace: workspace_label,
        file_count: files.len(),
        files,
    })
}

fn collect_excel_files(
    root: &Path,
    dir: &Path,
    out: &mut Vec<WorkspaceFileEntry>,
) -> Result<(), PathNotAllowedError> {
    collect_files(root, dir, true, out)
}

fn collect_files(
    root: &Path,
    dir: &Path,
    excel_only: bool,
    out: &mut Vec<WorkspaceFileEntry>,
) -> Result<(), PathNotAllowedError> {
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) if dir != root => return Ok(()),
        Err(_) => return Err(PathNotAllowedError::NotADirectory(root.to_path_buf())),
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_files(root, &path, excel_only, out)?;
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
        if excel_only {
            let suffix = path
                .extension()
                .and_then(|e| e.to_str())
                .map(|e| format!(".{}", e.to_ascii_lowercase()))
                .unwrap_or_default();
            if !EXCEL_SUFFIXES.contains(&suffix.as_str()) {
                continue;
            }
        }
        let rel = path
            .strip_prefix(root)
            .unwrap_or(&path)
            .to_string_lossy()
            .replace('\\', "/");
        let size_bytes = entry.metadata().map(|m| m.len()).unwrap_or(0);
        out.push(WorkspaceFileEntry {
            path: rel,
            name: name.to_string(),
            size_bytes,
        });
    }
    Ok(())
}

fn expand_user(path: &Path) -> PathBuf {
    let raw = path.to_string_lossy();
    if let Some(rest) = raw.strip_prefix("~/") {
        if let Some(home) = home_dir() {
            return home.join(rest);
        }
    } else if raw == "~" {
        if let Some(home) = home_dir() {
            return home;
        }
    }
    path.to_path_buf()
}

fn home_dir() -> Option<PathBuf> {
    std::env::var_os("HOME").map(PathBuf::from)
}

fn normalize_lexical(path: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    for comp in path.components() {
        match comp {
            std::path::Component::ParentDir => {
                out.pop();
            }
            std::path::Component::CurDir => {}
            other => out.push(other.as_os_str()),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::{Mutex, OnceLock};

    fn env_lock() -> &'static Mutex<()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(()))
    }

    #[test]
    fn requires_data_dir_env() {
        let _guard = env_lock().lock().unwrap();
        // SAFETY: serialized by env_lock for tests in this module.
        unsafe {
            std::env::remove_var(DATA_DIR_ENV);
        }
        let err = get_data_dir().unwrap_err();
        assert!(matches!(err, PathNotAllowedError::MissingDataDir));
    }

    #[test]
    fn rejects_path_outside_data_dir() {
        let _guard = env_lock().lock().unwrap();
        let root = tempfile::tempdir().unwrap();
        let outside = tempfile::tempdir().unwrap();
        let secret = outside.path().join("secret.xlsx");
        fs::write(&secret, b"not-excel").unwrap();

        let err = resolve_allowed_path(secret.to_str().unwrap(), Some(root.path())).unwrap_err();
        assert!(matches!(err, ResolvePathError::NotAllowed(_)));
    }

    #[test]
    fn rejects_traversal_outside_data_dir() {
        let _guard = env_lock().lock().unwrap();
        let root = tempfile::tempdir().unwrap();
        fs::write(root.path().join("inside.xlsx"), b"x").unwrap();

        let err = resolve_allowed_path("../README.md", Some(root.path())).unwrap_err();
        assert!(matches!(err, ResolvePathError::NotAllowed(_)));
    }

    #[test]
    fn lists_workspaces_grouped_by_folder() {
        let _guard = env_lock().lock().unwrap();
        let root = tempfile::tempdir().unwrap();
        fs::create_dir_all(root.path().join("nested")).unwrap();
        fs::write(root.path().join("a.xlsx"), b"a").unwrap();
        fs::write(root.path().join("nested/b.xls"), b"b").unwrap();
        fs::write(root.path().join("skip.txt"), b"no").unwrap();
        fs::write(root.path().join("~$lock.xlsx"), b"lock").unwrap();

        let listed = list_workspaces(Some(root.path())).unwrap();
        assert_eq!(listed.workspace_count, 2);
        assert_eq!(listed.workspaces[0].path, ".");
        assert_eq!(listed.workspaces[0].files[0].path, "a.xlsx");
        assert_eq!(listed.workspaces[1].path, "nested");
        assert_eq!(listed.workspaces[1].files[0].path, "nested/b.xls");
        assert_eq!(listed.workspaces[1].files[0].size_bytes, 1);
    }

    #[test]
    fn missing_data_dir_is_not_a_directory() {
        let _guard = env_lock().lock().unwrap();
        let missing = tempfile::tempdir().unwrap().path().join("nope");
        let err = list_workspaces(Some(&missing)).unwrap_err();
        assert!(matches!(err, PathNotAllowedError::NotADirectory(_)));
    }

    #[test]
    fn lists_all_files_in_workspace_including_non_excel() {
        let _guard = env_lock().lock().unwrap();
        let root = tempfile::tempdir().unwrap();
        let ws = root.path().join("cafe");
        fs::create_dir_all(ws.join("notes")).unwrap();
        fs::write(ws.join("sales.xlsx"), b"a").unwrap();
        fs::write(ws.join("workspace.json"), b"{}").unwrap();
        fs::write(ws.join("notes/readme.md"), b"hi").unwrap();
        fs::write(ws.join("~$lock.xlsx"), b"lock").unwrap();
        fs::write(root.path().join("other.xlsx"), b"x").unwrap();

        let listed = list_workspace_files("cafe", Some(root.path())).unwrap();
        assert_eq!(listed.workspace, "cafe");
        assert_eq!(listed.file_count, 3);
        let paths: Vec<&str> = listed.files.iter().map(|f| f.path.as_str()).collect();
        assert_eq!(
            paths,
            vec![
                "cafe/notes/readme.md",
                "cafe/sales.xlsx",
                "cafe/workspace.json"
            ]
        );
    }

    #[test]
    fn list_workspace_files_rejects_traversal() {
        let _guard = env_lock().lock().unwrap();
        let root = tempfile::tempdir().unwrap();
        fs::create_dir_all(root.path().join("cafe")).unwrap();

        let err = list_workspace_files("../etc", Some(root.path())).unwrap_err();
        assert!(matches!(err, ResolvePathError::NotAllowed(_)));
    }

    #[test]
    fn list_workspace_files_missing_dir() {
        let _guard = env_lock().lock().unwrap();
        let root = tempfile::tempdir().unwrap();
        let err = list_workspace_files("missing", Some(root.path())).unwrap_err();
        assert!(matches!(err, ResolvePathError::Io(_)));
    }
}
