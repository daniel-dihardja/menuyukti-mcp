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
# or: MENUYUKTI_DATA_DIR="$PWD/workspaces" cargo run -p menuyukti-mcp
```

Optional: open [MCP Inspector](https://modelcontextprotocol.io/) against the running server (needs Node / `npx`):

```bash
make mcp-inspect
```

## Next steps

- Put sales files in a [workspace](./workspaces.md).
- In **ChatGPT Desktop** settings, add Menuyukti MCP as an MCP server at `http://127.0.0.1:8787/mcp`, then use it in chat.
- If your POS export isn’t recognized, see [Custom POS columns](./custom-pos-columns.md).
