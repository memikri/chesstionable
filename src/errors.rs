use thiserror::Error;

#[derive(Error, Debug)]
pub enum ChessError {
    #[error("Could not parse '{0}' as a {1}")]
    ParseError(String, &'static str),
    #[error("Invalid FEN: {0}")]
    InvalidFEN(String),
}
