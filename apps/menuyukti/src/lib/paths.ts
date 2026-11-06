import {
  existsSync,
  readdirSync,
  readFileSync,
  statSync,
  realpathSync,
} from "node:fs";
import { homedir } from "node:os";
import {
  basename,
  extname,
  isAbsolute,
  join,
  normalize,
  relative,
  resolve,
  sep,
} from "node:path";

export const DATA_DIR_ENV = "MENUYUKTI_DATA_DIR";
const EXCEL_SUFFIXES = new Set([".xlsx", ".xls"]);

export class PathNotAllowedError extends Error {
  constructor(message: string) {
    super(message);
    this.name = "PathNotAllowedError";
  }
}

export class ResolvePathError extends Error {
  constructor(message: string) {
    super(message);
    this.name = "ResolvePathError";
  }
}

export type WorkspaceFileEntry = {
  path: string;
  name: string;
  size_bytes: number;
};

export type WorkspaceEntry = {
  path: string;
  name: string;
  file_count: number;
  files: WorkspaceFileEntry[];
};

export type WorkspaceList = {
  data_dir: string;
  workspace_count: number;
  workspaces: WorkspaceEntry[];
};

export type WorkspaceFilesList = {
  data_dir: string;
  workspace: string;
  file_count: number;
  files: WorkspaceFileEntry[];
};

export function getDataDir(override?: string): string {
  const raw = (override ?? process.env[DATA_DIR_ENV] ?? "").trim();
  if (!raw) {
    throw new PathNotAllowedError(
      `${DATA_DIR_ENV} is not set. Set it to the directory that holds analysis workspaces.`,
    );
  }
  return expandUser(raw);
}

export function resolveAllowedPath(
  path: string,
  dataDir?: string,
): string {
  const root = canonicalizeRoot(dataDir ? expandUser(dataDir) : getDataDir());
  const candidate = isAbsolute(expandUser(path))
    ? expandUser(path)
    : join(root, expandUser(path));
  const resolved = canonicalizeMaybe(candidate);
  if (!isInsideRoot(root, resolved)) {
    throw new ResolvePathError(
      `Path is outside ${DATA_DIR_ENV} (${root}): ${path}. Use a folder under the data directory; call list_workspaces or get_setup_status to see available paths.`,
    );
  }
  if (!existsSync(resolved) || !statSync(resolved).isFile()) {
    throw new ResolvePathError(
      `Workspace file not found: ${resolved}. Verify the path is under ${DATA_DIR_ENV}; call list_workspaces or get_setup_status.`,
    );
  }
  return resolved;
}

export function resolveAllowedDir(
  path: string,
  dataDir?: string,
): string {
  const root = canonicalizeRoot(dataDir ? expandUser(dataDir) : getDataDir());
  const trimmed = path.trim();
  const candidate =
    trimmed === "" || trimmed === "."
      ? root
      : isAbsolute(expandUser(trimmed))
        ? expandUser(trimmed)
        : join(root, expandUser(trimmed));
  const resolved = canonicalizeMaybe(candidate);
  if (!isInsideRoot(root, resolved)) {
    throw new ResolvePathError(
      `Path is outside ${DATA_DIR_ENV} (${root}): ${path}. Use a folder under the data directory; call list_workspaces or get_setup_status to see available paths.`,
    );
  }
  if (!existsSync(resolved) || !statSync(resolved).isDirectory()) {
    throw new ResolvePathError(
      `Workspace directory not found: ${resolved}. Verify the folder is under ${DATA_DIR_ENV}; call list_workspaces or get_setup_status.`,
    );
  }
  return resolved;
}

