//! Market-basket affinity: co-occurrence, support, confidence, and lift per menu pair.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use serde::Serialize;

use crate::line_item::LineItem;

pub const MAX_FOCUS_ITEMS: usize = 12;
pub const MAX_TOP_PAIRS: usize = 25;
pub const MIN_CO_OCCURRENCE: i64 = 3;

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BasketScope {
    Stars,
    TopByPresence,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct MenuBasketPair {
    pub menu_a: String,
    pub menu_b: String,
    pub co_order_count: i64,
    pub support: f64,
    pub confidence_a_to_b: f64,
    pub confidence_b_to_a: f64,
    pub lift: f64,
    pub menu_a_category: Option<String>,
    pub menu_b_category: Option<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct MenuBasketAffinitiesResult {
    pub total_orders: usize,
    pub multi_item_order_count: usize,
    pub avg_distinct_items_per_order: f64,
    pub scope: BasketScope,
    pub focus_menus: Vec<String>,
    pub pairs: Vec<MenuBasketPair>,
    pub matrix_lift: Vec<Vec<Option<f64>>>,
}

fn empty_result(scope: BasketScope) -> MenuBasketAffinitiesResult {
    MenuBasketAffinitiesResult {
        total_orders: 0,
        multi_item_order_count: 0,
        avg_distinct_items_per_order: 0.0,
        scope,
        focus_menus: Vec::new(),
        pairs: Vec::new(),
        matrix_lift: Vec::new(),
    }
}

fn round6(value: f64) -> f64 {
    (value * 1_000_000.0).round() / 1_000_000.0
}

fn round4(value: f64) -> f64 {
    (value * 10_000.0).round() / 10_000.0
}

fn category_label(raw: &str) -> Option<String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn orders_with_menus(rows: &[LineItem]) -> Vec<BTreeSet<String>> {
    let mut by_bill: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for row in rows {
        let bill = row.bill_number.trim();
        let menu = row.menu.trim();
        if bill.is_empty() || menu.is_empty() {
            continue;
        }
        by_bill
            .entry(bill.to_string())
            .or_default()
            .insert(menu.to_string());
    }
    by_bill
        .into_values()
        .filter(|menus| !menus.is_empty())
        .collect()
}

fn menu_category_by_menu(rows: &[LineItem]) -> HashMap<String, Option<String>> {
    let mut out: HashMap<String, Option<String>> = HashMap::new();
    for row in rows {
        let menu = row.menu.trim();
        if menu.is_empty() {
            continue;
        }
        out.entry(menu.to_string())
            .or_insert_with(|| category_label(&row.menu_category));
    }
    out
}

fn resolve_focus_menus(
    orders: &[BTreeSet<String>],
    focus_menus: Option<&[String]>,
) -> (Vec<String>, BasketScope) {
    if let Some(focus) = focus_menus {
        let mut seen = HashSet::new();
        let mut unique = Vec::new();
        for menu in focus {
            let cleaned = menu.trim();
            if cleaned.is_empty() || !seen.insert(cleaned.to_string()) {
                continue;
            }
            unique.push(cleaned.to_string());
        }
        if unique.len() >= 2 {
            unique.truncate(MAX_FOCUS_ITEMS);
            return (unique, BasketScope::Stars);
        }
    }

    let mut presence: HashMap<String, i64> = HashMap::new();
    for order_menus in orders {
        for menu in order_menus {
            *presence.entry(menu.clone()).or_insert(0) += 1;
        }
    }
    let mut ranked: Vec<(String, i64)> = presence.into_iter().collect();
    ranked.sort_by(|(a_menu, a_count), (b_menu, b_count)| {
        b_count.cmp(a_count).then_with(|| a_menu.cmp(b_menu))
    });
    let focus: Vec<String> = ranked
        .into_iter()
        .take(MAX_FOCUS_ITEMS)
        .map(|(menu, _)| menu)
        .collect();
    (focus, BasketScope::TopByPresence)
}

fn canonical_pair_key(menu_a: &str, menu_b: &str) -> (String, String) {
    if menu_a <= menu_b {
        (menu_a.to_string(), menu_b.to_string())
    } else {
        (menu_b.to_string(), menu_a.to_string())
    }
}

fn build_matrix_lift(
    focus: &[String],
    lift_by_pair: &HashMap<(String, String), f64>,
) -> Vec<Vec<Option<f64>>> {
    let n = focus.len();
    let mut grid = vec![vec![None; n]; n];
    for (i, menu_a) in focus.iter().enumerate() {
        for (j, menu_b) in focus.iter().enumerate() {
            if i == j {
                continue;
            }
            let key = canonical_pair_key(menu_a, menu_b);
            if let Some(lift) = lift_by_pair.get(&key) {
                grid[i][j] = Some(*lift);
            }
        }
    }
    grid
}

/// Compute basket affinity metrics from line-level order rows.
///
/// Uses distinct menu presence per bill (qty does not affect pairing).
pub fn compute_menu_basket_affinities_from_orders(
    rows: &[LineItem],
    focus_menus: Option<&[String]>,
) -> MenuBasketAffinitiesResult {
    if rows.is_empty() {
        return empty_result(BasketScope::TopByPresence);
    }

    let orders = orders_with_menus(rows);
    let total_orders = orders.len();
    if total_orders == 0 {
        return empty_result(BasketScope::TopByPresence);
    }

    let distinct_sum: usize = orders.iter().map(BTreeSet::len).sum();
    let avg_distinct = distinct_sum as f64 / total_orders as f64;
    let multi_item_order_count = orders.iter().filter(|o| o.len() >= 2).count();

    let (focus, scope) = resolve_focus_menus(&orders, focus_menus);
    if focus.len() < 2 {
        return MenuBasketAffinitiesResult {
            total_orders,
            multi_item_order_count,
            avg_distinct_items_per_order: round4(avg_distinct),
            scope,
            focus_menus: focus,
            pairs: Vec::new(),
            matrix_lift: Vec::new(),
        };
    }

    let focus_set: HashSet<&str> = focus.iter().map(String::as_str).collect();
    let menu_category = menu_category_by_menu(rows);

    let mut orders_with_menu: HashMap<String, i64> = HashMap::new();
    let mut pair_counts: HashMap<(String, String), i64> = HashMap::new();

    for order_menus in &orders {
        let mut in_focus: Vec<&str> = order_menus
            .iter()
            .filter(|m| focus_set.contains(m.as_str()))
            .map(String::as_str)
            .collect();
        in_focus.sort_unstable();
        for menu in &in_focus {
            *orders_with_menu.entry((*menu).to_string()).or_insert(0) += 1;
        }
        if in_focus.len() < 2 {
            continue;
        }
        for i in 0..in_focus.len() {
            for j in (i + 1)..in_focus.len() {
                let key = (in_focus[i].to_string(), in_focus[j].to_string());
                *pair_counts.entry(key).or_insert(0) += 1;
            }
        }
    }

    if multi_item_order_count == 0 {
        return MenuBasketAffinitiesResult {
            total_orders,
            multi_item_order_count: 0,
            avg_distinct_items_per_order: round4(avg_distinct),
            scope,
            focus_menus: focus.clone(),
            pairs: Vec::new(),
            matrix_lift: build_matrix_lift(&focus, &HashMap::new()),
        };
    }

    let mut pairs: Vec<MenuBasketPair> = Vec::new();
    let mut lift_by_pair: HashMap<(String, String), f64> = HashMap::new();

    for ((menu_a, menu_b), co_count) in &pair_counts {
        if *co_count < MIN_CO_OCCURRENCE {
            continue;
        }
        let orders_a = *orders_with_menu.get(menu_a).unwrap_or(&0);
        let orders_b = *orders_with_menu.get(menu_b).unwrap_or(&0);
        if orders_a == 0 || orders_b == 0 {
            continue;
        }
        let support = *co_count as f64 / multi_item_order_count as f64;
        let p_a = orders_a as f64 / total_orders as f64;
        let p_b = orders_b as f64 / total_orders as f64;
        let denominator = p_a * p_b;
        let lift = round6(if denominator > 0.0 {
            support / denominator
        } else {
            0.0
        });
        lift_by_pair.insert((menu_a.clone(), menu_b.clone()), lift);
        pairs.push(MenuBasketPair {
            menu_a: menu_a.clone(),
            menu_b: menu_b.clone(),
            co_order_count: *co_count,
            support: round6(support),
            confidence_a_to_b: round6(*co_count as f64 / orders_a as f64),
            confidence_b_to_a: round6(*co_count as f64 / orders_b as f64),
            lift,
            menu_a_category: menu_category.get(menu_a).cloned().flatten(),
            menu_b_category: menu_category.get(menu_b).cloned().flatten(),
        });
    }

    pairs.sort_by(|a, b| {
        b.lift
            .partial_cmp(&a.lift)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| b.co_order_count.cmp(&a.co_order_count))
            .then_with(|| a.menu_a.cmp(&b.menu_a))
            .then_with(|| a.menu_b.cmp(&b.menu_b))
    });
    pairs.truncate(MAX_TOP_PAIRS);

    let matrix_lift = build_matrix_lift(&focus, &lift_by_pair);

    MenuBasketAffinitiesResult {
        total_orders,
        multi_item_order_count,
        avg_distinct_items_per_order: round4(avg_distinct),
        scope,
        focus_menus: focus,
        pairs,
        matrix_lift,
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use super::*;

    fn item(bill: &str, menu: &str, category: &str) -> LineItem {
        LineItem {
            bill_number: bill.into(),
            menu: menu.into(),
            qty: 1,
            price: 1.0,
            total_after_bill_discount: 1.0,
            order_time: NaiveDate::from_ymd_opt(2026, 7, 1)
                .unwrap()
                .and_hms_opt(12, 0, 0)
                .unwrap(),
            menu_category: category.into(),
            menu_category_detail: "TEST".into(),
        }
    }

    #[test]
    fn empty_input_returns_structured_empty() {
        let result = compute_menu_basket_affinities_from_orders(&[], None);
        assert_eq!(result.total_orders, 0);
        assert!(result.pairs.is_empty());
        assert!(result.matrix_lift.is_empty());
        assert!(result.focus_menus.is_empty());
    }

    #[test]
    fn single_item_orders_produce_no_pairs() {
        let rows = vec![
            item("B1", "Burger", ""),
            item("B2", "Fries", ""),
            item("B3", "Salad", ""),
        ];
        let result = compute_menu_basket_affinities_from_orders(&rows, None);
        assert_eq!(result.total_orders, 3);
        assert_eq!(result.multi_item_order_count, 0);
        assert!(result.pairs.is_empty());
    }

    #[test]
    fn correlated_pair_has_lift_above_one() {
        let mut rows = Vec::new();
        for i in 0..10 {
            rows.push(item(&format!("B{i}"), "Burger", "Mains"));
            rows.push(item(&format!("B{i}"), "Fries", "Sides"));
        }
        for i in 10..20 {
            rows.push(item(&format!("B{i}"), "Salad", "Mains"));
        }

        let focus = vec![
            "Burger".to_string(),
            "Fries".to_string(),
            "Salad".to_string(),
        ];
        let result = compute_menu_basket_affinities_from_orders(&rows, Some(&focus));
        assert_eq!(result.scope, BasketScope::Stars);
        assert_eq!(result.multi_item_order_count, 10);
        let burger_fries = result
            .pairs
            .iter()
            .find(|p| p.menu_a == "Burger" && p.menu_b == "Fries")
            .expect("Burger-Fries pair");
        assert_eq!(burger_fries.co_order_count, 10);
        assert!(burger_fries.lift > 1.0, "lift={}", burger_fries.lift);
        assert_eq!(burger_fries.menu_a_category.as_deref(), Some("Mains"));
        assert_eq!(burger_fries.menu_b_category.as_deref(), Some("Sides"));
    }

    #[test]
    fn focus_menus_limits_pairs_to_focus_set() {
        let mut rows = Vec::new();
        for i in 0..MIN_CO_OCCURRENCE {
            rows.push(item(&format!("B{i}"), "Burger", ""));
            rows.push(item(&format!("B{i}"), "Fries", ""));
            rows.push(item(&format!("B{i}"), "Cola", ""));
        }
        let focus = vec!["Burger".to_string(), "Fries".to_string()];
        let result = compute_menu_basket_affinities_from_orders(&rows, Some(&focus));
        let menus_in_pairs: HashSet<&str> = result
            .pairs
            .iter()
            .flat_map(|p| [p.menu_a.as_str(), p.menu_b.as_str()])
            .collect();
        assert!(menus_in_pairs.is_subset(&HashSet::from(["Burger", "Fries"])));
        assert!(!menus_in_pairs.contains("Cola"));
    }

    #[test]
    fn top_by_presence_when_no_focus_menus() {
        let mut rows = Vec::new();
        for i in 0..MIN_CO_OCCURRENCE {
            rows.push(item(&format!("B{i}"), "Alpha", ""));
            rows.push(item(&format!("B{i}"), "Beta", ""));
        }
        let result = compute_menu_basket_affinities_from_orders(&rows, None);
        assert_eq!(result.scope, BasketScope::TopByPresence);
        assert!(result.focus_menus.contains(&"Alpha".to_string()));
        assert!(result.focus_menus.contains(&"Beta".to_string()));
    }

    #[test]
    fn matrix_lift_symmetric_with_null_diagonal() {
        let mut rows = Vec::new();
        for i in 0..MIN_CO_OCCURRENCE {
            rows.push(item(&format!("B{i}"), "A", ""));
            rows.push(item(&format!("B{i}"), "B", ""));
        }
        let focus = vec!["A".to_string(), "B".to_string()];
        let result = compute_menu_basket_affinities_from_orders(&rows, Some(&focus));
        let matrix = &result.matrix_lift;
        assert_eq!(matrix.len(), 2);
        assert_eq!(matrix[0].len(), 2);
        assert!(matrix[0][0].is_none());
        assert!(matrix[1][1].is_none());
        assert_eq!(matrix[0][1], matrix[1][0]);
        assert!(matrix[0][1].is_some());
    }

    #[test]
    fn matrix_lift_matches_pairs_when_focus_order_not_alphabetical() {
        let mut rows = Vec::new();
        for i in 0..MIN_CO_OCCURRENCE {
            rows.push(item(&format!("B{i}"), "Burger", ""));
            rows.push(item(&format!("B{i}"), "Apple", ""));
        }
        let focus = vec!["Burger".to_string(), "Apple".to_string()];
        let result = compute_menu_basket_affinities_from_orders(&rows, Some(&focus));
        let pair = result
            .pairs
            .iter()
            .find(|p| p.menu_a == "Apple" && p.menu_b == "Burger")
            .expect("Apple-Burger pair");
        let matrix = &result.matrix_lift;
        assert_eq!(matrix[0][1], Some(pair.lift));
        assert_eq!(matrix[1][0], Some(pair.lift));
    }

    #[test]
    fn pairs_below_min_co_occurrence_omitted() {
        let rows = vec![
            item("B1", "A", ""),
            item("B1", "B", ""),
            item("B2", "A", ""),
            item("B2", "B", ""),
        ];
        let focus = vec!["A".to_string(), "B".to_string()];
        let result = compute_menu_basket_affinities_from_orders(&rows, Some(&focus));
        assert!(result.pairs.is_empty());
        assert!(result.matrix_lift[0][1].is_none());
        assert!(result.matrix_lift[1][0].is_none());
    }
}
