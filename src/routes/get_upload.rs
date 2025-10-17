use std::path::Path;

use crate::api_error::ApiError;
use crate::database::postgresql::{PgPool, PgPooled};
use crate::database::schemas::uploads::dsl as uploads_dsl;
use crate::utils::files::{save_temp_file, verify_file};
use actix_multipart::form::MultipartForm;
use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::text::Text;
use actix_web::http::StatusCode;
use actix_web::web::ThinData;
use actix_web::{HttpResponse, post};
use diesel::dsl::exists;
use diesel::query_dsl::methods::{FilterDsl, SelectDsl};
use diesel::{ExpressionMethods, select};
use diesel_async::RunQueryDsl;
use uuid::Uuid;

#[derive(Debug, MultipartForm)]
pub struct Info {
    pub uuid: Text<String>,
    pub file: TempFile,
}

#[post("/get_upload")]
pub async fn get_upload(
    MultipartForm(info): MultipartForm<Info>,
    ThinData(pool): ThinData<PgPool>,
) -> Result<HttpResponse, ApiError> {
    let uuid = Uuid::parse_str(&info.uuid)?;
    let mut conn: PgPooled = pool.get().await?;

    let exist: bool = select(exists(
        uploads_dsl::uploads.filter(uploads_dsl::uuid.eq(uuid)),
    ))
    .get_result(&mut conn)
    .await?;

    if !exist {
        return Err(ApiError::new_empty(StatusCode::NOT_FOUND));
    }

    let (expiration, getted): (i64, bool) = uploads_dsl::uploads
        .select((uploads_dsl::expiration, uploads_dsl::getted))
        .filter(uploads_dsl::uuid.eq(uuid))
        .get_result(&mut conn)
        .await?;

    if chrono::Utc::now().timestamp() > expiration || getted == true {
        return Err(ApiError::new_empty(StatusCode::NOT_FOUND));
    }

    let file_name: String = info
        .file
        .file_name
        .clone()
        .unwrap_or_else(|| "temp.zip".to_string());

    let file_path = Path::new("apps/files").join(&file_name);

    let mut new_file_path = file_path.to_owned();
    new_file_path.set_file_name(uuid.to_string());

    let temp_file_path = save_temp_file(info.file, new_file_path).await?;
    verify_file(&temp_file_path).await?;

    diesel::update(uploads_dsl::uploads)
        .filter(uploads_dsl::uuid.eq(uuid))
        .set(uploads_dsl::getted.eq(true))
        .execute(&mut conn)
        .await?;

    Ok(HttpResponse::Ok().content_type("text/plain").body("Done"))
}
