use serde::{Serialize, Serializer};

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("recurso não encontrado: {0}")]
    NotFound(String),
    #[error("requisição inválida: {0}")]
    InvalidRequest(String),
    #[error("operação não permitida: {0}")]
    Forbidden(String),
}

// Tauri serializes command errors as JSON for the frontend; expose a stable
// `{ kind, message }` shape instead of leaking Rust's Display formatting.
impl Serialize for AppError {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let (kind, message) = match self {
            AppError::NotFound(m) => ("not_found", m.clone()),
            AppError::InvalidRequest(m) => ("invalid_request", m.clone()),
            AppError::Forbidden(m) => ("forbidden", m.clone()),
        };
        let mut state = serializer.serialize_struct("AppError", 2)?;
        state.serialize_field("kind", kind)?;
        state.serialize_field("message", &message)?;
        state.end()
    }
}

pub type AppResult<T> = Result<T, AppError>;
