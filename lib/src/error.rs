use thiserror::Error;

#[derive(Error, Debug)]
pub enum CompilerError {
    #[error("Syntax error: {description:?}")]
    ParserError { description: String },
}
