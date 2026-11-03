#!/usr/bin/env python3
"""Anonymize an ESB Sales Recapitulation Detail Report for committed fixtures.

Keeps prices, quantities, and order timing. Replaces menu names with a
stable fictional cafe menu and scrubs venue / staff labels.
"""

from __future__ import annotations

import argparse
import hashlib
from pathlib import Path

import pandas as pd
from openpyxl import Workbook
from openpyxl.utils.dataframe import dataframe_to_rows

REPO_ROOT = Path(__file__).resolve().parents[5]
DEFAULT_SOURCE = (
    REPO_ROOT / "sales-reports" / "SalesRecapitulationDetailReport_JUL_2026.xlsx"
)
DEFAULT_OUTPUT = REPO_ROOT / "sales-reports" / "demo-cafe-sales-july-2026.xlsx"

ESB_SKIPROWS = 10
VENUE_NAME = "Demo Cafe"
CITY_NAME = "Demo City"
WAITER_NAME = "Cashier 01"
USERNAME = "DEMOADMIN"

# Plausible fictional names by category detail family. Fallbacks stay generic.
DETAIL_NAME_POOLS: dict[str, list[str]] = {
    "ES KOPI SUSU": [
        "Iced Milk Coffee Aren",
        "Iced Milk Coffee Pandan",
        "Iced Milk Coffee Dolce",
    ],
    "COFFEE": [
        "Iced Americano",
        "Hot Americano",
        "Iced Latte",
        "Hot Latte",
        "Cappuccino",
        "Iced Mocha",
        "Hot Mocha",
        "Iced Vanilla Latte",
        "Hot Vanilla Latte",
        "Iced Hazelnut Latte",
        "Hot Hazelnut Latte",
        "Iced Caramel Latte",
        "Cinnamon Latte Hot",
        "Cinnamon Latte Ice",
        "Avocado Cafe Latte",
        "Single Espresso",
        "Double Espresso",
        "Shakerato",
        "Iced Americano Dolce",
        "Hot Americano Aren",
        "Cremano Butterscotch",
        "Coffee Tonic",
    ],
    "CHOCOLATE": [
        "Iced Chocolate",
        "Hot Chocolate",
        "Choco Banana",
        "Choco Hazelnut",
        "Avocado Chocolate",
    ],
    "MATCHA LATTE": ["Iced Matcha", "Hot Matcha"],
    "NON COFFEE": ["Mineral Water", "Taro Latte Ice", "Avocado Latte"],
    "ADDITIONAL BVG": ["Espresso Shot", "Honey Lemongrass"],
    "ICE TEA": ["Lemon Ice Tea", "Earl Grey Honey", "Thai Ice Tea"],
    "HOT TEA": ["Earl Grey", "Sencha", "Chamomile"],
    "TEA": ["Thai Tea"],
    "HOJICA LATTE": ["Hojicha Latte Ice", "Hojicha Latte Hot"],
    "OAT SERIES": ["Butterscotch Oat Latte"],
    "GORENGAN": [
        "Fried Snack A",
        "Fried Snack B",
        "Fried Snack C",
        "Fried Snack D",
        "Fried Snack E",
    ],
    "ADDITIONAL FOOD": [
        "Onigiri Tuna Hot",
        "Onigiri Chicken Hot",
        "Onigiri Tuna Mayo",
        "Onigiri Chicken Mayo",
        "Fried Rice House",
        "Fried Noodles House",
        "Soft Roll",
        "Omelette Side",
    ],
    "KUE BASAH TOKO KUNING": [
        "Steamed Cake A",
        "Steamed Cake B",
        "Steamed Cake C",
        "Steamed Cake D",
        "Steamed Cake E",
        "Steamed Cake F",
        "Steamed Cake G",
        "Steamed Cake H",
        "Steamed Cake I",
        "Steamed Cake J",
    ],
    "KUE BASAH BELINDA": ["Moist Cake A", "Moist Cake B"],
    "KUE BASAH IBU ACHIN": [
        "Layer Cake A",
        "Layer Cake B",
        "Layer Cake C",
        "Layer Cake D",
        "Layer Cake E",
    ],
    "DON BAKESHOP": [
        "Almond Croissant",
        "Chocolate Croissant",
        "Ovaltine Croissant",
        "Sausage Croissant",
        "Banana Nutella Croissant",
    ],
    "MIE INSTANT": ["Instant Noodles Soup", "Instant Noodles Fried"],
    "TOPPING INDOMIE": ["Egg Topping"],
    "NASI IBU ANYU": ["Coconut Rice Plate", "Rice Noodle Plate", "Yellow Rice Plate"],
    "MENU LAINNYA": [
        "Savory Bun A",
        "Savory Bun B",
        "Savory Bun C",
        "Savory Bun D",
        "Cheese Stick",
        "Spring Roll House",
    ],
    "Happy Hour": [
        "Happy Hour Cake Duo",
        "Happy Hour Fried Duo",
        "Happy Hour Onigiri Duo",
    ],
    "OTHER": ["Custom Plate"],
    "SNABB PASTA": ["Creamy Pasta"],
}

