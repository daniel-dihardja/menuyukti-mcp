---
id: TASK-10
title: Add tag-triggered GitHub release binaries for menuyukti-mcp
status: Done
assignee: []
created_date: '2026-11-03T12:33:00Z'
updated_date: '2026-11-03T12:34:00Z'
labels: []
dependencies: []
references:
  - .github/workflows/release.yml
  - README.md
modified_files:
  - .github/workflows/release.yml
  - README.md
priority: high
type: chore
ordinal: 18000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Ship prebuilt menuyukti-mcp binaries so owners can run the local MCP server without installing Rust. A GitHub Actions workflow should build on tag push (v*) and attach platform archives to a GitHub Release.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Pushing a v* tag creates a GitHub Release with Windows, macOS, and Linux binaries for menuyukti-mcp
- [x] #2 README documents where to download binaries and that Rust is only needed for building from source
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add .github/workflows/release.yml: on push tags v[0-9]+.*, create release then matrix-build menuyukti-mcp for linux-x64, linux-arm64, macos-arm64, macos-x64, windows-x64 via taiki-e actions.
2. Include LICENSE + README in archives; sha256 checksums; --locked builds.
3. Document prebuilt binaries in README (download from Releases; Rust only for source builds).
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Added .github/workflows/release.yml (taiki-e create + upload on v* tags). Matrix: linux x64/arm64, macOS arm64/x64, windows x64. Archives include LICENSE+README and sha256. README documents Releases download path.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Tag-triggered Release workflow publishes menuyukti-mcp binaries for Windows, macOS, and Linux. README covers download + run without Rust. First real verification is pushing a v* tag after this lands on main.
<!-- SECTION:FINAL_SUMMARY:END -->
