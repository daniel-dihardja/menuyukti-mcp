import { existsSync, readFileSync } from "node:fs";
import { homedir } from "node:os";
import { isAbsolute, join } from "node:path";
import {
  cellAsDateTime,
  cellAsF64,
  cellAsI64,
  cellAsString,
  getCell,
  normalizeHeader,
  type RowMap,
  readTable,
} from "./excel.js";
import { type LineItem, REQUIRED_COLUMNS } from "./line-item.js";

export const COLUMN_MAPPING_ENV = "MENUYUKTI_COLUMN_MAPPING";
export const DEFAULT_MAPPING_FILENAME = "pos-column-mapping.json";
export const DEFAULT_POS_NAME = "custom";

export type ColumnMappingConfig = {
  name: string;
  skip_rows: number;
  columns: Record<string, string>;
};

export class ColumnMappingError extends Error {
  constructor(message: string) {
    super(message);
    this.name = "ColumnMappingError";
  }
}

export function mappingConfigPath(dataDir: string): string | null {
  const env = (process.env[COLUMN_MAPPING_ENV] ?? "").trim();
  if (env) return expandUser(env);
  const candidate = join(dataDir, DEFAULT_MAPPING_FILENAME);
  return existsSync(candidate) ? candidate : null;
}

export function tryLoadMappingForDataDir(
  dataDir: string,
): ColumnMappingConfig | null {
  const path = mappingConfigPath(dataDir);
  if (!path) return null;
  if (!existsSync(path)) {
    throw new ColumnMappingError(
      `Column-mapping file from ${COLUMN_MAPPING_ENV} was not found: ${path}`,
    );
  }
  return loadColumnMapping(path);
}

export function loadColumnMapping(path: string): ColumnMappingConfig {
  const text = readFileSync(path, "utf8");
  let raw: {
    name?: string;
    skip_rows?: number;
    columns?: Record<string, string>;
  };
  try {
    raw = JSON.parse(text) as typeof raw;
  } catch (err) {
    throw new ColumnMappingError(
      `Invalid column-mapping JSON: ${err instanceof Error ? err.message : String(err)}`,
    );
  }
  if (!raw.columns || Object.keys(raw.columns).length === 0) {
    throw new ColumnMappingError(
      `Column-mapping config needs a non-empty 'columns' object: ${path}`,
    );
  }
  const cleaned: Record<string, string> = {};
  for (const [field, source] of Object.entries(raw.columns)) {
    const fieldKey = field.trim();
    const sourceKey = source.trim();
    if (!fieldKey || !sourceKey) {
      throw new ColumnMappingError(
        `Column-mapping entries cannot be empty: ${path}`,
      );
    }
    if (sourceKey.includes("<YOUR_")) {
      throw new ColumnMappingError(
        `Column-mapping still contains placeholder '${sourceKey}'. Replace every <YOUR_…_COLUMN> value with your spreadsheet header.`,
      );
    }
    cleaned[fieldKey] = sourceKey;
  }
  const missing = REQUIRED_COLUMNS.filter((f) => !(f in cleaned));
  if (missing.length) {
    throw new ColumnMappingError(
      `Column-mapping is missing required fields: ${JSON.stringify(missing)}. Required: ${JSON.stringify(REQUIRED_COLUMNS)}`,
    );
  }
  const unknown = Object.keys(cleaned).filter(
    (k) => !(REQUIRED_COLUMNS as readonly string[]).includes(k),
  );
  if (unknown.length) {
    throw new ColumnMappingError(
      `Column-mapping has unknown fields: ${JSON.stringify(unknown)}. Allowed: ${JSON.stringify(REQUIRED_COLUMNS)}`,
    );
  }
  return {
    name: safePosName(raw.name ?? DEFAULT_POS_NAME),
    skip_rows: raw.skip_rows ?? 0,
    columns: cleaned,
  };
}

export function normalizeMappedExcel(
  data: Buffer | Uint8Array,
  config: ColumnMappingConfig,
): LineItem[] {
  const { headers, rows } = readTable(data, config.skip_rows);
  const rename = new Map<string, string>();
  const missingHeaders: string[] = [];
  for (const [field, header] of Object.entries(config.columns)) {
    const normalizedHeader = normalizeHeader(header);
    if (!headers.includes(normalizedHeader)) {
      missingHeaders.push(header);
      continue;
    }
    rename.set(normalizedHeader, field);
  }
  if (missingHeaders.length) {
    throw new ColumnMappingError(
      `Mapped spreadsheet headers were not found after reading the export: ${JSON.stringify(missingHeaders)}. Available columns: ${JSON.stringify(headers)}`,
    );
  }
  const out: LineItem[] = [];
  for (const row of rows) {
    const remapped: RowMap = {};
    for (const [header, value] of Object.entries(row)) {
      const field = rename.get(header);
      if (field) remapped[field] = value;
    }
    const item = tryMappedLineItem(remapped);
    if (item) out.push(item);
  }
  if (out.length === 0) {
    throw new ColumnMappingError(
      "Mapped export produced no valid rows after type conversion.",
    );
  }
  return out;
}

function tryMappedLineItem(row: RowMap): LineItem | null {
  const bill_number = cellAsString(getCell(row, "bill_number"))?.trim();
  const menu = cellAsString(getCell(row, "menu"))?.trim();
  if (!bill_number || !menu) return null;
  const qty = cellAsI64(getCell(row, "qty"));
  const price = cellAsF64(getCell(row, "price"));
  const total = cellAsF64(getCell(row, "total_after_bill_discount"));
  const order_time = cellAsDateTime(getCell(row, "order_time"));
  const menu_category = cellAsString(getCell(row, "menu_category"))?.trim();
  const menu_category_detail = cellAsString(
    getCell(row, "menu_category_detail"),
  )?.trim();
  if (
    qty == null ||
    price == null ||
    total == null ||
    !order_time ||
    !menu_category ||
    !menu_category_detail
  ) {
    return null;
  }
  return {
    bill_number,
    menu,
    qty,
    price,
    total_after_bill_discount: total,
    order_time,
    menu_category,
    menu_category_detail,
  };
}

function safePosName(name: string): string {
  const slug = name
    .toLowerCase()
    .replace(/[^a-z0-9_-]+/g, "-")
    .replace(/^-+|-+$/g, "");
  return slug || DEFAULT_POS_NAME;
}

function expandUser(path: string): string {
  if (path === "~") return homedir();
  if (path.startsWith("~/")) return join(homedir(), path.slice(2));
  return isAbsolute(path) ? path : path;
}
