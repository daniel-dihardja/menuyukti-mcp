import * as XLSX from "xlsx";

export type CellValue = string | number | boolean | Date | null | undefined;
export type RowMap = Record<string, CellValue>;

export class ExcelError extends Error {
  constructor(message: string) {
    super(message);
    this.name = "ExcelError";
  }
}

export function normalizeHeader(value: string): string {
  return value
    .trim()
    .replaceAll("\u00a0", " ")
    .replaceAll("\t", "")
    .toLowerCase()
    .replaceAll(" ", "_");
}

export function readA1String(data: Buffer | Uint8Array): string | null {
  const workbook = openWorkbook(data);
  const sheet = firstSheet(workbook);
  const rows = XLSX.utils.sheet_to_json<CellValue[]>(sheet, {
    header: 1,
    raw: true,
    defval: null,
  });
  const first = rows[0]?.[0];
  return cellAsString(first);
}

export function readTable(
  data: Buffer | Uint8Array,
  skipRows: number,
): { headers: string[]; rows: RowMap[] } {
  const workbook = openWorkbook(data);
  return readTableFromSheet(firstSheet(workbook), skipRows);
}

export function readTableSheet(
  data: Buffer | Uint8Array,
  sheetName: string,
  skipRows: number,
): { headers: string[]; rows: RowMap[] } {
  const workbook = openWorkbook(data);
  const want = sheetName.trim().toLowerCase();
  const name = workbook.SheetNames.find((n) => n.trim().toLowerCase() === want);
  if (!name) {
    throw new ExcelError(
      `Excel workbook has no sheet named '${sheetName}' (available: ${workbook.SheetNames.join(", ")})`,
    );
  }
  return readTableFromSheet(workbook.Sheets[name]!, skipRows);
}

function openWorkbook(data: Buffer | Uint8Array): XLSX.WorkBook {
  try {
    return XLSX.read(data, {
      type: "buffer",
      cellDates: true,
      dense: false,
    });
  } catch (err) {
    throw new ExcelError(
      `failed to open Excel workbook: ${err instanceof Error ? err.message : String(err)}`,
    );
  }
}

function firstSheet(workbook: XLSX.WorkBook): XLSX.WorkSheet {
  const name = workbook.SheetNames[0];
  if (!name) throw new ExcelError("Excel workbook has no worksheets");
  return workbook.Sheets[name]!;
}

function readTableFromSheet(
  sheet: XLSX.WorkSheet,
  skipRows: number,
): { headers: string[]; rows: RowMap[] } {
  // Keep blank rows so skip_rows matches physical Excel rows (calamine parity).
  const matrix = XLSX.utils.sheet_to_json<CellValue[]>(sheet, {
    header: 1,
    raw: true,
    defval: null,
    blankrows: true,
  });
  if (matrix.length <= skipRows) {
    throw new ExcelError("Excel has no header row after skip_rows");
  }
  const headerRow = matrix[skipRows] ?? [];
  const headers = headerRow.map((cell) =>
    normalizeHeader(cellAsString(cell) ?? ""),
  );
  const rows: RowMap[] = [];
  for (let i = skipRows + 1; i < matrix.length; i += 1) {
    const row = matrix[i] ?? [];
    if (row.every((c) => c === null || c === undefined || c === "")) continue;
    const map: RowMap = {};
    headers.forEach((header, idx) => {
      if (!header) return;
      map[header] = row[idx] ?? null;
    });
    rows.push(map);
  }
  return { headers, rows };
}

export function cellAsString(cell: CellValue): string | null {
  if (cell === null || cell === undefined || cell === "") return null;
  if (cell instanceof Date) {
    return formatDateTime(cell);
  }
  if (typeof cell === "string") {
    const t = cell.trim();
    return t === "" ? null : t;
  }
  if (typeof cell === "number") {
    return Number.isInteger(cell) ? String(cell) : String(cell);
  }
  if (typeof cell === "boolean") return String(cell);
  return null;
}

