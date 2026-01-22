//! Comments part
//!
//! The corresponding ZIP item is `/word/comments.xml`.
#![allow(unused_must_use)]

use hard_xml::{XmlRead, XmlResult, XmlWrite, XmlWriter};
use std::{borrow::Cow, io::Write};

use crate::{
    document::Paragraph,
    schema::{SCHEMA_MAIN, SCHEMA_WORDML_14},
};

/// The root element of the comments document part.
#[derive(Debug, Default, XmlRead, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:comments")]
pub struct Comments<'a> {
    // Specifies the comments
    #[xml(child = "w:comment")]
    pub comments: Vec<Comment<'a>>,
}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:comment")]
pub struct Comment<'a> {
    // Specifies the id of the comment.
    #[xml(attr = "w:id")]
    pub id: Option<isize>,

    #[xml(attr = "w:author")]
    pub author: Cow<'a, str>,

    // Specifies the body of the comment.
    #[xml(child = "w:p")]
    pub content: Paragraph<'a>,
}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:annotationRef")]
pub struct AnnotationRef;

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:commentReference")]
pub struct CommentReference<'a> {
    #[xml(attr = "w:id")]
    pub id: Option<Cow<'a, str>>,
}

impl<'a> XmlWrite for Comments<'a> {
    fn to_writer<W: Write>(&self, writer: &mut XmlWriter<W>) -> XmlResult<()> {
        let Comments { comments } = self;

        log::debug!("[Comments] Started writing.");
        let _ = write!(writer.inner, "{}", crate::schema::SCHEMA_XML);

        writer.write_element_start("w:comments")?;

        writer.write_attribute("xmlns:w", SCHEMA_MAIN)?;

        writer.write_attribute("xmlns:w14", SCHEMA_WORDML_14)?;

        writer.write_element_end_open()?;

        for c in comments {
            c.to_writer(writer)?;
        }

        writer.write_element_end_close("w:comments")?;

        log::debug!("[Comments] Finished writing.");

        Ok(())
    }
}
