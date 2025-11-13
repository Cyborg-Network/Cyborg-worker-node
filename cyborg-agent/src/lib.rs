use std::path::PathBuf;

use ::clap::Parser;
use anyhow::Result;

mod clap;
use crate::clap::App;
mod api;
mod client;
mod config;
mod formats;
mod crypto;
mod auth;
mod error_handling;
//#[cfg(test)]
//mod unit_tests;

lazy_static::lazy_static! {
    /// uses [home::home_dir] to determine $HOME or its equivalent
    pub static ref CONFIG_PATH: PathBuf = home::home_dir()
        .expect("Failed to get home directory")
        .join(".config/cyborg/config.toml");

    // pub static ref RELEASE_SERVER_URL: String = "https://localhost:9000/releases".to_string()
    //     + &format!(
    //         "v{}.{}.{}/",
    //         pkg_version_major!(),
    //         pkg_version_minor!(),
    //         pkg_version_patch!()
    //     )
    //     + "scripts/";
}

pub async fn run_agent() -> Result<()> {
    // parse command line arguments
    let app = App::parse();

    // initialize logger
    let config_str = include_str!("log.yml");

    let config = serde_yaml::from_str(config_str).unwrap();

    log4rs::init_raw_config(config).unwrap();

    println!("Running");

    /* let config = config::load_config(&CONFIG_PATH)?; */
    client::run_client(/* &config */).await.unwrap();

    Ok(())
}
