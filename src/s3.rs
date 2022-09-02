/* used to interact with s3 buckets.

the plan:
1. multipart upload saves file to temp dir on server
2. once complete server then uploads it to the bucket

*/

use anyhow::{Result, Error};

pub async fn _upload_s3(_bucket_name: &str, _file_name: &str) -> Result<(), Error> {
    Ok(())
}

