use crate::terms::BlankNode;
use crate::terms::Literal;
use crate::terms::NamedNode;
use std::fmt::{Display, Formatter};

#[derive(PartialEq)]
pub struct Quad {
    pub subject: Subject,
    pub predicate: NamedNode,
    pub object: Object,
    pub graph: Option<NamedNode>,
}

impl Display for Quad {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let subject_str = match &self.subject {
            Subject::NamedNode(sub) => format!("{sub}"),
            Subject::BlankNode(sub) => format!("{sub}"),
        };

        let object_str = match &self.object {
            Object::NamedNode(obj) => format!("{obj}"),
            Object::BlankNode(obj) => format!("{obj}"),
            Object::Literal(obj) => format!("{obj}"),
        };

        write!(f, "{subject_str} {} {object_str} .", &self.predicate,)
    }
}

#[derive(PartialEq, Eq)]
pub enum Subject {
    NamedNode(NamedNode),
    BlankNode(BlankNode),
}

#[derive(PartialEq)]
pub enum Object {
    NamedNode(NamedNode),
    BlankNode(BlankNode),
    Literal(Literal),
}

#[cfg(test)]
mod tests {
    use crate::error::Error;
    use crate::quad::{Object, Quad, Subject};
    use crate::terms::{Literal, NamedNode};

    #[test]
    fn test_construct_quad_literal_object() -> Result<(), Error> {
        let s = NamedNode::new("http://foo.com/bar")?;
        let p = NamedNode::new("https://schema.org/name")?;
        let o = Literal::String("bar".to_string(), None);

        let _q = Quad {
            subject: Subject::NamedNode(s),
            predicate: p,
            object: Object::Literal(o),
            graph: None,
        };

        Ok(())
    }

    #[test]
    fn test_construct_quad_named_node_object() -> Result<(), Error> {
        let s = NamedNode::new("http://foo.com/bar")?;
        let p = NamedNode::new("https://schema.org/hasPart")?;
        let o = NamedNode::new("http://foo.com/baz")?;

        let _q = Quad {
            subject: Subject::NamedNode(s),
            predicate: p,
            object: Object::NamedNode(o),
            graph: None,
        };

        Ok(())
    }

    #[test]
    fn test_display_quad() -> Result<(), Error> {
        let s = NamedNode::new("http://foo.com/bar")?;
        let p = NamedNode::new("https://schema.org/name")?;
        let o = Literal::String("bar".to_string(), None);

        let q = Quad {
            subject: Subject::NamedNode(s),
            predicate: p,
            object: Object::Literal(o),
            graph: None,
        };

        let expected = "<http://foo.com/bar> <https://schema.org/name> \"bar\" .".to_string();
        let actual = format!("{q}");

        assert_eq!(expected, actual);

        Ok(())
    }
}
