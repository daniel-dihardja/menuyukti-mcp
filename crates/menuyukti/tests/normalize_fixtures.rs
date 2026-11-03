use std::path::PathBuf;

use menuyukti::{detect_and_normalize, detect_pos_from_excel_bytes, load_normalized_sales};
use rust_xlsxwriter::Workbook;

fn fixtures_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures/pos-sales-reports")
        .canonicalize()
        .expect("fixtures path")
}

#[test]
fn detects_and_normalizes_esb_demo() {
    let path = fixtures_root().join("esb/SalesRecapitulationDetailReport_DEMO.xlsx");
    let bytes = std::fs::read(&path).expect("read esb fixture");
    assert_eq!(detect_pos_from_excel_bytes(&bytes).unwrap(), Some("esb"));

    let (pos, rows) = detect_and_normalize(&bytes).unwrap();
    assert_eq!(pos, "esb");
    assert_eq!(rows.len(), 9, "ESB demo fixture has 9 line items");
    assert!(rows.iter().all(|r| !r.menu.is_empty() && r.qty > 0));
    assert!(rows.iter().any(|r| r.total_after_bill_discount > 0.0));
}

#[test]
fn detects_and_normalizes_moka_demo() {
    let path = fixtures_root().join("moka/ItemDetails_DEMO.xlsx");
    let bytes = std::fs::read(&path).expect("read moka fixture");
    assert_eq!(detect_pos_from_excel_bytes(&bytes).unwrap(), Some("moka"));

    let (pos, rows) = detect_and_normalize(&bytes).unwrap();
    assert_eq!(pos, "moka");
    assert!(rows.len() >= 5, "expected Moka rows, got {}", rows.len());
    assert!(rows
        .iter()
        .all(|r| r.qty > 0 && r.total_after_bill_discount > 0.0));
}

#[test]
fn detects_and_normalizes_synthetic_quino() {
    let bytes = build_quino_xlsx();
    assert_eq!(detect_pos_from_excel_bytes(&bytes).unwrap(), Some("quino"));

    let (pos, rows) = detect_and_normalize(&bytes).unwrap();
    assert_eq!(pos, "quino");
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].menu, "Flat White");
    assert!((rows[0].total_after_bill_discount - 40.0).abs() < 1e-9);
}

#[test]
fn unknown_pos_without_mapping_errors_clearly() {
    let bytes = build_unknown_xlsx();
    let err = detect_and_normalize(&bytes).unwrap_err().to_string();
    assert!(
        err.contains("Unsupported POS system detected"),
        "unexpected error: {err}"
    );
    assert!(err.contains("pos-column-mapping"));
}

#[test]
fn load_normalized_sales_gates_and_loads_esb() {
    let root = fixtures_root();
    let loaded =
        load_normalized_sales("esb/SalesRecapitulationDetailReport_DEMO.xlsx", Some(&root))
            .unwrap();
    assert_eq!(loaded.pos, "esb");
    assert!(!loaded.rows.is_empty());
}

fn build_quino_xlsx() -> Vec<u8> {
    let mut workbook = Workbook::new();
    let sheet = workbook.add_worksheet();
    sheet
        .write_string(0, 0, "Transaction Item Detail Report")
        .unwrap();
    sheet.write_string(1, 0, "Demo").unwrap();
    sheet.write_string(2, 0, "meta").unwrap();
    let headers = [
        "Inv No.",
        "Name",
        "Qty",
        "Subtotal",
        "Discount",
        "Order Time",
        "Department",
        "Category",
    ];
    for (col, header) in headers.iter().enumerate() {
        sheet.write_string(3, col as u16, *header).unwrap();
    }
    sheet.write_string(4, 0, "INV-1").unwrap();
    sheet.write_string(4, 1, "Flat White").unwrap();
    sheet.write_number(4, 2, 2.0).unwrap();
    sheet.write_number(4, 3, 50.0).unwrap();
    sheet.write_number(4, 4, 10.0).unwrap();
    sheet.write_string(4, 5, "2026-07-01 08:12:00").unwrap();
    sheet.write_string(4, 6, "Drink").unwrap();
    sheet.write_string(4, 7, "Coffee").unwrap();

    sheet.write_string(5, 0, "INV-2").unwrap();
    sheet.write_string(5, 1, "Croissant").unwrap();
    sheet.write_number(5, 2, 1.0).unwrap();
    sheet.write_number(5, 3, 30.0).unwrap();
    sheet.write_number(5, 4, 0.0).unwrap();
    sheet.write_string(5, 5, "2026-07-01 09:00:00").unwrap();
    sheet.write_string(5, 6, "Food").unwrap();
    sheet.write_string(5, 7, "Bakery").unwrap();

    workbook.save_to_buffer().unwrap()
}

fn build_unknown_xlsx() -> Vec<u8> {
    let mut workbook = Workbook::new();
    let sheet = workbook.add_worksheet();
    sheet.write_string(0, 0, "Mystery POS Export").unwrap();
    sheet.write_string(1, 0, "ColA").unwrap();
    sheet.write_string(1, 1, "ColB").unwrap();
    sheet.write_string(2, 0, "x").unwrap();
    sheet.write_string(2, 1, "y").unwrap();
    workbook.save_to_buffer().unwrap()
}

#[test]
fn load_demo_cafe_sales_reports_esb() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../sales-reports")
        .canonicalize()
        .expect("sales-reports");
    let loaded = load_normalized_sales("demo-cafe-sales-july-2026.xlsx", Some(&root)).unwrap();
    assert_eq!(loaded.pos, "esb");
    assert!(loaded.rows.len() > 1000, "got {}", loaded.rows.len());
}
