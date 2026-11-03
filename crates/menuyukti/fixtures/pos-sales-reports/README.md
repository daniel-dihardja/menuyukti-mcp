# POS sales-report fixtures

Small, committed Excel demos for POS ingest / normalizer tests in `crates/menuyukti`.

Every POS workbook encodes the same line items from [`shared/demo-dataset.json`](./shared/demo-dataset.json).

| POS key | Demo workbook |
| ------- | ------------- |
| `esb`   | [`esb/SalesRecapitulationDetailReport_DEMO.xlsx`](./esb/SalesRecapitulationDetailReport_DEMO.xlsx) |
| `moka`  | [`moka/ItemDetails_DEMO.xlsx`](./moka/ItemDetails_DEMO.xlsx) |
| `quino` | Synthetic workbook built in Rust tests (not committed here) |

The default owner-facing sample lives at repo-root [`sales-reports/demo-cafe-sales-july-2026.xlsx`](../../../../sales-reports/demo-cafe-sales-july-2026.xlsx).

Personal POS dumps belong under gitignored files in repo-root `sales-reports/` — do not commit them here.
