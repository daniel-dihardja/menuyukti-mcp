# Develop

Contributor workflow for building Gastro MCP from source.

## Commands

```bash
make check     # fmt --check + clippy + tests
make test
# or: cargo test -p menuyukti && cargo test -p menuyukti-mcp
make mcp
make mcp-inspect
make backlog   # Backlog.md board (optional)
```

## Crate layout

- `crates/menuyukti` — Excel normalize + analytics library
- `crates/menuyukti-mcp` — MCP HTTP binary

## Local data

By default, `make mcp` sets `MENUYUKTI_DATA_DIR` to [`workspaces/`](../../workspaces/). See [Workspaces](./workspaces.md).

## Related guides

- [Install and run](./install-and-run.md)
- [Custom POS columns](./custom-pos-columns.md)
- Documentation hub: [docs/](../README.md)
