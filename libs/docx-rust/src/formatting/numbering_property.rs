#![allow(unused_must_use)]
use std::borrow::Cow;

use hard_xml::{XmlRead, XmlWrite};

use crate::__xml_test_suites;
use crate::formatting::{IndentLevel, NumberingId};

/// Numbering Property
///
/// ```rust
/// use docx_rust::formatting::*;
///
/// let prop = NumberingProperty::from((20, 40));
/// ```
#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:numPr")]
pub struct NumberingProperty<'a> {
    /// Specifies the numbering level of the numbering definition to use for the paragraph.
    #[xml(child = "w:ilvl")]
    pub level: Option<IndentLevel>,
    /// Specifies a reference to a numbering definition instance
    #[xml(child = "w:numId")]
    pub id: Option<NumberingId>,
    /// Previous Paragraph Numbering Properties
    #[xml(child = "w:numberingChange")]
    pub numbering_change: Option<NumberingChange<'a>>,
    /// Inserted Numbering Properties
    #[xml(child = "w:ins")]
    pub ins: Option<InsertedProperties<'a>>,
}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:ins")]
pub struct InsertedProperties<'a> {
    #[xml(attr = "w:id")]
    pub id: Option<isize>,
    #[xml(attr = "w:author")]
    pub author: Option<Cow<'a, str>>,
    #[xml(attr = "w:date")]
    pub date: Option<Cow<'a, str>>,
}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:numberingChange")]
pub struct NumberingChange<'a> {
    #[xml(attr = "w:id")]
    pub id: Option<isize>,
    #[xml(attr = "w:author")]
    pub author: Option<Cow<'a, str>>,
    #[xml(attr = "w:date")]
    pub date: Option<Cow<'a, str>>,
    #[xml(attr = "w:original")]
    pub original: Option<Cow<'a, str>>,
}

impl From<(isize, isize)> for NumberingProperty<'_> {
    fn from(val: (isize, isize)) -> Self {
        NumberingProperty {
            id: Some(NumberingId { value: val.0 }),
            level: Some(IndentLevel { value: val.1 }),
            numbering_change: None,
            ins: None,
        }
    }
}

__xml_test_suites!(
    NumberingProperty,
    NumberingProperty::default(),
    r#"<w:numPr/>"#,
    NumberingProperty::from((20, 40)),
    r#"<w:numPr><w:ilvl w:val="40"/><w:numId w:val="20"/></w:numPr>"#,
);
