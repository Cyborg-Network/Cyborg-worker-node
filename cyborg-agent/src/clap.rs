
use clap::Parser;

#[derive(Parser)]
#[clap(author, version)]
pub enum App {
    CreateConfig {
        #[clap(long = "force")]
        force: bool,
        /// user token, used to authenticate with the server
        user_token: String,
    },
    Run,
}