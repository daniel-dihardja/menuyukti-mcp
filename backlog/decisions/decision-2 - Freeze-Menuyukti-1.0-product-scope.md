---
id: decision-2
title: Freeze Menuyukti 1.0 product scope
date: '2026-11-06T11:24:00Z'
status: accepted
---
## Context

Menuyukti has shipped local MCP analytics and visualisations under 0.x (`Cargo.toml` / plugin still 0.x). Moving to **1.0** needs a shared contract so release work and future features do not redefine the product ad hoc.

The intended 1.0 value for restaurant owners is already clear: **plan promotions** and **trim / compact the menu**, using four analytics capabilities with tools + visualisations. The written contract lives in [doc-1 Menuyukti 1.0 product definition](../docs/product/doc-1%20-%20Menuyukti-1.0-product-definition.md).

## Decision

**Accept Menuyukti 1.0 scope as follows.**

### In scope (guaranteed for semver `1.0.0`)

**Owner outcomes**

1. Plan promotions
2. Trim / compact the menu

**Capabilities (each with data tool + visualisation path)**

1. Venue slot strength — `get_venue_slot_strength` / `render_venue_slot_strength` (+ Codex skill fallback)
2. Menu heatmaps — `get_menu_heatmaps` / `render_menu_heatmaps` (+ Codex skill fallback)
3. Pair lift index — `get_pair_lift_matrix` / `render_pair_lift_matrix` (Codex: JSON numbers; no dedicated HTML skill required in 1.0)
4. Weak items analysis — `get_weak_items_review` / `render_weak_items_review` (+ Codex skill fallback; needs sales + COGS)

Platform surfaces needed to *reach* those capabilities (local MCP, workspaces/onboarding, Desktop plugin, demo workspace, optional column mapping) are part of the 1.0 product, but they do not expand the outcome contract.

`analyze_menu_engineering` remains available as the matrix behind weak items / optional star focus; it is not a fifth headline 1.0 capability.

### Out of scope / deferred (post-1.0)

- Cloud / multi-tenant SaaS or remote sales upload
- POS writeback or automated keep/remove menu actions
- Multi-venue rollup dashboards
- Dedicated Codex HTML skill for pair lift
- Pricing optimizer beyond weak-items / menu-engineering review
- Non-Desktop hosting of localhost MCP

New ideas that are not required for the two owner outcomes go to **1.1+** unless they are must-have gaps found in the 1.0 audit.

## Consequences

- Tagging / publishing `v1.0.0` asserts this contract (doc-1 + this decision).
- Gap-audit (TASK-43.3) may only add must-have fix tasks under milestone Menuyukti 1.0; polish goes to deferred list or later milestones.
- After 1.0: breaking changes require a major bump; compatible additions are minor; fixes are patch.
- Owner docs should frame the product around the two outcomes, not only a tool laundry list.
