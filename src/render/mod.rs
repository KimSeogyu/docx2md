mod escape;
mod markdown;

use crate::core::ast::DocumentAst;
use crate::Result;

pub use escape::{escape_html_attr, escape_markdown_link_destination, escape_markdown_link_text};
pub use markdown::MarkdownRenderer;

pub trait Renderer {
    fn render(&self, document: &DocumentAst) -> Result<String>;
}