export function listWorkspaces(dataDir?: string): WorkspaceList {
  const rootPath = dataDir ? expandUser(dataDir) : getDataDir();
  if (!existsSync(rootPath) || !statSync(rootPath).isDirectory()) {
    throw new PathNotAllowedError(
      `${DATA_DIR_ENV} is not a directory: ${rootPath}`,
    );
  }
  const root = canonicalizeRoot(rootPath);
  const files = collectFiles(root, root, true);
  files.sort((a, b) => a.path.localeCompare(b.path));

  const grouped = new Map<string, WorkspaceFileEntry[]>();
  for (const file of files) {
    const slash = file.path.indexOf("/");
    const workspacePath = slash === -1 ? "." : file.path.slice(0, slash);
    const list = grouped.get(workspacePath) ?? [];
    list.push(file);
    grouped.set(workspacePath, list);
  }

  const workspaces: WorkspaceEntry[] = [...grouped.entries()]
    .sort(([a], [b]) => a.localeCompare(b))
    .map(([path, wsFiles]) => ({
      path,
      name: path === "." ? "." : path,
      file_count: wsFiles.length,
      files: wsFiles,
    }));

  return {
    data_dir: root,
    workspace_count: workspaces.length,
    workspaces,
  };
}

export function listWorkspaceFiles(
  workspace: string,
  dataDir?: string,
): WorkspaceFilesList {
  const rootPath = dataDir ? expandUser(dataDir) : getDataDir();
  const root = canonicalizeRoot(rootPath);
  if (!existsSync(root) || !statSync(root).isDirectory()) {
    throw new PathNotAllowedError(`${DATA_DIR_ENV} is not a directory: ${root}`);
  }
  const dir = resolveAllowedDir(workspace, root);
  const files = collectFiles(root, dir, false);
  files.sort((a, b) => a.path.localeCompare(b.path));
  const trimmed = workspace.trim();
  const workspaceLabel =
    trimmed === "" || trimmed === "." ? "." : trimmed.replaceAll("\\", "/");
  return {
    data_dir: root,
    workspace: workspaceLabel,
    file_count: files.length,
    files,
  };
}

function collectFiles(
  root: string,
  dir: string,
  excelOnly: boolean,
): WorkspaceFileEntry[] {
  const out: WorkspaceFileEntry[] = [];
  let entries;
  try {
    entries = readdirSync(dir, { withFileTypes: true });
  } catch {
    if (dir !== root) return out;
    throw new PathNotAllowedError(`${DATA_DIR_ENV} is not a directory: ${root}`);
  }
  for (const entry of entries) {
    const full = join(dir, entry.name);
    if (entry.isDirectory()) {
      out.push(...collectFiles(root, full, excelOnly));
      continue;
    }
    if (!entry.isFile()) continue;
    if (entry.name.startsWith("~$")) continue;
    if (excelOnly) {
      const suffix = extname(entry.name).toLowerCase();
      if (!EXCEL_SUFFIXES.has(suffix)) continue;
    }
    const rel = relative(root, full).split(sep).join("/");
    const size = statSync(full).size;
    out.push({ path: rel, name: entry.name, size_bytes: size });
  }
  return out;
}

function expandUser(path: string): string {
  if (path === "~") return homedir();
  if (path.startsWith("~/")) return join(homedir(), path.slice(2));
  return path;
}

function canonicalizeRoot(root: string): string {
  try {
    return realpathSync(root);
  } catch {
    return normalizeLexical(root);
  }
}

function canonicalizeMaybe(path: string): string {
  try {
    return realpathSync(path);
  } catch {
    return normalizeLexical(path);
  }
}

function normalizeLexical(path: string): string {
  const absolute = resolve(normalize(path));
  const parts = absolute.split(sep).filter(Boolean);
  const out: string[] = [];
  for (const part of parts) {
    if (part === "..") out.pop();
    else if (part !== ".") out.push(part);
  }
  return absolute.startsWith(sep) ? sep + out.join(sep) : out.join(sep);
}

function isInsideRoot(root: string, candidate: string): boolean {
  const rel = relative(root, candidate);
  return rel === "" || (!rel.startsWith("..") && !isAbsolute(rel));
}

/** Test helpers / shared. */
export function readTextFileAllowed(path: string, dataDir?: string): string {
  return readFileSync(resolveAllowedPath(path, dataDir), "utf8");
}

export function displayBasename(path: string): string {
  return basename(path);
}
