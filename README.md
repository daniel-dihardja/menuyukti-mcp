# Menuyukti MCP

Local MCP server for restaurant POS sales analytics. Connect it to your personal ChatGPT account so analytics run on your machine — sales data never leaves your computer or your own GPT account.

Licensed under the [MIT License](LICENSE).

## Requirements

- [Rust](https://rustup.rs/) (stable toolchain with Cargo)

Python is **not** required for the normal Excel → MCP workflow.

## Run the local server

1. Put POS Excel exports in [`sales-reports/`](sales-reports/) (a demo file is included).
2. Start the Rust MCP server:

```bash
make mcp
# or: bash scripts/run-mcp.sh
# or: MENUYUKTI_DATA_DIR="$PWD/sales-reports" cargo run -p menuyukti-mcp
```

The server listens at `http://127.0.0.1:8787/mcp` (streamable HTTP).

Optional Inspector (needs Node/`npx` for the Inspector UI only):

```bash
make mcp-inspect
```

Point ChatGPT / MCP clients at that URL. Tools:

- `list_sales_reports`
- `summarize_local_sales`
- `list_normalized_sales`
- `get_menu_heatmaps`

## Sales reports folder

Put your POS Excel exports in [`sales-reports/`](sales-reports/).

That folder is the default data directory for the local MCP server. A sample file is included:

- [`sales-reports/demo-cafe-sales-july-2026.xlsx`](sales-reports/demo-cafe-sales-july-2026.xlsx) — anonymized cafe month with real data

Example tool argument:

```json
{
  "path": "demo-cafe-sales-july-2026.xlsx"
}
```

## Map my columns (custom POS export)

Built-in support covers known POS exports (ESB, Moka, Quino). If your export uses different column names, add a mapping file so Menuyukti can normalize it locally.

1. Copy [`pos-column-mapping.example.json`](pos-column-mapping.example.json) into [`sales-reports/`](sales-reports/).
2. Rename it to `pos-column-mapping.json`.
3. Replace every `<YOUR_…_COLUMN>` placeholder with the exact header text from your spreadsheet.

```json
{
  "name": "my-pos",
  "skip_rows": 0,
  "columns": {
    "bill_number": "<YOUR_ORDER_ID_COLUMN>",
    "menu": "<YOUR_ITEM_NAME_COLUMN>",
    "qty": "<YOUR_QTY_COLUMN>",
    "price": "<YOUR_UNIT_PRICE_COLUMN>",
    "total_after_bill_discount": "<YOUR_NET_SALES_COLUMN>",
    "order_time": "<YOUR_SOLD_AT_COLUMN>",
    "menu_category": "<YOUR_CATEGORY_COLUMN>",
    "menu_category_detail": "<YOUR_SUBCATEGORY_COLUMN>"
  }
}
```

Example: if your sheet calls the order id `Bill No.`, set `"bill_number": "Bill No."`.

Notes:

- All eight fields above are required — leave no `<YOUR_…_COLUMN>` placeholders.
- `skip_rows` is how many rows to skip before the header row (use `0` when the first row is headers).
- `name` is an optional label shown in tool results (defaults to `custom`).
- Optional override: set `MENUYUKTI_COLUMN_MAPPING` to a full path if the file lives elsewhere.
- Known POS formats still use their built-in normalizers; this mapping is used when the export is unrecognized.

## Develop

```bash
make test
# or: cargo test -p menuyukti && cargo test -p menuyukti-mcp
make mcp
make backlog   # Backlog.md board (optional)
```

Workspace layout:

- `crates/menuyukti` — Excel normalize + analytics library
- `crates/menuyukti-mcp` — MCP HTTP binary

