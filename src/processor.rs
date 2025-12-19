use tracing::instrument;

use crate::{err::Rs, reader::read_file};

#[instrument]
pub async fn process_config() -> Rs<i32> {
    read_file("config.txt").await
}
