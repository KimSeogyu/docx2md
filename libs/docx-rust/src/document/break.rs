use hard_xml::{XmlRead, XmlWrite};

use crate::{__string_enum, __xml_test_suites};

/// Break
///
/// ```rust
/// use docx_rust::document::*;
///
/// let br = Break::from(BreakType::Page);
/// ```
#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:br")]
pub struct Break {
    /// Specifies the break type of this break.
    #[xml(attr = "w:type")]
    pub ty: Option<BreakType>,
}

impl<T: Into<Option<BreakType>>> From<T> for Break {
    fn from(val: T) -> Self {
        Break { ty: val.into() }
    }
}

/// Specifies the break type of a break
///
/// The default value is TextWrapping.
#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub enum BreakType {
    /// Text restarts on the next column.
    Column,
    /// Text restarts on the next page.
    Page,
    /// Text restarts on the next line.
    TextWrapping,
}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:lastRenderedPageBreak")]
pub struct LastRenderedPageBreak {}

__string_enum! {
    BreakType {
        Column = "column",
        Page = "page",
        TextWrapping = "textWrapping",
    }
}

__xml_test_suites!(
    Break,
    Break::default(),
    r#"<w:br/>"#,
    Break::from(BreakType::Page),
    r#"<w:br w:type="page"/>"#,
);
