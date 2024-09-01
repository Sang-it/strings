use crate::runtime::lexer::Location;
use std::{path::PathBuf, sync::Arc};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error(transparent)]
    TreeError { source: syntree::Error },

    #[error("Error in generated source: {0}")]
    DataError(&'static str),

    #[error("Error in input: {cause}")]
    PredictionError { cause: String },

    #[error("Syntax error(s)")]
    SyntaxErrors { entries: Vec<SyntaxError> },

    #[error("Unprocessed input is left after parsing has finished")]
    UnprocessedInput {
        input: Box<FileSource>,
        last_token: Box<Location>,
    },

    #[error("{context}Tried to pop from an empty scanner stack")]
    PopOnEmptyScannerStateStack {
        context: String,
        input: FileSource,
        source: LexerError,
    },

    #[error("Too many errors: {count}")]
    TooManyErrors { count: usize },

    #[error("Error recovery failed")]
    RecoveryFailed,

    #[error("{0}")]
    InternalError(String),
}

#[derive(Error, Debug)]
pub enum LexerError {
    #[error("No valid token read")]
    TokenBufferEmptyError,

    #[error("{0}")]
    InternalError(String),

    #[error("Lookahead exceeds its maximum")]
    LookaheadExceedsMaximum,

    #[error("Lookahead exceeds token buffer length")]
    LookaheadExceedsTokenBufferLength,

    #[error("pop_scanner: Tried to pop from an empty scanner stack!")]
    ScannerStackEmptyError,

    #[error("{0}")]
    RecoveryError(String),
}

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    ParserError(#[from] ParserError),
    #[error(transparent)]
    LexerError(#[from] LexerError),
    #[error(transparent)]
    UserError(#[from] anyhow::Error),
}

#[derive(Error, Debug, Default)]
#[error("{cause}")]
pub struct SyntaxError {
    pub cause: String,
    pub input: Option<Box<FileSource>>,
    pub error_location: Box<Location>,
    pub unexpected_tokens: Vec<UnexpectedToken>,
    pub expected_tokens: TokenVec,
    pub source: Option<Box<Error>>,
}

#[derive(Debug)]
pub struct FileSource {
    pub file_name: Arc<PathBuf>,
    pub input: String,
}

#[derive(Debug)]
pub struct UnexpectedToken {
    pub name: String,
    pub token_type: String,
    pub token: Location,
}

#[derive(Debug, Default)]
pub struct TokenVec(Vec<String>);
