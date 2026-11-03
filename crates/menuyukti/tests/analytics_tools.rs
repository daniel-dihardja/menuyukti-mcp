use std::path::PathBuf;

use menuyukti::{
    calculate_menu_heatmaps, list_normalized_rows, load_normalized_sales, summarize_sales,
};

fn sales_reports() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../sales-reports")
        .canonicalize()
        .expect("sales-reports")
}

fn fixtures() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures/pos-sales-reports")
        .canonicalize()
        .expect("fixtures")
}

#[test]
fn summarizes_demo_cafe_sales() {
    let fixture = "demo-cafe-sales-july-2026.xlsx";
    let loaded = load_normalized_sales(fixture, Some(&sales_reports())).unwrap_or_else(|err| {
        panic!("fixture={fixture} tool=load_normalized_sales error={err}");
    });
    let summary = summarize_sales(loaded.pos.clone(), &loaded.rows).unwrap_or_else(|err| {
        panic!("fixture={fixture} tool=summarize_sales error={err}");
    });
    assert_eq!(summary.pos, "esb", "fixture={fixture} field=pos");
    assert_eq!(
        summary.total_items_sold, 1748,
        "fixture={fixture} field=total_items_sold"
    );
    assert!(
        (summary.total_revenue - 29_795_300.0002).abs() < 0.01,
        "fixture={fixture} field=total_revenue got={}",
        summary.total_revenue
    );
    assert_eq!(
        summary.period_start.as_deref(),
        Some("2026-07-01"),
        "fixture={fixture} field=period_start"
    );
    assert_eq!(
        summary.period_end.as_deref(),
        Some("2026-07-31"),
        "fixture={fixture} field=period_end"
    );
    assert_eq!(summary.top_items.len(), 10, "fixture={fixture} field=top_items");
    assert_eq!(
        summary.top_items[0].menu, "Iced Milk Coffee Aren",
        "fixture={fixture} field=top_items[0].menu"
    );
    assert_eq!(
        summary.top_items[0].quantity, 167,
        "fixture={fixture} field=top_items[0].quantity"
    );

    let listed = list_normalized_rows(loaded.pos.clone(), &loaded.rows);
    assert_eq!(listed.row_count, 1441, "fixture={fixture} field=row_count");

    let heatmaps = calculate_menu_heatmaps(&loaded.rows).unwrap_or_else(|err| {
        panic!("fixture={fixture} tool=get_menu_heatmaps error={err}");
    });
    assert_eq!(heatmaps.len(), 98, "fixture={fixture} field=menu_count");
    assert_eq!(
        heatmaps[0].daily_heatmap.len(),
        24,
        "fixture={fixture} field=daily_heatmap.len"
    );
    assert_eq!(
        heatmaps[0].weekly_heatmap.len(),
        7,
        "fixture={fixture} field=weekly_heatmap.len"
    );
}

#[test]
fn lists_normalized_esb_demo_rows() {
    let fixture = "esb/SalesRecapitulationDetailReport_DEMO.xlsx";
    let loaded = load_normalized_sales(fixture, Some(&fixtures())).unwrap();
    let listed = list_normalized_rows(loaded.pos, &loaded.rows);
    assert_eq!(listed.pos, "esb");
    assert_eq!(listed.row_count, 9, "fixture={fixture}");
    assert!(listed.rows[0].get("menu").is_some());
}

#[test]
fn heatmaps_for_esb_demo() {
    let fixture = "esb/SalesRecapitulationDetailReport_DEMO.xlsx";
    let loaded = load_normalized_sales(fixture, Some(&fixtures())).unwrap();
    let heatmaps = calculate_menu_heatmaps(&loaded.rows).unwrap();
    assert!(!heatmaps.is_empty(), "fixture={fixture}");
    assert_eq!(heatmaps[0].daily_heatmap.len(), 24);
    assert_eq!(heatmaps[0].weekly_heatmap.len(), 7);
}

#[test]
fn summarize_rejects_outside_path() {
    let err = load_normalized_sales("/tmp/not-in-data-dir.xlsx", Some(&fixtures()))
        .unwrap_err()
        .to_string();
    assert!(
        err.contains("outside") || err.contains("MENUYUKTI_DATA_DIR"),
        "tool=summarize_local_sales unexpected error={err}"
    );
}
