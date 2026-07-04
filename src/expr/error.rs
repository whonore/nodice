use std::num::TryFromIntError;

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("Overflow")]
    Overflow,
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<TryFromIntError> for Error {
    fn from(_err: TryFromIntError) -> Self {
        Self::Overflow
    }
}
