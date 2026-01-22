#![allow(unused_must_use)]

use hard_xml::{XmlRead, XmlWrite};
use std::borrow::Cow;

use crate::{__setter, document::bidir::BidirectionalEmbedding, document::Run};

/// The root element of a hyperlink within the paragraph
#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:hyperlink")]
pub struct Hyperlink<'a> {
    /// Specifies the ID of the relationship in the relationships part for an external link.
    #[xml(attr = "r:id")]
    pub id: Option<Cow<'a, str>>,
    /// Specifies the name of a bookmark within the document.
    #[xml(attr = "w:anchor")]
    pub anchor: Option<Cow<'a, str>>,
    #[xml(child = "w:r")]
    /// Link content
    pub content: Vec<Run<'a>>,
    #[xml(child = "w:dir")]
    // Link can contain a bi-directional embedding layer
    pub bidirectional_embedding: Option<BidirectionalEmbedding<'a>>,
}

impl<'a> Hyperlink<'a> {
    __setter!(id: Option<Cow<'a, str>>);
    __setter!(anchor: Option<Cow<'a, str>>);
    __setter!(content: Vec<Run<'a>>);

    pub fn text(&self) -> String {
        self.iter_text()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join("")
    }

    pub fn iter_text(&self) -> Box<dyn Iterator<Item = &Cow<'a, str>> + '_> {
        Box::new(
            self.content.iter().flat_map(|run| run.iter_text()).chain(
                self.bidirectional_embedding
                    .iter()
                    .flat_map(|bidi| bidi.iter_text()),
            ),
        )
    }

    pub fn iter_text_mut(&mut self) -> Box<dyn Iterator<Item = &mut Cow<'a, str>> + '_> {
        Box::new(
            self.content
                .iter_mut()
                .flat_map(|run| run.iter_text_mut())
                .chain(
                    self.bidirectional_embedding
                        .iter_mut()
                        .flat_map(|bidi| bidi.iter_text_mut()),
                ),
        )
    }
}
