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
2. If nothing is configured, offer `demo-cafe-july-2026` with example asks: summarize; show venue slot strength chart; menu engineering (needs COGS).

## Onboard a new folder

1. `list_workspace_files`
2. Infer sales / COGS roles; confirm with the owner if unclear
3. `write_workspace_config`
4. `summarize_local_sales` with the workspace folder path

## Goal → tool (prefer MCP)

| Goal | Tool |
| --- | --- |
| Setup / diagnosis | `get_setup_status` |
| How did we do? | `summarize_local_sales` |
| When each dish sells | `get_menu_heatmaps` |
| Venue busy / staffing chart | `render_venue_slot_strength` |
| Venue busy (numbers only) | `get_venue_slot_strength` |
| Combos / upsells | `get_pair_lift_matrix` (sales alone; COGS upgrades star focus) |
| Stars / plow horses / puzzles / weak | `analyze_menu_engineering` (**sales + COGS**) |
| Reprice / improve / remove | `get_weak_items_review` (**sales + COGS**) |
| Cleaned rows | `list_normalized_sales` |
| Venue notes | `read_workspace_knowledge` then `write_workspace_knowledge` (full replace) |

## COGS gate

Menu engineering and weak-items need a unit-cost Excel as `files.cogs`. If missing: ask the owner to drop COGS, then `list_workspace_files` + `write_workspace_config`. Sales-only tools still work without COGS.

## Offline venue slot strength

If Menuyukti MCP is **not** available and the user only needs weekday × meal-period demand from a local Excel (especially custom columns), use the **`menuyukti-venue-slot-strength`** skill / script. Do **not** reimplement menu engineering, pair lift, or heatmaps in this skill.
