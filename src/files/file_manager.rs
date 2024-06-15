use super::FileManagerError;

pub struct S3FileManager {
    bucket: String,
    client: aws_sdk_s3::Client,
}

impl S3FileManager {
    pub async fn build(bucket: String) -> Self {
        let config = aws_config::load_from_env().await;
        let client = aws_sdk_s3::Client::new(&config);

        S3FileManager { bucket, client }
    }

    pub async fn get_file_content(
        &self,
        key: impl Into<String>,
    ) -> Result<String, FileManagerError> {
        let key = key.into();
        let response = self
            .client
            .get_object()
            .bucket(&self.bucket)
            .key(&key)
            .send()
            .await?;

        let bytes = response.body.collect().await?.into_bytes();
        let content = std::str::from_utf8(&bytes)?;

        Ok(content.to_string())
    }
}
