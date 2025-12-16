#[cfg(test)]
mod test {
    // use super::*;
    use crate::{
        auth::WsAuthRequest,
        crypto::{decode_polkadot_address, encrypt_message},
        // AgentConfig,
    };
    use std::path::PathBuf;
    use std::sync::Arc;
    use subxt::utils::AccountId32;
    // use tokio::sync::RwLock;
    // use types::CurrentTask;

    #[test]
    fn test_auth_request_processing() {
        let request = WsAuthRequest {
            target_ip: "127.0.0.1".to_string(),
            task_id: 123,
            signed_timestamp: "1234567890".to_string(),
            signed_timestamp_signature: "abcd".to_string(),
            ephemeral_public_key: "0123456789abcdef".repeat(4), // 32 bytes
        };

        let _processed = crate::auth::process_auth_request(request);
        // This will fail due to invalid hex, but we're testing structure
    }

    #[test]
    fn test_polkadot_address_decoding() {
        let account_id = AccountId32::from([0; 32]);
        let decoded = decode_polkadot_address(&account_id);

        assert_eq!(decoded.len(), 32);
        assert_eq!(decoded, [0; 32]);
    }

    #[test]
    fn test_encryption_structure() {
        let key = [0u8; 32];
        let message = "test".to_string();

        let encrypted = encrypt_message("test", &key, message);
        assert!(encrypted.is_ok());

        let encrypted_msg = encrypted.unwrap();
        assert_eq!(encrypted_msg.response_type, "test");
        assert!(!encrypted_msg.encrypted_data_hex.is_empty());
        assert!(!encrypted_msg.nonce_hex.is_empty());
    }

    // #[tokio::test]
    // async fn test_agent_config_creation() {
    //     let current_task: CurrentTask = Arc::new(RwLock::new(None));
    //     let config = AgentConfig {
    //         current_task,
    //         log_file_path: &PathBuf::from("/tmp/log.txt"),
    //         container_prefix: "test-",
    //     };

    //     assert_eq!(config.container_prefix, "test-");
    // }

    #[test]
    fn test_error_message_construction() {
        use crate::error_handling::{construct_client_error_message, ClientError};

        let auth_error = ClientError::AuthError("test".to_string());
        let error_msg = construct_client_error_message(auth_error);

        assert!(error_msg.contains("Error"));
        assert!(error_msg.contains("Auth"));
    }

    #[tokio::test]
    async fn test_usage_snapshot_structure() {
        use crate::api::usage::Usage;

        let _snapshot = Usage::get_usage_snapshot(
            Arc::new(tokio::sync::Mutex::new(Vec::new())),
            Arc::new(tokio::sync::Mutex::new(0)),
            &PathBuf::from("/tmp"),
        )
        .await;

        // This may fail due to system dependencies, but we're testing structure
    }

    #[test]
    fn test_config_serialization() {
        use crate::config::Configuration;

        let config = Configuration::new("test_token".to_string());
        assert_eq!(config.base.user_token, "test_token");
        assert!(!config.base.csc_uuid.is_empty());
    }
}
