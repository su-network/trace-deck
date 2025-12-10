use thiserror::Error;

#[derive(Error, Debug)]
pub enum TraceDeckError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("PDF error: {0}")]
    PdfError(String),

    #[error("DOCX error: {0}")]
    DocxError(String),

    #[error("Image error: {0}")]
    ImageError(String),

    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, TraceDeckError>;
