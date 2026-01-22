use std::borrow::Cow;

use hard_xml::{XmlRead, XmlWrite};

/// Symbol Character
#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:sym")]
pub struct Sym<'a> {
    #[xml(attr = "w:font")]
    pub font: Option<Cow<'a, str>>,
    #[xml(attr = "w:char")]
    pub char: Option<Cow<'a, str>>,
}
