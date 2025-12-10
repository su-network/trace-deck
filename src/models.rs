use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentMetadata {
    pub file_type: String,      // pdf, docx, png, jpg, etc.
    pub file_size: u64,         // bytes
    pub pages: Option<u32>,     // for PDFs
    pub title: Option<String>,
    pub author: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedContent {
    pub text: String,
    pub images: Vec<ImageData>,
    pub tables: Vec<TableData>,
    pub metadata: DocumentMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageData {
    pub id: String,
    pub format: String,
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableData {
    pub rows: Vec<Vec<String>>,
    pub headers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessedData {
    pub text_blocks: Vec<TextBlock>,
    pub visual_elements: Vec<VisualElement>,
    pub structure: DocumentStructure,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextBlock {
    pub content: String,
    pub block_type: String,  // heading, paragraph, bullet, etc.
    pub confidence: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualElement {
    pub element_type: String,  // image, chart, diagram
    pub position: (u32, u32),
    pub size: (u32, u32),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentStructure {
    pub sections: Vec<Section>,
    pub total_pages: u32,
    pub language: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Section {
    pub title: String,
    pub content_blocks: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentResult {
    pub extracted: ExtractedContent,
    pub processed: ProcessedData,
    pub processing_time_ms: u128,
}
