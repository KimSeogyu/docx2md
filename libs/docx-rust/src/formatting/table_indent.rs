use hard_xml::{XmlRead, XmlWrite};

use crate::{__string_enum, __xml_test_suites};

/// Table Indent
///
/// ```rust
/// use docx_rust::formatting::*;
///
/// let ind = TableIndent::from(42);
/// let ind = TableIndent::from(TableIndentUnit::Pct);
/// let ind = TableIndent::from((42, TableIndentUnit::Dxa));
/// ```
#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:tblInd")]
pub struct TableIndent {
    #[xml(attr = "w:w")]
    pub value: Option<isize>,
    #[xml(attr = "w:type")]
    pub unit: Option<TableIndentUnit>,
}

impl From<isize> for TableIndent {
    fn from(val: isize) -> Self {
        TableIndent {
            value: Some(val),
            unit: None,
        }
    }
}

impl From<TableIndentUnit> for TableIndent {
    fn from(val: TableIndentUnit) -> Self {
        TableIndent {
            value: None,
            unit: Some(val),
        }
    }
}

impl From<(isize, TableIndentUnit)> for TableIndent {
    fn from(val: (isize, TableIndentUnit)) -> Self {
        TableIndent {
            value: Some(val.0),
            unit: Some(val.1),
        }
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub enum TableIndentUnit {
    Auto,
    Dxa,
    Nil,
    Pct,
}

__string_enum! {
    TableIndentUnit {
        Auto = "auto",
        Dxa = "dxa",
        Nil = "nil",
        Pct = "pct",
    }
}

__xml_test_suites!(
    TableIndent,
    TableIndent::default(),
    "<w:tblInd/>",
    TableIndent::from(42),
    r#"<w:tblInd w:w="42"/>"#,
    TableIndent::from(TableIndentUnit::Pct),
    r#"<w:tblInd w:type="pct"/>"#,
    TableIndent::from((42, TableIndentUnit::Dxa)),
    r#"<w:tblInd w:w="42" w:type="dxa"/>"#,
);
