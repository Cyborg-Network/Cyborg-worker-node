#[cfg(test)]
mod test {
    use crate::client::{get_help_message, sample_from_logits};
    use crate::{TensorData, TritonClient};

    // use super::*;
    // use std::collections::HashMap;
    use std::path::PathBuf;
    use tokio;

    #[test]
    fn test_tensor_data_serialization() {
        let data_f32 = TensorData::F32(vec![1.0, 2.0, 3.0]);
        let serialized = data_f32.to_serializable();
        assert!(serialized.is_array());

        let data_i32 = TensorData::I32(vec![1, 2, 3]);
        let serialized = data_i32.to_serializable();
        assert!(serialized.is_array());

        let data_str = TensorData::Str(vec!["test".to_string()]);
        let serialized = data_str.to_serializable();
        assert!(serialized.is_array());
    }

    #[tokio::test]
    async fn test_triton_client_initialization() {
        // Test client initialization (would need Triton server for full test)
        let _client = TritonClient::new("http://localhost:8000/v2", PathBuf::from("/tmp"));
        // This will fail without server, but we're testing the structure
    }

    #[test]
    fn test_tokenizer_functions() {
        use crate::tokenizer::make_llm_inputs_with_past;
        // use std::collections::HashMap;

        let token_ids = vec![1, 2, 3];
        let inputs = make_llm_inputs_with_past(token_ids, None);

        assert!(inputs.contains_key("input_ids"));
        assert!(inputs.contains_key("attention_mask"));
        assert!(inputs.contains_key("position_ids"));
    }

    #[test]
    fn test_help_message_generation() {
        let help_msg = get_help_message();
        assert!(help_msg.contains("infer"));
        assert!(help_msg.contains("metadata"));
        assert!(help_msg.contains("load"));
    }

    #[test]
    fn test_sample_from_logits() {
        let logits = vec![1.0, 2.0, 3.0];
        let sampled = sample_from_logits(&logits, 1.0);

        // Should return a valid index
        assert!(sampled >= 0);
        assert!(sampled < logits.len() as i64);
    }
}
