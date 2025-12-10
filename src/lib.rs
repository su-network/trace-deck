// Main library interface
pub mod parsers;
pub mod extractors;
pub mod processors;
pub mod models;
pub mod error;
pub mod ui;

pub use models::*;
pub use error::*;
pub use parsers::*;
pub use extractors::*;
pub use processors::*;
pub use ui::*;

/// Process any document (PDF, DOCX, Images)
pub async fn process_document(path: &str) -> Result<DocumentResult> {
    let start = std::time::Instant::now();
    let parser = parsers::DocumentParser::new(path)?;
    let extracted = parser.extract().await?;
    let processed = processors::process(&extracted)?;
    let processing_time_ms = start.elapsed().as_millis();
    
    Ok(DocumentResult {
        extracted,
        processed,
        processing_time_ms,
    })
}
