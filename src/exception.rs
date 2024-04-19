use thiserror::Error;

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    #[error("Provided index is greater than max index.")]
    OutOfIndex,
}
