use crate::api_error::ApiError;
use actix_multipart::form::tempfile::TempFile;
use actix_web::http::StatusCode;
use infer::{Infer, MatcherType};
use std::path::{Path, PathBuf};

pub async fn save_temp_file(
    temp_file: TempFile,
    destination_path: impl AsRef<Path>,
) -> Result<PathBuf, ApiError> {
    let destination_path = destination_path.as_ref();
    if !destination_path.exists() {
        tokio::fs::create_dir_all(destination_path.parent().unwrap()).await?;
    }
    let mut destination_file = tokio::fs::File::create(&destination_path).await?;
    let mut source_file = tokio::fs::File::open(&temp_file.file).await?;
    tokio::io::copy(&mut source_file, &mut destination_file).await?;
    Ok(destination_path.to_path_buf())
}

pub async fn verify_file(file_path: impl AsRef<Path>) -> Result<bool, ApiError> {
    let file_content = tokio::fs::read(&file_path).await?;

    let Some(mime_type) = Infer::new().get(&file_content) else {
        tokio::fs::remove_file(file_path).await?;
        return Err(ApiError::new_empty(StatusCode::UNSUPPORTED_MEDIA_TYPE));
    };

    if mime_type.matcher_type() == MatcherType::Archive {
        return Ok(true);
    }

    Err(ApiError::new_empty(StatusCode::UNSUPPORTED_MEDIA_TYPE))
}
