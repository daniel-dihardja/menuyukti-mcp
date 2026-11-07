---
id: doc-3
title: OpenAI local MCP plugin release
type: specification
created_date: '2026-11-07T03:32:00Z'
updated_date: '2026-11-07T03:33:00Z'
---
# OpenAI local MCP plugin release

Plan to list **Menuyukti** in the OpenAI Plugin Directory as a **Desktop-only** plugin with a **partner-allowlisted local MCP**, so restaurant sales Excel stays on the owner’s machine.

Related: decision-4 (proposed), epic TASK-48.

## Why this path

| Option | Fit for Menuyukti |
| --- | --- |
| Remote HTTPS MCP | OpenAI can scan a public URL, but POS Excel must be uploaded or stored on our servers — conflicts with local-privacy product |
| Partner local MCP | Desktop-only directory listing; MCP runs on the user’s computer and reads local files |
| Repo marketplace only | Works today for early owners; not discoverable in the public directory |

Surfaces after publish: ChatGPT Desktop and Codex Desktop (local threads). Not ChatGPT web/mobile.

## Official OpenAI references

- Desktop-only / local MCP: https://developers.openai.com/plugins/deploy/desktop-only
- Upload and submit: https://developers.openai.com/plugins/deploy/submission
- Plugin guidelines: https://developers.openai.com/plugins/plugin-guidelines
- Plugins hub: https://developers.openai.com/plugins
- Platform (org ID / verification): https://platform.openai.com

## Current repo state (gaps)

| Area | Today (plugins/menuyukti) | Directory local-MCP need |
| --- | --- | --- |
| MCP transport | streamable-http to http://127.0.0.1:8787/mcp | Bundled stdio MCP in the ZIP (command + PLUGIN_ROOT) |
| Server start | Owner runs pnpm mcp / pnpm mcp:prod separately | Server (or launcher) shipped inside the plugin package |
| Hooks | SessionStart check via hooks.json | Remove from public ZIP (hooks not allowed in directory packages) |
| Listing URLs | Missing privacy / support / terms | HTTPS website, support, privacy, terms required for proper listing |
| Review pack | None | 5 positive + 3 negative cases, demo video, release notes |
| Distribution | Repo marketplace | Keep for early users; add directory install after publish |
| OpenAI access | No partner allowlist | Org must be allowlisted for local MCP |

TASK-31 shipped the portable plugin + repo marketplace and left public directory submission out of scope.

## Requirements checklist

### A. OpenAI access (gate)

1. OpenAI Platform organization that will own the plugin.
2. Individual or business identity verification.
3. Copy Organization ID (org-…).
4. Contact an OpenAI representative / Apps support: request local MCP allowlist for that org.
5. Wait for confirmation before uploading a local-MCP ZIP (otherwise expect mcp_configuration_excluded).

### B. Package shape

1. Agent Plugins layout: plugin.json, skills/, mcp.json, assets.
2. mcp.json uses stdio with command + args under PLUGIN_ROOT (see OpenAI desktop-only docs).
3. Bundle Node server + dependencies needed to start Menuyukti (Skybridge production build / self-contained widgets). Exact layout is the packaging spike.
4. No lifecycle hooks in the directory ZIP. Repo-local hooks may remain for manual Codex installs if desired.
5. Version bump and directory-facing listing copy (Desktop only; Node runtime assumptions documented).

### C. Listing and review

1. Verified developer identity.
2. interface fields: displayName, short/long description, category, icons, capabilities.
3. HTTPS: websiteURL, supportURL, privacyPolicyURL, termsOfServiceURL.
4. Review: five positive test cases, three negative, demo recording URL, release notes.
5. Submit via portal Create plugin then Skills only (approved local MCP path), then Publish after approval.

### D. Product / privacy invariants

- Sales Excel remains under a path the local MCP can read (MENUYUKTI_DATA_DIR or equivalent documented default).
- No requirement for owners to upload POS files to Menuyukti cloud or OpenAI storage for the happy path.
- Remote HTTPS MCP / ChatGPT web connectors stay out of scope for this release.

## Outreach email (template)

Subject: Local MCP allowlist request — Desktop-only plugin (Menuyukti)

Body:

I would like my organization allowlisted for local MCP / desktop-only plugin submissions.

Organization ID: org-…
Organization name: …
Publisher: individual / freelancer developer
Plugin: Menuyukti — local restaurant POS analytics for ChatGPT Desktop / Codex

Why local MCP: Reads the owner POS Excel on their machine. We will not host a public HTTPS MCP or upload spreadsheets to our servers. Matches https://developers.openai.com/plugins/deploy/desktop-only

Please confirm individual/freelancer eligibility and allowlist this org for local MCP submissions.

## Recommended work order

1. Allowlist outreach (can start immediately) — TASK-48.1
2. Stdio packaging spike (parallel) — TASK-48.2
3. Directory-ready plugin package — TASK-48.3 (depends on spike)
4. Legal/listing URLs — TASK-48.4 (parallel with packaging)
5. Review materials — TASK-48.5
6. Submit + publish after allowlist — TASK-48.6
7. Owner docs for directory vs repo marketplace — TASK-48.7

## Out of scope for this epic

- Hosting a public remote MCP or Alpic-only product path.
- Changing analytics math or adding new 1.0 capabilities.
- Guaranteeing OpenAI partner approval (external gate; epic tracks the ask and follow-through).

## Definition of success

Menuyukti appears in the OpenAI Plugin Directory with a Desktop only badge; a clean Desktop install runs a local MCP that analyzes demo and owner workspaces without uploading sales Excel to a Menuyukti server.
