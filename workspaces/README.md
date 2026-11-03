# Workspaces

Put venue/period analysis folders here. Each folder can hold sales exports, COGS, notes, and later AI knowledge.

Menuyukti reads files from here by default (`MENUYUKTI_DATA_DIR=workspaces`).

## Default demo workspace

| Path | Purpose |
| --- | --- |
| [`demo-cafe-july-2026/sales.xlsx`](./demo-cafe-july-2026/sales.xlsx) | Anonymized cafe month with real sales data |
| [`demo-cafe-july-2026/cogs.xlsx`](./demo-cafe-july-2026/cogs.xlsx) | Matching menu COGS (hospitality starter estimates) |

Use the sales path in MCP tools:

```json
{
  "path": "demo-cafe-july-2026/sales.xlsx"
}
```

The COGS sheet is `Menu` + `COGS` (unit cost in IDR). Replace values with your real recipe costs when you have them.

## Your own workspaces

1. Create a subdirectory (for example `my-venue-aug-2026/`) and drop Excel files into it.
2. Call `list_workspaces` to see available folders and files, then other tools with the relative path, for example `{"path": "my-venue-aug-2026/sales.xlsx"}`.
3. For unsupported POS column layouts, add `pos-column-mapping.json` under `workspaces/` (see the root README).

Personal uploads in this folder are gitignored. Only the default demo workspace above is committed.
