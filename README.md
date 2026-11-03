# Menuyukti GPT Plugin

Local MCP server that runs Menuyukti sales analytics on POS Excel files that stay on the restaurant owner's machine.

Licensed under the [MIT License](LICENSE).

## Setup

```bash
uv sync --group dev
```

Analytics live in this repo under [`packages/menuyukti`](packages/menuyukti). By default the inspect script sets `MENUYUKTI_DATA_DIR` to the repo root so both local exports under `sales-reports/` (gitignored) and the committed fixtures are allowed. Narrow it if you want a smaller sandbox:

```bash
export MENUYUKTI_DATA_DIR="$(pwd)/sales-reports"
```

## Debug with MCP Inspector (recommended)

Starts the MCP server and opens Inspector already pointed at it:

```bash
npm run mcp:inspect
```

Inspector opens against [http://127.0.0.1:8787/mcp](http://127.0.0.1:8787/mcp) (Streamable HTTP). Server logs stay in the same terminal so you can debug tool calls and errors.

In Inspector:

1. Confirm the connection to `http://127.0.0.1:8787/mcp`
2. Open **Tools** → `summarize_local_sales`
3. Call with a path under `MENUYUKTI_DATA_DIR`, for example:

```json
{
  "path": "sales-reports/SalesRecapitulationDetailReport_JUL_2026.xlsx"
}
```

Demo fixture (same `MENUYUKTI_DATA_DIR`):

```json
{
  "path": "packages/menuyukti/fixtures/pos-sales-reports/esb/SalesRecapitulationDetailReport_DEMO.xlsx"
}
```

Stopping Inspector (Ctrl+C) also stops the MCP server.

## Run the MCP server alone

```bash
export MENUYUKTI_DATA_DIR="$(pwd)"
npm run mcp
```

The server listens at [http://localhost:8787/mcp](http://localhost:8787/mcp).

Then in another terminal you can open Inspector manually:

```bash
npx @modelcontextprotocol/inspector@latest --web \
  --transport http \
  --server-url http://127.0.0.1:8787/mcp
```

## Backlog board

```bash
npm run dev
```

Opens the Backlog.md UI at [http://127.0.0.1:6421](http://127.0.0.1:6421).

## Tests

```bash
uv run pytest
```
