use hard_xml::{XmlRead, XmlWrite};
use std::borrow::Cow;

use crate::__setter;

/// Start of comment
#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:commentRangeStart")]
pub struct CommentRangeStart<'a> {
    /// Specifies a unique identifier for the comment.
    #[xml(attr = "w:id")]
    pub id: Cow<'a, str>,
}

impl<'a> CommentRangeStart<'a> {
    __setter!(id: Cow<'a, str>);
}

/// End of comment
#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:commentRangeEnd")]
pub struct CommentRangeEnd<'a> {
    /// Specifies a unique identifier for the comment.
    #[xml(attr = "w:id")]
    pub id: Cow<'a, str>,
}

impl<'a> CommentRangeEnd<'a> {
    __setter!(id: Cow<'a, str>);
}

// __xml_test_suites!(
//     BookmarkEnd,
//     BookmarkEnd::default(),
//     r#"<w:bookmarkEnd/>"#,
//     BookmarkEnd::default().id("id"),
//     r#"<w:bookmarkEnd w:id="id"/>"#,
// );
