use hard_xml::{XmlRead, XmlWrite};
use std::borrow::Cow;

use crate::{__string_enum, __xml_test_suites};

/// Literal Text
///
/// A literal text that shall be displayed in the document.
///
/// ```rust
/// use docx_rust::document::{Text, TextSpace};
///
/// let text = Text::from("text");
/// let text = Text::from(String::from("text"));
/// let text = Text::from(("text", TextSpace::Preserve));
/// ```
#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:instrText")]
pub struct InstrText<'a> {
    /// Specifies how to handle whitespace
    #[xml(attr = "xml:space")]
    pub space: Option<TextSpace>,
    /// Specifies a literal text
    #[xml(text)]
    pub text: Cow<'a, str>,
}

impl From<String> for InstrText<'_> {
    fn from(val: String) -> Self {
        InstrText {
            text: val.into(),
            space: None,
        }
    }
}

impl<'a> From<&'a str> for InstrText<'a> {
    fn from(val: &'a str) -> Self {
        InstrText {
            text: val.into(),
            space: None,
        }
    }
}

impl From<(String, TextSpace)> for InstrText<'_> {
    fn from(val: (String, TextSpace)) -> Self {
        InstrText {
            text: val.0.into(),
            space: Some(val.1),
        }
    }
}

impl<'a> From<(&'a str, TextSpace)> for InstrText<'a> {
    fn from(val: (&'a str, TextSpace)) -> Self {
        InstrText {
            text: val.0.into(),
            space: Some(val.1),
        }
    }
}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:delInstrText")]
pub struct DelInstrText<'a> {
    /// Specifies how to handle whitespace
    #[xml(attr = "xml:space")]
    pub space: Option<TextSpace>,
    /// Specifies a literal text
    #[xml(text)]
    pub text: Cow<'a, str>,
}

impl From<String> for DelInstrText<'_> {
    fn from(val: String) -> Self {
        DelInstrText {
            text: val.into(),
            space: None,
        }
    }
}

impl<'a> From<&'a str> for DelInstrText<'a> {
    fn from(val: &'a str) -> Self {
        DelInstrText {
            text: val.into(),
            space: None,
        }
    }
}

impl From<(String, TextSpace)> for DelInstrText<'_> {
    fn from(val: (String, TextSpace)) -> Self {
        DelInstrText {
            text: val.0.into(),
            space: Some(val.1),
        }
    }
}

impl<'a> From<(&'a str, TextSpace)> for DelInstrText<'a> {
    fn from(val: (&'a str, TextSpace)) -> Self {
        DelInstrText {
            text: val.0.into(),
            space: Some(val.1),
        }
    }
}

/// Text Space Rules
///
/// Specifies how whitespace should be handled in a literal text.
#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub enum TextSpace {
    /// Default rules
    Default,
    /// Using the W3C space preservation rules
    Preserve,
}

__string_enum! {
    TextSpace {
        Default = "default",
        Preserve = "preserve",
    }
}

__xml_test_suites!(
    InstrText,
    InstrText::from("text"),
    "<w:instrText>text</w:instrText>",
    InstrText::from(String::from("text")),
    "<w:instrText>text</w:instrText>",
    InstrText::from(("text", TextSpace::Preserve)),
    r#"<w:instrText xml:space="preserve">text</w:instrText>"#,
    InstrText::from((String::from("text"), TextSpace::Default)),
    r#"<w:instrText xml:space="default">text</w:instrText>"#,
);
