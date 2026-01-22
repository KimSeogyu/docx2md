use hard_xml::{XmlRead, XmlWrite};

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:vMerge")]
pub struct VMerge {
    #[xml(attr = "w:val")]
    pub val: Option<String>,
}
