//! Localization strategy for language-specific formatting.

/// Strategy for handling language-specific formatting conventions.
pub trait LocalizationStrategy: Send + Sync {
    /// Determines if a numbering marker represents a heading (e.g., "제1조").
    /// Returns Some(heading_prefix) if it is a heading, None otherwise.
    fn handle_numbering(&self, marker: &str) -> Option<String>;

    /// Parses a style name to determine the heading level.
    fn parse_heading_style(&self, style: &str) -> Option<usize>;
}

/// Default localization strategy (mostly pass-through).
pub struct DefaultLocalization;

impl LocalizationStrategy for DefaultLocalization {
    fn handle_numbering(&self, _marker: &str) -> Option<String> {
        None
    }

    fn parse_heading_style(&self, style: &str) -> Option<usize> {
        let style_lower = style.to_lowercase();
        if let Some(rest) = style_lower.strip_prefix("heading") {
            return rest.trim().parse().ok();
        }
        match style_lower.as_str() {
            "title" => Some(1),
            "subtitle" => Some(2),
            _ => None,
        }
    }
}

/// Korean localization strategy (handles "제N조", "제목").
pub struct KoreanLocalization;

impl LocalizationStrategy for KoreanLocalization {
    fn handle_numbering(&self, marker: &str) -> Option<String> {
        // Handle "제N조." or "제N조"
        if marker.starts_with("제") && (marker.ends_with("조.") || marker.ends_with("조")) {
            // Return formatted header marker (Article usually H3)
            let clean_marker = marker.trim_end_matches('.');
            return Some(format!("### {}", clean_marker));
        }
        None
    }

    fn parse_heading_style(&self, style: &str) -> Option<usize> {
        let style_lower = style.to_lowercase();

        // Standard headings first
        if let Some(rest) = style_lower.strip_prefix("heading") {
            return rest.trim().parse().ok();
        }

        // Korean heading styles: "제목 1" -> 1
        if style_lower.starts_with("제목") {
            return style_lower
                .chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<String>()
                .parse()
                .ok();
        }

        match style_lower.as_str() {
            "title" => Some(1),
            "subtitle" => Some(2),
            _ => None,
        }
    }
}
