use hard_xml::{XmlRead, XmlWrite};

use crate::__string_enum;

/// Numbering Id
///
/// ```rust
/// use docx_rust::formatting::*;
///
/// let id = NumberingId::from(42isize);
/// ```
#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:docGrid")]
pub struct PageGrid {
    #[xml(attr = "w:type")]
    pub ty: Option<GridType>,
    #[xml(attr = "w:linePitch")]
    pub line_pitch: Option<isize>,
    #[xml(attr = "w:charSpace")]
    pub char_space: Option<isize>,
}

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub enum GridType {
    Default,       //	No Document Grid
    Lines,         //	Line Grid Only
    LinesAndChars, //	Line and Character Grid
    SnapToChars,   //	Character Grid Only
}

__string_enum! {
    GridType {
        Default = "default",
        Lines = "lines",
        LinesAndChars = "linesAndChars",
        SnapToChars = "snapToChars",
    }
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
