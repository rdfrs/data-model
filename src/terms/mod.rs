use std::fmt::Display;

pub trait Subject: Display {}

pub trait Object: Display {}

mod blank_node;
mod literal;
mod named_node;

pub use blank_node::BlankNode;
pub use literal::Literal;
pub use named_node::NamedNode;
