mod blank_node;
mod dataset;
mod literal;
mod named_node;
mod object;
mod quad;
mod subject;

pub use blank_node::BlankNode;
pub use dataset::Dataset;
pub use literal::Literal;
pub use named_node::NamedNode;
pub use quad::Quad;

pub type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type GenericResult<T> = Result<T, GenericError>;
