//! Heading style parsing utilities.

/// Parses a DOCX style name to determine the heading level.
///
/// Recognizes standard heading styles like "Heading1", "Heading2", etc.,
/// as well as "Title" (level 1) and "Subtitle" (level 2).
///
/// Returns `None` if the style is not recognized as a heading.
pub fn parse_heading_style(style: &str) -> Option<usize> {
    let style_lower = style.to_lowercase();

    // Standard headings: "Heading1", "Heading 1", "heading1", etc.
    if let Some(rest) = style_lower.strip_prefix("heading") {
        return rest.trim().parse().ok();
    }

    // Common title styles
    match style_lower.as_str() {
        "title" => Some(1),
        "subtitle" => Some(2),
        _ => None,
    }
}
