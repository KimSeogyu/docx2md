use hard_xml::{XmlRead, XmlWrite};

/// Tab
#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:tab")]
pub struct Tab;

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:cr")]
pub struct CarriageReturn;
