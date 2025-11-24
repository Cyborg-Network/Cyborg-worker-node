use bollard::models::{ContainerCreateBody, HostConfig, PortBinding};
use bollard::query_parameters::{
    CreateContainerOptionsBuilder, InspectContainerOptions, RemoveContainerOptions,
    StartContainerOptions,
};
use bollard::Docker;
use futures::{stream::StreamExt, Future, Stream};
use serde_json::json;
use std::collections::HashMap;

const PORT: u16 = 3005;

#[derive(Debug)]
pub struct FlashInferEngine {
    hf_id: String,
    torch_infer_port: u16,
    // Identifier for naming the container, to be able to nuke it from outside the engine
    container_name: String,
    // Identifier for the running container, used internally by the engine
    container_id: Option<String>,
    client: reqwest::Client,
}

impl FlashInferEngine {
    /// Creates a new `FlashInferEngine` instance.
    ///
    /// # Arguments
    /// * `hf_id` - The huggingface identifier of the model
    ///
    /// # Returns
    /// A new `FlashInferEngine` instance
    pub fn new(
        hf_id: &str,
        port: u16,
        containe_name: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();

        Ok(Self {
            hf_id: hf_id.to_string(),
            torch_infer_port: port,
            container_name: containe_name.to_string(),
            container_id: None,
            client,
        })
    }

    pub async fn setup(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let image = "miniserver:local";

        let docker = Docker::connect_with_local_defaults()?;

        //This would be for if an image is hosted
        /*
        let mut stream = docker.create_image(
            Some(bollard::query_parameters::CreateImageOptions {
                from_image: Some(image.to_string()),
                ..Default::default()
            }),
            None,
            None,
        );
        while let Some(pull_result) = stream.try_next().await? {
            if let Some(status) = pull_result.status {
                println!("pull: {}", status);
            }
        }
        */

        let container = docker
            .inspect_container(&self.container_name, None::<InspectContainerOptions>)
            .await
            .ok();

        let container_id: String;
        // Extract ID option without further nesting
        if let Some((_c, id)) = container
            .as_ref()
            .and_then(|c| c.id.as_ref().map(|id| (c, id)))
        {
            container_id = id.to_string();
            println!("Found existing container {}", id);
        } else {
            let mut port_bindings = HashMap::new();
            port_bindings.insert(
                format!("{}/tcp", PORT),
                Some(vec![PortBinding {
                    host_ip: Some("0.0.0.0".to_string()),
                    host_port: Some(PORT.to_string()),
                }]),
            );

            let config = ContainerCreateBody {
                image: Some(image.to_string()),
                host_config: Some(HostConfig {
                    port_bindings: Some(port_bindings),
                    runtime: Some("nvidia".to_string()),
                    ..Default::default()
                }),
                env: Some(vec![format!("HF_ID={}", self.hf_id)]),
                ..Default::default()
            };

            let create_contaienr_options = CreateContainerOptionsBuilder::new()
                .name(&self.container_name)
                .build();

            let container = docker
                .create_container(Some(create_contaienr_options), config)
                .await?;

            println!("Created new container {}", container.id);

            container_id = container.id;
        }

        docker
            .start_container(&container_id, None::<StartContainerOptions>)
            .await?;

        println!("Started container {}", container_id);

        self.container_id = Some(container_id);

        Ok(())
    }

    /// Takes a stream of inference data and starts performing inference.
    ///
    /// # Arguments
    /// * `&self`
    /// * `request_stream` - The stream of inference data
    /// * `response_closure` - A closure that takes a string and returns a future that resolves to ()
    ///
    /// # Returns
    /// A result containing either the inference output stream, or an Error `Result<(), Box<dyn std::error::Error>>`
    pub async fn run<S, C, CFut>(
        &self,
        mut request_stream: S,
        mut response_closure: C,
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        S: Stream<Item = String> + Unpin + Send + 'static,
        C: FnMut(String) -> CFut + Send + 'static,
        CFut: Future<Output = ()> + Send + 'static,
    {
        while let Some(request) = request_stream.next().await {
            println!("Processing inference for request: {}", request);

            let response: String;

            match self.generate_inference_result(request.clone()).await {
                Ok(result) => {
                    response = result;
                }
                Err(e) => {
                    println!("Failed to generate inference result, likely incorrect request format! Error: {}", e);
                    response =
                        "Failed to generate inference result, likely incorrect request format!"
                            .to_string();
                }
            }

            println!("Generated inference result: {}", response);

            response_closure(response).await;
        }

        Ok(())
    }

    /// Takes input and performs inference on the model currently loaded into the miner.
    ///
    /// # Arguments
    /// * `&self`
    /// * `input_data` - The input used to run inference on the model
    ///
    /// # Returns
    /// `Result<(), Box<dyn std::error::Error>>`
    async fn generate_inference_result(
        &self,
        input_data: String,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let url = format!("http://localhost:{}/chat", self.torch_infer_port);

        println!("Sending inference request to {}", url);
        println!("Input data: {}", input_data);
        let res = self
            .client
            .post(url)
            .json(&json!({
                "session_id": "test1",
                "message": input_data,
                "max_new_tokens": 100
            }))
            .send()
            .await?;

        let res_string = res.text().await?;

        Ok(res_string)
    }

    pub async fn kill_engine(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Killing engine");

        let container_id = self.container_id.as_ref().ok_or("No container id")?;

        let docker = Docker::connect_with_local_defaults()?;

        let container = docker
            .inspect_container(&self.container_name, None::<InspectContainerOptions>)
            .await
            .ok();

        if let Some(_container) = container {
            docker
                .remove_container(
                    container_id,
                    Some(RemoveContainerOptions {
                        force: true,
                        ..Default::default()
                    }),
                )
                .await?;
            println!("Force-removed container {}", container_id);

            return Ok(());
        } else {
            println!(
                "Could not find container {}, likely already removed.",
                container_id
            );

            return Ok(());
        }
    }
}
