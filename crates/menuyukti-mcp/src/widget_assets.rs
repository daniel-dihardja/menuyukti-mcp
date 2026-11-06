//! MCP Apps widget HTML assets with shared theme inlining.
//!
//! ChatGPT mounts each widget as one self-contained HTML resource, so shared
//! CSS is authored once under `assets/shared/` and substituted into a marker
//! before `resources/read` / local preview serve.

use std::sync::OnceLock;

/// Marker placed inside each heatmap widget `<style>` block.
pub const SHARED_THEME_MARKER: &str = "/* __MENUYUKTI_SHARED_THEME__ */";

const SHARED_THEME_CSS: &str = include_str!("../assets/shared/widget-theme.css");
const HELLO_WORLD_WIDGET_HTML: &str = include_str!("../assets/hello_world_widget.html");
const VENUE_SLOT_STRENGTH_WIDGET_SHELL: &str =
    include_str!("../assets/venue_slot_strength_widget.html");
const PAIR_LIFT_MATRIX_WIDGET_SHELL: &str = include_str!("../assets/pair_lift_matrix_widget.html");
const MENU_HEATMAPS_WIDGET_SHELL: &str = include_str!("../assets/menu_heatmaps_widget.html");

/// Replace the shared-theme marker with `widget-theme.css` (no-op if absent).
pub fn inline_shared_theme(html: &str) -> String {
    if html.contains(SHARED_THEME_MARKER) {
        html.replace(SHARED_THEME_MARKER, SHARED_THEME_CSS)
    } else {
        html.to_string()
    }
}

/// Hello World smoke widget (no shared theme marker).
pub fn hello_world_widget_html() -> &'static str {
    HELLO_WORLD_WIDGET_HTML
}

/// Venue slot strength heatmap HTML with shared theme inlined.
pub fn venue_slot_strength_widget_html() -> &'static str {
    static HTML: OnceLock<String> = OnceLock::new();
    HTML.get_or_init(|| inline_shared_theme(VENUE_SLOT_STRENGTH_WIDGET_SHELL))
        .as_str()
}

/// Pair lift matrix heatmap HTML with shared theme inlined.
pub fn pair_lift_matrix_widget_html() -> &'static str {
    static HTML: OnceLock<String> = OnceLock::new();
    HTML.get_or_init(|| inline_shared_theme(PAIR_LIFT_MATRIX_WIDGET_SHELL))
        .as_str()
}

/// Menu heatmaps HTML with shared theme inlined.
pub fn menu_heatmaps_widget_html() -> &'static str {
    static HTML: OnceLock<String> = OnceLock::new();
    HTML.get_or_init(|| inline_shared_theme(MENU_HEATMAPS_WIDGET_SHELL))
        .as_str()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shared_theme_file_defines_host_and_heatmap_tokens() {
        assert!(SHARED_THEME_CSS.contains("--color-background-primary"));
        assert!(SHARED_THEME_CSS.contains("--pad-top: 48px"));
        assert!(SHARED_THEME_CSS.contains("--heatmap-low"));
        assert!(SHARED_THEME_CSS.contains("--heatmap-high"));
        assert!(SHARED_THEME_CSS.contains("color-scheme: light dark"));
        assert!(SHARED_THEME_CSS.contains(".legend-bar"));
        assert!(SHARED_THEME_CSS.contains(".panel"));
    }

    #[test]
    fn venue_and_pair_shells_use_marker_not_full_chrome_tokens() {
        assert!(VENUE_SLOT_STRENGTH_WIDGET_SHELL.contains(SHARED_THEME_MARKER));
        assert!(PAIR_LIFT_MATRIX_WIDGET_SHELL.contains(SHARED_THEME_MARKER));
        assert!(MENU_HEATMAPS_WIDGET_SHELL.contains(SHARED_THEME_MARKER));
        // Shells should not hard-code the shared pad token (lives in theme CSS).
        assert!(!VENUE_SLOT_STRENGTH_WIDGET_SHELL.contains("--pad-top: 48px"));
        assert!(!PAIR_LIFT_MATRIX_WIDGET_SHELL.contains("--pad-top: 48px"));
        assert!(!MENU_HEATMAPS_WIDGET_SHELL.contains("--pad-top: 48px"));
    }

    #[test]
    fn inlined_widgets_are_self_contained_with_theme() {
        let venue = venue_slot_strength_widget_html();
        assert!(!venue.contains(SHARED_THEME_MARKER));
        assert!(venue.contains("--pad-top: 48px"));
        assert!(venue.contains("Venue slot strength"));
        assert!(venue.contains("--heatmap-high"));

        let pair = pair_lift_matrix_widget_html();
        assert!(!pair.contains(SHARED_THEME_MARKER));
        assert!(pair.contains("--pad-top: 48px"));
        assert!(pair.contains("Pair lift index"));
        assert!(pair.contains("--heatmap-low"));

        let menus = menu_heatmaps_widget_html();
        assert!(!menus.contains(SHARED_THEME_MARKER));
        assert!(menus.contains("--pad-top: 48px"));
        assert!(menus.contains("Menu heatmaps"));
        assert!(menus.contains("matrix_category") || menus.contains("data-category=\"weak\""));
        assert!(menus.contains("--heatmap-high"));
    }

    #[test]
    fn inline_is_noop_without_marker() {
        let raw = "<html><style>.x{color:red}</style></html>";
        assert_eq!(inline_shared_theme(raw), raw);
    }
}
