use thiserror::Error;

#[derive(Debug, Error)]
pub enum FileManagerError {
    #[error("AWS S3 error: {0}")]
    S3Error(
        #[from]
        aws_smithy_runtime_api::client::result::SdkError<
            aws_sdk_s3::operation::get_object::GetObjectError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    ),
    #[error("UTF-8 error: {0}")]
    Utf8Error(#[from] std::str::Utf8Error),

    #[error("ByteStream read error: {0}")]
    ByteStreamError(#[from] aws_sdk_s3::primitives::ByteStreamError),
}
