"""Unit tests for local sales path gate, normalization, and aggregate summary."""

from __future__ import annotations

from pathlib import Path
from unittest.mock import patch

import pytest

from menuyukti.core.models.pos_transaction import POSTransactionLineItem
from menuyukti.core.analytics import WEEKDAY_ORDER
from menuyukti_gpt.sales import (
    DATA_DIR_ENV,
    PathNotAllowedError,
    get_menu_heatmaps,
    list_normalized_sales,
    list_sales_reports,
    load_normalized_sales,
    summarize_local_sales,
)

REPO_ROOT = Path(__file__).resolve().parents[1]
FIXTURES = (
    REPO_ROOT
    / "packages"
    / "menuyukti"
    / "fixtures"
    / "pos-sales-reports"
)
SALES_REPORTS = REPO_ROOT / "sales-reports"
ESB_DEMO = FIXTURES / "esb" / "SalesRecapitulationDetailReport_DEMO.xlsx"
ESB_REL = "esb/SalesRecapitulationDetailReport_DEMO.xlsx"
DEMO_CAFE = SALES_REPORTS / "demo-cafe-sales-july-2026.xlsx"
DEMO_CAFE_REL = "demo-cafe-sales-july-2026.xlsx"


@pytest.fixture
def data_dir(monkeypatch: pytest.MonkeyPatch) -> Path:
    monkeypatch.setenv(DATA_DIR_ENV, str(FIXTURES))
    return FIXTURES.resolve()


@pytest.fixture
def sales_reports_dir(monkeypatch: pytest.MonkeyPatch) -> Path:
    monkeypatch.setenv(DATA_DIR_ENV, str(SALES_REPORTS))
    return SALES_REPORTS.resolve()


def test_rejects_path_outside_data_dir_without_reading(
    data_dir: Path, tmp_path: Path
) -> None:
    outside = tmp_path / "secret.xlsx"
    outside.write_bytes(b"not-excel")

    with patch("pathlib.Path.read_bytes") as read_bytes:
        with pytest.raises(PathNotAllowedError, match="outside"):
            load_normalized_sales(str(outside), data_dir=data_dir)
        read_bytes.assert_not_called()


def test_rejects_traversal_without_reading(data_dir: Path) -> None:
    with patch("pathlib.Path.read_bytes") as read_bytes:
        with pytest.raises(PathNotAllowedError, match="outside"):
            load_normalized_sales("../README.md", data_dir=data_dir)
        read_bytes.assert_not_called()


def test_requires_data_dir_env(monkeypatch: pytest.MonkeyPatch) -> None:
    monkeypatch.delenv(DATA_DIR_ENV, raising=False)
    with patch("pathlib.Path.read_bytes") as read_bytes:
        with pytest.raises(PathNotAllowedError, match=DATA_DIR_ENV):
            load_normalized_sales(ESB_REL)
        read_bytes.assert_not_called()


def test_esb_fixture_load_normalized_sales(data_dir: Path) -> None:
    assert ESB_DEMO.is_file()
    loaded = load_normalized_sales(ESB_REL, data_dir=data_dir)

    assert loaded["pos"] == "esb"
    frame = loaded["frame"]
    assert not frame.empty
    for column in POSTransactionLineItem.get_required_columns():
        assert column in frame.columns


def test_demo_cafe_sales_fixture_load_normalized_sales(
    sales_reports_dir: Path,
) -> None:
    assert DEMO_CAFE.is_file()
    loaded = load_normalized_sales(DEMO_CAFE_REL, data_dir=sales_reports_dir)

    assert loaded["pos"] == "esb"
    frame = loaded["frame"]
    assert len(frame) > 1000
    assert frame["menu"].nunique() > 50
    assert float(frame["total_after_bill_discount"].sum()) > 0
    blob = " ".join(map(str, frame["menu"].unique()))
    assert "Snabb" not in blob
    assert "Es Kopi Susu Aren" not in blob


