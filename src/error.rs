use pyo3::exceptions::PyException;
use pyo3::PyErr;
impl std::error::Error for EditAnkiError {}
impl std::fmt::Display for EditAnkiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EditAnkiError::AnkiError(e) => write!(f, "Anki lib error: {}", e),
            EditAnkiError::NoteType(e) => write!(f, "note type error: {}", e),
        }
    }
}
pub type Result<T> = std::result::Result<T, EditAnkiError>;

#[derive(Debug)]
pub enum EditAnkiError {
    AnkiError(String),
    NoteType(String),
}
impl std::convert::From<anki::error::AnkiError> for EditAnkiError {
    fn from(e: anki::error::AnkiError) -> Self {
        EditAnkiError::AnkiError(e.to_string())
    }
}
impl std::convert::From<EditAnkiError> for PyErr {
    fn from(e: EditAnkiError) -> Self {
        match e {
            EditAnkiError::AnkiError(e) => PyException::new_err(e),
            EditAnkiError::NoteType(e) => PyException::new_err(e),
        }
    }
}
