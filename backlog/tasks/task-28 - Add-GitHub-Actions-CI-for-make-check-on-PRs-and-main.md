---
id: TASK-28
title: Add GitHub Actions CI for make check on PRs and main
status: Done
assignee:
  - '@Daniel'
created_date: '2026-11-05T09:17:00Z'
updated_date: '2026-11-05T09:17:00Z'
labels:
  - ci
dependencies: []
references:
  - .github/workflows/release.yml
  - scripts/check.sh
documentation:
  - docs/howtos/develop.md
priority: medium
type: chore
ordinal: 63000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
This repo only has a tag-triggered release workflow. Feature branches and PRs never run fmt/clippy/tests in CI, so regressions can land unnoticed. Mirror the menuyukti v3 CI pattern: quality gate on pull requests and on push to main, running the same make check / scripts/check.sh gate used locally.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Workflow runs on pull_request and on push to main (not on version tags; release.yml stays separate)
- [x] #2 CI installs the stable Rust toolchain with rustfmt and clippy, then runs make check (fmt --check, clippy -D warnings --locked, test --workspace --locked)
- [x] #3 Concurrency cancels in-progress runs for the same ref
- [x] #4 docs/howtos/develop.md mentions that PRs and main pushes run make check in GitHub Actions
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add .github/workflows/ci.yml: pull_request + push to main, concurrency, rust toolchain with rustfmt/clippy, make check.
2. Add rust-toolchain.toml (stable + components) for local/CI alignment.
3. Document CI in docs/howtos/develop.md.
4. Finalize TASK-28.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Added .github/workflows/ci.yml (PR + push main, concurrency, dtolnay/rust-toolchain + rust-cache, make check), rust-toolchain.toml, develop.md CI section.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Mirrored menuyukti v3 CI: pull requests and pushes to main run make check (fmt, clippy, tests). Release tags stay on release.yml. Documented in develop.md; rust-toolchain.toml pins stable + rustfmt/clippy.
<!-- SECTION:FINAL_SUMMARY:END -->
