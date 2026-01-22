use hard_xml::{XmlRead, XmlWrite};
use std::borrow::Cow;

use crate::{__setter, __xml_test_suites, formatting::BorderStyle};

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:between")]
pub struct BetweenBorder<'a> {
    #[xml(attr = "w:val")]
    pub style: super::BorderStyle,
    #[xml(attr = "w:color")]
    pub color: Option<Cow<'a, str>>,
    #[xml(attr = "w:themeColor")]
    pub theme_color: Option<crate::formatting::ThemeColor>,
    #[xml(attr = "w:themeTint")]
    pub theme_tint: Option<Cow<'a, str>>,
    #[xml(attr = "w:themeShade")]
    pub theme_shade: Option<Cow<'a, str>>,
    #[xml(attr = "w:sz")]
    pub size: Option<isize>, // Measurement in Eighths of a Point
    #[xml(attr = "w:space")]
    pub space: Option<isize>,
    #[xml(attr = "w:shadow")]
    pub shadow: Option<bool>,
    #[xml(attr = "w:frame")]
    pub frame: Option<bool>,
}

impl<'a> BetweenBorder<'a> {
    __setter!(color: Option<Cow<'a, str>>);
    __setter!(shadow: Option<bool>);
    __setter!(space: Option<isize>);
    __setter!(size: Option<isize>);
    __setter!(style: BorderStyle);
}

__xml_test_suites!(
    BetweenBorder,
    BetweenBorder::default(),
    r#"<w:between w:val="none"/>"#,
    BetweenBorder::default().color("000000"),
    r#"<w:between w:val="none" w:color="000000"/>"#,
    BetweenBorder::default().shadow(false),
    r#"<w:between w:val="none" w:shadow="false"/>"#,
    BetweenBorder::default().space(40isize),
    r#"<w:between w:val="none" w:space="40"/>"#,
    BetweenBorder::default().size(20isize),
    r#"<w:between w:val="none" w:sz="20"/>"#,
    BetweenBorder::default().style(BorderStyle::Dotted),
    r#"<w:between w:val="dotted"/>"#,
);
