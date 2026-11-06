import {
  cellAsDateTime,
  cellAsF64,
  cellAsI64,
  cellAsString,
  getCell,
  readTable,
  type RowMap,
} from "../excel.js";
import { REQUIRED_COLUMNS, type LineItem } from "../line-item.js";
import { NormalizeError } from "./errors.js";

export function normalizeEsbExcel(
  data: Buffer | Uint8Array,
  skipRows: number,
): LineItem[] {
  const { headers, rows } = readTable(data, skipRows);
  for (const col of REQUIRED_COLUMNS) {
    if (!headers.includes(col)) {
      throw new NormalizeError(`Missing required columns: [${col}]`);
    }
  }
  const out: LineItem[] = [];
  for (const row of rows) {
    const item = tryLineItem(row);
    if (item) out.push(item);
  }
  return out;
}

function tryLineItem(row: RowMap): LineItem | null {
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
