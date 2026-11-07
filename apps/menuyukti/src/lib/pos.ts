import { ExcelError, readA1String } from "./excel.js";

export type PosConfig = {
  key: string;
  detectionPattern: string;
  skipRows: number;
};

export const POS_CONFIG: PosConfig[] = [
  {
    key: "esb",
    detectionPattern: "sales recapitulation",
    skipRows: 10,
  },
  {
    key: "quino",
    detectionPattern: "transaction item detail report",
    skipRows: 3,
  },
  {
    key: "moka",
    detectionPattern: "item details",
    skipRows: 3,
  },
];

export function detectFromA1(cellValue: string): string | null {
  const v = cellValue.toLowerCase();
  return (
    POS_CONFIG.find((cfg) => v.includes(cfg.detectionPattern))?.key ?? null
  );
}

export function configFor(pos: string): PosConfig | null {
  return POS_CONFIG.find((cfg) => cfg.key === pos) ?? null;
}

export function detectPosFromExcelBytes(
  data: Buffer | Uint8Array,
): string | null {
  try {
    const a1 = readA1String(data);
    return a1 ? detectFromA1(a1) : null;
  } catch (err) {
    if (err instanceof ExcelError) throw err;
    throw new ExcelError(String(err));
  }
}
