---
id: doc-1
title: Menuyukti 1.0 product definition
type: specification
created_date: '2026-11-06T11:27:00Z'
updated_date: '2026-11-06T11:29:00Z'
---
# Menuyukti 1.0 product definition

Menuyukti **1.0** is the first stable local MCP product for restaurant owners analyzing POS Excel sales on their own machine (ChatGPT Desktop / Codex).

This document is the release contract: what must be true to claim 1.0. Semver `1.0.0` means this contract. Scope freeze: [decision-2](../decisions/decision-2%20-%20Freeze-Menuyukti-1.0-product-scope.md).

## Owner outcomes

| Outcome | What the owner decides | Primary capabilities |
| --- | --- | --- |
| **Plan promotions** | When to promote, which dishes/combos to push, where demand is soft or strong | Venue slot strength, menu heatmaps, pair lift index |
| **Trim / compact the menu** | Which dishes to reprice, improve, or remove so the menu is tighter | Weak items analysis (via menu engineering); heatmaps/pair lift as supporting context |

Supporting setup (workspaces, local MCP, plugin) is required to *reach* those outcomes but is not itself a third product outcome for 1.0.

## Capability → tool / visualisation matrix

| Capability | Owner use | Data needs | Data tool | Visualisation (preferred) | Codex / no-widget fallback |
| --- | --- | --- | --- | --- | --- |
| **Venue slot strength** | Staffing and promotion timing by weekday × meal period | Sales | `get_venue_slot_strength` | `render_venue_slot_strength` (MCP Apps widget) | `menuyukti-venue-slot-strength` → one local HTML heatmap |
| **Menu heatmaps** | When each dish sells (hour / weekday); optional matrix-category filters | Sales; COGS optional (`matrix_category`) | `get_menu_heatmaps` | `render_menu_heatmaps` (MCP Apps widget) | `menuyukti-visualize-menu-heatmaps` → one local HTML file |
| **Pair lift index** | Combo / upsell ideas (co-purchase lift) | Sales; COGS optional (star focus) | `get_pair_lift_matrix` | `render_pair_lift_matrix` (MCP Apps widget) | Numbers via `get_pair_lift_matrix` (no dedicated Codex HTML skill in 1.0) |
| **Weak items analysis** | Reprice / improve / remove candidates (weak quadrant depth) | Sales + COGS | `get_weak_items_review` | `render_weak_items_review` (MCP Apps widget) | `menuyukti-visualize-weak-items` → one local HTML table |

**Presentation rule:** one chart path per ask — prefer the MCP Apps widget when the host mounts it; otherwise data tool + skill HTML. Never widget + skill redraw in the same turn.

**Related (enables weak items / star focus, not a fifth 1.0 headline capability):** `analyze_menu_engineering` — full star / plow_horse / puzzle / weak matrix; requires sales + COGS.

## Outcome → capability mapping

### Plan promotions

1. **Venue slot strength** — which dayparts are weak/strong → when to run promos or staff up.
2. **Menu heatmaps** — which dishes move in which hours/weekdays → time-box featured items.
3. **Pair lift index** — which items lift together → combo offers and upsells (stars as focus when COGS present).

### Trim / compact the menu

1. **Weak items analysis** — weak-quadrant items with margin share and depth → candidates to reprice, improve, or cut.
2. **Menu heatmaps** (optional context) — confirm whether a weak item has any strong daypart before cutting.
3. **Pair lift** (optional context) — avoid cutting an item that strongly lifts a star.

## Enabling surfaces (in 1.0 platform, not expanding the outcome contract)

These must work well enough for owners to reach the four capabilities:

- Local MCP server under `MENUYUKTI_DATA_DIR` (path-gated Excel)
- Workspaces + `workspace.yaml` onboarding (`get_setup_status`, `list_workspaces`, `write_workspace_config`, …)
- Codex / ChatGPT Desktop plugin (skills + localhost MCP)
- Optional custom POS column mapping
- Demo workspace `demo-cafe-july-2026`
- Summary / normalized sales / knowledge.md as coach aids

## Explicitly out of scope for 1.0 (deferred)

- Cloud / multi-tenant SaaS analytics or remote sales upload
- Writing back menu changes to a POS
- Automated “remove this item” actions or keep/remove columns in weak-items UI
- Multi-venue rollup dashboards
- Dedicated Codex HTML skill for pair lift (widget + JSON is enough for 1.0)
- Pricing optimizer beyond weak-items / menu-engineering review
- Mobile-native or non-Desktop ChatGPT hosting of localhost MCP

## Audit checklist (filled by TASK-43.3)

Evidence date: 2026-11-06. Sources: `crates/menuyukti-mcp/tests/mcp_http.rs` tool list, `crates/menuyukti-mcp/assets/`, `crates/menuyukti-mcp/preview/`, `README.md`, `docs/howtos/workspaces.md`, `.agents/skills/`.

| Capability | Data tool | Render tool | Widget asset | Owner docs | Status |
| --- | --- | --- | --- | --- | --- |
| Venue slot strength | `get_venue_slot_strength` | `render_venue_slot_strength` | `venue_slot_strength_widget.html` + preview | README + workspaces howto + skill | **Pass** |
| Menu heatmaps | `get_menu_heatmaps` | `render_menu_heatmaps` | `menu_heatmaps_widget.html` + preview | README + workspaces howto + skill | **Pass** |
| Pair lift index | `get_pair_lift_matrix` | `render_pair_lift_matrix` | `pair_lift_matrix_widget.html` + preview | README + workspaces howto | **Pass** (Codex HTML skill deferred by design) |
| Weak items analysis | `get_weak_items_review` | `render_weak_items_review` | `weak_items_review_widget.html` + preview | README + workspaces howto + skill | **Pass** |

### Audit notes

- No must-have gaps for the 1.0 capability contract; do not file follow-up fix tasks from this audit.
- Non-blocking polish: pair-lift Codex HTML skill remains explicitly out of scope. (`render_venue_slot_strength` tool-list bullet was added during TASK-43.4.)
- HTTP integration tests assert all four `get_` and `render_` tools are listed and demo-cafe paths succeed.
