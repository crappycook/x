mod core;
mod model;

use anyhow::Result;
use core::cli;
use core::config;
use core::dao;
use core::tracker;
use std::fs;
use tracing::{info, Level};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::fmt::format::FmtSpan;
use sea_orm_migration::MigratorTrait;
use core::migration::Migrator;

#[tokio::main]
async fn main() -> Result<()> {
    // Load configuration
    let config = config::AppConfig::new()?;

    // Create logs directory if it doesn't exist
    fs::create_dir_all(&config.logging.directory).expect("Failed to create logs directory");

    // Set up file appender
    let file_appender = RollingFileAppender::new(
        Rotation::DAILY,
        &config.logging.directory,
        &config.logging.filename,
    );

    // Initialize tracing
    let subscriber = tracing_subscriber::fmt()
        .with_writer(file_appender)
        .with_ansi(false)
        .with_max_level(Level::TRACE)
        .with_span_events(FmtSpan::CLOSE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    info!("Welcome to the Trading Bot!");

    // Get arguments (either from command line or interactively)
    let args = cli::get_args()?;

    let pair = cli::get_crypto_pair(&args.base.unwrap(), &args.quote.unwrap());
    info!("Selected pair: {}", pair.to_string());

    // Ensure the database file exists
    dao::check_database_file(&config.database.url)?;

    // Establish database connection and run migrations
    let db_conn = dao::establish_connection(&config.database.url).await?;

    // Run migrations
    Migrator::up(&db_conn, None).await?;

    // Call the async track_price function
    tracker::ticker(&pair).await?;

    Ok(())
}
