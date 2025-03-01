use core::array::TryFromSliceError;
use core::fmt::Display;
use std::error::Error as StdError;
use std::fmt::Debug;
use std::io::Error as StdioError;
use std::num::{ParseFloatError, ParseIntError};

pub type SqliteResult<T> = Result<T, SqliteError>;

#[derive(Debug)]
pub enum SqliteError {
    EmptyDb,
    InvalidFileUriMode,
    HeaderValidationError(String),
    TryFromSliceError(TryFromSliceError),
    ParseFloatError(ParseFloatError),
    ParseIntError(ParseIntError),
    StdioError(StdioError),
    Custom(String),
    ParsingField(FieldParsingError),
    InvalidPayloadSize(InvalidPayloadSizeError),
    SqlParser(SqlParserError),
}

#[derive(Debug)]
pub struct SqlParserError(pub String);

#[derive(Debug)]
pub struct FieldParsingError(pub String);

#[derive(Debug)]
pub struct InvalidPayloadSizeError(pub String);

impl Display for SqliteError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // TODO
        write!(f, "{:?}", self)
    }
}

impl From<TryFromSliceError> for SqliteError {
    fn from(error: TryFromSliceError) -> Self {
        Self::TryFromSliceError(error)
    }
}

impl From<ParseFloatError> for SqliteError {
    fn from(error: ParseFloatError) -> Self {
        Self::ParseFloatError(error)
    }
}

impl From<ParseIntError> for SqliteError {
    fn from(error: ParseIntError) -> Self {
        Self::ParseIntError(error)
    }
}

impl StdError for SqliteError {}

impl From<StdioError> for SqliteError {
    fn from(io_error: StdioError) -> Self {
        Self::StdioError(io_error)
    }
}
