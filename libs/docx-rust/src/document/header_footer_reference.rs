#![allow(unused_must_use)]
use derive_more::From;
use hard_xml::{XmlRead, XmlWrite};
use std::borrow::Cow;

use crate::{__setter, __string_enum, __xml_test_suites};

/// A set of elements that can be contained as the content of a run.
#[derive(Debug, From, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub enum HeaderFooterReference<'a> {
    #[xml(tag = "w:headerReference")]
    Header(HeaderReference<'a>),
    #[xml(tag = "w:footerReference")]
    Footer(FooterReference<'a>),
}

/// HeaderReference
///
#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:headerReference")]
pub struct HeaderReference<'a> {
    /// Specifies the HeaderReference type of this HeaderReference.
    #[xml(attr = "w:type")]
    pub ty: Option<HeaderFooterReferenceType>,
    #[xml(attr = "r:id")]
    pub id: Option<Cow<'a, str>>,
}

impl<'a> HeaderReference<'a> {
    __setter!(ty: Option<HeaderFooterReferenceType>);
    __setter!(id: Option<Cow<'a, str>>);
}

/// FooterReference
///
#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:footerReference")]
pub struct FooterReference<'a> {
    /// Specifies the FooterReference type of this FooterReference.
    #[xml(attr = "w:type")]
    pub ty: Option<HeaderFooterReferenceType>,
    #[xml(attr = "r:id")]
    pub id: Option<Cow<'a, str>>,
}

impl<'a> FooterReference<'a> {
    __setter!(ty: Option<HeaderFooterReferenceType>);
    __setter!(id: Option<Cow<'a, str>>);
}

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub enum HeaderFooterReferenceType {
    Default,
    Even,
    First,
}

__string_enum! {
    HeaderFooterReferenceType {
        Default = "default",
        Even = "even",
        First = "first",
    }
}

__xml_test_suites!(
    HeaderReference,
    HeaderReference::default(),
    r#"<w:headerReference/>"#,
    HeaderReference::default().ty(HeaderFooterReferenceType::Default),
    r#"<w:headerReference w:type="default"/>"#,
);
