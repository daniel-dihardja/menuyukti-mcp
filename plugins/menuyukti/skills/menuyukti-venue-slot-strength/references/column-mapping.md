# Custom POS column mapping

Same contract as Menuyukti `pos-column-mapping.json` (see repo `docs/howtos/custom-pos-columns.md`).

## Required fields

| Field | Meaning |
| --- | --- |
| `bill_number` | Order / check / bill id (distinct bills = orders for slot strength) |
| `menu` | Item name |
| `qty` | Line quantity |
| `price` | Unit price |
| `total_after_bill_discount` | Net sales for the line |
| `order_time` | Sold-at timestamp (date + time) |
| `menu_category` | Category |
| `menu_category_detail` | Subcategory |

All eight keys are required for a valid mapping. Leave no `<YOUR_…_COLUMN>` placeholders.

## Example

```json
{
  "name": "my-pos",
  "skip_rows": 0,
  "columns": {
    "bill_number": "Bill No.",
    "menu": "Item Name",
    "qty": "Qty",
    "price": "Unit Price",
    "total_after_bill_discount": "Net Sales",
    "order_time": "Sold At",
    "menu_category": "Category",
    "menu_category_detail": "Sub Category"
  }
}
```

- `skip_rows`: rows to skip **before** the header row (`0` if row 1 is headers).
- `name`: optional label (slugified; defaults to `custom`).
- Values are the **exact spreadsheet header text** as shown to the owner.

## Header matching

When matching headers, normalize both mapping values and sheet headers:

1. Trim whitespace
2. Replace non-breaking spaces with spaces; strip tabs
3. Lowercase
4. Replace spaces with `_`

Example: `Bill No.` → `bill_no.`

## Slot-strength minimum

Venue slot strength aggregation needs `bill_number` + `order_time`. Keep the full eight-field mapping so the same file works with Menuyukti MCP analytics later.
