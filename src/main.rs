mod cli;
//mod worker;

use clap::Parser;

fn main() {
    let args = cli::Cli::parse();

    if args.verbose {
        println!("Verbose logging enabled.");
    }

    // Initialize the worker and start task processing
    println!("Starting the off-chain worker...");

    // Call worker logic (e.g., fetch tasks, execute, submit results)
    //worker::run_worker(args.source_url);
}
