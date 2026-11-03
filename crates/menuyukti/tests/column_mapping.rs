use std::fs;

use menuyukti::{load_column_mapping, load_normalized_sales, normalize_mapped_excel};
use rust_xlsxwriter::Workbook;
use serde_json::json;

#[test]
fn loads_mapped_custom_export() {
    let dir = tempfile::tempdir().unwrap();
    let xlsx = dir.path().join("custom.xlsx");
    fs::write(&xlsx, build_custom_xlsx()).unwrap();

    let mapping_path = dir.path().join("pos-column-mapping.json");
    fs::write(
        &mapping_path,
        serde_json::to_string_pretty(&json!({
            "name": "My POS!",
            "skip_rows": 0,
            "columns": {
                "bill_number": "Order ID",
                "menu": "Item Name",
                "qty": "Qty",
                "price": "Unit Price",
                "total_after_bill_discount": "Net Sales",
                "order_time": "Sold At",
                "menu_category": "Category",
                "menu_category_detail": "Subcategory"
            }
        }))
        .unwrap(),
    )
    .unwrap();

    let mapping = load_column_mapping(&mapping_path).unwrap();
    assert_eq!(mapping.name, "my-pos");

    let rows = normalize_mapped_excel(&fs::read(&xlsx).unwrap(), &mapping).unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].menu, "Latte");

    let loaded = load_normalized_sales("custom.xlsx", Some(dir.path())).unwrap();
    assert_eq!(loaded.pos, "my-pos");
    assert_eq!(loaded.rows.len(), 1);
}

#[test]
fn rejects_incomplete_mapping() {
    let dir = tempfile::tempdir().unwrap();
    let mapping_path = dir.path().join("pos-column-mapping.json");
    fs::write(
        &mapping_path,
        serde_json::to_string_pretty(&json!({
            "columns": {
                "bill_number": "Order ID",
                "menu": "Item Name"
            }
        }))
        .unwrap(),
    )
    .unwrap();

    let err = load_column_mapping(&mapping_path).unwrap_err().to_string();
    assert!(err.contains("missing required fields"), "{err}");
}

#[test]
fn rejects_placeholder_mapping() {
    let dir = tempfile::tempdir().unwrap();
    let mapping_path = dir.path().join("pos-column-mapping.json");
    fs::write(
        &mapping_path,
        serde_json::to_string_pretty(&json!({
            "columns": {
                "bill_number": "<YOUR_ORDER_ID_COLUMN>",
                "menu": "Item Name",
                "qty": "Qty",
                "price": "Unit Price",
                "total_after_bill_discount": "Net Sales",
                "order_time": "Sold At",
                "menu_category": "Category",
                "menu_category_detail": "Subcategory"
            }
        }))
        .unwrap(),
    )
    .unwrap();

    let err = load_column_mapping(&mapping_path).unwrap_err().to_string();
    assert!(err.contains("placeholder"), "{err}");
}

fn build_custom_xlsx() -> Vec<u8> {
    let mut workbook = Workbook::new();
    let sheet = workbook.add_worksheet();
    let headers = [
        "Order ID",
        "Item Name",
        "Qty",
        "Unit Price",
        "Net Sales",
        "Sold At",
        "Category",
        "Subcategory",
    ];
    for (col, header) in headers.iter().enumerate() {
        sheet.write_string(0, col as u16, *header).unwrap();
    }
    sheet.write_string(1, 0, "B-1").unwrap();
    sheet.write_string(1, 1, "Latte").unwrap();
    sheet.write_number(1, 2, 1.0).unwrap();
    sheet.write_number(1, 3, 4.5).unwrap();
    sheet.write_number(1, 4, 4.5).unwrap();
    sheet.write_string(1, 5, "2026-07-01 10:00:00").unwrap();
    sheet.write_string(1, 6, "Drink").unwrap();
    sheet.write_string(1, 7, "Coffee").unwrap();
    workbook.save_to_buffer().unwrap()
}
