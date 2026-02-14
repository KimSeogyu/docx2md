#[derive(Debug, Clone, Default)]
pub struct DocumentAst {
    pub blocks: Vec<BlockNode>,
    pub references: ReferenceDefinitions,
}

#[derive(Debug, Clone)]
pub enum BlockNode {
    Paragraph(String),
    TableHtml(String),
    RawHtml(String),
}

#[derive(Debug, Clone, Default)]
pub struct ReferenceDefinitions {
    pub footnotes: Vec<String>,
    pub endnotes: Vec<String>,
    pub comments: Vec<(String, String)>,
}
