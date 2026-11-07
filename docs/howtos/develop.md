# Develop

Contributor workflow for Menuyukti MCP (Skybridge / TypeScript).

## Prerequisites

- **Node.js 24+**
- **pnpm 10+** (see `packageManager` in root `package.json`)

### Editor TypeScript

Skybridge’s base `tsconfig` uses `${configDir}` (TypeScript **5.5+**). This repo pins the workspace SDK at `apps/menuyukti/node_modules/typescript` via `.vscode/settings.json`. In Cursor/VS Code, choose **TypeScript: Select TypeScript Version → Use Workspace Version** if the editor still shows spurious red squiggles while `pnpm typecheck` is clean.

## Commands

From the repo root (`package.json` is the source of truth):

```bash
pnpm scripts              # list package scripts
pnpm mcp                  # Skybridge Vite dev + DevTools
pnpm mcp:prod             # Production + self-contained widgets (Codex Desktop)
pnpm mcp:skybridge        # Alias for pnpm mcp
pnpm mcp:inspect          # Skybridge + MCP Inspector
pnpm check                # tsc --noEmit + vitest (required quality gate)
pnpm typecheck
pnpm test
pnpm sync:plugin-skills   # copy .agents/skills → plugins/menuyukti/skills
pnpm sync:codex-plugin    # sync skills + reinstall Menuyukti into Codex CLI
pnpm backlog              # Backlog.md board (optional)
```

`pnpm mcp:prod` runs `skybridge build`, inlines each view into `dist/self-contained/*.html`, then `skybridge start`. Use that for Codex; use `pnpm mcp` for DevTools/HMR.

After changing MCP Apps views or the widget inliner, also verify self-contained CSP artifacts:

```bash
pnpm --filter menuyukti build   # writes dist/self-contained/*.html
pnpm check                      # vitest asserts CSP-clean HTML when dist exists;
                                # always covers the inliner fixture + view-name list
```

`pnpm check` alone does **not** run `skybridge build` (dist is gitignored). The inliner fixture smoke still runs in CI; validating the real production blobs needs the build step above (or `pnpm mcp:prod`).

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

Analytics charts mount as Skybridge React views under [`apps/menuyukti/src/views/`](../../apps/menuyukti/src/views/).

- **Fixture preview (no Excel):** from `apps/menuyukti`, run `pnpm widgets:preview` → `http://127.0.0.1:8790/` (query `view` + `kind=populated|empty|error`). Fixtures: [`src/views/fixtures`](../../apps/menuyukti/src/views/fixtures/).
- **Live DevTools:** `pnpm mcp` → `http://127.0.0.1:8787/` and call the matching `render_*` tool.

Codex Desktop requires self-contained HTML (no `http://127.0.0.1` asset fetches). Production serving hard-fails if `dist/self-contained/<view>.html` is missing after build. See [`scripts/self-contained-csp.mjs`](../../apps/menuyukti/scripts/self-contained-csp.mjs) and [`scripts/inline-widgets.mjs`](../../apps/menuyukti/scripts/inline-widgets.mjs).

The inliner minifies each view into its own HTML blob. React is still duplicated per view: a shared `/assets` chunk would reintroduce localhost fetches that Codex CSP blocks. Further size cuts would need a multi-view shell or a non-React production path.

## Local data

By default, `pnpm mcp` sets `MENUYUKTI_DATA_DIR` to [`workspaces/`](../../workspaces/). See [Workspaces](./workspaces.md).

## Related guides

- [Install and run](./install-and-run.md)
- [Custom POS columns](./custom-pos-columns.md)
- Documentation hub: [docs/](../README.md)
