use crate::blank_node::BlankNode;
use crate::literal::Literal;
use crate::named_node::NamedNode;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Object {
    NamedNode(NamedNode),
    BlankNode(BlankNode),
    Literal(Literal),
}

impl From<NamedNode> for Object {
    fn from(value: NamedNode) -> Self {
        Object::NamedNode(value)
    }
}

impl From<BlankNode> for Object {
    fn from(value: BlankNode) -> Self {
        Object::BlankNode(value)
    }
}

impl From<Literal> for Object {
    fn from(value: Literal) -> Self {
        Object::Literal(value)
    }
}

#[cfg(test)]
mod tests {}
