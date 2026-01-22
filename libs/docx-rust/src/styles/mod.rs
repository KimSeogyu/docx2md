//! Style Definitions
//!
//! The corresponding ZIP item is `/word/styles.xml`.

mod default_style;
mod latent_style;
mod latent_styles;
mod priority;
mod semi_hidden;
mod style;
mod unhidden_when_used;

use self::latent_styles::LatentStyles;
pub use self::{default_style::*, style::*};

use hard_xml::{XmlRead, XmlResult, XmlWrite, XmlWriter};
use std::io::Write;

use crate::schema::{SCHEMA_MAIN, SCHEMA_XML};
use crate::{__xml_test_suites, write_attr};

/// Styles of the document
///
/// Styles are predefined sets of properties which can be applied to text.
///
/// ```rust
/// use docx_rust::styles::*;
///
/// let style = Styles::new()
///     .default(DefaultStyle::default())
///     .push(Style::new(StyleType::Paragraph, "style_id"));
/// ```
#[derive(Debug, Default, XmlRead, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:styles")]
pub struct Styles<'a> {
    /// Specifies the default set of properties.
    #[xml(child = "w:docDefaults")]
    pub default: Option<DefaultStyle<'a>>,
    #[xml(child = "w:latentStyles")]
    pub latent_styles: Option<LatentStyles<'a>>,
    /// Specifies a set of properties.
    #[xml(child = "w:style")]
    pub styles: Vec<Style<'a>>,
}

impl<'a> XmlWrite for Styles<'a> {
    fn to_writer<W: Write>(&self, writer: &mut XmlWriter<W>) -> XmlResult<()> {
        let Styles {
            default,
            latent_styles,
            styles,
        } = self;

        log::debug!("[Styles] Started writing.");

        let _ = write!(writer.inner, "{}", SCHEMA_XML);
        writer.write_element_start("w:styles")?;

        writer.write_attribute("xmlns:w", SCHEMA_MAIN)?;

        writer.write_element_end_open()?;

        write_attr(default, writer)?;
        write_attr(latent_styles, writer)?;

        for ele in styles {
            ele.to_writer(writer)?;
        }

        writer.write_element_end_close("w:styles")?;

        log::debug!("[Styles] Finished writing.");

        Ok(())
    }
}

impl<'a> Styles<'a> {
    pub fn new() -> Self {
        <Styles as Default>::default()
    }

    pub fn default(&mut self, style: DefaultStyle<'a>) -> &mut Self {
        self.default = Some(style);
        self
    }

    pub fn push(&mut self, style: Style<'a>) -> &mut Self {
        self.styles.push(style);
        self
    }
}

__xml_test_suites!(
    Styles,
    Styles::new(),
    format!(
        r#"{}<w:styles xmlns:w="{}"></w:styles>"#,
        SCHEMA_XML, SCHEMA_MAIN
    )
    .as_str(),
    Styles {
        styles: vec![Style::new(crate::styles::StyleType::Paragraph, "id")],
        ..Default::default()
    },
    format!(
        r#"{}<w:styles xmlns:w="{}"><w:style w:type="paragraph" w:styleId="id"/></w:styles>"#,
        SCHEMA_XML, SCHEMA_MAIN
    )
    .as_str(),
);
