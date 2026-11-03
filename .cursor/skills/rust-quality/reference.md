# rust-quality reference

## Gate commands

```bash
make check
bash scripts/check.sh

# pieces
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked

make fmt          # write formatting
CHECK=1 make fmt  # fmt --check via scripts/fmt.sh
make clippy
make test
```

## Suggested PR check workflow (when adding CI)

Minimal PR job should mirror `scripts/check.sh` on the pinned toolchain:

1. checkout
2. install/use `rust-toolchain.toml` (once present)
3. `bash scripts/check.sh`

Keep release binary upload in `release.yml` separate from the PR quality job.

## Review prompt snippets

Use when running Bugbot / security / human review on Rust changes:

**Correctness**

- Fixture goldens and MCP tool schemas still match behavior
- Error messages name `MENUYUKTI_DATA_DIR` when configuration is missing

**Safety**

- Path traversal and absolute-path escapes cannot bypass the data dir
- Tool handlers do not concatenate paths manually

**Supply chain / release**

- Lockfile updated intentionally with dependency changes
- Release tags still build `menuyukti-mcp` only from reviewed main
## Clippy allow policy

Allowed only when:

1. The lint is false-positive or intentional for a tested reason
2. Scope is the smallest item (fn/statement), not the whole module
3. Comment or `reason = "..."` explains why

Example:

```rust
#[allow(clippy::zombie_processes, reason = "child waited in Drop and failure path")]
```
