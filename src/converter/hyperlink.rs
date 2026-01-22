//! Hyperlink resolver - resolves hyperlink targets from relationships.

use std::collections::HashMap;

/// Resolves a relationship ID to its target URL.
pub fn resolve_hyperlink(r_id: &str, rels: &HashMap<String, String>) -> Option<String> {
    rels.get(r_id).cloned()
}
