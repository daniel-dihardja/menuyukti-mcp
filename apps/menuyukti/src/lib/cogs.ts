import { existsSync, readFileSync, statSync } from "node:fs";
import {
  cellAsF64,
  cellAsString,
  ExcelError,
  readTable,
  readTableSheet,
  type RowMap,
} from "./excel.js";
import { getDataDir, resolveAllowedPath, ResolvePathError } from "./paths.js";
import { resolveWorkspaceCogsPath, WorkspaceConfigError } from "./workspace.js";

export const COGS_SHEET_NAME = "COGS";

export type CogsTable = {
  path: string;
  unit_costs: Record<string, number>;
};

export class CogsError extends Error {
  constructor(message: string) {
    super(message);
    this.name = "CogsError";
  }
}

export function resolveCogsPath(path: string, dataDir?: string): string {
  return resolveWorkspaceCogsPath(path.trim(), dataDir);
}

export function loadWorkspaceCogs(
  workspace: string,
  dataDir?: string,
): CogsTable {
  try {
    const root = dataDir ?? getDataDir();
    const cogsRel = resolveCogsPath(workspace, root);
    const allowed = resolveAllowedPath(cogsRel, root);
    if (!existsSync(allowed) || !statSync(allowed).isFile()) {
      throw new CogsError(
        `COGS file not found: ${cogsRel}. Drop a COGS Excel into the workspace and update files.cogs via write_workspace_config.`,
      );
    }
    const payload = readFileSync(allowed);
    const unitCosts = parseCogsExcel(payload);
    if (Object.keys(unitCosts).length === 0) {
      throw new CogsError(
        `COGS file has no usable Menu/COGS rows: ${cogsRel}`,
      );
    }
    return { path: cogsRel, unit_costs: unitCosts };
  } catch (err) {
    if (
      err instanceof CogsError ||
      err instanceof ResolvePathError ||
      err instanceof WorkspaceConfigError ||
      err instanceof ExcelError
    ) {
      throw err instanceof CogsError
        ? err
        : new CogsError(err.message);
    }
    throw new CogsError(err instanceof Error ? err.message : String(err));
  }
}

export function parseCogsExcel(
  data: Buffer | Uint8Array,
): Record<string, number> {
  let headers: string[];
  let rows: RowMap[];
  try {
    ({ headers, rows } = readTableSheet(data, COGS_SHEET_NAME, 0));
  } catch {
    ({ headers, rows } = readTable(data, 0));
  }
  if (!headers.includes("menu") || !headers.includes("cogs")) {
    throw new CogsError(
      "COGS Excel must include Menu and COGS columns (found sheet headers without both)",
    );
  }
  return rowsToUnitCosts(rows);
}

function rowsToUnitCosts(rows: RowMap[]): Record<string, number> {
  const out: Record<string, number> = {};
  for (const row of rows) {
    const menu = cellAsString(row.menu)?.trim();
    if (!menu) continue;
    const cost = cellAsF64(row.cogs);
    if (cost == null || !Number.isFinite(cost) || cost < 0) continue;
    out[menu] = cost;
  }
  return out;
}
