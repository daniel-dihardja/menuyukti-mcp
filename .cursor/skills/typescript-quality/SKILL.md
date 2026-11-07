---
name: typescript-quality
description: Maintain Menuyukti Skybridge TypeScript quality gates, app boundaries, tests, and local sales-data safety. Use when changing apps/menuyukti, pnpm check, vitest, tsc, CI, or TypeScript MCP/server code.
---

# Menuyukti typescript-quality

## When this skill applies

Read and follow this skill before substantive work on:

- `apps/menuyukti/**`
- `scripts/check.sh`, root `package.json` quality scripts
- CI that runs TypeScript checks

## Quality gate (required)

After TypeScript / Skybridge changes, run and pass:

```bash
pnpm check
```

That is `scripts/check.sh`:

1. `pnpm lint` (Biome lint + format check)
2. `pnpm check:skills` (`.agents/skills` ↔ `plugins/menuyukti/skills`)
3. `tsc --noEmit` (`pnpm typecheck`)
4. `vitest run` (`pnpm test`)

Helpers: `pnpm lint:fix`, `pnpm typecheck`, `pnpm test`, `pnpm sync:plugin-skills`.

Do not weaken the gate (skipping lint/typecheck, broad `any`, empty tests) unless the user explicitly asks.

## App boundaries

| Area | Own | Avoid |
| --- | --- | --- |
| `src/lib` | paths, workspace, Excel normalize, analytics | MCP transport / Skybridge wiring |
| `src/server.ts` | MCP tool registration, thin handlers | Reimplementing analytics math |
| `src/views` | MCP Apps React views | Fetching sales files directly |

New shared behavior → `src/lib` first, thin tool/view wrapper second.

## Change workflow

1. Match existing module layout and naming under `apps/menuyukti/src`.
2. Add or update vitest coverage next to behavior (`*.test.ts`), using `workspaces/demo-cafe-july-2026` when relevant.
3. Run `pnpm check`; fix failures before finishing.
4. Keep README / `pnpm scripts` in sync if you add developer commands.

## Safety checklist (local POS data)

- [ ] User paths go through `resolveAllowedPath` / data-dir APIs
- [ ] No new way to read files outside `MENUYUKTI_DATA_DIR`
- [ ] No logging of raw sales dumps or secrets
- [ ] No network exfiltration of spreadsheet contents
- [ ] Skybridge telemetry remains opted out in `pnpm mcp` scripts
