//! Error types for docx2md.

use thiserror::Error;

/// Result type for docx2md operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Error types that can occur during DOCX to Markdown conversion.
#[derive(Error, Debug)]
pub enum Error {
    /// Error occurred while parsing DOCX file.
    #[error("Failed to parse DOCX file: {0}")]
    DocxParse(String),

    /// Error occurred during file I/O operations.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Error occurred during conversion.
    #[error("Conversion error: {0}")]
    Conversion(String),

    /// Relationship not found in document.
    #[error("Relationship not found: {0}")]
    RelationshipNotFound(String),

    /// A note/comment reference target was not found in the source document.
    #[error("Missing reference: {0}")]
    MissingReference(String),

    #[error("ZIP error: {0}")]
    Zip(#[from] zip::result::ZipError),

    /// Media file not found in DOCX archive.
    #[error("Media not found: {0}")]
    MediaNotFound(String),
}
