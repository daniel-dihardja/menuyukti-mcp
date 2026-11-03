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

Both modes listen at **`http://127.0.0.1:8787/mcp`** (streamable HTTP). Local tools such as MCP Inspector can use that URL directly. ChatGPT cannot reach localhost, so use [Secure MCP Tunnel](#connect-chatgpt-with-secure-mcp-tunnel) for ChatGPT.

Tools:

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

## Connect ChatGPT with Secure MCP Tunnel

ChatGPT cannot call `http://127.0.0.1:8787/mcp` on your machine. [OpenAI Secure MCP Tunnel](https://developers.openai.com/api/docs/guides/secure-mcp-tunnels) bridges that gap: you run OpenAI’s `tunnel-client` locally; it opens an **outbound** HTTPS connection to OpenAI and forwards MCP requests to Menuyukti. Your sales files stay on your computer — the tunnel does not publish the MCP server on the public internet.

Official docs: [Secure MCP Tunnel](https://developers.openai.com/api/docs/guides/secure-mcp-tunnels) · [tunnel-client](https://github.com/openai/tunnel-client) · [Add custom MCP server](https://developers.openai.com/api/docs/guides/custom-mcp-server)

### 1. Start Menuyukti MCP

Use [Option A](#option-a--prebuilt-binary) or [Option B](#option-b--from-source) and leave the server running at `http://127.0.0.1:8787/mcp`.

### 2. Create a tunnel and runtime API key

1. In the [Platform tunnel settings](https://platform.openai.com/settings/organization/tunnels), create a tunnel and note its `tunnel_id` (looks like `tunnel_…`).
2. Associate the tunnel with the **ChatGPT workspace** you will use (not only a Platform organization), or it may not appear in ChatGPT.
3. Create a **runtime** API key under [API keys](https://platform.openai.com/settings/organization/api-keys). The key principal needs **Tunnels Read + Use**. Creating/editing tunnels needs **Tunnels Read + Manage**.
4. Download `tunnel-client` from the tunnel settings page or the [latest tunnel-client release](https://github.com/openai/tunnel-client/releases/latest).

Ask your ChatGPT workspace admin for permission to add custom MCP servers if your workspace restricts that.

### 3. Run tunnel-client against Menuyukti

On the same machine as Menuyukti (or any host that can reach `127.0.0.1:8787`):

```bash
export CONTROL_PLANE_API_KEY="sk-..."   # runtime API key — do not commit this

tunnel-client init \
  --profile menuyukti-local \
  --tunnel-id tunnel_YOUR_ID_HERE \
  --mcp-server-url http://127.0.0.1:8787/mcp

tunnel-client doctor --profile menuyukti-local --explain
tunnel-client run --profile menuyukti-local
```

Keep `tunnel-client run` healthy while you use ChatGPT. Check the local admin UI (default `http://127.0.0.1:8080/ui`) for healthy / ready / connected before testing tools.

Equivalent env-style setup (also supported by tunnel-client):

```bash
export CONTROL_PLANE_API_KEY="sk-..."
export CONTROL_PLANE_TUNNEL_ID="tunnel_YOUR_ID_HERE"
export MCP_SERVER_URL="http://127.0.0.1:8787/mcp"
tunnel-client run
```

### 4. Add Menuyukti as a ChatGPT plugin

1. In ChatGPT, open **Plugins** → **+** → **Add custom MCP server**.
2. Under **Connection**, choose **Tunnel**.
3. Select your tunnel (or paste the `tunnel_id`).
4. Authentication: Menuyukti does not require auth today — choose **No authentication** (or the equivalent option in the UI).
5. Accept the risk warning and **Create as a plugin**, then install/enable it.
6. In a chat, `@` the plugin and ask something like: list my sales reports, then summarize `demo-cafe-sales-july-2026.xlsx`.

If the tunnel does not appear in ChatGPT, confirm workspace association and **Tunnels Read + Use** on the account adding the plugin. If tool calls fail, confirm both `menuyukti-mcp` and `tunnel-client run` are still running, then re-run `tunnel-client doctor --profile menuyukti-local --explain`.

---

## Sales reports folder

In the source checkout, put POS Excel exports in [`sales-reports/`](sales-reports/). That folder is the default `MENUYUKTI_DATA_DIR` when you use `make mcp`.

With a prebuilt binary, use any folder and set `MENUYUKTI_DATA_DIR` to it (see Option A).

Sample files included in this repo:

- [`sales-reports/demo-cafe-sales-july-2026.xlsx`](sales-reports/demo-cafe-sales-july-2026.xlsx) — anonymized cafe month with real data
- [`sales-reports/demo-cafe-cogs-july-2026.xlsx`](sales-reports/demo-cafe-cogs-july-2026.xlsx) — matching menu COGS (hospitality starter estimates; `Menu` + `COGS`)

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
make check     # fmt --check + clippy + tests
make test
# or: cargo test -p menuyukti && cargo test -p menuyukti-mcp
make mcp
make backlog   # Backlog.md board (optional)
```

Workspace layout:

- `crates/menuyukti` — Excel normalize + analytics library
- `crates/menuyukti-mcp` — MCP HTTP binary
