use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use diesel::result::Error as DieselError;
use diesel_async::pooled_connection::bb8::RunError as BB8RunError;
use serde_json::json;
use std::env::VarError;
use std::fmt;
use std::time::SystemTimeError;

#[derive(Debug)]
pub struct ApiError {
    pub status_code: StatusCode,
    pub log_message: Option<String>,
    pub http_message: Option<String>,
}

impl ApiError {
    #[must_use]
    pub const fn new(
        status_code: StatusCode,
        log_message: Option<String>,
        http_message: Option<String>,
    ) -> Self {
        Self {
            status_code,
            log_message,
            http_message,
        }
    }

    pub fn new_log<S: Into<String>>(status_code: StatusCode, log_message: S) -> Self {
        Self {
            status_code,
            log_message: Some(log_message.into()),
            http_message: None,
        }
    }

    pub fn new_message<S: Into<String>>(status_code: StatusCode, http_message: S) -> Self {
        Self {
            status_code,
            log_message: None,
            http_message: Some(http_message.into()),
        }
    }

    pub fn new_unknown<S: Into<String>>(status_code: StatusCode, log_message: S) -> Self {
        Self {
            status_code,
            log_message: Some(log_message.into()),
            http_message: Some(
                "Une erreur inconnue s'est produite, veuillez réessayer.".to_owned(),
            ),
        }
    }

    #[must_use]
    pub const fn new_empty(status_code: StatusCode) -> Self {
        Self {
            status_code,
            log_message: None,
            http_message: None,
        }
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(log_message) = &self.log_message {
            f.write_str(log_message.as_str())
        } else if let Some(http_message) = &self.http_message {
            f.write_str(http_message.as_str())
        } else {
            f.write_str("No error messages.")
        }
    }
}

impl From<DieselError> for ApiError {
    fn from(error: DieselError) -> Self {
        Self::new_log(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("A diesel error has occurred: {error}"),
        )
    }
}
impl From<VarError> for ApiError {
    fn from(error: VarError) -> Self {
        Self::new_log(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("A var error has occurred: {error}"),
        )
    }
}
impl From<std::io::Error> for ApiError {
    fn from(error: std::io::Error) -> Self {
        Self::new_log(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("A std io error has occurred: {error}"),
        )
    }
}
impl From<SystemTimeError> for ApiError {
    fn from(error: SystemTimeError) -> Self {
        Self::new_log(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("A generate unixtime error has occurred: {error}"),
        )
    }
}
impl From<BB8RunError> for ApiError {
    fn from(error: BB8RunError) -> Self {
        Self::new_log(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to retrieve a database connection from the pool: {error}"),
        )
    }
}
impl From<uuid::Error> for ApiError {
    fn from(error: uuid::Error) -> Self {
        Self::new_log(
            StatusCode::BAD_REQUEST,
            format!("Error with the given uuid: {error}"),
        )
    }
}
impl From<askama::Error> for ApiError {
    fn from(error: askama::Error) -> Self {
        Self::new_log(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error while parsing template: {error}"),
        )
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        if let Some(log_message) = &self.log_message {
            log::error!("{}", log_message);
        }

        self.http_message.as_ref().map_or_else(
            || HttpResponse::build(self.status_code).finish(),
            |http_message| {
                HttpResponse::build(self.status_code)
                    .json(json!({"status": "failed", "message": http_message}))
            },
        )
    }
}
