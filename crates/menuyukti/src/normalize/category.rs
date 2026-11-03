use std::collections::HashMap;

use crate::line_item::LineItem;

pub(crate) fn normalize_category(value: Option<&str>, fallback: &str) -> String {
    match value {
        None => fallback.to_string(),
        Some(raw) => {
            let text = raw.trim().to_uppercase();
            if text.is_empty() || text == "NAN" {
                fallback.to_string()
            } else {
                text
            }
        }
    }
}

/// Harmonize category fields to the mode value per menu name.
pub(crate) fn harmonize_categories(mut rows: Vec<LineItem>) -> Vec<LineItem> {
    let mut cat_counts: HashMap<String, HashMap<String, usize>> = HashMap::new();
    let mut detail_counts: HashMap<String, HashMap<String, usize>> = HashMap::new();
    for row in &rows {
        *cat_counts
            .entry(row.menu.clone())
            .or_default()
            .entry(row.menu_category.clone())
            .or_default() += 1;
        *detail_counts
            .entry(row.menu.clone())
            .or_default()
            .entry(row.menu_category_detail.clone())
            .or_default() += 1;
    }
    let mode = |counts: &HashMap<String, HashMap<String, usize>>, menu: &str| {
        counts
            .get(menu)
            .and_then(|m| m.iter().max_by_key(|(_, c)| *c).map(|(k, _)| k.clone()))
    };
    for row in &mut rows {
        if let Some(c) = mode(&cat_counts, &row.menu) {
            row.menu_category = c;
        }
        if let Some(d) = mode(&detail_counts, &row.menu) {
            row.menu_category_detail = d;
        }
    }
    rows
}
