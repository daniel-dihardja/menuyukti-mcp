# Develop

Contributor workflow for Menuyukti MCP (Skybridge / TypeScript).

## Prerequisites

- **Node.js 24+**
- **pnpm 10+** (see `packageManager` in root `package.json`)

## Commands

From the repo root (`package.json` is the source of truth):

```bash
pnpm scripts              # list package scripts
pnpm mcp                  # Skybridge MCP (http://127.0.0.1:8787/mcp)
pnpm mcp:skybridge        # Alias for pnpm mcp
pnpm mcp:inspect          # Skybridge + MCP Inspector
pnpm check                # tsc --noEmit + vitest (required quality gate)
pnpm typecheck
pnpm test
pnpm sync:plugin-skills   # copy .agents/skills → plugins/menuyukti/skills
pnpm sync:codex-plugin    # sync skills + reinstall Menuyukti into Codex CLI
pnpm backlog              # Backlog.md board (optional)
```

First time (or after lockfile changes):

```bash
pnpm install
```

After editing Codex skills under `.agents/skills/`, run `pnpm sync:plugin-skills` before committing so the installable plugin stays in sync. To also refresh your local Codex install, use `pnpm sync:codex-plugin` (then start a new session). See [Codex plugin](./codex-plugin.md).

From the app package (workspace `menuyukti`):

```bash
cd apps/menuyukti
pnpm check    # typecheck + test
pnpm dev      # same as pnpm mcp from root
```

## CI

GitHub Actions runs `pnpm check` on every **pull request** and on **push to `main`** (see [`.github/workflows/ci.yml`](../../.github/workflows/ci.yml)): Node 24, `pnpm install --frozen-lockfile` at the repo root, then typecheck + vitest.

Rust binary tag releases are retired (TASK-44.10).

## Layout

- `apps/menuyukti` — **Skybridge MCP server** (source of truth)
- `plugins/menuyukti` — Codex / ChatGPT Desktop plugin package
- `.agents/skills` — skill source of truth (synced into the plugin)
- `workspaces/` — default `MENUYUKTI_DATA_DIR` (includes `demo-cafe-july-2026`)

## MCP Apps views

Analytics charts mount as Skybridge React views under [`apps/menuyukti/src/views/`](../../apps/menuyukti/src/views/). Preview them with `pnpm mcp` → DevTools at `http://127.0.0.1:8787/` by calling the matching `render_*` tool.

## Local data

By default, `pnpm mcp` sets `MENUYUKTI_DATA_DIR` to [`workspaces/`](../../workspaces/). See [Workspaces](./workspaces.md).

## Related guides

- [Install and run](./install-and-run.md)
- [Custom POS columns](./custom-pos-columns.md)
- Documentation hub: [docs/](../README.md)
