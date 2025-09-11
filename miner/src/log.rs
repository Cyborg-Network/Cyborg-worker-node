use crate::error::Result;
use once_cell::sync::Lazy;
use std::path::PathBuf;
use std::sync::Mutex;
use tracing_appender::non_blocking;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::fmt::writer::BoxMakeWriter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::Layer;
use tracing_subscriber::filter::{EnvFilter, LevelFilter};

static LOG_GUARD: Lazy<Mutex<Option<(WorkerGuard, WorkerGuard)>>> = Lazy::new(|| Mutex::new(None));

pub fn init_logger() -> Result<()> {
    // Create directories if they don't exist
    std::fs::create_dir_all("miner/logs")?;
    std::fs::create_dir_all("miner/logs/operator")?;
    std::fs::create_dir_all("miner/logs/user")?;

    // Operator logs (miner owner) - more verbose
    let operator_appender =
        tracing_appender::rolling::never("miner/logs/operator", "miner_operator.log");
    let (non_blocking_operator, operator_guard) = non_blocking(operator_appender);

    // User logs (end user) - less verbose
    let user_appender = tracing_appender::rolling::never("miner/logs/user", "miner_user.log");
    let (non_blocking_user, user_guard) = non_blocking(user_appender);

    // Create filter for operator logs (DEBUG level)
    let operator_filter = EnvFilter::from_default_env()
        .add_directive(LevelFilter::DEBUG.into());

    // Create filter for user logs (INFO level)
    let user_filter = EnvFilter::from_default_env()
        .add_directive(LevelFilter::INFO.into());

    // Operator layer - more detailed logs
    let operator_layer = tracing_subscriber::fmt::layer()
        .with_writer(BoxMakeWriter::new(non_blocking_operator))
        .with_ansi(false)
        .with_level(true)
        .with_target(true)
        .with_thread_ids(true)
        .with_filter(operator_filter);

    // User layer - less detailed logs
    let user_layer = tracing_subscriber::fmt::layer()
        .with_writer(BoxMakeWriter::new(non_blocking_user))
        .with_ansi(false)
        .with_level(true)
        .with_target(false)
        .with_thread_ids(false)
        .with_filter(user_filter);

    // Combined subscriber with both layers
    let subscriber = tracing_subscriber::registry()
        .with(operator_layer)
        .with(user_layer);

    subscriber.init();

    *LOG_GUARD.lock().unwrap() = Some((operator_guard, user_guard));
    Ok(())
}

#[allow(dead_code)]
fn reset_log_files() -> Result<()> {
    *LOG_GUARD.lock().unwrap() = None;

    let _ = std::fs::remove_file("miner/logs/operator/miner_operator.log");
    let _ = std::fs::remove_file("miner/logs/user/miner_user.log");

    Ok(())
}

// Helper function to get log file paths
pub fn get_operator_log_path() -> PathBuf {
    PathBuf::from("miner/logs/operator/miner_operator.log")
}

pub fn get_user_log_path() -> PathBuf {
    PathBuf::from("miner/logs/user/miner_user.log")
}