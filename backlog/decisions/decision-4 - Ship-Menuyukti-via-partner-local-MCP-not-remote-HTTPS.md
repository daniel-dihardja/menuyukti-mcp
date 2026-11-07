---
id: decision-4
title: Ship Menuyukti via partner local MCP not remote HTTPS
date: '2026-11-07T03:32:00Z'
status: proposed
---
## Context

Menuyukti’s value proposition is local POS analytics: sales Excel stays on the owner’s machine under `MENUYUKTI_DATA_DIR`, and ChatGPT Desktop / Codex talk to a localhost MCP. TASK-31 shipped a portable plugin + repo marketplace and left **public OpenAI directory** submission out of scope.

OpenAI’s public Plugin Directory supports:

1. **Remote HTTPS MCP** (“With MCP”) — OpenAI connects to a public URL; local files are not readable unless uploaded or stored on the publisher’s backend.
2. **Partner-allowlisted local MCP** (Desktop only) — stdio MCP bundled in the plugin ZIP; org must be allowlisted; listing shows Desktop only.

A remote MCP would force POS upload/hosting and conflict with 1.0 privacy scope (decision-2 / doc-1). Freelancer eligibility for the partner allowlist is not guaranteed and must be confirmed with OpenAI.

Full release checklist and outreach template: [doc-3 OpenAI local MCP plugin release](../docs/product/doc-3%20-%20OpenAI-local-MCP-plugin-release.md). Epic: TASK-48.

## Decision

**Proposed:** Pursue OpenAI Plugin Directory distribution via **partner local MCP** (Desktop-only), not via a public remote HTTPS MCP that hosts or receives sales Excel.

Until allowlisted and published:

- Keep the **repo marketplace** path for early owners.
- Do not build a cloud upload / remote POS workspace product for directory listing.

Accept this decision as **accepted** only after OpenAI confirms local-MCP eligibility for the submitting org (or an explicit pivot is chosen).

## Consequences

- Packaging must move from localhost HTTP `mcp.json` to a **bundled stdio** server inside the plugin ZIP; directory ZIPs cannot include lifecycle hooks.
- Listing needs privacy/support/terms URLs and a review pack (doc-3).
- ChatGPT **web/mobile** remain unsupported for this release path.
- If OpenAI denies freelancer/local-MCP allowlisting, revisit options: remain repo-marketplace-only, or a separate remote product with different privacy terms (new decision).
