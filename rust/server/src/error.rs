use actix_web::error::BlockingError;
use actix_web::{HttpResponse, ResponseError};
use diesel::result::DatabaseErrorKind;
use std::error::Error;
use std::fmt::{Display, Formatter};
use tracing::{debug, error};

pub type ServiceResult<T> = Result<T, ServiceErr>;

#[derive(Debug)]
pub enum ServiceErr {
    // 400
    NotFound,
    NoAdminPermissions,
    BadRequest(&'static str),
    Conflict(&'static str),
    Unauthorized(&'static str),
    TokenExpiredError,
    JWTokenError,
    // 500
    JWTCreationError(jsonwebtoken::errors::Error),
    ConnectionNotFound(r2d2::Error),
    DbActionFailed(diesel::result::Error),
    InternalServerError(String),
    IntoDTOError(String),
}

impl std::error::Error for ServiceErr {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ServiceErr::ConnectionNotFound(err) => Some(err),
            ServiceErr::DbActionFailed(err) => Some(err),
            ServiceErr::JWTCreationError(err) => Some(err),
            _ => None,
        }
    }
}

impl Display for ServiceErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ServiceErr::ConnectionNotFound(err) => format!("ConnectionNotFound: {}", err),
                ServiceErr::DbActionFailed(err) => format!("DbActionFailed: {}", err),
                ServiceErr::JWTCreationError(err) => format!("JWTCreationError: {}", err),
                ServiceErr::TokenExpiredError => "token-expired".to_string(),
                ServiceErr::JWTokenError => "invalid-token".to_string(),
                ServiceErr::NotFound => "Not found".to_string(),
                ServiceErr::InternalServerError(msg) => format!("Internal Server Error: {}", msg),
                ServiceErr::Unauthorized(msg) => msg.to_string(),
                ServiceErr::IntoDTOError(msg) => msg.to_string(),
                ServiceErr::BadRequest(msg) => msg.to_string(),
                ServiceErr::Conflict(msg) => msg.to_string(),
                ServiceErr::NoAdminPermissions => "perms/no-admin".to_string(),
            }
        )
    }
}

impl ResponseError for ServiceErr {
    fn error_response(&self) -> HttpResponse {
        debug!(err = %self, "an error occurred");
        match self {
            ServiceErr::TokenExpiredError => HttpResponse::Unauthorized().body("token-expired"),
            ServiceErr::JWTokenError => HttpResponse::Unauthorized().body("invalid-token"),
            ServiceErr::BadRequest(msg) => HttpResponse::BadRequest().body(*msg),
            ServiceErr::NotFound => HttpResponse::NotFound().body("Not Found"),
            ServiceErr::Unauthorized(msg) => HttpResponse::Unauthorized().body(*msg),
            ServiceErr::NoAdminPermissions => HttpResponse::Unauthorized().body("no-admin"),
            ServiceErr::Conflict(msg) => HttpResponse::Conflict().body(msg.to_string()),
            err => {
                error!(%err, "an error occurred");
                HttpResponse::InternalServerError().finish()
            }
        }
    }
}

impl From<diesel::result::Error> for ServiceErr {
    fn from(err: diesel::result::Error) -> Self {
        match err {
            diesel::result::Error::NotFound => {
                debug!(%err, "Handled Db error occurred");
                Self::NotFound
            }
            diesel::result::Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _) => {
                debug!(%err, "Handled Db error occurred");
                Self::Conflict("already-exists")
            }
            diesel::result::Error::DatabaseError(DatabaseErrorKind::ForeignKeyViolation, _) => {
                debug!(%err, "Handled Db error occurred");
                Self::Conflict("does-not-exist")
            }
            _ => Self::DbActionFailed(err),
        }
    }
}

impl From<r2d2::Error> for ServiceErr {
    fn from(err: r2d2::Error) -> Self {
        Self::ConnectionNotFound(err)
    }
}

impl From<uuid::Error> for ServiceErr {
    fn from(_: uuid::Error) -> Self {
        Self::BadRequest("invalid-uuid")
    }
}

impl<E: std::fmt::Debug> From<BlockingError<E>> for ServiceErr
where
    Self: From<E>,
{
    fn from(err: BlockingError<E>) -> Self {
        match err {
            BlockingError::Error(inner) => inner.into(),
            BlockingError::Canceled => Self::InternalServerError("Thread pool is gone".to_string()),
        }
    }
}