export function cellAsF64(cell: CellValue): number | null {
  if (typeof cell === "number" && Number.isFinite(cell)) return cell;
  if (typeof cell === "string") {
    const cleaned = cell.trim().replaceAll(",", "");
    const n = Number(cleaned);
    return Number.isFinite(n) ? n : null;
  }
  if (cell instanceof Date) return null;
  return null;
}

export function cellAsI64(cell: CellValue): number | null {
  const n = cellAsF64(cell);
  if (n === null) return null;
  return Math.round(n);
}

export function cellAsDateTime(cell: CellValue): string | null {
  if (cell instanceof Date && !Number.isNaN(cell.getTime())) {
    return formatDateTime(cell);
  }
  if (typeof cell === "number" && Number.isFinite(cell)) {
    return excelSerialToDateTime(cell);
  }
  if (typeof cell === "string") {
    return parseFlexibleDateTime(cell);
  }
  return null;
}

export function getCell(row: RowMap, key: string): CellValue {
  return row[key];
}

function formatDateTime(d: Date): string {
  const pad = (n: number) => String(n).padStart(2, "0");
  // SheetJS/Date often in local or UTC; use UTC components if getTimezoneOffset suggests Excel serials were UTC.
  // Prefer local wall time matching typical POS exports.
  return `${d.getFullYear()}-${pad(d.getMonth() + 1)}-${pad(d.getDate())}T${pad(d.getHours())}:${pad(d.getMinutes())}:${pad(d.getSeconds())}`;
}

function excelSerialToDateTime(serial: number): string | null {
  // Excel epoch 1899-12-30 (with Lotus bug compatibility used by calamine)
  const ms = Date.UTC(1899, 11, 30) + serial * 86400000;
  const d = new Date(ms);
  if (Number.isNaN(d.getTime())) return null;
  const pad = (n: number) => String(n).padStart(2, "0");
  return `${d.getUTCFullYear()}-${pad(d.getUTCMonth() + 1)}-${pad(d.getUTCDate())}T${pad(d.getUTCHours())}:${pad(d.getUTCMinutes())}:${pad(d.getUTCSeconds())}`;
}

function parseFlexibleDateTime(raw: string): string | null {
  const s = raw.trim();
  if (!s) return null;
  const patterns: Array<{
    re: RegExp;
    y: number;
    mo: number;
    d: number;
    h?: number;
    mi?: number;
    se?: number;
  }> = [];
  void patterns;
  // Try Date.parse for ISO-ish first
  const iso = Date.parse(s.includes("T") ? s : s.replace(" ", "T"));
  if (!Number.isNaN(iso)) {
    return formatDateTime(new Date(iso));
  }
  const m =
    s.match(/^(\d{4})-(\d{2})-(\d{2})(?:[ T](\d{2}):(\d{2})(?::(\d{2}))?)?/) ||
    s.match(
      /^(\d{2})[/-](\d{2})[/-](\d{4})(?:[ T](\d{2}):(\d{2})(?::(\d{2}))?)?/,
    );
  if (!m) return null;
  // Heuristic: if first group is 4 digits it's Y-M-D else D-M-Y or M-D-Y — prefer D-M-Y for ID locales
  let year: number;
  let month: number;
  let day: number;
  if (m[1]!.length === 4) {
    year = Number(m[1]);
    month = Number(m[2]);
    day = Number(m[3]);
  } else {
    day = Number(m[1]);
    month = Number(m[2]);
    year = Number(m[3]);
  }
  const h = Number(m[4] ?? 0);
  const mi = Number(m[5] ?? 0);
  const se = Number(m[6] ?? 0);
  const pad = (n: number) => String(n).padStart(2, "0");
  return `${year}-${pad(month)}-${pad(day)}T${pad(h)}:${pad(mi)}:${pad(se)}`;
}
