# Codex / ChatGPT Desktop plugin

Install Menuyukti as one **Desktop plugin** instead of adding the MCP URL and each skill by hand.

## What you get

- **MCP connection** to `http://127.0.0.1:8787/mcp` (localhost — **Desktop only**)
- **Skills** bundled under `plugins/menuyukti/skills/`:
  - `menuyukti-mcp` — coaching / setup ritual
  - `menuyukti-visualize-venue-slot-strength`
  - `menuyukti-visualize-menu-heatmaps`
  - `menuyukti-visualize-weak-items`
  - `menuyukti-venue-slot-strength` — offline / custom-column compute

## Prerequisites

The plugin does **not** replace local setup:

1. [Install and run](./install-and-run.md) the Menuyukti MCP binary with `MENUYUKTI_DATA_DIR` set.
2. Keep the server listening at `http://127.0.0.1:8787/mcp`.
3. Put sales (and optional COGS) in a [workspace](./workspaces.md) (try `demo-cafe-july-2026`).

Then ask Codex: *What’s my Menuyukti setup status?*

## Install from this repo (marketplace)

This repository declares a local marketplace at [`.agents/plugins/marketplace.json`](../../.agents/plugins/marketplace.json) pointing at [`plugins/menuyukti/`](../../plugins/menuyukti/).

### Codex CLI

From this repository root:

```bash
codex plugin marketplace add .
codex plugin add menuyukti@menuyukti-repo
codex plugin list -m menuyukti-repo
```

You can also browse with `codex` then `/plugins`. Start a **new** Codex session after install so bundled skills load.

### ChatGPT Desktop

1. Open the **Plugins** tab.
2. Use your **Personal** / repo marketplace entry for this project when available, or install the local `plugins/menuyukti` package per current Desktop plugin docs.
3. Connect the Menuyukti MCP app when prompted (localhost).
4. Start a **new** chat / Codex session.

Because `mcp.json` targets localhost, the plugin is **Desktop only** — not for ChatGPT web/mobile.

## Optional SessionStart hook

`plugins/menuyukti/hooks/check_mcp.sh` only checks whether the MCP URL responds. It never starts the binary and never reads POS files.

Codex **skips** plugin hooks until you review and trust them (CLI: `/hooks`). Prefer docs + `get_setup_status` if you do not want hooks.

## Maintainers

- Author skills in [`.agents/skills/`](../../.agents/skills/).
- Refresh the installable copies:

```bash
make sync-plugin-skills
```

- Package layout: [`plugins/menuyukti/README.md`](../../plugins/menuyukti/README.md).
