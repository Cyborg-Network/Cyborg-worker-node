use std::sync::Arc;

use tokio::sync::RwLock;

use crate::{error::Result, parent_runtime::storage_interactor};
use types::{
    substrate_interface::api::{
        runtime_types::cyborg_primitives::task::FlashInferTask,
        runtime_types::cyborg_primitives::task::OpenInferenceTask,
        task_management::events::task_scheduled::TaskKind,
    },
    CurrentTask,
};

pub async fn process_task(task: Arc<RwLock<CurrentTask>>) -> Result<()> {
    match &task.read().await.task_type {
        TaskKind::OpenInference(oi_task) => match oi_task {
            OpenInferenceTask::Onnx(onnx_task) => {
                storage_interactor::onnx::download_onnx_model(onnx_task).await?;
                Ok(())
            }
        },
        TaskKind::FlashInfer(fi_task) => match fi_task {
            FlashInferTask::Huggingface(_huggingface_task) => {
                println!("Received FlashInfer Huggingface Task, passing download responsibility on to docker container.");
                Ok(())
            }
        },
        TaskKind::CyCloud(_) => {
            println!("Received CyCloud task, passing responsibility to CyCloud engine");
            Ok(())
        }
    }
}
