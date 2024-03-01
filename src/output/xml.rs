use std::io::Write;

use crate::NameId;

/// XML output method.
///
/// You can use this method with [`Xot::serialize_xml`] to control the XML
/// output generated by Xot.
///
/// This follows the rules
/// https://www.w3.org/TR/xslt-xquery-serialization/#xml-output but this API is
/// modified to include only those features that make sense for Xot, and use
/// enums to make the API more ergonomic where there are multiple interacting
/// parameters.
///
/// The `normalization_form` parameter is only included if the `icu` feature is
/// enabled.
///
/// Here is how we diverge from the specification:
///
/// * There is no way to declare the `version` parameter, as only XML 1.0 is
///   permitted at this time.
/// * You can only influence encoding parameter of the XML declaration, and
///   this does not trigger actual encoding; output is always UTF-8 and it's up
///   to you to do any further re-encoding.
/// * The `item-separator` parameter is specific to XPath/XSLT sequences and is
///   not supported directly by Xot.
/// * The `media-type` property is only meaningful in the context of a larger
///   system and is not supported directly by Xot.
/// * `undeclare-prefixes` is only supported by XML 1.1, which Xot does not
///   support at present.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Parameters {
    /// Pretty-print XML, and a list of elements where this is suppressed.
    pub indentation: Option<Indentation>,
    /// Elements that should be serialized as CDATA sections.
    pub cdata_section_elements: Vec<NameId>,
    /// The XML declaration, if any.
    pub declaration: Option<Declaration>,
    /// The doctype declaration, if any.
    pub doctype: Option<DocType>,
    /// Unicode normalization form, if any.
    #[cfg(feature = "icu")]
    pub normalization_form: Option<NormalizationForm>,
    // TODO: character maps
}

/// The output encoding.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum Encoding {
    /// UTF-8 is the default.
    ///
    /// Even though byte-order-mark is possible with UTF-8, it is at present
    /// not supported.
    #[default]
    Utf8,
    /// UTF-16 with or without a byte order mark
    Utf16 {
        /// Whether to include the byte order mark.
        byte_order_mark: bool,
    },
}

/// Indentation: pretty-print XML.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Indentation {
    /// A list of element names where indentation changes are suppressed.
    pub suppress: Vec<NameId>,
}

/// How to format the XML declaration.
///
/// Examples:
///
/// ```xml
/// <?xml version="1.0"?>
/// ```
///
/// ```xml
/// <?xml version="1.0" standalone="yes"?>
/// ```
///
/// ```xml
/// <?xml version="1.0" encoding="UTF-8"?>
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Declaration {
    /// This causes an encoding declaration to be included in the XML declaration.
    /// The text given here is taken literally. It does not affect the encoding of
    /// the output of serialization; that is always UTF-8.
    pub encoding: Option<String>,
    /// This causes a standalone declaration to be included in the XML declaration.
    pub standalone: Option<bool>,
    // TODO: if in the future we add XML 1.1 support, we can add the version here.
    // This way without a declaration it is automatically an XML 1.0 document. Potentially
    // also include `undeclare-prefixes` here, as that's only supported in XML 1.1.
}

impl Declaration {
    pub(crate) fn serialize(&self, buf: &mut Vec<u8>) -> Result<(), std::io::Error> {
        buf.write_all(b"<?xml version=\"1.0\"")?;
        if let Some(encoding) = &self.encoding {
            buf.write_all(b" encoding=\"")?;
            buf.write_all(encoding.as_bytes())?;
            buf.write_all(b"\"")?;
        }
        if let Some(standalone) = self.standalone {
            buf.write_all(b" standalone=\"")?;
            buf.write_all(if standalone { b"yes" } else { b"no" })?;
            buf.write_all(b"\"")?;
        }
        buf.write_all(b"?>\n")?;
        Ok(())
    }
}
/// The doctype declaration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DocType {
    /// Public identifier and system identifier.
    ///
    /// Example:
    /// ```xml
    /// <!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.0 Strict//EN" "http://www.w3.org/TR/xhtml1/DTD/xhtml1-strict.dtd">
    /// ```
    Public {
        /// The public identifier.
        public: String,
        /// The system identifier.
        system: String,
    },
    /// System identifier only.
    ///
    /// Example:
    /// ```xml
    /// <!DOCTYPE html SYSTEM "http://www.w3.org/TR/xhtml1/DTD/xhtml1-strict.dtd">
    /// ```
    System {
        /// The system identifier.
        system: String,
    },
}

/// Unicode normalization.
#[cfg(feature = "icu")]
#[derive(Debug, Clone, PartialEq, Eq)]
enum NormalizationForm {
    /// Normalization Form C, using the rules specified in [Character Model for
    /// the World Wide Web 1.0:
    /// Normalization](https://www.w3.org/TR/xslt-xquery-serialization/#charmod-norm).
    Nfc,
    /// NFD specifies the serialized result will be in Normalization Form D, as
    /// specified in [UAX #15: Unicode Normalization
    /// Forms](https://www.w3.org/TR/xslt-xquery-serialization/#UNICODE-NORMALIZATION-FORM).
    Nfd,
    /// Normalization Form KC, as specified in [UAX #15: Unicode Normalization
    /// Forms](https://www.w3.org/TR/xslt-xquery-serialization/#UNICODE-NORMALIZATION-FORM).
    Nfkc,
    /// Normalization Form KD, as specified in [UAX #15: Unicode Normalization
    /// Forms](https://www.w3.org/TR/xslt-xquery-serialization/#UNICODE-NORMALIZATION-FORM).
    Nfkd,
    // TODO: fully normalized
}

#[cfg(test)]
mod tests {
    use crate::Xot;

    use super::*;

    #[test]
    fn test_xml_output_default() {
        let m = Parameters {
            ..Default::default()
        };
        let mut xot = Xot::new();
        let doc = xot.parse("<doc><p>hello</p></doc>").unwrap();

        assert_eq!(
            xot.serialize_xml(m, doc).unwrap(),
            r#"<doc><p>hello</p></doc>"#
        );
    }

    #[test]
    fn test_xml_output_indent() {
        let m = Parameters {
            indentation: Some(Default::default()),
            ..Default::default()
        };
        let mut xot = Xot::new();
        let doc = xot.parse("<doc><p>hello</p></doc>").unwrap();

        assert_eq!(
            xot.serialize_xml(m, doc).unwrap(),
            r#"<doc>
  <p>hello</p>
</doc>
"#
        );
    }

    #[test]
    fn test_xml_output_declaration() {
        let m = Parameters {
            declaration: Some(Default::default()),
            ..Default::default()
        };
        let mut xot = Xot::new();
        let doc = xot.parse("<doc/>").unwrap();

        assert_eq!(
            xot.serialize_xml(m, doc).unwrap(),
            r#"<?xml version="1.0"?>
<doc/>"#
        );
    }

    #[test]
    fn test_xml_output_declaration_standalone() {
        let m = Parameters {
            declaration: Some(Declaration {
                standalone: Some(true),
                ..Default::default()
            }),
            ..Default::default()
        };
        let mut xot = Xot::new();
        let doc = xot.parse("<doc/>").unwrap();

        assert_eq!(
            xot.serialize_xml(m, doc).unwrap(),
            r#"<?xml version="1.0" standalone="yes"?>
<doc/>"#
        );
    }
}
