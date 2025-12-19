use std::num::ParseIntError;

use thiserror::Error;
use tracing_error::SpanTrace;

#[derive(Debug, thiserror::Error)]
#[error("{error}\nTrace:\n{trace}")]
pub struct TracedError<E> {
    pub error: E,
    pub trace: SpanTrace,
}

impl<E> From<E> for TracedError<AppError>
where
    E: Into<AppError>,
{
    fn from(source: E) -> Self {
        TracedError {
            error: source.into(),
            trace: SpanTrace::capture(),
        }
    }
}

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum AppError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("invalid config: {0}")]
    Config(String),

    #[error("ParseIntError: {0}")]
    ParseInt(#[from] ParseIntError),
}

pub type Rs<T> = Result<T, TracedError<AppError>>;
