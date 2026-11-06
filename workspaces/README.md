# Workspaces

Put venue/period folders here. Each folder can hold sales exports, COGS, and location notes (`knowledge.md`).

Menuyukti MCP reads files from here by default (`MENUYUKTI_DATA_DIR=workspaces`).

**Owner ritual:** run the MCP server → drop sales (+ optional COGS) into a folder → in ChatGPT ask Menuyukti to set up the workspace, then ask outcome questions. See **[Workspaces howto](../docs/howtos/workspaces.md)** and the **[What to ask](../README.md#what-to-ask)** cheat sheet.

## Default demo workspace

| Path | Purpose |
| --- | --- |
| [`demo-cafe-july-2026/workspace.yaml`](./demo-cafe-july-2026/workspace.yaml) | Declares which file is sales vs COGS (optional `knowledge` → `knowledge.md`) |
| [`demo-cafe-july-2026/sales.xlsx`](./demo-cafe-july-2026/sales.xlsx) | Anonymized cafe month with real sales data |
| [`demo-cafe-july-2026/cogs.xlsx`](./demo-cafe-july-2026/cogs.xlsx) | Matching menu COGS (hospitality starter estimates) |
| `demo-cafe-july-2026/knowledge.md` | Optional location/venue notes (created by ChatGPT via knowledge tools when used) |

Owners only drop Excel files; ChatGPT writes `workspace.yaml` and maintains `knowledge.md` for location notes. Summarize or run menu engineering with the folder path:

```json
{
  "path": "demo-cafe-july-2026"
}
```

Menu engineering (`analyze_menu_engineering`) and weak items review (`get_weak_items_review`) need both sales and COGS roles configured. Summary, heatmaps (`get_menu_heatmaps` / `render_menu_heatmaps`), venue slot strength (`get_venue_slot_strength` / `render_venue_slot_strength`), and pair lift (`get_pair_lift_matrix` / `render_pair_lift_matrix`) work with sales alone; COGS upgrades pair-lift focus to stars and adds `matrix_category` on menu heatmaps when available.

In ChatGPT, ask to **show menu heatmaps**, **show the pair lift heatmap**, or **show venue slot strength** on `demo-cafe-july-2026` — the agent should call `render_menu_heatmaps`, `render_pair_lift_matrix`, or `render_venue_slot_strength` for the MCP Apps widget. In Codex, ask to **show venue slot strength** or **show menu heatmaps**; MCP returns JSON and the matching visualisation skill draws local HTML.

## Your own workspaces

1. Create a subdirectory (for example `my-venue-aug-2026/`) and drop **sales** + optional **COGS** Excel into it.
2. In ChatGPT, ask Menuyukti MCP to set up the workspace (it writes `workspace.yaml` — you do not edit that file).
3. Ask it to summarize `my-venue-aug-2026`, or run menu engineering once COGS is configured.
4. For unsupported POS layouts, see [Custom POS columns](../docs/howtos/custom-pos-columns.md).

Personal uploads in this folder are gitignored. Only the default demo workspace above is committed.
