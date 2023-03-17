use std::fmt::Display;

pub trait Subject: Display + PartialEq {}

pub trait Object: Display + PartialEq {}

mod blank_node;
mod literal;
mod named_node;

pub use blank_node::BlankNode;
pub use literal::Literal;
pub use named_node::NamedNode;
