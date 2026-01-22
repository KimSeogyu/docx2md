#![allow(unused_must_use)]
use hard_xml::{XmlRead, XmlWrite};
use std::borrow::Cow;

use crate::{
    __setter, __string_enum, __xml_test_suites,
    formatting::{CharacterProperty, ParagraphProperty, TableProperty},
};

use crate::styles::priority::Priority;

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
#[xml(tag = "w:style")]
pub struct Style<'a> {
    /// Specifies the type of style.
    #[xml(attr = "w:type")]
    pub ty: Option<StyleType>,
    /// Specifies the unique identifier
    ///
    /// This identifier is used throughout the document to apply style in content.
    #[xml(attr = "w:styleId")]
    pub style_id: Cow<'a, str>,
    #[xml(attr = "w:default")]
    pub default: Option<bool>,
    #[xml(attr = "w:customStyle")]
    pub custom_style: Option<bool>,

    /// Specifies the primary name
    #[xml(child = "w:name")]
    pub name: Option<StyleName<'a>>,
    #[xml(child = "w:aliases")]
    pub aliases: Option<Aliases<'a>>,
    #[xml(child = "w:basedOn")]
    pub base: Option<BasedOn<'a>>,
    /// Style For Next Paragraph
    #[xml(child = "w:next")]
    pub next: Option<Next<'a>>,
    #[xml(child = "w:link")]
    pub link: Option<Link<'a>>,
    #[xml(child = "w:autoRedefine")]
    pub auto_redefine: Option<AutoRedefine>,
    #[xml(child = "w:hidden")]
    pub hidden: Option<Hidden>,
    /// Specifies the priority.
    #[xml(child = "w:uiPriority")]
    pub priority: Option<Priority>,
    #[xml(child = "w:semiHidden")]
    pub semi_hidden: Option<super::semi_hidden::SemiHidden>,
    #[xml(child = "w:unhideWhenUsed")]
    pub unhide_when_used: Option<super::unhidden_when_used::UnhideWhenUsed>,
    #[xml(child = "w:qFormat")]
    pub q_format: Option<QFormat>,
    /// Style Cannot Be Applied
    #[xml(child = "w:locked")]
    pub locked: Option<Locked>,
    /// E-Mail Message Text Style
    #[xml(child = "w:personal")]
    pub personal: Option<Personal>,
    /// E-Mail Message Composition Style
    #[xml(child = "w:personalCompose")]
    pub personal_compose: Option<PersonalCompose>,
    /// E-Mail Message Reply Style
    #[xml(child = "w:personalReply")]
    pub personal_reply: Option<PersonalReply>,
    #[xml(child = "w:rsid")]
    pub rsid: Option<Rsid<'a>>,
    /// Specifies a set of paragraph properties
    #[xml(default, child = "w:pPr")]
    pub paragraph: Option<ParagraphProperty<'a>>,
    /// Specifies a set of character properties
    #[xml(default, child = "w:rPr")]
    pub character: Option<CharacterProperty<'a>>,
    #[xml(default, child = "w:tblPr")]
    pub table: Option<TableProperty<'a>>,
    #[xml(child = "w:trPr")]
    pub table_row: Option<crate::formatting::TableRowProperty>,
    #[xml(child = "w:tcPr")]
    pub table_cell: Option<crate::formatting::TableCellProperty>,
    #[xml(child = "w:tblStylePr")]
    pub conditional_table_property: Vec<crate::formatting::ConditionalTableProperty<'a>>,
}

impl<'a> Style<'a> {
    pub fn new<T: Into<Cow<'a, str>>>(ty: StyleType, style_id: T) -> Self {
        Style {
            ty: Some(ty),
            style_id: style_id.into(),
            default: None,
            custom_style: None,
            name: None,
            base: None,
            q_format: None,
            priority: None,
            semi_hidden: None,
            unhide_when_used: None,
            paragraph: None,
            character: None,
            table: None,
            aliases: None,
            next: None,
            link: None,
            auto_redefine: None,
            hidden: None,
            locked: None,
            personal: None,
            personal_compose: None,
            personal_reply: None,
            rsid: None,
            table_row: None,
            table_cell: None,
            conditional_table_property: Vec::new(),
        }
    }

    __setter!(ty: Option<StyleType>);
    __setter!(name: Option<StyleName<'a>>);
    __setter!(paragraph: Option<ParagraphProperty<'a>>);
    __setter!(character: Option<CharacterProperty<'a>>);
}

#[derive(Debug, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:name")]
pub struct StyleName<'a> {
    #[xml(attr = "w:val")]
    pub value: Cow<'a, str>,
}

impl<'a, S: Into<Cow<'a, str>>> From<S> for StyleName<'a> {
    fn from(val: S) -> Self {
        StyleName { value: val.into() }
    }
}

#[derive(Debug, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:aliases")]
pub struct Aliases<'a> {
    #[xml(attr = "w:val")]
    pub value: Cow<'a, str>,
}

#[derive(Debug, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:next")]
pub struct Next<'a> {
    #[xml(attr = "w:val")]
    pub value: Cow<'a, str>,
}

#[derive(Debug, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:link")]
pub struct Link<'a> {
    #[xml(attr = "w:val")]
    pub value: Cow<'a, str>,
}

#[derive(Debug, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:autoRedefine")]
pub struct AutoRedefine {
    #[xml(attr = "w:val")]
    pub value: Option<bool>,
}

#[derive(Debug, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:hidden")]
pub struct Hidden {
    #[xml(attr = "w:val")]
    pub value: Option<bool>,
}

#[derive(Debug, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:locked")]
pub struct Locked {
    #[xml(attr = "w:val")]
    pub value: Option<bool>,
}

#[derive(Debug, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:personal")]
pub struct Personal {
    #[xml(attr = "w:val")]
    pub value: Option<bool>,
}

#[derive(Debug, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:personalCompose")]
pub struct PersonalCompose {
    #[xml(attr = "w:val")]
    pub value: Option<bool>,
}

#[derive(Debug, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:personalReply")]
pub struct PersonalReply {
    #[xml(attr = "w:val")]
    pub value: Option<bool>,
}

#[derive(Debug, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:rsid")]
pub struct Rsid<'a> {
    #[xml(attr = "w:val")]
    pub value: Cow<'a, str>,
}

#[derive(Debug, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:basedOn")]
pub struct BasedOn<'a> {
    #[xml(attr = "w:val")]
    pub value: Cow<'a, str>,
}

#[derive(Debug, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:qFormat")]
pub struct QFormat {
    #[xml(attr = "w:val")]
    pub value: Option<bool>,
}

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub enum StyleType {
    Character,
    Paragraph,
    Table,
    Numbering,
}

__string_enum! {
    StyleType {
        Character = "character",
        Paragraph = "paragraph",
        Table = "table",
        Numbering = "numbering",
    }
}

__xml_test_suites!(
    Style,
    Style::new(StyleType::Numbering, "id"),
    r#"<w:style w:type="numbering" w:styleId="id"/>"#,
    Style::new(StyleType::Table, "id").name("name"),
    r#"<w:style w:type="table" w:styleId="id"><w:name w:val="name"/></w:style>"#,
    Style::new(StyleType::Paragraph, "id"),
    r#"<w:style w:type="paragraph" w:styleId="id"/>"#,
    Style::new(StyleType::Character, "id"),
    r#"<w:style w:type="character" w:styleId="id"/>"#,
);
