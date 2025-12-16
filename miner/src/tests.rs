#[cfg(test)]
mod test {
    use crate::builder::validate_miner_type;
    // use crate::cli::Cli;
    use crate::error::Error;
    use crate::{
        builder::MinerBuilder, /* miner_types::Miner,*/ utils::tx_queue::TransactionQueue,
    };
    // use std::sync::Arc;
    // use subxt_signer::sr25519::Keypair;
    // use types::substrate_interface::api::edge_connect::calls::types::remove_miner::MinerId;

    #[test]
    fn test_validate_miner_type() {
        assert!(validate_miner_type("cloud").is_ok());
        assert!(validate_miner_type("edge").is_ok());
        assert!(validate_miner_type("invalid").is_err());
    }

    #[test]
    fn test_miner_builder_creation() {
        let builder = MinerBuilder::new();
        assert!(builder.miner_type.is_none());
        assert!(builder.parachain_url.is_none());
    }

    #[test]
    fn test_miner_builder_miner_type() {
        let builder = MinerBuilder::new().miner_type("cloud");
        assert!(builder.is_ok());

        let builder = MinerBuilder::new().miner_type("invalid");
        assert!(builder.is_err());
    }

    #[tokio::test]
    async fn test_transaction_queue_creation() {
        let queue = TransactionQueue::new();
        assert!(queue
            .enqueue(|| async { Ok(crate::utils::tx_queue::TxOutput::Success) })
            .await
            .is_ok());
    }

    // #[test]
    // fn test_cli_parsing() {
    //     use clap::Parser;

    //     let args = vec![
    //         "cyborg-miner",
    //         "startminer",
    //         "--parachain-url",
    //         "ws://localhost:9944",
    //         "--account-seed",
    //         "//Alice",
    //         "--miner-type",
    //         "cloud",
    //         "--miner-uuid",
    //         "CL-1234abcd",
    //     ];

    //     let cli = Cli::try_parse_from(args);
    //     assert!(cli.is_ok());
    // }

    // #[test]
    // fn test_task_container_name_generation() {
    //     use crate::utils::task_handling::return_task_container_name;

    //     let container_name = return_task_container_name(123);
    //     assert!(container_name.starts_with("task-"));
    //     assert!(container_name.ends_with("123"));
    // }

    #[test]
    fn test_error_conversions() {
        let io_error: Error = std::io::Error::new(std::io::ErrorKind::NotFound, "test").into();
        assert!(matches!(io_error, Error::Io(_)));

        let custom_error: Error = "test error".into();
        assert!(matches!(custom_error, Error::Custom(_)));
    }
}

// #[cfg(test)]
// mod miner_types_tests {
//     use std::sync::Arc;

//     use crate::miner_types::{Miner, ParentRuntime};

//     use super::*;
//     use sp_runtime::BoundedVec;
//     use subxt_signer::sr25519::Keypair;
//     use tokio::sync::RwLock;
//     use types::CurrentTask;

//     #[tokio::test]
//     async fn test_miner_activation() {
//         let miner = Arc::new(Miner {
//             miner_type: Arc::new(types::substrate_interface::api::runtime_types::cyborg_primitives::miner::MinerType::Cloud),
//             keypair: Arc::new(Keypair::generate()),
//             parent_runtime: Arc::new(RwLock::new(ParentRuntime { port: None })),
//             identity: Arc::new(types::MinerIdentity {
//                 miner_owner: Default::default(),
//                 miner_id: vec![],
//                 miner_type: types::substrate_interface::api::runtime_types::cyborg_primitives::miner::MinerType::Cloud,
//             }),
//             current_task: Arc::new(RwLock::new(None)),
//         });

//         // Test activation
//         let task = CurrentTask {
//             task_type: types::substrate_interface::api::task_management::events::task_scheduled::TaskKind::OpenInference(
//                 types::substrate_interface::api::runtime_types::cyborg_primitives::task::OpenInferenceTask::Onnx(
//                     types::substrate_interface::api::runtime_types::cyborg_primitives::task::OnnxTask {
//                         storage_location_identifier: BoundedVec(vec![]),
//                     }
//                 )
//             ),
//             task_owner: Default::default(),
//             id: 1,
//             container_name: "test".to_string(),
//         };

//         miner.activate_task(task).await;
//         assert!(miner.is_active().await);

//         // Test deactivation
//         let deactivated = miner.deactivate_task().await;
//         assert!(deactivated.is_some());
//         assert!(!miner.is_active().await);
//     }
// }
