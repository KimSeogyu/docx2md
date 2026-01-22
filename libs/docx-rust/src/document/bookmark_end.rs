use hard_xml::{XmlRead, XmlWrite};
use std::borrow::Cow;

use crate::{__setter, __xml_test_suites};

/// End of bookmark
#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:bookmarkEnd")]
pub struct BookmarkEnd<'a> {
    /// Specifies a unique identifier for the bookmark.
    #[xml(attr = "w:id")]
    pub id: Option<Cow<'a, str>>,
}

impl<'a> BookmarkEnd<'a> {
    __setter!(id: Option<Cow<'a, str>>);
}

__xml_test_suites!(
    BookmarkEnd,
    BookmarkEnd::default(),
    r#"<w:bookmarkEnd/>"#,
    BookmarkEnd::default().id("id"),
    r#"<w:bookmarkEnd w:id="id"/>"#,
);
