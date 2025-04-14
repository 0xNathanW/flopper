use thiserror::Error;
use crate::Card;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Duplicate card: {0:?}")]
    DuplicateCard(Card),

    #[error("Invalid board size: {0}. Must be either 0, 3, 4 or 5.")]
    InvalidBoardSize(usize),

    #[error("Invalid hand size: {0}. Must be between 5 and 7.")]
    InvalidHandSize(usize),

    #[error("Error loading lookup table: {0}")]
    LookupTableError(#[from] std::io::Error),

    #[error("Could not find lookup table, should have been generated at build @ {0} ")]
    LookupTableNotFound(String),

    #[error("No lookup path set, should have been generated at build, or set existing env variable POKER_LOOKUP_TABLE_PATH")]
    LookupPathNotSet,
}

pub type Result<T> = std::result::Result<T, Error>;