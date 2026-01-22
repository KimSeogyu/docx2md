//! Hyperlink resolver - resolves hyperlink targets from relationships.

use std::collections::HashMap;

/// Resolver for hyperlink targets.
pub struct HyperlinkResolver;

impl HyperlinkResolver {
    /// Resolves a relationship ID to its target URL.
    pub fn resolve(r_id: &str, rels: &HashMap<String, String>) -> Option<String> {
        rels.get(r_id).cloned()
    }
}
