import { ExcelError } from "../excel.js";
import type { LineItem } from "../line-item.js";
import { configFor, detectPosFromExcelBytes } from "../pos.js";
import { NormalizeError } from "./errors.js";
import { normalizeEsbExcel } from "./esb.js";
import { normalizeMokaExcel } from "./moka.js";
import { normalizeQuinoExcel } from "./quino.js";

export { NormalizeError } from "./errors.js";

export function normalizeForPos(
  pos: string,
  data: Buffer | Uint8Array,
): LineItem[] {
  const skip = configFor(pos)?.skipRows ?? 0;
  try {
    switch (pos) {
      case "esb":
        return normalizeEsbExcel(data, skip);
      case "moka":
        return normalizeMokaExcel(data, skip);
      case "quino":
        return normalizeQuinoExcel(data, skip);
      default:
        throw new NormalizeError(`Unsupported POS system detected: ${pos}`);
    }
  } catch (err) {
    if (err instanceof NormalizeError || err instanceof ExcelError) throw err;
    throw new NormalizeError(err instanceof Error ? err.message : String(err));
  }
}

export function detectAndNormalize(
  data: Buffer | Uint8Array,
): { pos: string; rows: LineItem[] } {
  const pos = detectPosFromExcelBytes(data) ?? "unknown";
  if (!configFor(pos)) {
    throw new NormalizeError(
      `Unsupported POS system detected: ${pos}. Add a pos-column-mapping.json under MENUYUKTI_DATA_DIR (or set MENUYUKTI_COLUMN_MAPPING) to map your export columns.`,
    );
  }
  return { pos, rows: normalizeForPos(pos, data) };
}
