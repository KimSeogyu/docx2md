use std::borrow::Cow;

use hard_xml::{XmlRead, XmlWrite};

use crate::{__setter, __xml_test_suites};

/// Size
///
/// ```rust
/// use docx_rust::formatting::*;
///
/// let sz = Size::from(42isize);
/// ```
#[derive(Debug, XmlRead, XmlWrite, Clone, Default)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:lang")]
pub struct Lang<'a> {
    #[xml(attr = "w:val")]
    pub val: Option<Cow<'a, str>>,
    #[xml(attr = "w:eastAsia")]
    pub east_asia: Option<Cow<'a, str>>,
    #[xml(attr = "w:bidi")]
    pub bidi: Option<Cow<'a, str>>,
}

impl<'a> Lang<'a> {
    __setter!(east_asia: Option<Cow<'a, str>>);
    __setter!(bidi: Option<Cow<'a, str>>);
    __setter!(val: Option<Cow<'a, str>>);
}

__xml_test_suites!(
    Lang,
    Lang::default().east_asia("zh-CN"),
    r#"<w:lang w:eastAsia="zh-CN"/>"#,
);
