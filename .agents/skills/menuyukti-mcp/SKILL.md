---
name: menuyukti-mcp
description: >-
  Coach owners and agents through Menuyukti MCP: local POS Excel workspaces,
  setup status, onboarding, and analytics via MCP tools. Use when the user
  mentions Menuyukti, workspace.yaml, MENUYUKTI_DATA_DIR, restaurant sales MCP,
  menu engineering, pair lift, venue slot strength with MCP, or ChatGPT Desktop
  local sales tools. Prefer Menuyukti MCP tools when connected; do not recompute
  analytics by hand.
---

# Menuyukti MCP coaching

Help the user work with **Menuyukti MCP** (local POS Excel analytics). Prefer MCP tools over reinventing math.

## Privacy

Keep POS / COGS files on the user’s machine. Do not upload spreadsheets to remote services unless the user explicitly asks.

## 3-step ritual

1. Menuyukti MCP server running and connected (`http://127.0.0.1:8787/mcp` by default).
2. Sales Excel (+ optional COGS) in one folder per venue/month under `MENUYUKTI_DATA_DIR`.
3. Ask to **set up the workspace**, then ask outcome questions.

## First contact

When unclear or path/config fails:

1. Call `get_setup_status` (and `list_workspaces` if useful).
2. If nothing is configured, offer `demo-cafe-july-2026` with example asks: summarize; show venue slot strength; show menu heatmaps; show weak items table (needs COGS); menu engineering (needs COGS).

## Onboard a new folder

1. `list_workspace_files`
2. Infer sales / COGS roles; confirm with the owner if unclear
3. `write_workspace_config`
4. `summarize_local_sales` with the workspace folder path

## Goal → tool (prefer MCP)

| Goal | Tool / skill |
| --- | --- |
| Setup / diagnosis | `get_setup_status` |
| How did we do? | `summarize_local_sales` |
| When each dish sells (numbers) | `get_menu_heatmaps` |
| When each dish sells (show / chart) | `get_menu_heatmaps`, then **`menuyukti-visualize-menu-heatmaps`** |
| Venue busy / staffing (show / chart) | **One path only:** `render_venue_slot_strength` (widget) **or** `get_venue_slot_strength` + **`menuyukti-venue-slot-strength`** redraw — never both |
| Venue busy / staffing (numbers only) | `get_venue_slot_strength` |
| Combos / upsells | `get_pair_lift_matrix` (sales alone; COGS upgrades star focus) |
| Stars / plow horses / puzzles / weak | `analyze_menu_engineering` (**sales + COGS**) |
| Reprice / improve / remove (numbers) | `get_weak_items_review` (**sales + COGS**) |
| Weak items table / show weak dishes | `get_weak_items_review`, then **`menuyukti-visualize-weak-items`** (**sales + COGS**) |
| Cleaned rows | `list_normalized_sales` |
| Venue notes | `read_workspace_knowledge` then `write_workspace_knowledge` (full replace) |

For charts: pick **one** presentation. If `render_venue_slot_strength` mounted a widget, narrate only — do not also write HTML. If using a visualisation skill, write **one local HTML** file from the recipe (no Python/render scripts; no second markdown grid for the same ask).

## COGS gate

Menu engineering and weak-items need a unit-cost Excel as `files.cogs`. If missing: ask the owner to drop COGS, then `list_workspace_files` + `write_workspace_config`. Sales-only tools still work without COGS.

## Visualisation skills

Charts come from MCP JSON + the matching skill’s HTML recipe (`menuyukti-venue-slot-strength`, `menuyukti-visualize-menu-heatmaps`, `menuyukti-visualize-weak-items`). Prefer MCP tools for all analytics math — do **not** reimplement menu engineering, pair lift, heatmaps, or slot strength by hand.
