use crate::named_node::NamedNode;
use crate::object::Object;
use crate::subject::Subject;
use crate::GenericResult;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Quad {
    pub(crate) subject: Subject,
    pub(crate) predicate: NamedNode,
    pub(crate) object: Object,
    pub(crate) graph: Option<NamedNode>,
}

impl Quad {
    pub fn new<S, O>(
        subject: S,
        predicate: NamedNode,
        object: O,
        graph: Option<NamedNode>,
    ) -> GenericResult<Self>
    where
        S: Into<Subject>,
        O: Into<Object>,
    {
        let q = Quad {
            subject: subject.into(),
            predicate,
            object: object.into(),
            graph,
        };
        Ok(q)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quad_new() -> GenericResult<()> {
        let expected = Quad {
            subject: Subject::NamedNode(NamedNode::try_from("https://acme.org/")?),
            predicate: NamedNode::try_from("https://schema.org/name")?,
            object: Object::NamedNode(NamedNode::try_from("https://bar.com/")?),
            graph: None,
        };

        let actual = Quad::new(
            NamedNode::try_from("https://acme.org")?,
            NamedNode::try_from("https://schema.org/name")?,
            NamedNode::try_from("https://bar.com")?,
            None,
        )?;

        assert_eq!(expected, actual);

        Ok(())
    }
}
