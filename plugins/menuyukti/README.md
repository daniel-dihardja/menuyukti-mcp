# Menuyukti Codex plugin

Portable Agent Plugin for **ChatGPT Desktop / Codex** (version **0.9.0**, approaching 1.0).

It bundles coaching + visualisation skills and connects Codex to the **local** Menuyukti MCP server at `http://127.0.0.1:8787/mcp`. Sales files stay on your machine.

Use this guide to install and operate the plugin. Deeper server, workspace, and contributor docs live in the [repo docs](../../docs/).

## What you can do

**Menuyukti 0.9** helps restaurant owners:

- **Plan promotions** — venue demand timing, per-dish heatmaps, combo / pair-lift ideas
- **Trim / compact the menu** — weak-items review and menu engineering (**needs COGS**)

Four analytics capabilities ship with tools + charts: venue slot strength, menu heatmaps, pair lift index, and weak items analysis.

**Desktop only.** The MCP URL is localhost. The plugin does **not** start the server for you — keep Skybridge running while you chat.

## Prerequisites

1. **Run the Menuyukti Skybridge MCP server** with `MENUYUKTI_DATA_DIR` set → [Install and run](../../docs/howtos/install-and-run.md) (**Node.js 24+**; no Rust required)
2. Keep **`http://127.0.0.1:8787/mcp`** reachable
3. Put sales (+ optional COGS) in a [workspace](../../docs/howtos/workspaces.md) folder (try `demo-cafe-july-2026` in this repo)

Then ask: *What’s my Menuyukti setup status?*

### Start the server (quick)

```bash
# From this repo (recommended)
pnpm install
pnpm mcp

# Or from apps/menuyukti
cd apps/menuyukti && MENUYUKTI_DATA_DIR="/path/to/workspaces" pnpm dev
```

## Install the plugin

This repository declares a local marketplace at [`.agents/plugins/marketplace.json`](../../.agents/plugins/marketplace.json) pointing at this package.

### Codex CLI

From the **repository root**:

```bash
codex plugin marketplace add .
codex plugin add menuyukti@menuyukti-repo
codex plugin list -m menuyukti-repo
```

You can also browse with `codex` then `/plugins`. Start a **new** Codex session after install so bundled skills load.

### ChatGPT Desktop

1. Open the **Plugins** tab.
2. Use your **Personal** / repo marketplace entry for this project when available, or install this local `plugins/menuyukti` package per current Desktop plugin docs.
3. Connect the Menuyukti MCP app when prompted (localhost).
4. Start a **new** chat / Codex session.

## Owner ritual

1. Keep Menuyukti MCP running and connected.
2. Create a folder under your data dir (for example `my-venue-aug-2026/`) and drop **sales** Excel (+ optional **COGS**) into it.
3. In Codex, ask Menuyukti to **set up the workspace**, then ask outcome questions.

You do **not** edit `workspace.yaml` by hand — ChatGPT / Codex writes it after you drop the files.

## What to ask

### Plan promotions

| Goal | Capability | Example prompt |
| --- | --- | --- |
| When should we promote / staff? | Venue slot strength | “Show venue slot strength for demo-cafe-july-2026” |
| When does each dish sell? | Menu heatmaps | “Show heatmaps for demo-cafe-july-2026” |
| Combo / upsell ideas | Pair lift index | “Show the pair lift heatmap for demo-cafe-july-2026” |

### Trim / compact the menu

| Goal | Capability | Example prompt |
| --- | --- | --- |
| What to reprice, improve, or cut (needs COGS) | Weak items analysis | “Show weak items table for demo-cafe-july-2026” |
| Full stars / plow horses / puzzles / weak (needs COGS) | Menu engineering | “Run menu engineering for demo-cafe-july-2026” |

### Setup and everyday asks

| Goal | Example prompt |
| --- | --- |
| Check setup / list folders | “What’s my Menuyukti setup status?” / “List my workspaces” |
| How did we do? | “Summarize demo-cafe-july-2026” |
| Remember venue facts | “Save our hours and seating in knowledge for this workspace” |

### Sales alone vs COGS

| Capability | Needs |
| --- | --- |
| Summary, menu heatmaps, venue slot strength, pair lift | Sales |
| Menu engineering matrix, weak items review | Sales + COGS |
| Pair lift with menu-engineering **stars** as focus | Sales + COGS (optional upgrade) |

Tool names and advanced options: [Workspaces](../../docs/howtos/workspaces.md). Unusual spreadsheet headers: [Custom POS columns](../../docs/howtos/custom-pos-columns.md).

## How charts work

Menuyukti MCP returns **structured analytics JSON**. Presentation depends on the host:

| Chart | Preferred presentation |
| --- | --- |
| Venue slot strength | **`render_venue_slot_strength`** (Skybridge MCP Apps view) when the host mounts widgets; otherwise `get_venue_slot_strength` + skill → **one** local HTML file |
| Pair lift index | **`render_pair_lift_matrix`** when mounted; otherwise `get_pair_lift_matrix` for numbers |
| Menu heatmaps | **`render_menu_heatmaps`** when mounted; otherwise `get_menu_heatmaps` + skill → **one** local HTML file |
| Weak items | **`render_weak_items_review`** when mounted; otherwise `get_weak_items_review` + skill → **one** local HTML table (**needs COGS**) |

**One chart per ask.** If a `render_*` view already mounted, narrate only — do **not** also write HTML in the same turn.

## Bundled skills

| Skill | Role |
| --- | --- |
| `menuyukti-mcp` | Setup ritual and goal → tool map |
| `menuyukti-venue-slot-strength` | Venue demand: prefer `render_venue_slot_strength`, else local HTML heatmap |
| `menuyukti-visualize-menu-heatmaps` | Codex fallback: per-dish HTML heatmaps (prefer `render_menu_heatmaps` when widgets mount) |
| `menuyukti-visualize-weak-items` | Codex fallback: sortable weak-items HTML table (prefer `render_weak_items_review`; **needs COGS**) |

Authoring lives under [`.agents/skills/`](../../.agents/skills/); `pnpm sync:plugin-skills` copies into this package.

## Optional SessionStart hook

`hooks/check_mcp.sh` only probes whether the MCP URL responds. It never starts the server and never reads POS files.

Codex **skips** plugin hooks until you review and trust them (CLI: `/hooks`). Prefer docs + `get_setup_status` if you do not want hooks.

## Maintainers

```bash
pnpm sync:plugin-skills   # refresh skills/ from .agents/skills
pnpm sync:codex-plugin    # package + reinstall into Codex CLI
```

Do not hand-edit copies under `skills/` except for packaging smoke tests. Start a **new** Codex session after reinstall.
