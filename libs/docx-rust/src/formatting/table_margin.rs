use hard_xml::{XmlRead, XmlWrite};

use crate::__setter;

use super::margin::{BottomMargin, LeftMargin, RightMargin, TopMargin};

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:tblCellMar")]
pub struct TableMargins<'a> {
    #[xml(child = "w:top")]
    pub top: Option<TopMargin<'a>>,
    #[xml(child = "w:left")]
    pub left: Option<LeftMargin<'a>>,
    #[xml(child = "w:bottom")]
    pub bottom: Option<BottomMargin<'a>>,
    #[xml(child = "w:right")]
    pub right: Option<RightMargin<'a>>,
}

impl<'a> TableMargins<'a> {
    __setter!(top: Option<TopMargin<'a>>);
    __setter!(left: Option<LeftMargin<'a>>);
    __setter!(right: Option<RightMargin<'a>>);
    __setter!(bottom: Option<BottomMargin<'a>>);
}

// __xml_test_suites!(
//     TableBorders,
//     TableBorders::default(),
//     r#"<w:tblBorders/>"#,
//     TableBorders::default().top(TopBorder::default()),
//     r#"<w:tblBorders><w:top/></w:tblBorders>"#,
//     TableBorders::default().bottom(BottomBorder::default()),
//     r#"<w:tblBorders><w:bottom/></w:tblBorders>"#,
// );
