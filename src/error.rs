use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum Error {
    #[error("{0} is not a valid absolute URI")]
    InvalidURIError(String),
}
