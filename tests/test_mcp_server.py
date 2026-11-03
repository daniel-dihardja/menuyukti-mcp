"""HTTP integration test for the Menuyukti GPT MCP server."""

from __future__ import annotations

import asyncio
import json
import socket
from pathlib import Path

import pytest
import uvicorn
from mcp.client.client import Client

from menuyukti_gpt.sales import DATA_DIR_ENV
from menuyukti_gpt.server import MCP_PATH, create_server

FIXTURES = (
    Path(__file__).resolve().parents[1]
    / "packages"
    / "menuyukti"
    / "fixtures"
    / "pos-sales-reports"
)


def _free_port() -> int:
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as sock:
        sock.bind(("127.0.0.1", 0))
        return int(sock.getsockname()[1])


@pytest.mark.asyncio
async def test_summarize_local_sales_over_streamable_http(
    monkeypatch: pytest.MonkeyPatch,
) -> None:
    monkeypatch.setenv(DATA_DIR_ENV, str(FIXTURES))
    port = _free_port()
    mcp_server = create_server()
    app = mcp_server.streamable_http_app(
        streamable_http_path=MCP_PATH,
        json_response=True,
        stateless_http=True,
        host="127.0.0.1",
    )
    config = uvicorn.Config(app, host="127.0.0.1", port=port, log_level="warning")
    http_server = uvicorn.Server(config)
    serve_task = asyncio.create_task(http_server.serve())

    try:
        for _ in range(100):
            if http_server.started:
                break
            await asyncio.sleep(0.05)
        assert http_server.started, "MCP HTTP server failed to start"

        url = f"http://127.0.0.1:{port}{MCP_PATH}"
        async with Client(url) as client:
            tools = await client.list_tools()
            tool_names = {tool.name for tool in tools.tools}
            assert "summarize_local_sales" in tool_names
            assert "list_normalized_sales" in tool_names
            assert "get_menu_heatmaps" in tool_names
            assert "list_sales_reports" in tool_names

            available = await client.call_tool("list_sales_reports", {})

            result = await client.call_tool(
                "summarize_local_sales",
                {"path": "esb/SalesRecapitulationDetailReport_DEMO.xlsx"},
            )

            listed = await client.call_tool(
                "list_normalized_sales",
                {"path": "esb/SalesRecapitulationDetailReport_DEMO.xlsx"},
            )

            heatmaps = await client.call_tool(
                "get_menu_heatmaps",
                {"path": "esb/SalesRecapitulationDetailReport_DEMO.xlsx"},
            )

            blocked = await client.call_tool(
                "get_menu_heatmaps",
                {"path": "/tmp/outside.xlsx"},
            )

        assert available.is_error is False
        available_blocks = [
            block
            for block in available.content
            if getattr(block, "type", None) == "text"
        ]
        assert available_blocks
        available_payload = json.loads(available_blocks[0].text)
        assert available_payload["report_count"] >= 1
        available_paths = [item["path"] for item in available_payload["reports"]]
        assert "esb/SalesRecapitulationDetailReport_DEMO.xlsx" in available_paths

        assert result.is_error is False
        assert result.content
        text_blocks = [
            block for block in result.content if getattr(block, "type", None) == "text"
        ]
        assert text_blocks
        payload = json.loads(text_blocks[0].text)
        assert payload["pos"] == "esb"
        assert payload["total_revenue"] > 0
        assert payload["total_items_sold"] > 0
        assert 1 <= len(payload["top_items"]) <= 10
        assert "bill_number" not in json.dumps(payload)

        assert listed.is_error is False
        listed_blocks = [
            block for block in listed.content if getattr(block, "type", None) == "text"
        ]
        assert listed_blocks
        listed_payload = json.loads(listed_blocks[0].text)
        assert listed_payload["pos"] == "esb"
        assert listed_payload["row_count"] == len(listed_payload["rows"])
        assert listed_payload["row_count"] > 0
        assert "bill_number" in listed_payload["rows"][0]

        assert heatmaps.is_error is False
        heatmap_blocks = [
            block
            for block in heatmaps.content
            if getattr(block, "type", None) == "text"
        ]
        assert heatmap_blocks
        heatmap_payload = json.loads(heatmap_blocks[0].text)
        assert heatmap_payload["pos"] == "esb"
        assert heatmap_payload["menu_count"] == len(heatmap_payload["heatmaps"])
        assert heatmap_payload["menu_count"] > 0
        first_menu = heatmap_payload["heatmaps"][0]
        assert len(first_menu["daily_heatmap"]) == 24
        assert len(first_menu["weekly_heatmap"]) == 7
        assert "bill_number" not in json.dumps(heatmap_payload)

        assert blocked.is_error is False
        blocked_blocks = [
            block for block in blocked.content if getattr(block, "type", None) == "text"
        ]
        assert blocked_blocks
        blocked_payload = json.loads(blocked_blocks[0].text)
        assert "error" in blocked_payload
        assert "outside" in blocked_payload["error"].lower()
    finally:
        http_server.should_exit = True
        await serve_task
