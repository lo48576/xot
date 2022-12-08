use crate::namespace::NamespaceId;
use crate::xmldata::Node;

/// Xot errors
#[derive(Debug)]
pub enum Error {
    // access errors
    /// The node is not a root node.
    NotRoot(Node),

    // manipulation errors
    /// Invalid operation on XML. You get this when
    /// trying to remove the document, or trying to
    /// insert something under a text node, for instance.
    InvalidOperation(String),

    /// You aren't allowed to use this string as a comment.
    /// Happens if you include `--` in a comment.
    InvalidComment(String),
    /// You aren't allowed to use this string as a processing instruction
    /// target. Happens if you use `XML` or any case variation of this.
    InvalidTarget(String),
    /// The node you tried to act on is not an element.
    NotElement(Node),
    /// Indextree error that can happen during manipulation.
    NodeError(indextree::NodeError),

    // serializer
    /// Missing prefix for namespace.
    /// Can occur during serialization if a namespace is used that has no
    /// prefix is declared. Use [`XmlData::create_missing_prefixes`](crate::xmldata::XmlData::create_missing_prefixes)
    /// to fix this.
    MissingPrefix(NamespaceId),

    // parser errors
    /// The XML is not well-formed - a tag is opened and never closed.
    UnclosedTag,
    /// The XML is not well-formed - a tag is closed that was never opened.
    InvalidCloseTag(String, String),
    /// The XML is not well-formed - you use `&` to open an entity without
    /// closing it with `;`.
    UnclosedEntity(String),
    /// The entity is not known. Only the basic entities are supported
    /// right now, not any user defined ones.
    InvalidEntity(String),
    /// You used a namespace prefix that is not declared.
    UnknownPrefix(String),
    /// You declared an attribute of the same name twice.
    DuplicateAttribute(String),
    /// Unsupported XML version. Only 1.0 is supported.
    UnsupportedVersion(String),
    /// Unsupported XML encoding. Only UTF-8 is supported.
    UnsupportedEncoding(String),
    /// Unsupported standalone declaration. Only `yes` is supported.
    UnsupportedNotStandalone,
    /// XML DTD is not supported.
    DtdUnsupported,
    /// xmlparser error
    Parser(xmlparser::Error),

    /// IO error
    Io(std::io::Error),
}

impl From<indextree::NodeError> for Error {
    #[inline]
    fn from(e: indextree::NodeError) -> Self {
        Error::NodeError(e)
    }
}

impl From<std::io::Error> for Error {
    #[inline]
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<xmlparser::Error> for Error {
    #[inline]
    fn from(e: xmlparser::Error) -> Self {
        Error::Parser(e)
    }
}
