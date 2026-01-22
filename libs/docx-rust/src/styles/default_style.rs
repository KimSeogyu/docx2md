#![allow(unused_must_use)]
use hard_xml::{XmlRead, XmlWrite};

use crate::{
    __setter, __xml_test_suites,
    formatting::{CharacterProperty, ParagraphProperty},
};

/// Default Style
///
/// This style is inherited by every paragraph and run.
///
/// ```rust
/// use docx_rust::formatting::*;
/// use docx_rust::styles::*;
///
/// let style = DefaultStyle::default()
///     .character(CharacterProperty::default())
///     .paragraph(ParagraphProperty::default());
/// ```
#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:docDefaults")]
pub struct DefaultStyle<'a> {
    #[xml(default, child = "w:rPrDefault")]
    pub character: DefaultCharacterProperty<'a>,
    #[xml(default, child = "w:pPrDefault")]
    pub paragraph: DefaultParagraphProperty<'a>,
}

impl<'a> DefaultStyle<'a> {
    __setter!(character: DefaultCharacterProperty<'a>);
    __setter!(paragraph: DefaultParagraphProperty<'a>);
}

/// Default Character Properties
#[derive(Default, Debug, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:rPrDefault")]
pub struct DefaultCharacterProperty<'a> {
    /// character properties
    #[xml(default, child = "w:rPr")]
    pub inner: Option<CharacterProperty<'a>>,
}

impl<'a, T: Into<CharacterProperty<'a>>> From<T> for DefaultCharacterProperty<'a> {
    fn from(val: T) -> Self {
        DefaultCharacterProperty {
            inner: Some(val.into()),
        }
    }
}

/// Default Paragraph Properties
#[derive(Default, Debug, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:pPrDefault")]
pub struct DefaultParagraphProperty<'a> {
    /// paragraph properties
    #[xml(default, child = "w:pPr")]
    pub inner: Option<ParagraphProperty<'a>>,
}

impl<'a, T: Into<ParagraphProperty<'a>>> From<T> for DefaultParagraphProperty<'a> {
    fn from(val: T) -> Self {
        DefaultParagraphProperty {
            inner: Some(val.into()),
        }
    }
}

__xml_test_suites!(
    DefaultStyle,
    DefaultStyle::default(),
    r#"<w:docDefaults><w:rPrDefault/><w:pPrDefault/></w:docDefaults>"#,
);
