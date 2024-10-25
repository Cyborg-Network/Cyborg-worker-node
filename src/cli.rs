use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    name = "cyborg-worker",
    about = "A standalone off-chain worker",
    version = "1.0"
)]
pub struct Cli {
    /// Enable verbose logging.
    #[clap(short, long)]
    pub verbose: bool,

    /// Specify the subcommand to run.
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Register a worker with specified API URL and Account ID.
    Registration {
        /// API URL for the registration
        #[clap(long, value_name = "API_URL")]
        api_url: String,

        /// Account ID for the worker registration
        #[clap(long, value_name = "ACCOUNT_ID")]
        account_id: String,
    },
    /// Start the worker with specified API URL and IFS URL.
    Start {
        /// API URL for starting the worker
        #[clap(long, value_name = "API_URL")]
        api_url: String,

        /// IFS URL for the worker
        #[clap(long, value_name = "IFS_URL")]
        ifs_url: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Registration {
            api_url,
            account_id,
        }) => {
            println!(
                "Registering worker with API URL: {}, Account ID: {}",
                api_url, account_id
            );
        }
        Some(Commands::Start { api_url, ifs_url }) => {
            println!(
                "Starting worker with API URL: {}, IFS URL: {}",
                api_url, ifs_url
            );
        }
        None => {
            println!("No subcommand was used");
        }
    }

    if cli.verbose {
        println!("Verbose mode enabled");
    }
}
