# Connect ChatGPT with Secure MCP Tunnel

ChatGPT cannot call `http://127.0.0.1:8787/mcp` on your machine. [OpenAI Secure MCP Tunnel](https://developers.openai.com/api/docs/guides/secure-mcp-tunnels) bridges that gap: you run OpenAI’s `tunnel-client` locally; it opens an **outbound** HTTPS connection to OpenAI and forwards MCP requests to Gastro MCP. Your sales files stay on your computer — the tunnel does not publish the MCP server on the public internet.

Official docs: [Secure MCP Tunnel](https://developers.openai.com/api/docs/guides/secure-mcp-tunnels) · [tunnel-client](https://github.com/openai/tunnel-client) · [Add custom MCP server](https://developers.openai.com/api/docs/guides/custom-mcp-server)

## 1. Start Gastro MCP

Follow [Install and run](./install-and-run.md) and leave the server running at `http://127.0.0.1:8787/mcp`.

## 2. Create a tunnel and runtime API key

1. In the [Platform tunnel settings](https://platform.openai.com/settings/organization/tunnels), create a tunnel and note its `tunnel_id` (looks like `tunnel_…`).
2. Associate the tunnel with the **ChatGPT workspace** you will use (not only a Platform organization), or it may not appear in ChatGPT.
3. Create a **runtime** API key under [API keys](https://platform.openai.com/settings/organization/api-keys). The key principal needs **Tunnels Read + Use**. Creating/editing tunnels needs **Tunnels Read + Manage**.
4. Download `tunnel-client` from the tunnel settings page or the [latest tunnel-client release](https://github.com/openai/tunnel-client/releases/latest).

Ask your ChatGPT workspace admin for permission to add custom MCP servers if your workspace restricts that.

## 3. Run tunnel-client against Gastro MCP

On the same machine as Gastro MCP (or any host that can reach `127.0.0.1:8787`):

```bash
export CONTROL_PLANE_API_KEY="sk-..."   # runtime API key — do not commit this

tunnel-client init \
  --profile gastro-local \
  --tunnel-id tunnel_YOUR_ID_HERE \
  --mcp-server-url http://127.0.0.1:8787/mcp

tunnel-client doctor --profile gastro-local --explain
tunnel-client run --profile gastro-local
```

Keep `tunnel-client run` healthy while you use ChatGPT. Check the local admin UI (default `http://127.0.0.1:8080/ui`) for healthy / ready / connected before testing tools.

Equivalent env-style setup (also supported by tunnel-client):

```bash
export CONTROL_PLANE_API_KEY="sk-..."
export CONTROL_PLANE_TUNNEL_ID="tunnel_YOUR_ID_HERE"
export MCP_SERVER_URL="http://127.0.0.1:8787/mcp"
tunnel-client run
```

## 4. Add Gastro MCP as a ChatGPT plugin

1. In ChatGPT, open **Plugins** → **+** → **Add custom MCP server**.
2. Under **Connection**, choose **Tunnel**.
3. Select your tunnel (or paste the `tunnel_id`).
4. Authentication: Gastro MCP does not require auth today — choose **No authentication** (or the equivalent option in the UI).
5. Accept the risk warning and **Create as a plugin**, then install/enable it.
6. In a chat, `@` the plugin and ask something like: list my workspaces, then summarize `demo-cafe-july-2026`.

## Troubleshooting

If the tunnel does not appear in ChatGPT, confirm workspace association and **Tunnels Read + Use** on the account adding the plugin.

If tool calls fail, confirm both `gastro-mcp` and `tunnel-client run` are still running, then re-run:

```bash
tunnel-client doctor --profile gastro-local --explain
```
