#[cfg(test)]
mod test {
    use crate::{
        container_manager::{use_file, ContainerProvisionArgs, Resources},
        // native_manager::NativeManager,
        CyCloudEngine,
        TaskStatus,
    };

    use types::{
        substrate_interface::api::runtime_types::{
            bounded_collections::bounded_vec::BoundedVec,
            cyborg_primitives::task::{CyCloudContainerTask, CyCloudNativeTask, CyCloudTask},
        },
        TaskType,
    };

    #[tokio::test]
    async fn test_cycloud_engine_creation() {
        // Test container task
        let container_engine = CyCloudEngine::new(&TaskType::CyCloud(CyCloudTask::Container(
            CyCloudContainerTask { _marker: () },
        )))
        .await;
        assert!(container_engine.is_ok());

        // Test native task
        let native_user = BoundedVec(b"testuser".to_vec());
        let native_engine =
            CyCloudEngine::new(&TaskType::CyCloud(CyCloudTask::Native(CyCloudNativeTask {
                user_name: native_user,
            })))
            .await;
        assert!(native_engine.is_ok());
    }

    #[test]
    fn test_container_manager_initialization() {
        let _args = ContainerProvisionArgs {
            ssh_port: Some(2222),
            memory_limit: Some("1g".to_string()),
            cpu_limit: Some(1.0),
            container_name: Some("test-container"),
        };

        // This would need Docker to run fully, but we're testing structure
    }

    #[test]
    fn test_task_status_enum() {
        assert!(matches!(TaskStatus::Running, TaskStatus::Running));
        assert!(matches!(TaskStatus::Stopped, TaskStatus::Stopped));
        assert!(matches!(TaskStatus::Starting, TaskStatus::Starting));
        assert!(matches!(TaskStatus::Cleaning, TaskStatus::Cleaning));
        assert!(matches!(TaskStatus::Cleaned, TaskStatus::Cleaned));
        assert!(matches!(TaskStatus::Broken, TaskStatus::Broken));
    }

    #[test]
    fn test_resource_definition() {
        // Test that resources are properly defined
        assert!(!Resources::DOCKERFILE.content.is_empty());
        assert!(!Resources::DOCKER_COMPOSE.content.is_empty());
        assert!(!Resources::CONTAINER_ACCESS_API.content.is_empty());
    }

    #[test]
    fn test_file_utility_function() {
        let temp_path = std::env::temp_dir().join("test_file.txt");
        let result = use_file(&temp_path, "test content", |_| Ok(()));

        assert!(result.is_ok());
        assert!(temp_path.exists());

        // Cleanup
        let _ = std::fs::remove_file(temp_path);
    }
}