DETAIL_LABELS: dict[str, str] = {
    "ES KOPI SUSU": "MILK COFFEE",
    "COFFEE": "COFFEE",
    "CHOCOLATE": "CHOCOLATE",
    "MATCHA LATTE": "MATCHA",
    "NON COFFEE": "NON COFFEE",
    "ADDITIONAL BVG": "DRINK ADD-ON",
    "ICE TEA": "ICE TEA",
    "HOT TEA": "HOT TEA",
    "TEA": "TEA",
    "HOJICA LATTE": "HOJICHA",
    "OAT SERIES": "OAT SERIES",
    "GORENGAN": "FRIED SNACKS",
    "ADDITIONAL FOOD": "FOOD ADD-ON",
    "KUE BASAH TOKO KUNING": "STEAMED CAKE",
    "KUE BASAH BELINDA": "MOIST CAKE",
    "KUE BASAH IBU ACHIN": "LAYER CAKE",
    "DON BAKESHOP": "BAKERY",
    "MIE INSTANT": "INSTANT NOODLES",
    "TOPPING INDOMIE": "NOODLE TOPPING",
    "NASI IBU ANYU": "RICE PLATES",
    "MENU LAINNYA": "OTHER FOOD",
    "Happy Hour": "Happy Hour",
    "OTHER": "OTHER",
    "SNABB PASTA": "PASTA",
}


def _stable_index(key: str, size: int) -> int:
    digest = hashlib.sha256(key.encode("utf-8")).hexdigest()
    return int(digest[:8], 16) % size


def _anon_detail(detail: object) -> object:
    if pd.isna(detail):
        return detail
    key = str(detail)
    return DETAIL_LABELS.get(key, "GENERAL")


def _anon_code(original: object, menu_name: str) -> object:
    if pd.isna(original) or str(original).strip() in {"", "-"}:
        return original
    return f"SKU{_stable_index(menu_name, 9999):04d}"


def _build_menu_map(df: pd.DataFrame) -> dict[str, str]:
    """Map each original menu to a unique fictional name, stable by appearance."""
    menu_map: dict[str, str] = {}
    counters: dict[str, int] = {}

    for menu, group in df.groupby("Menu", sort=False):
        original = str(menu)
        detail_key = group["Menu Category Detail"].iloc[0]
        detail_key = str(detail_key) if pd.notna(detail_key) else ""
        category = group["Menu Category"].iloc[0]
        category_key = str(category) if pd.notna(category) else "ITEM"

        pool = DETAIL_NAME_POOLS.get(detail_key, [])
        idx = counters.get(detail_key, 0)
        counters[detail_key] = idx + 1

        if idx < len(pool):
            name = pool[idx]
        else:
            label = DETAIL_LABELS.get(detail_key, category_key.title())
            name = f"{label} {idx + 1:02d}"

        menu_map[original] = name

    return menu_map


