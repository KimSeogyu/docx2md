use hard_xml::{XmlRead, XmlWrite};

use crate::__xml_test_suites;

/// Italics
///
/// ```rust
/// use docx_rust::formatting::*;
///
/// let i = Italics::from(false);
/// let i = Italics::from(true);
/// ```
#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:i")]
pub struct Italics {
    #[xml(attr = "w:val")]
    pub value: Option<bool>,
}

impl<T: Into<Option<bool>>> From<T> for Italics {
    fn from(val: T) -> Self {
        Italics { value: val.into() }
    }
}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:iCs")]
pub struct ItalicsComplex {
    #[xml(attr = "w:val")]
    pub value: Option<bool>,
}

impl<T: Into<Option<bool>>> From<T> for ItalicsComplex {
    fn from(val: T) -> Self {
        ItalicsComplex { value: val.into() }
    }
}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:caps")]
pub struct Caps {
    #[xml(attr = "w:val")]
    pub value: Option<bool>,
}

impl<T: Into<Option<bool>>> From<T> for Caps {
    fn from(val: T) -> Self {
        Caps { value: val.into() }
    }
}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:smallCaps")]
pub struct SmallCaps {
    #[xml(attr = "w:val")]
    pub value: Option<bool>,
}

impl<T: Into<Option<bool>>> From<T> for SmallCaps {
    fn from(val: T) -> Self {
        SmallCaps { value: val.into() }
    }
}

__xml_test_suites!(
    Italics,
    Italics::default(),
    r#"<w:i/>"#,
    Italics::from(false),
    r#"<w:i w:val="false"/>"#,
    Italics::from(true),
    r#"<w:i w:val="true"/>"#,
);
