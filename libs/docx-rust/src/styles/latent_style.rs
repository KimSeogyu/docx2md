#![allow(unused_must_use)]
use hard_xml::{XmlRead, XmlWrite};
use std::borrow::Cow;

/// Style
///
/// A style that applied to a region of the document.
///
/// ```rust
/// use docx_rust::formatting::*;
/// use docx_rust::styles::*;
///
/// let style = Style::new(StyleType::Paragraph, "style_id")
///     .name("Style Name")
///     .paragraph(ParagraphProperty::default())
///     .character(CharacterProperty::default());
/// ```
#[derive(Debug, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:lsdException")]
pub struct LatentStyle<'a> {
    /// Specifies the type of style.
    #[xml(attr = "w:name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(attr = "w:locked")]
    pub locked: Option<bool>,
    #[xml(attr = "w:uiPriority")]
    pub priority: Option<isize>,
    #[xml(attr = "w:semiHidden")]
    pub semi_hidden: Option<bool>,
    #[xml(attr = "w:unhideWhenUsed")]
    pub unhiden_when_used: Option<bool>,
    #[xml(attr = "w:qFormat")]
    pub q_format: Option<bool>,
}
