use crate::{models::*, error::*};

pub fn process(content: &ExtractedContent) -> Result<ProcessedData> {
    // Process extracted content
    let text_blocks = vec![
        TextBlock {
            content: content.text.clone(),
            block_type: "content".to_string(),
            confidence: 0.95,
        },
    ];

    let visual_elements = content
        .images
        .iter()
        .enumerate()
        .map(|(idx, img)| VisualElement {
            element_type: "image".to_string(),
            position: (0, idx as u32 * 100),
            size: (img.width, img.height),
        })
        .collect();

    let structure = DocumentStructure {
        sections: vec![],
        total_pages: content.metadata.pages.unwrap_or(1),
        language: None,
    };

    Ok(ProcessedData {
        text_blocks,
        visual_elements,
        structure,
    })
}
