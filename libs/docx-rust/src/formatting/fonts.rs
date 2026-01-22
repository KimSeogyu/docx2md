use hard_xml::{XmlRead, XmlWrite};

use crate::{__define_enum, __define_struct, __xml_test_suites};

// __define_struct!{
//     (Fontss, "w:rFonts", 'a) {
//         hint, FontHint, "w:hint",
//         ascii, Cow<'a, str>, "w:ascii",
//         east_asia, Cow<'a, str>, "w:eastAsia",
//     }
// }

__define_struct! {
    ("w:rFonts", Fonts) {
        "w:hint", hint, FontHint
        "w:ascii", ascii, String
        "w:eastAsia", east_asia, String
        "w:hAnsi", h_ansi, String
        "w:cs", custom, String
        "w:asciiTheme", ascii_theme, ThemeFont
        "w:eastAsiaTheme", east_asia_theme, ThemeFont
        "w:hAnsiTheme", h_ansi_theme, ThemeFont
        "w:cstheme", custom_theme, String
    }
}

// #[derive(Debug, Clone)]
// #[cfg_attr(test, derive(PartialEq))]
// pub enum FontHint {
//     Default,       //	High ANSI Font
//     EastAsia,      //	East Asian Font
//     ComplexScript, //	Complex Script Font
// }

__define_enum! {
    FontHint {
        Default= "default",  //	High ANSI Font
        EastAsia = "eastAsia", //	East Asian Font
        ComplexScript = "cs",//	Complex Script Font
    }
}

__define_enum! {
    ThemeFont {
        MajorEastAsia = "majorEastAsia", // Major East Asian Theme Font
        MajorBidi = "majorBidi", // Major Complex Script Theme Font
        MajorAscii = "majorAscii", // Major ASCII Theme Font
        MajorHansi = "majorHAnsi", // Major High ANSI Theme Font
        MinorEastAsia = "minorEastAsia", // Minor East Asian Theme Font
        MinorBidi = "minorBidi", // Minor Complex Script Theme Font
        MinorAscii = "minorAscii", // Minor ASCII Theme Font
        MinorHansi = "minorHAnsi", // Minor High ANSI Theme Font
    }
}

__xml_test_suites!(
    Fonts,
    Fonts::default().east_asia("宋体"),
    r#"<w:rFonts w:eastAsia="宋体"/>"#,
    Fonts::default()
        .east_asia("宋体")
        .ascii("Batang")
        .h_ansi("Batang"),
    r#"<w:rFonts w:ascii="Batang" w:eastAsia="宋体" w:hAnsi="Batang"/>"#,
);
