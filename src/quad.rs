use crate::terms::*;
use crate::Result;
use std::fmt::{Display, Formatter};

#[derive(PartialEq)]
pub struct Quad<S: Subject, O: Object> {
    subject: Box<S>,
    predicate: NamedNode,
    object: Box<O>,
    graph: Option<NamedNode>,
}

impl<S: Subject, O: Object> Quad<S, O> {
    pub fn new(s: S, p: NamedNode, o: O, g: Option<NamedNode>) -> Result<Quad<S, O>> {
        Ok(Quad {
            subject: Box::new(s),
            predicate: p,
            object: Box::new(o),
            graph: g,
        })
    }

    // https://github.com/rust-lang/rfcs/blob/master/text/0344-conventions-galore.md#gettersetter-apis
    pub fn subject(&self) -> &S {
        self.subject.as_ref()
    }

    pub fn predicate(&self) -> &NamedNode {
        &self.predicate
    }

    pub fn object(&self) -> &O {
        self.object.as_ref()
    }

    pub fn graph(&self) -> &Option<NamedNode> {
        &self.graph
    }
}

impl<S: Subject, O: Object> Display for Quad<S, O> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} .",
            &self.subject, &self.predicate, &self.object
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_should_have_expected_members() -> Result<()> {
        let s = NamedNode::new("http://foo.com/bar")?;
        let p = NamedNode::new("https://schema.org/name")?;
        let o = Literal::String("bar".to_string(), None);

        let q = Quad::new(s, p, o, None)?;

        let expected_subject = NamedNode::new("http://foo.com/bar")?;
        let expected_predicate = NamedNode::new("https://schema.org/name")?;
        let expected_object = Literal::String("bar".to_string(), None);

        assert_eq!(&expected_subject, q.subject());
        assert_eq!(&expected_predicate, q.predicate());
        assert_eq!(&expected_object, q.object());
        assert!(q.graph().is_none());

        Ok(())
    }

    

    // #[test]
    // fn test_construct_quad_literal_object() -> Result<()> {
    //     let s = NamedNode::new("http://foo.com/bar")?;
    //     let p = NamedNode::new("https://schema.org/name")?;
    //     let o = Literal::String("bar".to_string(), None);

    //     let _q = Quad {
    //         subject: Subject::NamedNode(s),
    //         predicate: p,
    //         object: Object::Literal(o),
    //         graph: None,
    //     };

    //     Ok(())
    // }

    // #[test]
    // fn test_construct_quad_named_node_object() -> Result<()> {
    //     let s = NamedNode::new("http://foo.com/bar")?;
    //     let p = NamedNode::new("https://schema.org/hasPart")?;
    //     let o = NamedNode::new("http://foo.com/baz")?;

    //     let _q = Quad {
    //         subject: Subject::NamedNode(s),
    //         predicate: p,
    //         object: Object::NamedNode(o),
    //         graph: None,
    //     };

    //     Ok(())
    // }

    #[test]
    fn test_display_quad() -> Result<()> {
        let s = NamedNode::new("http://foo.com/bar")?;
        let p = NamedNode::new("https://schema.org/name")?;
        let o = Literal::String("bar".to_string(), None);

        let q = Quad::new(s, p, o, None)?;

        let expected = "<http://foo.com/bar> <https://schema.org/name> \"bar\" .".to_string();
        let actual = format!("{q}");

        assert_eq!(expected, actual);

        Ok(())
    }
}
