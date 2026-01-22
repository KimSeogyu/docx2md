use hard_xml::{XmlRead, XmlWrite};

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:gridSpan")]
pub struct GridSpan {
    #[xml(attr = "w:val")]
    pub val: isize,
}
