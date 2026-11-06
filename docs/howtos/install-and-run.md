# Install and run

Menuyukti MCP runs on your computer. ChatGPT / Codex (or another MCP client) talks to it locally — your sales files are not uploaded to Menuyukti.

The server is the **Skybridge** TypeScript app in [`apps/menuyukti/`](../../apps/menuyukti/). It serves the MCP URL the Codex plugin expects: **`http://127.0.0.1:8787/mcp`**.

## From this repo (recommended)

1. Install **[Node.js 24+](https://nodejs.org/)** (Skybridge `engines`) and **pnpm 10+**.
2. Clone this repository and open a terminal in the repo root.
3. Put POS Excel exports in [`workspaces/`](../../workspaces/) (a demo workspace is included), or point `MENUYUKTI_DATA_DIR` elsewhere.
4. Install deps once, then start the server:

```bash
pnpm install
pnpm mcp
# or: bash scripts/run-mcp-skybridge.sh
# or: cd apps/menuyukti && MENUYUKTI_DATA_DIR="$PWD/../../workspaces" pnpm dev
```

Keep that terminal open while ChatGPT / Codex is connected.

- **MCP:** `http://127.0.0.1:8787/mcp` (matches [`plugins/menuyukti/mcp.json`](../../plugins/menuyukti/mcp.json))
- **DevTools** (call tools / preview `render_*` views without ChatGPT): `http://127.0.0.1:8787/`
- Details: [`apps/menuyukti/README.md`](../../apps/menuyukti/README.md)

Optional: open [MCP Inspector](https://modelcontextprotocol.io/) against the running server:

```bash
pnpm mcp:inspect
```

## Preview views without ChatGPT

With the server running, open Skybridge DevTools at `http://127.0.0.1:8787/` and call:

- `render_venue_slot_strength` / `render_pair_lift_matrix` / `render_menu_heatmaps` — path `demo-cafe-july-2026`
- `render_weak_items_review` — workspace with sales + COGS (demo cafe works)
- `render_hello_world_widget` — mock smoke only

## Next steps

- Put sales files in a [workspace](./workspaces.md).
- Install the **[Menuyukti Codex plugin](../../plugins/menuyukti/README.md)** (skills + localhost MCP) from this repo’s marketplace in **ChatGPT Desktop / Codex**. Keep `pnpm mcp` running while you chat. Maintainer notes: [Codex plugin howto](./codex-plugin.md).
- Manual alternative: in ChatGPT Desktop settings, add Menuyukti MCP as an MCP server at `http://127.0.0.1:8787/mcp`, then add skills from [`.agents/skills/`](../../.agents/skills/) one by one.
- Ask *What’s my Menuyukti setup status?* or try `demo-cafe-july-2026`.
- If your POS export isn’t recognized, see [Custom POS columns](./custom-pos-columns.md).

## MCP Apps widgets

Tool status text like “Venue slot strength ready.” means the host called the tool.
An inline iframe is a separate step: the host must also mount the Skybridge `ui://` view.

If the tool succeeds but no widget appears:

1. Restart `pnpm mcp`, then refresh the MCP connection in ChatGPT / Codex and start a **new** chat.
2. On Chrome 142+, OpenAI’s Apps SDK examples require disabling
   `chrome://flags/#local-network-access-check` (set to Disabled) and restarting
   Chrome — otherwise localhost MCP widgets stay blank.
3. Prefer ChatGPT Desktop. Web connectors need a tunnel (`pnpm --filter menuyukti dev:tunnel`) — out of Menuyukti 1.0 local Desktop scope.
