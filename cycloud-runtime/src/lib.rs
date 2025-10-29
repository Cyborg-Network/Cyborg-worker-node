use bollard::query_parameters::{
    InspectContainerOptions, RemoveContainerOptions, StartContainerOptions,
};
use bollard::secret::ContainerStateStatusEnum;
use bollard::Docker;

use crate::util::provision_container;

mod run_script;
mod util;

static PORT: u16 = 22;
const MAX_SETUP_ATTEMPTS: u8 = 100;

#[derive(Debug)]
pub struct CyCloudEngine {
    // Identifier for naming the container, to be able to nuke it from outside the engine
    container_name: String,
}

impl CyCloudEngine {
    /// Creates a new `CyCloudEngine` instance.
    ///
    /// # Returns
    /// A new `CyCloudEngine` instance
    pub fn new(containe_name: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            container_name: containe_name.to_string(),
        })
    }

    /// Sets up the inference container by checking if it already exists and provisioned, and if not, creating it.
    ///
    /// # Returns
    /// `Ok(())` if the container was successfully set up, or `Err` if an unrecoverable error occurred.
    ///
    /// This function will loop until the container is running or the maximum number of attempts has been reached.
    /// If the container is found and running, the function will return.
    /// If the container is not found, it will be provisioned.
    /// If the container is found but not running, it will be started.
    /// If the maximum number of attempts has been reached and the container is still not running, an unrecoverable error will be returned.
    pub async fn setup(&mut self) -> Result<(), Box<dyn std::error::Error>> {
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

        let mut container_running = false;
        for attempt in 0..MAX_SETUP_ATTEMPTS {
            println!("Attempt {} of {}", attempt, MAX_SETUP_ATTEMPTS);

            let container = docker
                .inspect_container(&self.container_name, None::<InspectContainerOptions>)
                .await
                .ok();

            // Extract ID option without further nesting
            if let Some((container_inspect_response, id)) = container
                .as_ref()
                .and_then(|c| c.id.as_ref().map(|id| (c, id)))
            {
                println!("Found existing container {}", id);

                let container_state = match &container_inspect_response.state {
                    Some(state) => state,
                    None => {
                        println!("Container {} is not running, provisioning...", id);
                        provision_container(PORT, &self.container_name).await?;
                        continue;
                    }
                };

                if let Some(state) = container_state.status {
                    match state {
                        ContainerStateStatusEnum::RUNNING => {
                            println!("Container {} is running", id);
                            container_running = true;
                            break;
                        }
                        ContainerStateStatusEnum::CREATED => {
                            println!("Container {} is already provisioned, starting...", id);
                            if let Err(e) = docker
                                .start_container(
                                    &self.container_name,
                                    None::<StartContainerOptions>,
                                )
                                .await
                            {
                                println!("Failed to start container: {}", e);
                                continue;
                            }
                        }
                        ContainerStateStatusEnum::PAUSED => {
                            println!("Container {} is paused, starting...", id);
                            if let Err(e) = docker.unpause_container(&self.container_name).await {
                                println!("Failed to unpause container: {}", e);
                                continue;
                            }
                        }
                        ContainerStateStatusEnum::RESTARTING => {
                            println!("Container {} is restarting, waiting...", id);
                        }
                        ContainerStateStatusEnum::EXITED => {
                            println!("Container {} was exited, restarting...", id);
                            if let Err(e) = docker
                                .start_container(
                                    &self.container_name,
                                    None::<StartContainerOptions>,
                                )
                                .await
                            {
                                println!("Failed to start container: {}", e);
                                continue;
                            }
                        }
                        ContainerStateStatusEnum::DEAD => {
                            return Err("Unrecoverable error: Miner Container is dead".into());
                        }
                        ContainerStateStatusEnum::EMPTY => {
                            println!("Container {} is empty, reprovisioning...", id);
                            if let Err(e) = provision_container(PORT, &self.container_name).await {
                                println!("Failed to reprovision container: {}", e);
                                continue;
                            }
                        }
                        _ => {
                            println!(
                                "Container {} doesn't have a state assigned, provisioning...",
                                id
                            );
                            if let Err(e) = provision_container(PORT, &self.container_name).await {
                                println!("Failed to reprovision container: {}", e);
                                continue;
                            }
                        }
                    }
                } else {
                    println!("Container {} is not running, provisioning...", id);
                    provision_container(PORT, &self.container_name).await?;
                    continue;
                }
            } else {
                println!(
                    "Could not find container {}, provisioning...",
                    self.container_name
                );
                provision_container(PORT, &self.container_name).await?;
                continue;
            }

            let delay_ms = std::cmp::min(500u64 * attempt as u64, 20_000u64);
            println!("Container does not have the correct state, attempting setup and checking back in {} seconds...", delay_ms/1000);
            tokio::time::sleep(std::time::Duration::from_millis(delay_ms)).await;
        }

        if container_running {
            Ok(())
        } else {
            Err("Unrecoverable error: Could not start inference task container".into())
        }
    }

    /// Kills the inference server by force-removing the associated docker container.
    /// This will remove all data associated with the container and should only be used for radical cleanup.
    ///
    /// # Returns
    /// `Ok(())` if the container was successfully removed, or `Err` if an error occurred
    pub async fn kill_engine(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Cleaning up inference server");

        let docker = Docker::connect_with_local_defaults()?;

        docker
            .remove_container(
                &self.container_name,
                Some(RemoveContainerOptions {
                    force: true,
                    ..Default::default()
                }),
            )
            .await?;

        println!("Force-removed container {}", &self.container_name);
        Ok(())
    }
}
