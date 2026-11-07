import { readFileSync } from "node:fs";
import {
  ColumnMappingError,
  normalizeMappedExcel,
  tryLoadMappingForDataDir,
} from "./column-mapping.js";
import { ExcelError } from "./excel.js";
import type { LineItem } from "./line-item.js";
import { NormalizeError, normalizeForPos } from "./normalize/index.js";
import { getDataDir, ResolvePathError, resolveAllowedPath } from "./paths.js";
import { configFor, detectPosFromExcelBytes } from "./pos.js";
import {
  resolveWorkspaceSalesPath,
  WorkspaceConfigError,
} from "./workspace.js";

export type NormalizedSales = {
  pos: string;
  rows: LineItem[];
};

export class LoadError extends Error {
  constructor(message: string) {
    super(message);
    this.name = "LoadError";
  }
}

/** Resolve sales Excel path: explicit file or workspace `files.sales`. */
export function resolveSalesPath(path: string, dataDir?: string): string {
  const trimmed = path.trim();
  const lower = trimmed.toLowerCase();
  if (lower.endsWith(".xlsx") || lower.endsWith(".xls")) return trimmed;
  return resolveWorkspaceSalesPath(trimmed, dataDir);
}

export function loadNormalizedSales(
  path: string,
  dataDir?: string,
): NormalizedSales {
  try {
    const root = dataDir ?? getDataDir();
    const salesPath = resolveSalesPath(path, root);
    const allowed = resolveAllowedPath(salesPath, root);
    const payload = readFileSync(allowed);
    const detected = detectPosFromExcelBytes(payload) ?? "unknown";
    if (configFor(detected)) {
      return {
        pos: detected,
        rows: normalizeForPos(detected, payload),
      };
    }
    const mapping = tryLoadMappingForDataDir(root);
    if (mapping) {
      return {
        pos: mapping.name,
        rows: normalizeMappedExcel(payload, mapping),
      };
    }
    throw new LoadError(
      `Unsupported POS system detected: ${detected}. Add a pos-column-mapping.json under MENUYUKTI_DATA_DIR (or set MENUYUKTI_COLUMN_MAPPING) to map your export columns.`,
    );
  } catch (err) {
    if (err instanceof LoadError) throw err;
    if (
      err instanceof ResolvePathError ||
      err instanceof WorkspaceConfigError ||
      err instanceof NormalizeError ||
      err instanceof ColumnMappingError ||
      err instanceof ExcelError
    ) {
      throw new LoadError(err.message);
    }
    throw new LoadError(err instanceof Error ? err.message : String(err));
  }
}
