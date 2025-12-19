use tracing::instrument;

use crate::err::Rs;

#[instrument]
pub async fn parse_file(content: &str) -> Rs<i32> {
    content.parse().map_err(Into::into)
}
