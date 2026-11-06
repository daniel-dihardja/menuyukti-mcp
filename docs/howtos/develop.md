# Develop

Contributor workflow for building Menuyukti MCP from source.

## Commands

```bash
make check     # fmt --check + clippy + tests
make test
# or: cargo test -p menuyukti && cargo test -p menuyukti-mcp
make mcp
make mcp-inspect
make widgets-preview  # mock widget HTML (refresh after edits; no Node)
make sync-plugin-skills  # copy .agents/skills → plugins/menuyukti/skills
make sync-codex-plugin   # sync skills + reinstall Menuyukti into Codex CLI
make backlog   # Backlog.md board (optional)
```

After editing Codex skills under `.agents/skills/`, run `make sync-plugin-skills` before committing so the installable plugin stays in sync. To also refresh your local Codex install, use `make sync-codex-plugin` (then start a new session). See [Codex plugin](./codex-plugin.md).

## CI

GitHub Actions runs the same `make check` gate on every **pull request** and on **push to `main`** (see [`.github/workflows/ci.yml`](../../.github/workflows/ci.yml)). Version-tag releases stay in [`release.yml`](../../.github/workflows/release.yml). The toolchain is pinned via [`rust-toolchain.toml`](../../rust-toolchain.toml) (stable + rustfmt + clippy).

## Crate layout

- `crates/menuyukti` — Excel normalize + analytics library
- `crates/menuyukti-mcp` — MCP HTTP binary (`menuyukti-mcp`) plus optional `menuyukti-widget-preview` for local widget styling

## MCP Apps widget theme

Heatmap widgets (venue slot strength, pair lift) share chrome tokens under
[`crates/menuyukti-mcp/assets/shared/widget-theme.css`](../../crates/menuyukti-mcp/assets/shared/widget-theme.css).
Each widget HTML keeps a `/* __MENUYUKTI_SHARED_THEME__ */` marker; the MCP server
and `make widgets-preview` **inline** that CSS into a self-contained document before
serving (ChatGPT mounts one HTML resource, so runtime `<link>` is not used).

Edit the shared file for host colors, panel padding, legend/table chrome, and default
`--heatmap-*` scale tokens. Heatmap widgets share that blue sequential scale unless a
widget overrides `--heatmap-*` in its own `<style>`.

## Local data

By default, `make mcp` sets `MENUYUKTI_DATA_DIR` to [`workspaces/`](../../workspaces/). See [Workspaces](./workspaces.md).

## Related guides

- [Install and run](./install-and-run.md)
- [Custom POS columns](./custom-pos-columns.md)
- Documentation hub: [docs/](../README.md)
