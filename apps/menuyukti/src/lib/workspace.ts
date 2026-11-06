import {
  existsSync,
  mkdirSync,
  readFileSync,
  realpathSync,
  statSync,
  writeFileSync,
} from "node:fs";
import {
  dirname,
  isAbsolute,
  join,
  normalize,
  relative,
  resolve,
  sep,
} from "node:path";
import { parse as parseYaml, stringify as stringifyYaml } from "yaml";
import {
  DATA_DIR_ENV,
  getDataDir,
  PathNotAllowedError,
  resolveAllowedDir,
  resolveAllowedPath,
  ResolvePathError,
} from "./paths.js";

export const WORKSPACE_CONFIG_FILENAME = "workspace.yaml";
export const DEFAULT_KNOWLEDGE_FILENAME = "knowledge.md";
const SUPPORTED_VERSION = 1;

export type WorkspaceFileRole = {
  path: string;
  description?: string;
};

export type WorkspacePeriod = {
  label?: string;
  start?: string;
  end?: string;
};

export type WorkspaceConfig = {
  version: number;
  name: string;
  venue: string;
  period?: WorkspacePeriod;
  currency?: string;
  files: {
    sales: WorkspaceFileRole;
    cogs?: WorkspaceFileRole;
    knowledge?: WorkspaceFileRole;
  };
};

export type WorkspaceConfigWriteResult = {
  data_dir: string;
  workspace: string;
  config_path: string;
  config: WorkspaceConfig;
};

export type WorkspaceKnowledgeRead = {
  data_dir: string;
  workspace: string;
  knowledge_path: string;
  exists: boolean;
  content: string;
};

export type WorkspaceKnowledgeWriteResult = {
  data_dir: string;
  workspace: string;
  knowledge_path: string;
  content: string;
  config_updated: boolean;
};

export class WorkspaceConfigError extends Error {
  constructor(message: string) {
    super(message);
    this.name = "WorkspaceConfigError";
  }
}

export function writeWorkspaceConfig(
  workspace: string,
  config: WorkspaceConfig,
  dataDir?: string,
): WorkspaceConfigWriteResult {
  validateConfig(config);
  const { root, workspaceLabel, dir } = resolveWorkspaceDir(workspace, dataDir);
  const configAbs = join(dir, WORKSPACE_CONFIG_FILENAME);
  const configRel = relativeToRoot(root, configAbs);
  ensurePathInsideRoot(root, configAbs, configRel);
  writeFileSync(configAbs, stringifyYaml(config), "utf8");
  return {
    data_dir: root,
    workspace: workspaceLabel,
    config_path: configRel,
    config,
  };
}

export function loadWorkspaceConfig(
  workspace: string,
  dataDir?: string,
): WorkspaceConfigWriteResult {
  const { root, workspaceLabel, dir } = resolveWorkspaceDir(workspace, dataDir);
  const configAbs = join(dir, WORKSPACE_CONFIG_FILENAME);
  if (!existsSync(configAbs) || !statIsFile(configAbs)) {
    throw new WorkspaceConfigError(
      `Workspace config not found: ${relativeToRoot(root, configAbs)} (no files.sales mapping yet). Call list_workspace_files, confirm which Excel is sales (+ optional COGS), then write_workspace_config — or ask the owner which file is sales.`,
    );
  }
  const configRel = relativeToRoot(root, configAbs);
  const allowed = resolveAllowedPath(configRel, root);
  const config = parseYaml(readFileSync(allowed, "utf8")) as WorkspaceConfig;
  validateConfig(config);
  return {
    data_dir: root,
    workspace: workspaceLabel,
    config_path: configRel,
    config,
  };
}

/** Resolve sales Excel path relative to the data dir. */
export function resolveWorkspaceSalesPath(
  workspace: string,
  dataDir?: string,
): string {
  const loaded = loadWorkspaceConfig(workspace, dataDir);
  return joinWorkspaceRolePath(
    "sales",
    loaded.workspace,
    loaded.config.files.sales.path,
  );
}