def anonymize_frame(df: pd.DataFrame) -> pd.DataFrame:
    out = df.copy()
    unique_map = _build_menu_map(out)

    out["Menu"] = out["Menu"].map(lambda value: unique_map[str(value)])
    out["Menu Category Detail"] = out["Menu Category Detail"].map(_anon_detail)
    out["Custom Menu Name"] = out["Menu"].where(out["Custom Menu Name"].notna(), None)
    out["Menu Code"] = [
        _anon_code(code, menu)
        for code, menu in zip(out["Menu Code"], out["Menu"], strict=True)
    ]
    out["Menu Notes"] = None

    out["Branch"] = VENUE_NAME
    out["Brand"] = VENUE_NAME
    out["City"] = CITY_NAME
    out["Waiter"] = WAITER_NAME
    out["Employee Name"] = "-"
    out["Employee Code"] = "-"
    out["External Employee Name"] = "-"
    out["External Employee Code"] = "-"
    out["Customer Name"] = "-"
    out["Regular Member Name"] = out["Regular Member Name"].map(
        lambda value: "Non Member"
        if pd.isna(value) or str(value).strip().lower() in {"non member", ""}
        else "Member"
    )
    out["Loyalty Member Name"] = None
    out["Loyalty Member Code"] = None
    out["Loyalty Member Type"] = None

    return out


def write_esb_workbook(
    source: Path,
    output: Path,
    *,
    period_label: str,
) -> tuple[int, int]:
    raw_meta = pd.read_excel(source, header=None, nrows=ESB_SKIPROWS)
    df = pd.read_excel(source, skiprows=ESB_SKIPROWS)
    anon = anonymize_frame(df)

    wb = Workbook()
    ws = wb.active
    assert ws is not None
    ws.title = "Report"

    # Preserve ESB title/layout; scrub venue identity in the metadata block.
    ws["A1"] = "Sales Recapitulation Detail Report"
    ws["A2"] = VENUE_NAME
    ws["A4"] = "Generated"
    ws["B4"] = "01-01-2026 00:00:00"
    ws["A5"] = "Period"
    ws["B5"] = period_label
    ws["A6"] = "Branch"
    ws["B6"] = "All"
    ws["A7"] = "Sales Type"
    ws["B7"] = "Sales"
    ws["A8"] = "Generated Username"
    ws["B8"] = USERNAME
    ws["A9"] = "Report File Name"
    ws["B9"] = "Sales Recapitulation Detail Report - ANON"

    # Keep blank row 10, headers on row 11 (1-indexed), matching skiprows=10.
    for col_idx, header in enumerate(anon.columns.tolist(), start=1):
        ws.cell(row=11, column=col_idx, value=header)

    for row_idx, row in enumerate(
        dataframe_to_rows(anon, index=False, header=False), start=12
    ):
        for col_idx, value in enumerate(row, start=1):
            ws.cell(row=row_idx, column=col_idx, value=value)

    output.parent.mkdir(parents=True, exist_ok=True)
    wb.save(output)
    return len(anon), int(anon["Menu"].nunique())


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--source",
        type=Path,
        default=DEFAULT_SOURCE,
        help="Path to the original ESB Excel export",
    )
    parser.add_argument(
        "--output",
        type=Path,
        default=DEFAULT_OUTPUT,
        help="Path for the anonymized fixture workbook",
    )
    parser.add_argument(
        "--period",
        default="01-07-2026 - 31-07-2026",
        help="Period label written into the ESB metadata block",
    )
    args = parser.parse_args()

    if not args.source.is_file():
        raise SystemExit(f"Source workbook not found: {args.source}")

    rows, menus = write_esb_workbook(
        args.source.resolve(),
        args.output.resolve(),
        period_label=args.period,
    )
    print(f"Wrote {args.output} ({rows} rows, {menus} anonymized menus)")


if __name__ == "__main__":
    main()