def test_list_sales_reports_finds_demo_and_skips_non_excel(
    sales_reports_dir: Path,
) -> None:
    listed = list_sales_reports(data_dir=sales_reports_dir)
    paths = [item["path"] for item in listed["reports"]]

    assert listed["report_count"] == len(paths)
    assert DEMO_CAFE_REL in paths
    assert "README.md" not in paths
    assert all(path.endswith((".xlsx", ".xls")) for path in paths)


def test_list_sales_reports_empty_dir(tmp_path: Path) -> None:
    listed = list_sales_reports(data_dir=tmp_path)
    assert listed["report_count"] == 0
    assert listed["reports"] == []
    assert listed["data_dir"] == str(tmp_path.resolve())


def test_esb_fixture_summary_shape(data_dir: Path) -> None:
    assert ESB_DEMO.is_file()
    summary = summarize_local_sales(ESB_REL, data_dir=data_dir)

    assert summary["pos"] == "esb"
    assert summary["period_start"] is not None
    assert summary["period_end"] is not None
    assert summary["total_revenue"] > 0
    assert summary["total_items_sold"] > 0
    assert 1 <= len(summary["top_items"]) <= 10

    allowed_keys = {
        "pos",
        "period_start",
        "period_end",
        "total_revenue",
        "total_items_sold",
        "top_items",
    }
    assert set(summary.keys()) == allowed_keys

    for item in summary["top_items"]:
        assert set(item.keys()) == {"menu", "popularity", "quantity"}
        assert "bill_number" not in item
        assert "order_time" not in item

    # Ensure raw ticket fields did not leak into the summary.
    blob = str(summary)
    assert "bill_number" not in blob
    assert "total_after_bill_discount" not in blob


def test_esb_fixture_list_normalized_sales(data_dir: Path) -> None:
    assert ESB_DEMO.is_file()
    listed = list_normalized_sales(ESB_REL, data_dir=data_dir)

    assert listed["pos"] == "esb"
    assert listed["row_count"] == len(listed["rows"])
    assert listed["row_count"] > 0
    assert set(listed.keys()) == {"pos", "rows", "row_count"}

    required = set(POSTransactionLineItem.get_required_columns())
    first = listed["rows"][0]
    assert required.issubset(first.keys())
    assert first["bill_number"]
    assert first["menu"]
    assert first["qty"] > 0
    assert isinstance(first["order_time"], str)


def test_list_normalized_sales_rejects_path_outside_data_dir(
    data_dir: Path, tmp_path: Path
) -> None:
    outside = tmp_path / "secret.xlsx"
    outside.write_bytes(b"not-excel")

    with patch("pathlib.Path.read_bytes") as read_bytes:
        with pytest.raises(PathNotAllowedError, match="outside"):
            list_normalized_sales(str(outside), data_dir=data_dir)
        read_bytes.assert_not_called()


def test_esb_fixture_get_menu_heatmaps(data_dir: Path) -> None:
    assert ESB_DEMO.is_file()
    result = get_menu_heatmaps(ESB_REL, data_dir=data_dir)

    assert result["pos"] == "esb"
    assert result["menu_count"] == len(result["heatmaps"])
    assert result["menu_count"] > 0
    assert set(result.keys()) == {"pos", "menu_count", "heatmaps"}

    first = result["heatmaps"][0]
    assert set(first.keys()) == {
        "menu",
        "menu_category",
        "menu_category_detail",
        "daily_heatmap",
        "weekly_heatmap",
        "reporting_period",
    }
    assert first["menu"]
    assert len(first["daily_heatmap"]) == 24
    assert [row["hour"] for row in first["daily_heatmap"]] == list(range(24))
    assert [row["day"] for row in first["weekly_heatmap"]] == list(WEEKDAY_ORDER)
    assert first["reporting_period"]
    assert "bill_number" not in str(result)


def test_get_menu_heatmaps_rejects_path_outside_data_dir(
    data_dir: Path, tmp_path: Path
) -> None:
    outside = tmp_path / "secret.xlsx"
    outside.write_bytes(b"not-excel")

    with patch("pathlib.Path.read_bytes") as read_bytes:
        with pytest.raises(PathNotAllowedError, match="outside"):
            get_menu_heatmaps(str(outside), data_dir=data_dir)
        read_bytes.assert_not_called()
