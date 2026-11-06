use std::path::PathBuf;

use menuyukti::{
    apply_matrix_categories, calculate_menu_heatmaps, compute_menu_basket_affinities_from_orders,
    compute_menu_engineering_from_orders, compute_slot_demand_profile_from_orders,
    list_normalized_rows, load_normalized_sales, load_workspace_cogs, summarize_sales, BasketScope,
};

fn sales_reports() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../workspaces")
        .canonicalize()
        .expect("workspaces")
}

fn fixtures() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures/pos-sales-reports")
        .canonicalize()
        .expect("fixtures")
}

#[test]
fn summarizes_demo_cafe_sales() {
    let fixture = "demo-cafe-july-2026/sales.xlsx";
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

    let via_workspace = load_normalized_sales("demo-cafe-july-2026", Some(&sales_reports()))
        .unwrap_or_else(|err| {
            panic!("fixture=demo-cafe-july-2026 tool=load_normalized_sales error={err}");
        });
    assert_eq!(via_workspace.pos, "esb");
    assert_eq!(via_workspace.rows.len(), loaded.rows.len());
    assert!(
        (summary.total_revenue - 29_795_300.000_2).abs() < 0.01,
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
    assert_eq!(
        summary.top_items.len(),
        10,
        "fixture={fixture} field=top_items"
    );
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
    assert!(
        heatmaps.iter().all(|h| h.matrix_category.is_none()),
        "sales-only heatmaps must leave matrix_category unset; fixture={fixture}"
    );
}

#[test]
fn heatmaps_matrix_categories_match_menu_engineering() {
    let root = sales_reports();
    let sales = load_normalized_sales("demo-cafe-july-2026", Some(&root)).unwrap();
    let cogs = load_workspace_cogs("demo-cafe-july-2026", Some(&root)).unwrap();
    let matrix = compute_menu_engineering_from_orders(&sales.rows, &cogs.unit_costs).unwrap();
    let expected: std::collections::HashMap<_, _> = matrix
        .items
        .iter()
        .map(|item| (item.menu.clone(), item.category.clone()))
        .collect();

    let mut heatmaps = calculate_menu_heatmaps(&sales.rows).unwrap();
    apply_matrix_categories(&mut heatmaps, &expected);

    let mut matched = 0;
    for item in &heatmaps {
        let Some(category) = item.matrix_category.as_deref() else {
            continue;
        };
        matched += 1;
        assert_eq!(
            expected.get(&item.menu).map(String::as_str),
            Some(category),
            "menu={} heatmap matrix_category mismatch",
            item.menu
        );
        assert!(
            matches!(category, "star" | "plow_horse" | "puzzle" | "weak"),
            "menu={} unexpected matrix_category={category}",
            item.menu
        );
    }
    assert_eq!(
        matched,
        expected.len(),
        "expected every matrix menu to appear on a heatmap row"
    );
    assert!(matched > 0, "expected enriched heatmap categories");
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

#[test]
fn workspace_folder_without_config_errors_clearly() {
    let tmp = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(tmp.path().join("empty-cafe")).unwrap();
    let err = load_normalized_sales("empty-cafe", Some(tmp.path()))
        .unwrap_err()
        .to_string();
    assert!(
        err.contains("Workspace config not found") || err.contains("workspace.yaml"),
        "unexpected error={err}"
    );
}

#[test]
fn menu_engineering_for_demo_cafe() {
    let root = sales_reports();
    let sales = load_normalized_sales("demo-cafe-july-2026", Some(&root)).unwrap();
    let cogs = load_workspace_cogs("demo-cafe-july-2026", Some(&root)).unwrap();
    let matrix = compute_menu_engineering_from_orders(&sales.rows, &cogs.unit_costs).unwrap();
    assert!(!matrix.items.is_empty(), "expected matrix items");
    assert!(
        matrix.coverage.matched_menus > 0,
        "expected matched COGS coverage"
    );
    let cats: std::collections::HashSet<_> =
        matrix.items.iter().map(|i| i.category.as_str()).collect();
    for required in ["star", "plow_horse", "puzzle", "weak"] {
        assert!(
            cats.contains(required),
            "demo cafe missing category {required}; got {cats:?}"
        );
    }
}

#[test]
fn pair_lift_for_demo_cafe() {
    let root = sales_reports();
    let sales = load_normalized_sales("demo-cafe-july-2026", Some(&root)).unwrap();
    let without_focus = compute_menu_basket_affinities_from_orders(&sales.rows, None);
    assert!(
        without_focus.focus_menus.len() >= 2,
        "expected focus menus; got {:?}",
        without_focus.focus_menus
    );
    assert_eq!(without_focus.scope, BasketScope::TopByPresence);
    assert_eq!(
        without_focus.matrix_lift.len(),
        without_focus.focus_menus.len()
    );
    assert!(without_focus.matrix_lift[0][0].is_none());

    let cogs = load_workspace_cogs("demo-cafe-july-2026", Some(&root)).unwrap();
    let matrix = compute_menu_engineering_from_orders(&sales.rows, &cogs.unit_costs).unwrap();
    let stars: Vec<String> = matrix
        .items
        .iter()
        .filter(|i| i.category == "star")
        .map(|i| i.menu.clone())
        .collect();
    assert!(
        stars.len() >= 2,
        "demo cafe expected >=2 stars for pair-lift focus; got {stars:?}"
    );
    let with_stars = compute_menu_basket_affinities_from_orders(&sales.rows, Some(&stars));
    assert_eq!(with_stars.scope, BasketScope::Stars);
    assert_eq!(
        with_stars.focus_menus,
        stars[..stars.len().min(12)].to_vec()
    );
}

#[test]
fn venue_slot_strength_for_demo_cafe() {
    let root = sales_reports();
    let sales = load_normalized_sales("demo-cafe-july-2026", Some(&root)).unwrap();
    let profile = compute_slot_demand_profile_from_orders(&sales.rows);
    assert_eq!(profile.len(), 35, "expected 35 day×meal cells");
    assert!(profile.iter().any(|c| c.order_count > 0));
    let relatives: std::collections::HashSet<_> =
        profile.iter().map(|c| c.relative_demand.as_str()).collect();
    assert!(
        relatives.contains("low") || relatives.contains("high") || relatives.contains("average"),
        "expected relative_demand labels; got {relatives:?}"
    );
}
