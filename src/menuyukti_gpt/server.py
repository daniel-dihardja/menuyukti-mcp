"""Menuyukti GPT MCP server entrypoint."""

from __future__ import annotations

import json

from mcp.server.mcpserver import MCPServer

from menuyukti_gpt.sales import (
    PathNotAllowedError,
    summary_as_dict,
    summarize_local_sales,
)

DEFAULT_HOST = "127.0.0.1"
DEFAULT_PORT = 8787
MCP_PATH = "/mcp"

mcp = MCPServer(
    name="menuyukti-gpt",
    version="0.1.0",
    instructions=(
        "Local Menuyukti sales analytics for restaurant POS Excel files. "
        "Call summarize_local_sales with a path under MENUYUKTI_DATA_DIR. "
        "Results are aggregates only; raw tickets stay on the local machine."
    ),
)


@mcp.tool(
    name="summarize_local_sales",
    description=(
        "Summarize a local POS Excel sales export. "
        "Reads the file only when path resolves inside MENUYUKTI_DATA_DIR. "
        "Returns detected POS, period, revenue, quantity, and a capped top-item list."
    ),
)
def summarize_local_sales_tool(path: str) -> str:
    """Summarize a local POS Excel sales export under MENUYUKTI_DATA_DIR."""
    try:
        summary = summarize_local_sales(path)
    except PathNotAllowedError as exc:
        return json.dumps({"error": str(exc)})
    except FileNotFoundError as exc:
        return json.dumps({"error": str(exc)})
    except ValueError as exc:
        return json.dumps({"error": str(exc)})

    return json.dumps(summary_as_dict(summary))


def create_server() -> MCPServer:
    return mcp


def main() -> None:
    mcp.run(
        transport="streamable-http",
        host=DEFAULT_HOST,
        port=DEFAULT_PORT,
        streamable_http_path=MCP_PATH,
        json_response=True,
        stateless_http=True,
    )


if __name__ == "__main__":
    main()
