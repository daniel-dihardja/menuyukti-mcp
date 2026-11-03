# Menuyukti

Python library for **menu and sales analytics**: POS Excel normalization and numeric analytics used by the local GPT plugin MCP server in this repo.

## Boundary

- **Implement analytics here** — non-trivial calculation, aggregation, or domain rules about sales/menu data belong under `src/menuyukti/`, typically `core/analytics/`. Add or extend unit tests under `tests/`.
- **Keep the MCP server thin** — `src/menuyukti_gpt/` should gate local paths, call into this package, and return aggregates. Avoid duplicating business logic in the MCP layer.

## Layout

| Area                          | Role                                                                                                                                           |
| ----------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------- |
| `core/analytics/`             | Analytics modules (`calculate_*.py`), helpers (`extract_*`, `pos_detector`, `registry`, `utils`), and POS-specific normalization (e.g. `esb/`) |
| `core/models/`                | Shared data shapes (e.g. POS line items, mapping config) consumed by analytics and ingestion                                                   |
| `fixtures/pos-sales-reports/` | Small committed POS Excel demos + shared dataset / regenerate script (`make generate-pos-demos`)                                               |

Prefer **`compute_*_from_orders`** (or similar) entry points when the caller has order rows and the core wants a `DataFrame` or structured rows internally.

## Public API

Export stable entry points from `menuyukti.core.analytics` (`__init__.py` + `__all__`) so callers can `from menuyukti.core.analytics import ...` without reaching into deep module paths unless necessary.

## Versioning and contracts

Treat changes to public functions and return shapes as **API contracts**. Column names on `POSTransactionLineItem` and keys on public `TypedDict` results are part of the stable surface exported from `menuyukti.core.analytics`.

## Tests and tooling

- Unit tests: `tests/` (e.g. `tests/unit/core/analytics/`).
- From this directory: `make test`, `make type-check` (see `Makefile`).
- From the repo root: `uv sync` / `uv run` (this package is a path dependency of `menuyukti-gpt`).

For source-level notes, see `src/menuyukti/README.md`.
