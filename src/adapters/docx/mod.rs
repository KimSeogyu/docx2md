mod extractor;

use crate::converter::ConversionContext;
use crate::core::ast::DocumentAst;
use crate::Result;
use rs_docx::document::BodyContent;

pub trait AstExtractor {
    fn extract<'a>(
        &self,
        body: &[BodyContent<'a>],
        context: &mut ConversionContext<'a>,
    ) -> Result<DocumentAst>;
}

pub use extractor::DocxExtractor;
