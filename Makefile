# Menuyukti MCP — local developer commands (no Node/npm required for the product path).
.PHONY: help mcp mcp-inspect widgets-preview sync-plugin-skills backlog test check fmt clippy

MENUYUKTI_DATA_DIR ?= $(CURDIR)/workspaces
export MENUYUKTI_DATA_DIR

help:
	@echo "Targets:"
	@echo "  make mcp                 Start the Rust MCP server (http://127.0.0.1:8787/mcp)"
	@echo "  make mcp-inspect         Start the server and open MCP Inspector (needs npx)"
	@echo "  make widgets-preview     Local widget HTML preview with mock data (refresh after edits)"
	@echo "  make sync-plugin-skills  Copy .agents/skills into plugins/menuyukti/skills"
	@echo "  make test                Run Cargo tests"
	@echo "  make check               fmt --check + clippy -D warnings + tests"
	@echo "  make fmt                 Format Rust sources (CHECK=1 to verify only)"
	@echo "  make clippy              Lint with clippy (-D warnings)"
	@echo "  make backlog             Open the Backlog.md board"

mcp:
	bash scripts/run-mcp.sh

mcp-inspect:
	bash scripts/mcp-inspect.sh

widgets-preview:
	bash scripts/run-widget-preview.sh

sync-plugin-skills:
	bash scripts/sync-plugin-skills.sh

backlog:
	backlog browser --port 6421 --no-open

test:
	cargo test

check:
	bash scripts/check.sh

fmt:
	bash scripts/fmt.sh

clippy:
	bash scripts/clippy.sh
