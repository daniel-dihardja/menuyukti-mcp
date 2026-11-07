import {
  type Dirent,
  existsSync,
  readdirSync,
  realpathSync,
  statSync,
} from "node:fs";
import { extname, join } from "node:path";
import { DATA_DIR_ENV, getDataDir, PathNotAllowedError } from "./paths.js";
import { VERSION } from "./version.js";
import { WORKSPACE_CONFIG_FILENAME } from "./workspace.js";

export const DEMO_WORKSPACE_NAME = "demo-cafe-july-2026";
const EXCEL_SUFFIXES = new Set([".xlsx", ".xls"]);

export type SetupWorkspaceSummary = {
  path: string;
  has_workspace_yaml: boolean;
  excel_file_count: number;
};

export type SetupStatus = {
  data_dir: string;
  version: string;
  workspace_count: number;
  workspaces: SetupWorkspaceSummary[];
  folders_missing_workspace_yaml: string[];
  demo_workspace_present: boolean;
};

export function getSetupStatus(dataDir?: string): SetupStatus {
  const rootPath = dataDir ? dataDir : getDataDir();
  if (!existsSync(rootPath) || !statSync(rootPath).isDirectory()) {
    throw new PathNotAllowedError(
      `${DATA_DIR_ENV} is not a directory: ${rootPath}`,
    );
  }
  let root = rootPath;
  try {
    root = realpathSync(rootPath);
  } catch {
    /* keep lexical */
  }

  const workspaces: SetupWorkspaceSummary[] = [];
  for (const entry of readdirSync(root, { withFileTypes: true })) {
    if (!entry.isDirectory() || entry.name.startsWith(".")) continue;
    const dir = join(root, entry.name);
    workspaces.push({
      path: entry.name,
      has_workspace_yaml: existsSync(join(dir, WORKSPACE_CONFIG_FILENAME)),
      excel_file_count: countExcelFiles(dir),
    });
  }
  workspaces.sort((a, b) => a.path.localeCompare(b.path));

  return {
    data_dir: root,
    version: VERSION,
    workspace_count: workspaces.length,
    workspaces,
    folders_missing_workspace_yaml: workspaces
      .filter((w) => !w.has_workspace_yaml)
      .map((w) => w.path),
    demo_workspace_present: workspaces.some(
      (w) => w.path === DEMO_WORKSPACE_NAME,
    ),
  };
}

function countExcelFiles(dir: string): number {
  let count = 0;
  const walk = (d: string) => {
    let entries: Dirent[];
    try {
      entries = readdirSync(d, { withFileTypes: true });
    } catch {
      return;
    }
    for (const entry of entries) {
      const full = join(d, entry.name);
      if (entry.isDirectory()) {
        walk(full);
        continue;
      }
      if (!entry.isFile() || entry.name.startsWith("~$")) continue;
      if (EXCEL_SUFFIXES.has(extname(entry.name).toLowerCase())) count += 1;
    }
  };
  walk(dir);
  return count;
}
