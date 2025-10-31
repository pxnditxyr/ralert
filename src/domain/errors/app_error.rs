#[derive(Debug)]
pub enum AppError {
    BadRequest(String),
    InternalServerError(String),
    NotImplemented(String),
    Unauthorized(String),
    NotFound(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::BadRequest(msg) => write!(f, "BadRequest: {msg}"),
            AppError::InternalServerError(msg) => write!(f, "InternalServerError: {msg}"),
            AppError::NotImplemented(msg) => write!(f, "NotImplemented: {msg}"),
            AppError::Unauthorized(msg) => write!(f, "Unauthorized: {msg}"),
            AppError::NotFound(msg) => write!(f, "NotFound: {msg}"),
        }
    }
}

impl std::error::Error for AppError {}
