use tracing::instrument;

use crate::{err::Rs, parser::parse_file};

#[instrument]
pub async fn read_file(path: &str) -> Rs<i32> {
    let c = std::fs::read_to_string(path)?;
    parse_file(&c).await
}
