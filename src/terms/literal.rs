use crate::terms::NamedNode;

#[derive(PartialEq, Eq, Debug)]
pub struct Literal<T> {
    value: T,
    language: String, // Note: this should change to conform to a standard list of language codes
    data_type: NamedNode,
}
