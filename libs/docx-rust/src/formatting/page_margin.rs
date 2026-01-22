use hard_xml::{XmlRead, XmlWrite};

/// Numbering Id
///
/// ```rust
/// use docx_rust::formatting::*;
///
/// let id = NumberingId::from(42isize);
/// ```
#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:pgMar")]
pub struct PageMargin {
    #[xml(attr = "w:top")]
    pub top: Option<isize>,
    #[xml(attr = "w:right")]
    pub right: Option<isize>,
    #[xml(attr = "w:bottom")]
    pub bottom: Option<isize>,
    #[xml(attr = "w:left")]
    pub left: Option<isize>,
    #[xml(attr = "w:header")]
    pub header: Option<isize>,
    #[xml(attr = "w:footer")]
    pub footer: Option<isize>,
    #[xml(attr = "w:gutter")]
    pub gutter: Option<isize>,
}

// impl<T: Into<isize>> From<T> for NumberingId {
//     fn from(val: T) -> Self {
//         NumberingId { value: val.into() }
//     }
// }

// __xml_test_suites!(
//     NumberingId,
//     NumberingId::from(40isize),
//     r#"<w:numId w:val="40"/>"#,
// );
