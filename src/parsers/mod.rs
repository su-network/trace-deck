use crate::{models::*, error::*};
use std::path::Path;
use image::{ImageReader, GenericImageView};

pub struct DocumentParser {
    file_path: String,
    file_type: String,
}

impl DocumentParser {
    pub fn new(path: &str) -> Result<Self> {
        let ext = Path::new(path)
            .extension()
            .and_then(|e| e.to_str())
            .ok_or_else(|| TraceDeckError::ParseError("No file extension".into()))?
            .to_lowercase();

        Ok(Self {
            file_path: path.to_string(),
            file_type: ext,
        })
    }

    pub async fn extract(&self) -> Result<ExtractedContent> {
        match self.file_type.as_str() {
            "pdf" => self.parse_pdf().await,
            "docx" => self.parse_docx().await,
            "png" | "jpg" | "jpeg" | "webp" | "gif" => self.parse_image().await,
            _ => Err(TraceDeckError::UnsupportedFormat(self.file_type.clone())),
        }
    }

    async fn parse_pdf(&self) -> Result<ExtractedContent> {
        // PDF parsing logic
        let metadata = DocumentMetadata {
            file_type: "pdf".to_string(),
            file_size: std::fs::metadata(&self.file_path)?.len(),
            pages: Some(1), // Extract actual page count
            title: None,
            author: None,
            created_at: None,
        };

        Ok(ExtractedContent {
            text: String::new(), // Extract text from PDF
            images: vec![],      // Extract images from PDF
            tables: vec![],      // Extract tables from PDF
            metadata,
        })
    }

    async fn parse_docx(&self) -> Result<ExtractedContent> {
        // DOCX parsing logic
        let metadata = DocumentMetadata {
            file_type: "docx".to_string(),
            file_size: std::fs::metadata(&self.file_path)?.len(),
            pages: None,
            title: None,
            author: None,
            created_at: None,
        };

        Ok(ExtractedContent {
            text: String::new(),
            images: vec![],
            tables: vec![],
            metadata,
        })
    }

    async fn parse_image(&self) -> Result<ExtractedContent> {
        let img = ImageReader::open(&self.file_path)?
            .decode()
            .map_err(|e| TraceDeckError::ImageError(e.to_string()))?;

        let (width, height) = img.dimensions();

        let metadata = DocumentMetadata {
            file_type: self.file_type.clone(),
            file_size: std::fs::metadata(&self.file_path)?.len(),
            pages: Some(1),
            title: None,
            author: None,
            created_at: None,
        };

        let image_data = ImageData {
            id: "img_0".to_string(),
            format: self.file_type.clone(),
            width,
            height,
            data: vec![], // Serialize image
        };

        Ok(ExtractedContent {
            text: String::new(),
            images: vec![image_data],
            tables: vec![],
            metadata,
        })
    }
}
