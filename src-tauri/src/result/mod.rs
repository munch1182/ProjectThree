pub type AppResult<T, E = AppError> = std::result::Result<T, E>;

#[derive(Debug)]
pub struct AppError {
    pub msg: String,
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error: {}", self.msg)
    }
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self)
    }
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        AppError { msg: e.to_string() }
    }
}

impl From<std::num::ParseIntError> for AppError {
    fn from(e: std::num::ParseIntError) -> Self {
        AppError { msg: e.to_string() }
    }
}

impl From<std::net::AddrParseError> for AppError {
    fn from(e: std::net::AddrParseError) -> Self {
        AppError { msg: e.to_string() }
    }
}
