use hard_xml::{XmlRead, XmlWrite};
use std::borrow::Cow;

use crate::__xml_test_suites;

/// VML Object (Legacy Image)
#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:pict")]
pub struct Pict<'a> {
    /// Specifies the content of the VML object
    /// For now, we mainly care about v:shape -> v:imagedata accessing images
    /// But parsing full VML is complex. We will try to capture raw children or specific known children.
    /// hard_xml doesn't support "any child", so we define common VML shapes.

    // Attempt to parse <v:shape>
    #[xml(child = "v:shape")]
    pub shape: Option<Shape<'a>>,

    // Sometimes it's directly parseable? No, usually nested.
    // Let's support v:rect too just in case.
    #[xml(child = "v:rect")]
    pub rect: Option<Rect<'a>>,
}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "v:shape")]
pub struct Shape<'a> {
    #[xml(attr = "id")]
    pub id: Option<Cow<'a, str>>,
    #[xml(attr = "style")]
    pub style: Option<Cow<'a, str>>,
    #[xml(child = "v:imagedata")]
    pub image_data: Option<ImageData<'a>>,
}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "v:rect")]
pub struct Rect<'a> {
    #[xml(child = "v:imagedata")]
    pub image_data: Option<ImageData<'a>>,
}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "v:imagedata")]
pub struct ImageData<'a> {
    #[xml(attr = "r:id")]
    pub id: Option<Cow<'a, str>>,
    #[xml(attr = "o:title")]
    pub title: Option<Cow<'a, str>>,
}

impl<'a> Pict<'a> {
    // helpers if needed
}

__xml_test_suites!(Pict, Pict::default(), r#"<w:pict/>"#,);
