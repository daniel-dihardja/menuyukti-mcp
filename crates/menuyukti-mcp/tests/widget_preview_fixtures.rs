//! Fixture shape checks for the local widget preview harness.

use serde_json::Value;
use std::path::PathBuf;

fn preview_fixtures_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("preview/fixtures")
}

fn read_fixture(name: &str) -> Value {
    let path = preview_fixtures_dir().join(name);
    let raw = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("read {}: {err}", path.display()));
    serde_json::from_str(&raw).unwrap_or_else(|err| panic!("parse {}: {err}", path.display()))
}

#[test]
fn hello_world_fixture_has_message_and_source() {
    let v = read_fixture("hello_world.json");
    assert_eq!(v["message"], "Hello World");
    assert_eq!(v["source"], "mock");
}

#[test]
fn venue_slot_strength_fixture_has_35_cells() {
    let v = read_fixture("venue_slot_strength.json");
    assert_eq!(v["slot_count"], 35);
    let profile = v["slot_demand_profile"]
        .as_array()
        .expect("slot_demand_profile array");
    assert_eq!(profile.len(), 35);
    let first = &profile[0];
    assert!(first["day"].is_string());
    assert!(first["meal_period"].is_string());
    assert!(first["demand_index"].is_number());
    assert!(first["relative_demand"].is_string());
}

#[test]
fn venue_slot_strength_empty_fixture_has_empty_profile() {
    let v = read_fixture("venue_slot_strength_empty.json");
    assert_eq!(v["slot_count"], 0);
    assert_eq!(
        v["slot_demand_profile"].as_array().map(|a| a.len()),
        Some(0)
    );
}

#[test]
fn pair_lift_matrix_fixture_is_square_with_null_diagonal() {
    let v = read_fixture("pair_lift_matrix.json");
    let focus = v["focus_menus"].as_array().expect("focus_menus array");
    let matrix = v["matrix_lift"].as_array().expect("matrix_lift array");
    assert!(focus.len() >= 2, "expected >=2 focus menus");
    assert_eq!(matrix.len(), focus.len());
    for (i, row) in matrix.iter().enumerate() {
        let cells = row.as_array().expect("matrix row");
        assert_eq!(cells.len(), focus.len());
        assert!(cells[i].is_null(), "diagonal at {i} must be null");
    }
    assert!(v["pairs"]
        .as_array()
        .map(|a| !a.is_empty())
        .unwrap_or(false));
    assert!(v["scope"].is_string());
}

#[test]
fn pair_lift_matrix_empty_fixture_has_empty_focus() {
    let v = read_fixture("pair_lift_matrix_empty.json");
    assert_eq!(v["focus_menus"].as_array().map(|a| a.len()), Some(0));
    assert_eq!(v["matrix_lift"].as_array().map(|a| a.len()), Some(0));
    assert_eq!(v["pairs"].as_array().map(|a| a.len()), Some(0));
}

#[test]
fn menu_heatmaps_fixture_has_daily_weekly_and_categories() {
    let v = read_fixture("menu_heatmaps.json");
    let heatmaps = v["heatmaps"].as_array().expect("heatmaps array");
    assert!(!heatmaps.is_empty(), "expected populated heatmaps");
    assert_eq!(v["menu_count"].as_u64(), Some(heatmaps.len() as u64));
    let first = &heatmaps[0];
    assert!(first["menu"].is_string());
    assert_eq!(first["daily_heatmap"].as_array().map(|a| a.len()), Some(24));
    assert_eq!(first["weekly_heatmap"].as_array().map(|a| a.len()), Some(7));
    let cats: std::collections::HashSet<_> = heatmaps
        .iter()
        .filter_map(|item| item["matrix_category"].as_str())
        .collect();
    for required in ["star", "plow_horse", "puzzle", "weak"] {
        assert!(
            cats.contains(required),
            "fixture missing matrix_category={required}; cats={cats:?}"
        );
    }
}

#[test]
fn menu_heatmaps_empty_fixture_has_empty_list() {
    let v = read_fixture("menu_heatmaps_empty.json");
    assert_eq!(v["menu_count"], 0);
    assert_eq!(v["heatmaps"].as_array().map(|a| a.len()), Some(0));
}
