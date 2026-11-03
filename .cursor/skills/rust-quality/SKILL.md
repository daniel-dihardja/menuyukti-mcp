---
name: rust-quality
description: Maintain Menuyukti Rust workspace quality gates, crate boundaries, tests, and local sales-data safety. Use when changing Rust/Cargo code, adding checks/CI, fixing clippy/fmt/test failures, reviewing Rust PRs, or when the user mentions make check, clippy, rustfmt, cargo deny, or Rust best practices.
---

# Menuyukti rust-quality

## When this skill applies

Read and follow this skill before substantive work on:

- `crates/**`, root `Cargo.toml` / `Cargo.lock`
- `scripts/{check,fmt,clippy}.sh`, `Makefile` Rust targets
- CI that runs Rust checks or releases

## Quality gate (required)

After Rust changes, run and pass:

```bash
make check
```

That is `scripts/check.sh`: fmt `--check` → clippy `-D warnings --locked` → `cargo test --workspace --locked`.

If only formatting is needed: `make fmt`. Lint only: `make clippy`.

Do not weaken the gate (`--fix`, broad allows, skipping `--locked`) unless the user explicitly asks.

## Crate boundaries

| Crate | Own | Avoid |
| --- | --- | --- |
| `menuyukti` | normalize, paths, load, analytics, column mapping | HTTP/MCP transport details |
| `gastro-mcp` | MCP tools, HTTP server, process main | Reimplementing Excel/analytics logic |

New shared behavior → library first, thin MCP wrapper second.

## Change workflow

1. Match existing module layout and naming in the target crate.
2. Prefer workspace dependencies (`foo.workspace = true`).
3. Add or update tests:
   - Path/security: `paths` unit tests or load-gate tests
   - POS/normalize: fixtures under `crates/menuyukti/tests` + `workspaces/` demos when relevant
   - MCP: HTTP integration in `crates/gastro-mcp/tests` when tool surface changes
4. Run `make check`; fix failures before finishing.
5. Keep README/`make help` in sync if you add developer commands.

## Safety checklist (local POS data)

- [ ] User paths go through `resolve_allowed_path` / data-dir APIs
- [ ] No new way to read files outside `MENUYUKTI_DATA_DIR`
- [ ] No logging of raw sales dumps or secrets
- [ ] No network exfiltration of spreadsheet contents

## Industry practices (adopt when touching tooling)

Prefer these when the task includes “best practices”, CI, or supply chain:

- Pin toolchain via `rust-toolchain.toml` (clippy + rustfmt components)
- CI job on PRs that mirrors `make check` (separate from release uploads)
- Optional later: `cargo deny` / `deny.toml` for advisories and licenses
- Optional later: workspace `[lints]` so `cargo build` denies warnings locally too

For detailed gate commands and review prompts, see [reference.md](reference.md).
