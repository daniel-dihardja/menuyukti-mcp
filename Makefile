# Menuyukti MCP — local developer commands (no Node/npm required for the product path).
.PHONY: help mcp mcp-inspect backlog test

MENUYUKTI_DATA_DIR ?= $(CURDIR)/sales-reports
export MENUYUKTI_DATA_DIR

help:
	@echo "Targets:"
	@echo "  make mcp          Start the Rust MCP server (http://127.0.0.1:8787/mcp)"
	@echo "  make mcp-inspect  Start the server and open MCP Inspector (needs npx)"
	@echo "  make test         Run Cargo tests"
	@echo "  make backlog      Open the Backlog.md board"

mcp:
	bash scripts/run-mcp.sh

mcp-inspect:
	bash scripts/mcp-inspect.sh

backlog:
	backlog browser --port 6421 --no-open

test:
	cargo test
