use crate::__string_enum;

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub enum LineRule {
    Auto,
    Exact,
    AtLeast,
}

__string_enum! {
    LineRule {
        Auto = "auto",
        Exact = "exact",
        AtLeast = "atLeast",
    }
}
