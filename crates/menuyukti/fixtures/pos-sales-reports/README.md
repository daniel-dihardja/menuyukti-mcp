# POS sales-report fixtures

Small, committed Excel demos for POS ingest / normalizer tests in `crates/menuyukti`.

Every POS workbook encodes the same line items from [`shared/demo-dataset.json`](./shared/demo-dataset.json).

| POS key | Demo workbook |
| ------- | ------------- |
| `esb`   | [`esb/SalesRecapitulationDetailReport_DEMO.xlsx`](./esb/SalesRecapitulationDetailReport_DEMO.xlsx) |
| `moka`  | [`moka/ItemDetails_DEMO.xlsx`](./moka/ItemDetails_DEMO.xlsx) |
| `quino` | Synthetic workbook built in Rust tests (not committed here) |

The default owner-facing sample lives at repo-root [`workspaces/demo-cafe-july-2026/sales.xlsx`](../../../../workspaces/demo-cafe-july-2026/sales.xlsx) (paired with `cogs.xlsx` in the same directory).

Personal POS dumps belong under gitignored files in repo-root `workspaces/` — do not commit them here.
