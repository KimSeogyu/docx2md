use super::AstExtractor;
use crate::converter::{ConversionContext, ParagraphConverter, TableConverter};
use crate::core::ast::{BlockNode, DocumentAst};
use crate::render::escape_html_attr;
use crate::Result;
use rs_docx::document::BodyContent;

#[derive(Debug, Default, Clone, Copy)]
pub struct DocxExtractor;

impl AstExtractor for DocxExtractor {
    fn extract<'a>(
        &self,
        body: &[BodyContent<'a>],
        context: &mut ConversionContext<'a>,
    ) -> Result<DocumentAst> {
        let mut doc = DocumentAst::default();
        for content in body {
            self.extract_content(content, context, &mut doc)?;
        }
        Ok(doc)
    }
}

impl DocxExtractor {
    fn extract_content<'a>(
        &self,
        content: &BodyContent<'a>,
        context: &mut ConversionContext<'a>,
        output: &mut DocumentAst,
    ) -> Result<()> {
        match content {
            BodyContent::Paragraph(para) => {
                let converted = ParagraphConverter::convert(para, context)?;
                if !converted.is_empty() {
                    output.blocks.push(BlockNode::Paragraph(converted));
                }
            }
            BodyContent::Table(table) => {
                let converted = TableConverter::convert(table, context)?;
                output.blocks.push(BlockNode::TableHtml(converted));
            }
            BodyContent::Sdt(sdt) => {
                if let Some(sdt_content) = &sdt.content {
                    for child in &sdt_content.content {
                        self.extract_content(child, context, output)?;
                    }
                }
            }
            BodyContent::BookmarkStart(bookmark) => {
                if let Some(name) = &bookmark.name {
                    output.blocks.push(BlockNode::RawHtml(format!(
                        "<a id=\"{}\"></a>",
                        escape_html_attr(name)
                    )));
                }
            }
            _ => {}
        }
        Ok(())
    }
}
