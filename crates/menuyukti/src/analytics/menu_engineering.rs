//! Menu engineering matrix (star / plow_horse / puzzle / weak).

use std::collections::{HashMap, HashSet};

use serde::Serialize;

use crate::line_item::LineItem;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct MenuEngineeringThresholds {
    pub avg_popularity: f64,
    pub avg_contribution_margin: f64,
    pub total_cogs: f64,
    pub total_profit: f64,
    pub total_margin: f64,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct MenuEngineeringDistributionItem {
    pub category: String,
    pub item_count: usize,
    pub item_share: f64,
    pub margin_share: f64,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct MenuEngineeringMatrixItem {
    pub menu: String,
    pub quantity: i64,
    pub total_revenue: f64,
    pub cogs: f64,
    pub total_cogs: f64,
    pub contribution_margin: f64,
    pub contribution_margin_percentage: f64,
    pub margin_per_unit: f64,
    pub we_value: f64,
    pub category: String,
    pub action: String,
    pub menu_category: Option<String>,
    pub menu_category_detail: Option<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct MenuEngineeringCoverage {
    /// Distinct sold menus that joined a usable unit cost (cogs > 0).
    pub matched_menus: usize,
    /// Distinct sold menus missing from the COGS map or with non-positive cost.
    pub unmatched_menus: Vec<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct MenuEngineeringMatrixResult {
    pub thresholds: MenuEngineeringThresholds,
    pub distribution: Vec<MenuEngineeringDistributionItem>,
    pub items: Vec<MenuEngineeringMatrixItem>,
    pub coverage: MenuEngineeringCoverage,
}

#[derive(Debug, Clone)]
struct AggRow {
    menu: String,
    quantity: f64,
    total_revenue: f64,
    cogs: f64,
    menu_category: Option<String>,
    menu_category_detail: Option<String>,
}

/// Aggregate order lines, attach COGS, and classify the menu engineering matrix.
///
/// Items without usable COGS (`cogs <= 0`) are excluded from matrix membership
/// and listed under [`MenuEngineeringCoverage::unmatched_menus`].
pub fn compute_menu_engineering_from_orders(
    rows: &[LineItem],
    cogs_by_menu: &HashMap<String, f64>,
) -> Result<MenuEngineeringMatrixResult, String> {
    if rows.is_empty() {
        return Err("Sales rows are empty. Cannot calculate menu engineering.".into());
    }
    if cogs_by_menu.is_empty() {
        return Err(
            "COGS map is empty. Add Menu/COGS unit costs before running menu engineering.".into(),
        );
    }

    let mut by_menu: HashMap<String, AggRow> = HashMap::new();
    for row in rows {
        let entry = by_menu.entry(row.menu.clone()).or_insert_with(|| AggRow {
            menu: row.menu.clone(),
            quantity: 0.0,
            total_revenue: 0.0,
            cogs: 0.0,
            menu_category: None,
            menu_category_detail: None,
        });
        entry.quantity += row.qty as f64;
        entry.total_revenue += row.total_after_bill_discount;
        if entry.menu_category.is_none() && !row.menu_category.is_empty() {
            entry.menu_category = Some(row.menu_category.clone());
        }
        if entry.menu_category_detail.is_none() && !row.menu_category_detail.is_empty() {
            entry.menu_category_detail = Some(row.menu_category_detail.clone());
        }
    }

    let mut unmatched = HashSet::new();
    for agg in by_menu.values_mut() {
        let unit = cogs_by_menu.get(&agg.menu).copied().unwrap_or(0.0);
        if unit > 0.0 && unit.is_finite() {
            agg.cogs = unit;
        } else {
            agg.cogs = 0.0;
            unmatched.insert(agg.menu.clone());
        }
    }

    let mut unmatched_menus: Vec<String> = unmatched.into_iter().collect();
    unmatched_menus.sort();

    calculate_menu_engineering_matrix(by_menu.into_values().collect(), unmatched_menus)
}

fn calculate_menu_engineering_matrix(
    mut rows: Vec<AggRow>,
    unmatched_menus: Vec<String>,
) -> Result<MenuEngineeringMatrixResult, String> {
    let matched_menus = rows.iter().filter(|r| r.cogs > 0.0).count();

    let total_revenue_all: f64 = rows.iter().map(|r| r.total_revenue).sum();
    let total_cogs_all: f64 = rows.iter().map(|r| r.cogs * r.quantity).sum();
    let total_profit_all = total_revenue_all - total_cogs_all;
    let total_margin_ratio = if total_revenue_all > 0.0 {
        total_profit_all / total_revenue_all
    } else {
        0.0
    };

    rows.retain(|r| r.cogs > 0.0 && r.total_revenue > 0.0);
    if rows.is_empty() {
        return Err(format!(
            "No valid menu items with cogs > 0 and revenue > 0 \
             (unmatched menus: {}). Add or fix COGS for sold items.",
            if unmatched_menus.is_empty() {
                "none".into()
            } else {
                unmatched_menus.join(", ")
            }
        ));
    }

    #[derive(Clone)]
    struct MatrixRow {
        menu: String,
        quantity: f64,
        total_revenue: f64,
        cogs: f64,
        total_cogs: f64,
        contribution_margin: f64,
        contribution_margin_percentage: f64,
        margin_per_unit: f64,
        we_value: f64,
        category: String,
        action: String,
        menu_category: Option<String>,
        menu_category_detail: Option<String>,
    }

    let mut matrix: Vec<MatrixRow> = rows
        .into_iter()
        .map(|r| {
            let total_cogs = r.cogs * r.quantity;
            let contribution_margin = r.total_revenue - total_cogs;
            let we_value = if r.total_revenue > 0.0 {
                total_cogs / r.total_revenue
            } else {
                0.0
            };
            let margin_per_unit = if r.quantity > 0.0 {
                contribution_margin / r.quantity
            } else {
                0.0
            };
            MatrixRow {
                menu: r.menu,
                quantity: r.quantity,
                total_revenue: r.total_revenue,
                cogs: r.cogs,
                total_cogs,
                contribution_margin,
                contribution_margin_percentage: 0.0,
                margin_per_unit,
                we_value,
                category: String::new(),
                action: String::new(),
                menu_category: r.menu_category,
                menu_category_detail: r.menu_category_detail,
            }
        })
        .collect();

    let total_margin_matrix: f64 = matrix.iter().map(|r| r.contribution_margin).sum();
    for row in &mut matrix {
        row.contribution_margin_percentage = if total_margin_matrix > 0.0 {
            row.contribution_margin / total_margin_matrix
        } else {
            0.0
        };
    }

    let avg_popularity = matrix.iter().map(|r| r.quantity).sum::<f64>() / matrix.len() as f64;
    let avg_margin =
        matrix.iter().map(|r| r.contribution_margin).sum::<f64>() / matrix.len() as f64;

    for row in &mut matrix {
        let popular = row.quantity >= avg_popularity;
        let profitable = row.contribution_margin >= avg_margin;
        row.category = match (popular, profitable) {
            (true, true) => "star",
            (true, false) => "plow_horse",
            (false, true) => "puzzle",
            (false, false) => "weak",
        }
        .to_string();
        row.action = classify_action(
            &row.category,
            row.contribution_margin_percentage,
            row.margin_per_unit,
            row.quantity,
            avg_margin,
            avg_popularity,
        );
    }

    let total_items = matrix.len();
    let mut distribution: Vec<MenuEngineeringDistributionItem> = Vec::new();
    for category in ["star", "plow_horse", "puzzle", "weak"] {
        let group: Vec<&MatrixRow> = matrix.iter().filter(|r| r.category == category).collect();
        if group.is_empty() {
            continue;
        }
        let count = group.len();
        let category_margin: f64 = group.iter().map(|r| r.contribution_margin).sum();
        distribution.push(MenuEngineeringDistributionItem {
            category: category.to_string(),
            item_count: count,
            item_share: count as f64 / total_items as f64,
            margin_share: if total_margin_matrix > 0.0 {
                category_margin / total_margin_matrix
            } else {
                0.0
            },
        });
    }
    distribution.sort_by(|a, b| {
        b.margin_share
            .partial_cmp(&a.margin_share)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| {
                b.item_share
                    .partial_cmp(&a.item_share)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .then_with(|| a.category.cmp(&b.category))
    });

    matrix.sort_by(|a, b| {
        b.quantity
            .partial_cmp(&a.quantity)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| {
                b.total_revenue
                    .partial_cmp(&a.total_revenue)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .then_with(|| a.menu.cmp(&b.menu))
    });

    let items: Vec<MenuEngineeringMatrixItem> = matrix
        .into_iter()
        .map(|row| MenuEngineeringMatrixItem {
            menu: row.menu,
            quantity: row.quantity.round() as i64,
            total_revenue: round2(row.total_revenue),
            cogs: round2(row.cogs),
            total_cogs: round2(row.total_cogs),
            contribution_margin: round2(row.contribution_margin),
            contribution_margin_percentage: round4(row.contribution_margin_percentage),
            margin_per_unit: round2(row.margin_per_unit),
            we_value: round4(row.we_value),
            category: row.category,
            action: row.action,
            menu_category: row.menu_category,
            menu_category_detail: row.menu_category_detail,
        })
        .collect();

    Ok(MenuEngineeringMatrixResult {
        thresholds: MenuEngineeringThresholds {
            avg_popularity: round2(avg_popularity),
            avg_contribution_margin: round2(avg_margin),
            total_cogs: round2(total_cogs_all),
            total_profit: round2(total_profit_all),
            total_margin: round4(total_margin_ratio),
        },
        distribution,
        items,
        coverage: MenuEngineeringCoverage {
            matched_menus,
            unmatched_menus,
        },
    })
}

fn classify_action(
    category: &str,
    contribution_margin_percentage: f64,
    margin_per_unit: f64,
    quantity: f64,
    avg_margin: f64,
    avg_popularity: f64,
) -> String {
    let remove =
        category == "weak" && contribution_margin_percentage < 0.005 && quantity < avg_popularity;
    let reprice = category == "weak" && margin_per_unit >= avg_margin && !remove;
    let promote = category == "puzzle" && !remove && !reprice;
    if promote {
        "promote".into()
    } else if reprice {
        "reprice".into()
    } else if remove {
        "remove".into()
    } else {
        "keep".into()
    }
}

fn round2(v: f64) -> f64 {
    (v * 100.0).round() / 100.0
}

fn round4(v: f64) -> f64 {
    (v * 10_000.0).round() / 10_000.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn item(menu: &str, qty: i64, revenue: f64) -> LineItem {
        LineItem {
            bill_number: "1".into(),
            menu: menu.into(),
            qty,
            price: revenue / qty as f64,
            total_after_bill_discount: revenue,
            order_time: NaiveDate::from_ymd_opt(2026, 7, 1)
                .unwrap()
                .and_hms_opt(12, 0, 0)
                .unwrap(),
            menu_category: "DRINK".into(),
            menu_category_detail: "TEST".into(),
        }
    }

    /// Crafted so each quadrant is occupied:
    /// avg qty = 25, avg CM = 1500.
    /// star: high qty + high CM; plow_horse: high qty + low CM;
    /// puzzle: low qty + high CM; weak: low qty + low CM.
    fn quadrant_fixture() -> (Vec<LineItem>, HashMap<String, f64>) {
        let rows = vec![
            item("Star Drink", 40, 4_000.0),
            item("Plow Horse", 40, 2_000.0),
            item("Puzzle Drink", 10, 3_000.0),
            item("Weak Drink", 10, 1_000.0),
        ];
        let mut cogs = HashMap::new();
        cogs.insert("Star Drink".into(), 25.0); // CM = 4000 - 1000 = 3000
        cogs.insert("Plow Horse".into(), 40.0); // CM = 2000 - 1600 = 400
        cogs.insert("Puzzle Drink".into(), 50.0); // CM = 3000 - 500 = 2500
        cogs.insert("Weak Drink".into(), 80.0); // CM = 1000 - 800 = 200
        (rows, cogs)
    }

    #[test]
    fn classifies_all_four_categories() {
        let (rows, cogs) = quadrant_fixture();
        let result = compute_menu_engineering_from_orders(&rows, &cogs).unwrap();
        let mut by_cat: HashMap<String, String> = HashMap::new();
        for item in &result.items {
            by_cat.insert(item.menu.clone(), item.category.clone());
        }
        assert_eq!(by_cat.get("Star Drink").map(String::as_str), Some("star"));
        assert_eq!(
            by_cat.get("Plow Horse").map(String::as_str),
            Some("plow_horse")
        );
        assert_eq!(
            by_cat.get("Puzzle Drink").map(String::as_str),
            Some("puzzle")
        );
        assert_eq!(by_cat.get("Weak Drink").map(String::as_str), Some("weak"));
        assert_eq!(result.distribution.len(), 4);
        assert!(result.thresholds.avg_popularity > 0.0);
        assert!(result.items.iter().all(|i| !i.action.is_empty()));
        assert_eq!(
            by_cat.get("Puzzle Drink").and_then(|_| result
                .items
                .iter()
                .find(|i| i.menu == "Puzzle Drink")
                .map(|i| i.action.as_str())),
            Some("promote")
        );
    }

    #[test]
    fn excludes_missing_and_zero_cogs() {
        let rows = vec![
            item("Priced", 20, 2_000.0),
            item("Missing", 20, 2_000.0),
            item("ZeroCost", 20, 2_000.0),
        ];
        let mut cogs = HashMap::new();
        cogs.insert("Priced".into(), 10.0);
        cogs.insert("ZeroCost".into(), 0.0);
        let result = compute_menu_engineering_from_orders(&rows, &cogs).unwrap();
        assert_eq!(result.items.len(), 1);
        assert_eq!(result.items[0].menu, "Priced");
        assert_eq!(result.coverage.matched_menus, 1);
        assert_eq!(
            result.coverage.unmatched_menus,
            vec!["Missing".to_string(), "ZeroCost".to_string()]
        );
    }

    #[test]
    fn empty_sales_errors() {
        let cogs = HashMap::from([("A".into(), 1.0)]);
        let err = compute_menu_engineering_from_orders(&[], &cogs).unwrap_err();
        assert!(err.contains("empty"), "{err}");
    }

    #[test]
    fn empty_cogs_map_errors() {
        let rows = vec![item("A", 1, 10.0)];
        let err = compute_menu_engineering_from_orders(&rows, &HashMap::new()).unwrap_err();
        assert!(err.to_lowercase().contains("cogs"), "{err}");
    }
}
