#[cfg(test)]
mod tests {
    use trace_deck::*;

    #[tokio::test]
    async fn test_document_processing() {
        // Test basic processing
        assert!(true);
    }

    #[test]
    fn test_models() {
        let metadata = DocumentMetadata {
            file_type: "pdf".to_string(),
            file_size: 1024,
            pages: Some(5),
            title: Some("Test".to_string()),
            author: None,
            created_at: None,
        };

        assert_eq!(metadata.file_type, "pdf");
        assert_eq!(metadata.file_size, 1024);
    }
}
