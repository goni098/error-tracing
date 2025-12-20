use std::num::ParseIntError;

use tracing_error::SpanTrace;

#[derive(Debug, thiserror::Error)]
#[error("{error}\nTrace:\n{trace}")]
pub struct TracedAppErr {
    error: AppErr,
    trace: SpanTrace,
}

impl<E> From<E> for TracedAppErr
where
    E: Into<AppErr>,
{
    fn from(source: E) -> Self {
        TracedAppErr {
            error: source.into(),
            trace: SpanTrace::capture(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AppErr {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("invalid config: {0}")]
    #[allow(dead_code)]
    Config(String),

    #[error("ParseIntError: {0}")]
    ParseInt(#[from] ParseIntError),
}

pub type Rs<T> = Result<T, TracedAppErr>;
