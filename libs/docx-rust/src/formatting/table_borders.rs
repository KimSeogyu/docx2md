use hard_xml::{XmlRead, XmlWrite};

use crate::{
    __setter, __xml_test_suites,
    formatting::{
        BottomBorder, InsideHorizonBorder, InsideVerticalBorder, LeftBorder, RightBorder, TopBorder,
    },
};

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:tblBorders")]
pub struct TableBorders<'a> {
    #[xml(child = "w:top")]
    pub top: Option<TopBorder<'a>>,
    #[xml(child = "w:left")]
    pub left: Option<LeftBorder<'a>>,
    #[xml(child = "w:bottom")]
    pub bottom: Option<BottomBorder<'a>>,
    #[xml(child = "w:right")]
    pub right: Option<RightBorder<'a>>,
    #[xml(child = "w:insideH")]
    pub inside_horizon: Option<InsideHorizonBorder<'a>>,
    #[xml(child = "w:insideV")]
    pub inside_vertical: Option<InsideVerticalBorder<'a>>,
}

impl<'a> TableBorders<'a> {
    __setter!(top: Option<TopBorder<'a>>);
    __setter!(bottom: Option<BottomBorder<'a>>);
}

__xml_test_suites!(
    TableBorders,
    TableBorders::default(),
    r#"<w:tblBorders/>"#,
    TableBorders::default().top(TopBorder::default()),
    r#"<w:tblBorders><w:top w:val="none"/></w:tblBorders>"#,
    TableBorders::default().bottom(BottomBorder::default()),
    r#"<w:tblBorders><w:bottom w:val="none"/></w:tblBorders>"#,
);
