# Menuyukti MCP

Local MCP server for restaurant POS sales analytics. Connect it to your personal ChatGPT account so analytics run on your machine — sales data never leaves your computer or your own GPT account.

Licensed under the [MIT License](LICENSE).

## Choose how to run

| | **Prebuilt binary** | **From source** |
| --- | --- | --- |
| Best for | Owners who just want to run the server | Contributors / local development |
| Install | Download a release archive | Install [Rust](https://rustup.rs/) (Cargo) |
| Start | Run `menuyukti-mcp` / `menuyukti-mcp.exe` | `make mcp` or `cargo run -p menuyukti-mcp` |
| Needs Make / Git clone | No | Yes (clone this repo) |

Both modes listen at **`http://127.0.0.1:8787/mcp`** (streamable HTTP). Point ChatGPT or another MCP client at that URL. Tools:

- `list_sales_reports`
- `summarize_local_sales`
- `list_normalized_sales`
- `get_menu_heatmaps`

Put POS Excel files in a data folder and set `MENUYUKTI_DATA_DIR` to that folder (see [Sales reports folder](#sales-reports-folder)).

---

### Option A — Prebuilt binary

Tagged releases (`v*`) publish archives for Windows (x64), macOS (Apple Silicon + Intel), and Linux (x64 + ARM64). No Rust install.

1. Download the archive for your OS from [GitHub Releases](https://github.com/daniel-dihardja/menuyukti-mcp/releases).
2. Extract it. Inside you’ll find `menuyukti-mcp` (or `menuyukti-mcp.exe` on Windows), plus `LICENSE` and `README.md`.
3. Create a folder for Excel exports (any path you like).
4. Start the server:

```bash
# macOS / Linux
MENUYUKTI_DATA_DIR="/path/to/sales-reports" ./menuyukti-mcp

# Windows (PowerShell)
$env:MENUYUKTI_DATA_DIR="C:\path\to\sales-reports"
.\menuyukti-mcp.exe
```

Keep that terminal open while ChatGPT / your MCP client is connected.

---

### Option B — From source

Use this when you are developing or prefer to build locally.

1. Install [Rust](https://rustup.rs/) (stable toolchain with Cargo).
2. Clone this repository and open a terminal in the repo root.
3. Put POS Excel exports in [`sales-reports/`](sales-reports/) (a demo file is included), or point `MENUYUKTI_DATA_DIR` elsewhere.
4. Start the server:

```bash
make mcp
# or: bash scripts/run-mcp.sh
# or: MENUYUKTI_DATA_DIR="$PWD/sales-reports" cargo run -p menuyukti-mcp
```

Optional: open [MCP Inspector](https://modelcontextprotocol.io/) against the running server (needs Node / `npx`):

```bash
make mcp-inspect
```

For contributor workflow (`make test`, crate layout), see [Develop](#develop).

---

## Sales reports folder

In the source checkout, put POS Excel exports in [`sales-reports/`](sales-reports/). That folder is the default `MENUYUKTI_DATA_DIR` when you use `make mcp`.

With a prebuilt binary, use any folder and set `MENUYUKTI_DATA_DIR` to it (see Option A).

A sample file is included in this repo:

- [`sales-reports/demo-cafe-sales-july-2026.xlsx`](sales-reports/demo-cafe-sales-july-2026.xlsx) — anonymized cafe month with real data

Example tool argument:

```json
{
  "path": "demo-cafe-sales-july-2026.xlsx"
}
```

## Map my columns (custom POS export)

Built-in support covers known POS exports (ESB, Moka, Quino). If your export uses different column names, add a mapping file so Menuyukti can normalize it locally.

1. Copy [`pos-column-mapping.example.json`](pos-column-mapping.example.json) into your data folder (`sales-reports/` or the folder you set as `MENUYUKTI_DATA_DIR`).
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
