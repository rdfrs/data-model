use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum Error {
    #[error("{0} is not a valid absolute IRI")]
    InvalidURIError(String), // TODO: is it possible to accept both the string _and_ a #[from] parameter? does this even make sense?
}
