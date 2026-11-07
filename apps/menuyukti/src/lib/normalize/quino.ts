import {
  cellAsDateTime,
  cellAsF64,
  cellAsI64,
  cellAsString,
  getCell,
  type RowMap,
  readTable,
} from "../excel.js";
import type { LineItem } from "../line-item.js";
import { harmonizeCategories, normalizeCategory } from "./category.js";
import { NormalizeError } from "./errors.js";

const QUINO_SOURCE_COLUMNS = [
  "inv_no.",
  "name",
  "qty",
  "subtotal",
  "discount",
  "order_time",
  "department",
  "category",
];

export function normalizeQuinoExcel(
  data: Buffer | Uint8Array,
  skipRows: number,
): LineItem[] {
  const { headers, rows } = readTable(data, skipRows);
  for (const col of QUINO_SOURCE_COLUMNS) {
    if (!headers.includes(col)) {
      throw new NormalizeError(
        "Missing required QUINO columns for Transaction Item Detail Report: Inv No., Name, Qty, Subtotal, Discount, Order Time, Department, Category. Expected Quino Transaction Item Detail Report export (title row + header on row 4).",
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
  const bill_number = cellAsString(getCell(row, "inv_no."))?.trim();
  const menu = cellAsString(getCell(row, "name"))?.trim();
  if (!bill_number || !menu) return null;
  const menuUpper = menu.toUpperCase();
  if (menuUpper.startsWith("TOTAL ") || menuUpper === "GRAND TTL") return null;
  const qty = cellAsI64(getCell(row, "qty"));
  const subtotal = cellAsF64(getCell(row, "subtotal"));
  const discount = cellAsF64(getCell(row, "discount")) ?? 0;
  if (qty == null || subtotal == null) return null;
  const total = subtotal - discount;
  if (qty <= 0 || total <= 0) return null;
  const order_time = cellAsDateTime(getCell(row, "order_time"));
  if (!order_time) return null;
  return {
    bill_number,
    menu,
    qty,
    price: total / qty,
    total_after_bill_discount: total,
    order_time,
    menu_category: normalizeCategory(
      cellAsString(getCell(row, "department")),
      "OTHER",
    ),
    menu_category_detail: normalizeCategory(
      cellAsString(getCell(row, "category")),
      "UNCATEGORIZED",
    ),
  };
}
