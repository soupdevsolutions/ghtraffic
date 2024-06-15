use super::FileManagerError;

#[allow(async_fn_in_trait)]
pub trait FileManager {
    async fn get_file_content(&self, key: impl Into<String>) -> Result<String, FileManagerError>;
}
