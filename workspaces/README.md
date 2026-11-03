# Workspaces

Put venue/period folders here. Each folder can hold sales exports, COGS, notes, and later AI knowledge.

Gastro MCP reads files from here by default (`MENUYUKTI_DATA_DIR=workspaces`).

For the full guide, see **[Workspaces howto](../docs/howtos/workspaces.md)**.

## Default demo workspace

| Path | Purpose |
| --- | --- |
| [`demo-cafe-july-2026/workspace.json`](./demo-cafe-july-2026/workspace.json) | Declares which file is sales vs COGS |
| [`demo-cafe-july-2026/sales.xlsx`](./demo-cafe-july-2026/sales.xlsx) | Anonymized cafe month with real sales data |
| [`demo-cafe-july-2026/cogs.xlsx`](./demo-cafe-july-2026/cogs.xlsx) | Matching menu COGS (hospitality starter estimates) |

Use the sales path in MCP tools:

```json
{
  "path": "demo-cafe-july-2026/sales.xlsx"
}
```

## Your own workspaces

1. Create a subdirectory (for example `my-venue-aug-2026/`) and drop Excel files into it.
2. Copy [`workspace.example.json`](../workspace.example.json) in as `workspace.json` and set the `files` roles.
3. Call `list_workspaces` (or `list_workspace_files` with `{"path": "my-venue-aug-2026"}`), then pass a relative path such as `{"path": "my-venue-aug-2026/sales.xlsx"}`.
4. For unsupported POS layouts, see [Custom POS columns](../docs/howtos/custom-pos-columns.md).

Personal uploads in this folder are gitignored. Only the default demo workspace above is committed.