/** Resolve COGS Excel path relative to the data dir. */
export function resolveWorkspaceCogsPath(
  workspace: string,
  dataDir?: string,
): string {
  const loaded = loadWorkspaceConfig(workspace, dataDir);
  const cogs = loaded.config.files.cogs;
  if (!cogs) {
    throw new WorkspaceConfigError(
      `Workspace config missing files.cogs role: ${loaded.workspace}/${WORKSPACE_CONFIG_FILENAME}. Ask the owner to drop a unit-cost (COGS) Excel into the folder, then call list_workspace_files and write_workspace_config to set files.cogs.`,
    );
  }
  return joinWorkspaceRolePath("cogs", loaded.workspace, cogs.path);
}

export function readWorkspaceKnowledge(
  workspace: string,
  dataDir?: string,
): WorkspaceKnowledgeRead {
  const resolved = resolveKnowledgeFile(workspace, dataDir);
  if (!existsSync(resolved.abs) || !statIsFile(resolved.abs)) {
    ensurePathInsideRoot(resolved.root, resolved.abs, resolved.knowledgePath);
    return {
      data_dir: resolved.root,
      workspace: resolved.workspaceLabel,
      knowledge_path: resolved.knowledgePath,
      exists: false,
      content: "",
    };
  }
  const allowed = resolveAllowedPath(resolved.knowledgePath, resolved.root);
  return {
    data_dir: resolved.root,
    workspace: resolved.workspaceLabel,
    knowledge_path: resolved.knowledgePath,
    exists: true,
    content: readFileSync(allowed, "utf8"),
  };
}

export function writeWorkspaceKnowledge(
  workspace: string,
  content: string,
  dataDir?: string,
): WorkspaceKnowledgeWriteResult {
  const resolved = resolveKnowledgeFile(workspace, dataDir);
  ensurePathInsideRoot(resolved.root, resolved.abs, resolved.knowledgePath);
  mkdirSync(dirname(resolved.abs), { recursive: true });
  writeFileSync(resolved.abs, content, "utf8");

  let configUpdated = false;
  const configAbs = join(resolved.dir, WORKSPACE_CONFIG_FILENAME);
  if (existsSync(configAbs) && statIsFile(configAbs)) {
    const loaded = loadWorkspaceConfig(workspace, dataDir);
    if (!loaded.config.files.knowledge) {
      const next: WorkspaceConfig = {
        ...loaded.config,
        files: {
          ...loaded.config.files,
          knowledge: {
            path: resolved.roleRelative,
            description: "Location and venue notes",
          },
        },
      };
      writeWorkspaceConfig(workspace, next, dataDir);
      configUpdated = true;
    }
  }

  return {
    data_dir: resolved.root,
    workspace: resolved.workspaceLabel,
    knowledge_path: resolved.knowledgePath,
    content,
    config_updated: configUpdated,
  };
}

function resolveKnowledgeFile(workspace: string, dataDir?: string) {
  const { root, workspaceLabel, dir } = resolveWorkspaceDir(workspace, dataDir);
  const roleRelative = knowledgeRoleRelative(dir, workspace, dataDir);
  validateRolePath("knowledge", roleRelative);
  const abs = join(dir, roleRelative);
  const knowledgePath = joinWorkspaceRolePath(
    "knowledge",
    workspaceLabel,
    roleRelative,
  );
  return {
    root,
    workspaceLabel,
    dir,
    abs,
    knowledgePath,
    roleRelative,
  };
}

function knowledgeRoleRelative(
  dir: string,
  workspace: string,
  dataDir?: string,
): string {
  const configAbs = join(dir, WORKSPACE_CONFIG_FILENAME);
  if (!existsSync(configAbs) || !statIsFile(configAbs)) {
    return DEFAULT_KNOWLEDGE_FILENAME;
  }
  const loaded = loadWorkspaceConfig(workspace, dataDir);
  return loaded.config.files.knowledge?.path ?? DEFAULT_KNOWLEDGE_FILENAME;
}

