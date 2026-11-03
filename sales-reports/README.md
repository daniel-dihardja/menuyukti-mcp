# Sales reports

Put your POS sales Excel exports in this folder.

Menuyukti reads files from here by default (`MENUYUKTI_DATA_DIR=sales-reports`).

## Default sample

[`demo-cafe-sales-july-2026.xlsx`](./demo-cafe-sales-july-2026.xlsx) — anonymized cafe month with real prices. Use this path in MCP tools:

```json
{
  "path": "demo-cafe-sales-july-2026.xlsx"
}
```

## Your own exports

1. Drop your Excel file into this folder.
2. Call `list_sales_reports` to see available files, then other tools with the filename, for example `{"path": "my-export.xlsx"}`.
3. For unsupported POS column layouts, add `pos-column-mapping.json` here (see the root README).

Personal uploads in this folder are gitignored. Only the default sample above is committed.
