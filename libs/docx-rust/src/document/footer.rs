//! Footer part
//!
//! The corresponding ZIP item is `/word/footer{n}.xml`.
//!

use hard_xml::{XmlRead, XmlResult, XmlWrite, XmlWriter};
use std::io::Write;

use crate::__xml_test_suites;
use crate::schema::{SCHEMA_MAIN, SCHEMA_WORDML_14};

use crate::document::BodyContent;

/// The root element of the main document part.
#[derive(Debug, Default, XmlRead, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:ftr")]
pub struct Footer<'a> {
    #[xml(child = "w:sdt", child = "w:p", child = "w:tbl", child = "w:sectPr")]
    pub content: Vec<BodyContent<'a>>,
}

impl<'a> Footer<'a> {
    pub fn push<T: Into<BodyContent<'a>>>(&mut self, content: T) -> &mut Self {
        self.content.push(content.into());
        self
    }
}

impl<'a> XmlWrite for Footer<'a> {
    fn to_writer<W: Write>(&self, writer: &mut XmlWriter<W>) -> XmlResult<()> {
        let Footer { content } = self;

        log::debug!("[Footer] Started writing.");
        let _ = write!(writer.inner, "{}", crate::schema::SCHEMA_XML);

        writer.write_element_start("w:ftr")?;

        writer.write_attribute("xmlns:w", SCHEMA_MAIN)?;

        writer.write_attribute("xmlns:w14", SCHEMA_WORDML_14)?;

        writer.write_element_end_open()?;

        for c in content {
            c.to_writer(writer)?;
        }

        writer.write_element_end_close("w:ftr")?;

        log::debug!("[Document] Finished writing.");

        Ok(())
    }
}

__xml_test_suites!(
    Footer,
    Footer::default(),
    format!(
        r#"{}<w:ftr xmlns:w="{}" xmlns:w14="{}"></w:ftr>"#,
        crate::schema::SCHEMA_XML,
        SCHEMA_MAIN,
        SCHEMA_WORDML_14
    )
    .as_str(),
);