function resolveWorkspaceDir(workspace: string, dataDir?: string) {
  const rootPath = dataDir ? dataDir : getDataDir();
  let root = rootPath;
  try {
    root = realpathSync(rootPath);
  } catch {
    root = normalizeLexical(rootPath);
  }
  if (!existsSync(root) || !statIsDir(root)) {
    throw new WorkspaceConfigError(
      wrapPath(
        new PathNotAllowedError(`${DATA_DIR_ENV} is not a directory: ${root}`),
      ),
    );
  }
  let dir: string;
  try {
    dir = resolveAllowedDir(workspace, root);
  } catch (err) {
    throw new WorkspaceConfigError(err instanceof Error ? err.message : String(err));
  }
  const trimmed = workspace.trim();
  const workspaceLabel =
    trimmed === "" || trimmed === "." ? "." : trimmed.replaceAll("\\", "/");
  return { root, workspaceLabel, dir };
}

function validateConfig(config: WorkspaceConfig): void {
  if (config.version !== SUPPORTED_VERSION) {
    throw new WorkspaceConfigError(
      `Unsupported workspace config version: ${config.version} (expected ${SUPPORTED_VERSION})`,
    );
  }
  if (!config.name?.trim()) {
    throw new WorkspaceConfigError("Workspace config name must not be empty");
  }
  if (!config.venue?.trim()) {
    throw new WorkspaceConfigError("Workspace config venue must not be empty");
  }
  validateRolePath("sales", config.files.sales.path);
  if (config.files.cogs) validateRolePath("cogs", config.files.cogs.path);
  if (config.files.knowledge) {
    validateRolePath("knowledge", config.files.knowledge.path);
  }
}

function validateRolePath(role: string, path: string): void {
  const trimmed = path.trim();
  if (!trimmed) {
    throw new WorkspaceConfigError(
      `Workspace config files.${role}.path must not be empty`,
    );
  }
  if (isAbsolute(trimmed)) {
    throw new WorkspaceConfigError(
      `Workspace config files.${role}.path must be relative to the workspace folder: ${trimmed}`,
    );
  }
  const parts = trimmed.replaceAll("\\", "/").split("/");
  for (const part of parts) {
    if (part === "..") {
      throw new WorkspaceConfigError(
        `Workspace config files.${role}.path must not contain '..': ${trimmed}`,
      );
    }
  }
}

function joinWorkspaceRolePath(
  role: string,
  workspace: string,
  rolePath: string,
): string {
  validateRolePath(role, rolePath);
  const relativePath = rolePath.trim().replaceAll("\\", "/");
  return workspace === "." ? relativePath : `${workspace}/${relativePath}`;
}

function relativeToRoot(root: string, path: string): string {
  return relative(root, path).split(sep).join("/");
}

function ensurePathInsideRoot(
  root: string,
  absolute: string,
  displayPath: string,
): void {
  const normalized = existsSync(absolute)
    ? (() => {
        try {
          return realpathSync(absolute);
        } catch {
          return normalizeLexical(absolute);
        }
      })()
    : normalizeLexical(absolute);
  const rel = relative(root, normalized);
  if (rel.startsWith("..") || isAbsolute(rel)) {
    throw new WorkspaceConfigError(
      wrapPath(
        new ResolvePathError(
          `Path is outside ${DATA_DIR_ENV} (${root}): ${displayPath}.`,
        ),
      ),
    );
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

function wrapPath(err: Error): string {
  return err.message;
}

function statIsFile(path: string): boolean {
  try {
    return statSync(path).isFile();
  } catch {
    return false;
  }
}

function statIsDir(path: string): boolean {
  try {
    return statSync(path).isDirectory();
  } catch {
    return false;
  }
}
