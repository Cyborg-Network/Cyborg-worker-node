mod cli;
use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse_args();

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
        Some(Commands::Start { api_url, ipfs_url }) => {
            println!(
                "Starting worker with API URL: {}, IFS URL: {}",
                api_url, ipfs_url
            );
        }
        None => {
            println!("No subcommand was used");
        }
    }
}
