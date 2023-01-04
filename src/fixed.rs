//! A static representation of a tree of nodes.
//!
//! Xot trees are dynamic, but for the purposes of proptests it's useful to
//! have a static representation of a tree of nodes.
//!
//! This can be enabled by adding the proptest feature to your Cargo.toml:
//!
//! ```toml
//! [dependencies]
//! xot = { version = "0.9", features = ["proptest"] }
//! ```

use crate::xotdata::{Node, Xot};

/// A fixed representation of an XML tree
///
/// This can be generated by a proptest.
#[derive(Debug)]
pub struct FixedRoot {
    pub(crate) before: Vec<FixedRootContent>,
    pub(crate) document_element: FixedElement,
    pub(crate) after: Vec<FixedRootContent>,
}

#[derive(Debug)]
pub(crate) struct FixedElement {
    pub(crate) namespace: String,
    pub(crate) name: String,
    pub(crate) attributes: Vec<((String, String), String)>,
    pub(crate) prefixes: Vec<(String, String)>,
    pub(crate) children: Vec<FixedContent>,
}

#[derive(Debug)]
pub(crate) enum FixedContent {
    Text(String),
    Comment(String),
    ProcessingInstruction(String, Option<String>),
    Element(FixedElement),
}

#[derive(Debug)]
pub(crate) enum FixedRootContent {
    Comment(String),
    ProcessingInstruction(String, Option<String>),
}

impl FixedRoot {
    /// Turn a fixed root into a Xot node
    pub fn xotify<'a>(&'a self, xot: &mut Xot<'a>) -> Node {
        let child = self.document_element.xotify(xot);
        let root = xot.new_root(child).unwrap();
        for content in &self.before {
            let node = create_root_content_node(xot, content);
            xot.insert_before(root, node).unwrap();
        }
        for content in &self.after {
            let node = create_root_content_node(xot, content);
            xot.append(root, node).unwrap();
        }
        root
    }
}

impl FixedElement {
    fn xotify<'a>(&'a self, xot: &mut Xot<'a>) -> Node {
        let ns = xot.add_namespace(&self.namespace);
        let name = xot.add_name_ns(&self.name, ns);
        let prefixes = self
            .prefixes
            .iter()
            .map(|(prefix, ns)| {
                let prefix = xot.add_prefix(prefix);
                let ns = xot.add_namespace(ns);
                (prefix, ns)
            })
            .collect::<Vec<_>>();
        let attributes = self
            .attributes
            .iter()
            .map(|((name, ns), value)| {
                let ns = xot.add_namespace(ns);
                let name = xot.add_name_ns(name, ns);
                (name, value)
            })
            .collect::<Vec<_>>();

        let element_node = xot.new_element(name);
        let element_value = xot.element_mut(element_node).unwrap();

        for (prefix, ns) in prefixes {
            element_value.set_prefix(prefix, ns);
        }
        for (name, value) in attributes {
            element_value.set_attribute(name, value);
        }

        let children = self
            .children
            .iter()
            .map(|child| child.xotify(xot))
            .collect::<Vec<_>>();
        for child in children {
            xot.append(element_node, child).unwrap();
        }
        element_node
    }
}

impl FixedContent {
    fn xotify<'a>(&'a self, xot: &mut Xot<'a>) -> Node {
        match self {
            FixedContent::Text(text) => xot.new_text(text),
            FixedContent::Comment(comment) => xot.new_comment(comment),
            FixedContent::ProcessingInstruction(target, data) => {
                xot.new_processing_instruction(target, data.as_deref())
            }
            FixedContent::Element(element) => element.xotify(xot),
        }
    }
}

fn create_root_content_node(xot: &mut Xot, content: &FixedRootContent) -> Node {
    match content {
        FixedRootContent::Comment(comment) => xot.new_comment(comment),
        FixedRootContent::ProcessingInstruction(target, data) => {
            xot.new_processing_instruction(target, data.as_deref())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xotify() {
        let mut xot = Xot::new();
        let root = FixedRoot {
            before: vec![],
            document_element: FixedElement {
                namespace: "".to_string(),
                name: "foo".to_string(),
                attributes: vec![],
                prefixes: vec![],
                children: vec![FixedContent::Text("Example".to_string())],
            },
            after: vec![],
        };
        let root = root.xotify(&mut xot);
        assert_eq!(xot.serialize_to_string(root), "<foo>Example</foo>");
    }
}
