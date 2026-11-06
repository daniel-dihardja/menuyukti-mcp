# Codex / ChatGPT Desktop plugin

Install Menuyukti as one **Desktop plugin** instead of adding the MCP URL and each skill by hand.

## Owner guide (start here)

**All install and usage instructions** (prerequisites, marketplace install, ritual, what to ask, charts, skills, hooks) live in:

→ **[plugins/menuyukti/README.md](../../plugins/menuyukti/README.md)**

This howto only adds maintainer pointers and a manual (non-plugin) alternative.

## Prerequisites (summary)

The plugin does **not** replace local setup:

1. [Install and run](./install-and-run.md) the Menuyukti MCP binary with `MENUYUKTI_DATA_DIR` set.
2. Keep the server listening at `http://127.0.0.1:8787/mcp`.
3. Put sales (and optional COGS) in a [workspace](./workspaces.md) (try `demo-cafe-july-2026`).

Because `mcp.json` targets localhost, the plugin is **Desktop only** — not for ChatGPT web/mobile.

## Manual alternative (no plugin)

In ChatGPT Desktop settings, add Menuyukti MCP as an MCP server at `http://127.0.0.1:8787/mcp`, then add skills from [`.agents/skills/`](../../.agents/skills/) one by one. Prefer the [plugin package](../../plugins/menuyukti/README.md) when you can.

## Maintainers

- Author skills in [`.agents/skills/`](../../.agents/skills/).
- Refresh the installable copies, then reinstall into Codex CLI:

```bash
make sync-codex-plugin
```

That runs `make sync-plugin-skills`, ensures the repo marketplace is registered, and re-runs `codex plugin add menuyukti@menuyukti-repo` so `~/.codex/plugins/cache` picks up local changes. Start a **new** Codex session afterward.

Skills-only (no Codex reinstall):

```bash
make sync-plugin-skills
```

Full owner-facing package docs: [`plugins/menuyukti/README.md`](../../plugins/menuyukti/README.md).
