use tracing::instrument;

use crate::err::Rs;

#[instrument]
pub async fn parse_file(content: &str) -> Rs<i32> {
    let num = content.parse()?;
    Ok(num)
}
