/** Canonical POS transaction line item. */

export const REQUIRED_COLUMNS = [
  "bill_number",
  "menu",
  "qty",
  "price",
  "total_after_bill_discount",
  "order_time",
  "menu_category",
  "menu_category_detail",
] as const;

export type LineItem = {
  bill_number: string;
  menu: string;
  qty: number;
  price: number;
  total_after_bill_discount: number;
  /** ISO-like local datetime `YYYY-MM-DDTHH:mm:ss` */
  order_time: string;
  menu_category: string;
  menu_category_detail: string;
};
