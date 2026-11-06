import {
  cellAsDateTime,
  cellAsF64,
  cellAsI64,
  cellAsString,
  getCell,
  readTable,
  type RowMap,
} from "../excel.js";
import type { LineItem } from "../line-item.js";
import { harmonizeCategories, normalizeCategory } from "./category.js";
import { NormalizeError } from "./errors.js";

const MOKA_SOURCE_COLUMNS = [
  "payment_no",
  "item_name",
  "quantity",
  "price",
  "net_sales",
  "created_at",
  "category",
  "variant_name",
];

export function normalizeMokaExcel(
  data: Buffer | Uint8Array,
  skipRows: number,
): LineItem[] {
  const { headers, rows } = readTable(data, skipRows);
  for (const col of MOKA_SOURCE_COLUMNS) {
    if (!headers.includes(col)) {
      throw new NormalizeError(
        "Missing required MOKA columns for Export Item Details: Payment No, Item Name, Quantity, Price, Net Sales, Created At, Category, Variant Name. Expected Moka Reports → Transactions → Export Item Details (title row + header on row 4).",
      );
    }
  }
  const candidates: LineItem[] = [];
  for (const row of rows) {
    const item = tryLineItem(row);
    if (item) candidates.push(item);
  }
  return harmonizeCategories(candidates);
}

function tryLineItem(row: RowMap): LineItem | null {
  const bill_number = cellAsString(getCell(row, "payment_no"))?.trim();
  const menu = cellAsString(getCell(row, "item_name"))?.trim();
  if (!bill_number || !menu) return null;
  const qty = cellAsI64(getCell(row, "quantity"));
  const price = cellAsF64(getCell(row, "price"));
  const total = cellAsF64(getCell(row, "net_sales"));
  if (qty == null || price == null || total == null || qty <= 0 || total <= 0) {
    return null;
  }
  const order_time = cellAsDateTime(getCell(row, "created_at"));
  if (!order_time) return null;
  return {
    bill_number,
    menu,
    qty,
    price,
    total_after_bill_discount: total,
    order_time,
    menu_category: normalizeCategory(
      cellAsString(getCell(row, "category")),
      "OTHER",
    ),
    menu_category_detail: normalizeCategory(
      cellAsString(getCell(row, "variant_name")),
      "UNCATEGORIZED",
    ),
  };
}
