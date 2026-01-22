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
#[xml(tag = "w:fldChar")]
pub struct FieldChar {
    /// Specifies the break type of this break.
    #[xml(attr = "w:fldCharType")]
    pub ty: Option<CharType>,
}

impl<T: Into<Option<CharType>>> From<T> for FieldChar {
    fn from(val: T) -> Self {
        FieldChar { ty: val.into() }
    }
}

/// Specifies the break type of a break
///
/// The default value is TextWrapping.
#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub enum CharType {
    /// Text restarts on the next column.
    Begin,
    /// Text restarts on the next page.
    Separate,
    /// Text restarts on the next line.
    End,
}

__string_enum! {
    CharType {
        Begin = "begin",
        Separate = "separate",
        End = "end",
    }
}

__xml_test_suites!(
    FieldChar,
    FieldChar::from(CharType::Begin),
    r#"<w:fldChar w:fldCharType="begin"/>"#,
);
