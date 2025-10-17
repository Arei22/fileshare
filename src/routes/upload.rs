use crate::api_error::ApiError;
use crate::database::postgresql::{PgPool, PgPooled};
use crate::database::schemas::uploads::dsl as uploads_dsl;
use crate::utils::templates::Upload;
use actix_web::http::StatusCode;
use actix_web::web::ThinData;
use actix_web::{HttpResponse, get, web};
use askama::Template;
use diesel::dsl::exists;
use diesel::query_dsl::methods::{FilterDsl, SelectDsl};
use diesel::{ExpressionMethods, select};
use diesel_async::RunQueryDsl;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct Info {
    pub uuid: String,
}

#[get("/upload")]
pub async fn upload(
    info: web::Query<Info>,
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

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(Upload {}.render()?))
}
