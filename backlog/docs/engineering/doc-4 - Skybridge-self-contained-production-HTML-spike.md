---
id: doc-4
title: Skybridge self-contained production HTML spike
type: other
created_date: '2026-11-07T04:20:00Z'
updated_date: '2026-11-07T04:20:00Z'
---
# Skybridge self-contained production HTML spike (TASK-49.13)

Spike date: 2026-11-07. Skybridge pin in app: `skybridge` / `@skybridge/*` `^2.1.0`.

## Current Skybridge production HTML contract

Production MCP Apps / Apps SDK resources call private `templateHelper.renderProduction({ serverUrl, viewFile, styleFile })`. The generated shell:

- Injects `window.skybridge.serverUrl`
- Loads the Vite-built view JS and shared `style.css` as **separate assets** under `{serverUrl}/assets/...`

Docs ([Assets and Styling](https://docs.skybridge.tech/guides/assets-and-styling), [Configure CSP](https://docs.skybridge.tech/guides/csp), [Deploy](https://docs.skybridge.tech/ship/deploy)) assume a reachable HTTP(S) server origin that also serves `/assets`. CSP helpers auto-allow that origin for `resourceDomains` / `connectDomains`. There is **no** documented option to emit a single HTML blob with inlined CSS/JS.

Upstream direction (issue [#571](https://github.com/alpic-ai/skybridge/issues/571), Workers deploy) still assumes the **platform serves static assets** separately from the MCP HTML resource — the opposite of inlining.

## Codex Desktop CSP constraint

Codex Desktop mounts MCP Apps from **stdio** local MCP. The iframe CSP does **not** allow fetching `http://127.0.0.1` / `localhost` Vite or `/assets` / `/_skybridge` URLs. The default Skybridge production shell therefore times out / renders blank even when the MCP process is healthy.

Menuyukti restores the Rust-era contract:

1. Post-build `scripts/inline-widgets.mjs` → `dist/self-contained/<view>.html` (CSS + JS inlined, no localhost asset URLs).
2. Runtime `enableSelfContainedProductionViews()` monkey-patches `templateHelper.renderProduction` to serve those files (hard-fail if missing — TASK-49.1).
3. CSP regression smokes — TASK-49.3 / `self-contained-csp.mjs`.

## Options

| Option | Pros | Cons |
| --- | --- | --- |
| **A. Keep Menuyukti patch + inliner** (current) | Works on Codex today; covered by hard-fail + CSP smokes; no upstream wait | Private API patch can break on Skybridge minors; duplicate React per HTML (mitigated by TASK-49.7 minify) |
| **B. Upstream feature request** (official `inline` / self-contained mode or public render hook) | Long-term cleaner; helps other local/stdio MCP Apps | Not available in 2.1.x; unknown timeline; Workers/#571 path still asset-oriented |
| **C. Drop patch, require HTTP asset host** | Stock Skybridge | Breaks Codex Desktop primary host; conflicts with local-MCP privacy path |

## Recommendation

**Keep the patch (option A). NO-GO on waiting for upstream before shipping.** Optionally file a non-blocking upstream feature request (option B) so Alpic knows local/stdio hosts need inlined HTML; do not block Menuyukti releases on it.

## Version pin / smoke expectations (siblings)

- Depend on `skybridge@^2.1.0`; bump only after `pnpm --filter menuyukti build` + `assertSelfContainedDist` / vitest CSP smokes still pass.
- Production path must hard-fail when `dist/self-contained/<view>.html` is missing (TASK-49.1) — never fall back to the asset shell.
- Automated tests assert no `http://127.0.0.1` / `localhost` Vite/`assets`/`_skybridge` URLs in self-contained HTML (TASK-49.3); fixture path runs under `pnpm check`, full dist validation after build.
- Watch for `templateHelper` path/export or `renderProduction` signature changes when upgrading Skybridge — patch resolution is via `skybridge/server` → sibling `templateHelper.js`.

## GO / NO-GO

- **GO:** keep Menuyukti self-contained inliner + `renderProduction` patch for Codex Desktop.
- **NO-GO:** replace with stock Skybridge asset shell for local Desktop MCP until upstream ships an official inline mode.
