use hard_xml::{XmlRead, XmlWrite};
use std::borrow::Cow;

use crate::{__setter, __xml_test_suites, document::Run};

/// A bidirectional embedding, which can nest to more bidirectional embeddings
#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:dir")]
pub struct BidirectionalEmbedding<'a> {
    // A BidirectionalEmbedding can have a number of rich text runs
    #[xml(child = "w:r")]
    pub runs: Vec<Run<'a>>,
    // A BidirectionalEmbedding can include nested embedding layers
    #[xml(child = "w:dir")]
    pub nested_levels: Vec<BidirectionalEmbedding<'a>>,
}

impl<'a> BidirectionalEmbedding<'a> {
    __setter!(runs: Vec<Run<'a>>);
    __setter!(nested_levels: Vec<BidirectionalEmbedding<'a>>);

    pub fn iter_text(&self) -> Box<dyn Iterator<Item = &Cow<'a, str>> + '_> {
        Box::new(
            self.runs.iter().flat_map(|run| run.iter_text()).chain(
                self.nested_levels
                    .iter()
                    .flat_map(|nested| nested.iter_text()),
            ),
        )
    }

    pub fn iter_text_mut(&mut self) -> Box<dyn Iterator<Item = &mut Cow<'a, str>> + '_> {
        Box::new(
            self.runs
                .iter_mut()
                .flat_map(|run| run.iter_text_mut())
                .chain(
                    self.nested_levels
                        .iter_mut()
                        .flat_map(|nested| nested.iter_text_mut()),
                ),
        )
    }
}

__xml_test_suites!(
    BidirectionalEmbedding,
    BidirectionalEmbedding::default(),
    r#"<w:dir/>"#,
    BidirectionalEmbedding::default().runs(vec![Run::default(), Run::default().push_text("text")]),
    r#"<w:dir><w:r/><w:r><w:t>text</w:t></w:r></w:dir>"#,
    BidirectionalEmbedding::default().nested_levels(vec![
        BidirectionalEmbedding::default().runs(vec![Run::default()])
    ]),
    r#"<w:dir><w:dir><w:r/></w:dir></w:dir>"#,
);
