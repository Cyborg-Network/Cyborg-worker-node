use clap::{Parser, Subcommand};

#[derive(Debug, Parser, PartialEq)]
#[command(
    name = "cyborg-miner",                  // Name of the CLI tool.
    about = "A ZK ready AI inference miner for the Cyborg Network", // Description shown in the CLI help.
    version = "1.0"                          // Version number of the CLI tool.
)]

/// `Cli` struct defines the command-line interface for the Cyborg miner.
/// This struct uses the `clap` crate to parse command-line arguments.
/// It contains a single field `command` which specifies the subcommand to be executed.
pub struct Cli {
    /// Specify the subcommand to run.
    #[command(subcommand)]
    pub command: Option<Commands>, // Defines the possible subcommands, wrapped in an `Option`.
}

// Enum to define the available subcommands. Each variant corresponds to a different command.
#[derive(Debug, Subcommand, PartialEq)]
/// `Commands` enum defines the available subcommands for the Cyborg miner.
/// Each variant represents a specific action that can be performed by the miner.
/// - `Registration`: Registers the miner with the specified API URL and account seed.
/// - `Startmining`: Starts the mining process with the specified API URL, account seed, and IPFS URL.
pub enum Commands {
    /// Start the miner with specified API URL and IPFS URL.
    StartMiner {
        /// API URL for starting the miner
        #[clap(long, value_name = "API_URL")]
        parachain_url: String,

        /// Account ID for the miner registration.
        #[clap(long, value_name = "ACCOUNT_SEED")]
        account_seed: String,

        /// The type of miner (eg. cloud / edge)
        #[clap(long, value_name = "MINER_TYPE")]
        miner_type: String,
    },
    Install {
        /// API URL for starting the miner
        #[clap(long, value_name = "API_URL")]
        parachain_url: String,

        /// Account ID for the miner registration.
        #[clap(long, value_name = "ACCOUNT_SEED")]
        account_seed: String,

        /// The type of miner (eg. cloud / edge)
        #[clap(long, value_name = "MINER_TYPE")]
        miner_type: String,
    },
}

/*
//Unit tests
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_registration_command() {
        // Simulate running the CLI with the `registration` subcommand and arguments.
        let args = [
            "cyborg-miner",
            "registration",
            "--parachain-url",
            "http://example.com",
            "--account-seed",
            "12345678",
            "--ipfs-url",
            "ipfs_url",
            "--ipfs-api-key",
            "ipfs_api_key",
            "--ipfs-api-secret",
            "ipfs_api_secret",
        ];

        // Parse the arguments and check if they match the expected `Cli` struct.
        let cli = Cli::try_parse_from(args).unwrap();

        assert_eq!(
            cli,
            Cli {
                command: Some(Commands::Registration {
                    parachain_url: "http://example.com".to_string(),
                    account_seed: "12345678".to_string(),
                    ipfs_url: "ipfs_url".to_string(),
                    ipfs_api_key: "ipfs_api_key".to_string(),
                    ipfs_api_secret: "ipfs_api_secret".to_string(),
                })
            }
        );
    }

    #[test]
    fn test_start_command() {
        // Simulate running the CLI with the `start` subcommand and arguments.
        let args = [
            "cyborg-miner",
            "startmining",
            "--parachain-url",
            "http://example.com",
            "--account-seed",
            "12345678",
        ];

        // Parse the arguments and verify they match the expected `Cli` struct.
        let cli = Cli::try_parse_from(args).unwrap();

        assert_eq!(
            cli,
            Cli {
                command: Some(Commands::Startmining {
                    parachain_url: "http://example.com".to_string(),
                    account_seed: "12345678".to_string(),
                })
            }
        );
    }

    #[test]
    fn test_no_command() {
        // Simulate running the CLI without any subcommand.
        let args = ["cyborg-miner"];
        let cli = Cli::try_parse_from(args).unwrap();

        // Ensure `command` is `None` when no subcommand is provided.
        assert_eq!(cli, Cli { command: None });
    }

    #[test]
    fn test_invalid_command() {
        // Simulate running the CLI with an invalid subcommand.
        let args = ["cyborg-miner", "invalid"];

        // Attempt to parse the arguments and expect an error for the unrecognized command.
        let result = Cli::try_parse_from(args);
        assert!(result.is_err());
    }
}
*/
