use crate::blank_node::BlankNode;
use crate::named_node::NamedNode;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Subject {
    NamedNode(NamedNode),
    BlankNode(BlankNode),
}

impl From<NamedNode> for Subject {
    fn from(value: NamedNode) -> Self {
        Subject::NamedNode(value)
    }
}

impl From<BlankNode> for Subject {
    fn from(value: BlankNode) -> Self {
        Subject::BlankNode(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::GenericResult;

    #[test]
    fn subject_from_named_node() -> GenericResult<()> {
        let n = NamedNode::try_from("https://acme.org/")?;

        let s: Subject = n.into();

        let expected = Subject::NamedNode(NamedNode::try_from("https://acme.org/")?);

        assert_eq!(expected, s);
        Ok(())
    }
}
