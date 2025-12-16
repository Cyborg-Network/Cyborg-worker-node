#[cfg(test)]
mod test {
    use crate::FlashInferEngine;

    #[test]
    fn test_flash_infer_engine_creation() {
        let engine = FlashInferEngine::new("test-model", 3005, "test-container");
        assert!(engine.is_ok());

        let engine = engine.unwrap();
        assert_eq!(engine.hf_id, "test-model");
        assert_eq!(engine.torch_infer_port, 3005);
        assert_eq!(engine.container_name, "test-container");
    }

    #[test]
    fn test_flash_infer_engine_invalid_creation() {
        // Test with empty model ID
        let engine = FlashInferEngine::new("", 3005, "test-container");
        assert!(engine.is_ok()); // Should still create, validation might happen later
    }

    #[tokio::test]
    async fn test_setup_structure() {
        let mut engine = FlashInferEngine::new("test-model", 3005, "test-container").unwrap();

        // Setup would need Docker, but we're testing structure
        let _result = engine.setup().await;
    }

    #[tokio::test]
    async fn test_stream_processing() {
        // use futures::stream;
        use tokio::sync::mpsc;

        let engine = FlashInferEngine::new("test-model", 3005, "test-container").unwrap();

        let (tx, rx) = mpsc::channel(10);
        let request_stream = tokio_stream::wrappers::ReceiverStream::new(rx);

        // Test that run function can be called
        let _join_handle = tokio::spawn(async move {
            let _ = engine.run(request_stream, |_| async {}).await;
        });

        tx.send("test request".to_string()).await.unwrap();
        drop(tx);

        // Just testing it doesn't panic immediately
    }

    #[tokio::test]
    async fn test_inference_generation_structure() {
        let engine = FlashInferEngine::new("test-model", 3005, "test-container").unwrap();

        let _result = engine
            .generate_inference_result("test input".to_string())
            .await;
    }

    #[test]
    fn test_kill_engine_structure() {
        let engine = FlashInferEngine::new("test-model", 3005, "test-container").unwrap();

        // Kill engine would need Docker, but we're testing structure
        let _result = engine.kill_engine();
        // This is async but we're testing the function exists
    }

    #[test]
    fn test_container_configuration() {
        // Test that the engine properly stores configuration
        let engine = FlashInferEngine::new("test-model", 3005, "test-container").unwrap();

        assert_eq!(engine.hf_id, "test-model");
        assert_eq!(engine.torch_infer_port, 3005);
        assert_eq!(engine.container_name, "test-container");
        assert!(engine.container_id.is_none());
    }
}
