use hard_xml::{XmlRead, XmlWrite};

use crate::__xml_test_suites;

/// WidowControl
///
/// ```rust
/// use docx_rust::formatting::*;
///
/// let WidowControl = WidowControl::from(false);
/// let WidowControl = WidowControl::from(true);
/// ```
#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:widowControl")]
pub struct WidowControl {
    #[xml(attr = "w:val")]
    pub value: Option<bool>,
}

impl<T: Into<Option<bool>>> From<T> for WidowControl {
    fn from(val: T) -> Self {
        WidowControl { value: val.into() }
    }
}

__xml_test_suites!(
    WidowControl,
    WidowControl::default(),
    r#"<w:widowControl/>"#,
    WidowControl::from(false),
    r#"<w:widowControl w:val="false"/>"#,
    WidowControl::from(true),
    r#"<w:widowControl w:val="true"/>"#,
);
