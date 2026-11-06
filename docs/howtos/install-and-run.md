# Install and run

Menuyukti MCP runs on your computer. ChatGPT (or another MCP client) talks to it locally — your sales files are not uploaded to Menuyukti.

## Option A — Prebuilt binary (recommended for owners)

Tagged releases publish archives for Windows (x64), macOS (Apple Silicon + Intel), and Linux (x64 + ARM64). No Rust install.

1. Download the archive for your OS from [GitHub Releases](https://github.com/daniel-dihardja/menuyukti-mcp/releases).
2. Extract it. Inside you’ll find `menuyukti-mcp` (or `menuyukti-mcp.exe` on Windows), plus `LICENSE` and `README.md`.
3. Create a folder for your analysis workspaces (any path you like).
4. Start the server:

```bash
# macOS / Linux
MENUYUKTI_DATA_DIR="/path/to/workspaces" ./menuyukti-mcp

# Windows (PowerShell)
$env:MENUYUKTI_DATA_DIR="C:\path\to\workspaces"
.\menuyukti-mcp.exe
```

Keep that terminal open while ChatGPT / your MCP client is connected.

The server listens at **`http://127.0.0.1:8787/mcp`**.

## Option B — From source (developers)

1. Install [Rust](https://rustup.rs/) (stable toolchain with Cargo).
2. Clone this repository and open a terminal in the repo root.
3. Put POS Excel exports in [`workspaces/`](../../workspaces/) (a demo workspace is included), or point `MENUYUKTI_DATA_DIR` elsewhere.
4. Start the server:

```bash
make mcp
# or: bash scripts/run-mcp.sh
# or: MENUYUKTI_DATA_DIR="$PWD/workspaces" cargo run -p menuyukti-mcp --bin menuyukti-mcp
```

Optional: open [MCP Inspector](https://modelcontextprotocol.io/) against the running server (needs Node / `npx`):

```bash
make mcp-inspect
```

### Widget preview (styles / mock data)

To style MCP Apps widgets without ChatGPT or the MCP server, run the Rust preview harness:

```bash
make widgets-preview
# or: bash scripts/run-widget-preview.sh
# or: cargo run -p menuyukti-mcp --bin menuyukti-widget-preview
```

Open **`http://127.0.0.1:8790/`**, pick a widget (Hello World, venue slot strength, pair lift, or an empty edge case). The page iframes the real HTML under `crates/menuyukti-mcp/assets/` (with shared `widget-theme.css` inlined for heatmaps) and injects mock `structuredContent`. Saving a watched asset or preview fixture reloads the browser. Node is not required for this path. Optional port override: `MENUYUKTI_WIDGET_PREVIEW_PORT`. See [Develop](./develop.md#mcp-apps-widget-theme) for the shared theme.

## Next steps

- Put sales files in a [workspace](./workspaces.md).
- Prefer installing the **[Menuyukti Codex plugin](./codex-plugin.md)** (skills + localhost MCP) from this repo’s marketplace in **ChatGPT Desktop / Codex**. The plugin is Desktop-only and still requires this binary to be running.
- Manual alternative: in ChatGPT Desktop settings, add Menuyukti MCP as an MCP server at `http://127.0.0.1:8787/mcp`, then add skills from [`.agents/skills/`](../../.agents/skills/) one by one.
- Ask *What’s my Menuyukti setup status?* or try `demo-cafe-july-2026`.
- If your POS export isn’t recognized, see [Custom POS columns](./custom-pos-columns.md).

## MCP Apps widgets (Hello World / charts)

Tool status text like “Hello World widget ready.” means ChatGPT called the tool.
An inline iframe is a separate step: ChatGPT must also `resources/read` the
`ui://` HTML template.

If the tool succeeds but no widget appears:

1. Restart `make mcp`, then refresh the MCP connection in ChatGPT and start a **new** chat.
2. On Chrome 142+, OpenAI’s Apps SDK examples require disabling
   `chrome://flags/#local-network-access-check` (set to Disabled) and restarting
   Chrome — otherwise localhost MCP widgets stay blank.
3. Prefer ChatGPT Desktop or a ChatGPT plugin connected to an HTTPS tunnel
   (`ngrok http 8787` → `https://….ngrok.app/mcp`) when local Desktop MCP only
   shows tool text.
