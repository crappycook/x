use crate::model::instrument::{self, ActiveModel, Entity as Instrument};
use anyhow::{Context, Result};
use sea_orm::{Database, DatabaseConnection, EntityTrait, Set};
use std::path::Path;
use std::fs;
use tracing::info;

pub fn check_database_file(database_url: &str) -> Result<()> {
    // Extract the file path from the database URL
    let db_path = database_url.trim_start_matches("sqlite:");
    info!("Database path: {:?}", db_path);
    let path = Path::new(db_path);

    // Ensure the directory exists
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).context("Failed to create database directory")?;
    }

    // If the file doesn't exist, create it
    if !path.exists() {
        fs::File::create(path).context("Failed to create database file")?;
        info!("Created new database file at: {:?}", path);
    }

    Ok(())
}

pub async fn establish_connection(database_url: &str) -> Result<DatabaseConnection> {
    let conn = Database::connect(database_url)
        .await
        .context("Error connecting to database")?;

    info!("Successfully connected to database at: {:?}", database_url);

    Ok(conn)
}

pub async fn get_or_create_instrument(
    conn: &DatabaseConnection,
    symbol: &str,
    instrument_type: &str,
) -> Result<instrument::Model> {
    let instrument = Instrument::find_by_symbol(symbol).one(conn).await?;

    match instrument {
        Some(instrument) => Ok(instrument),
        None => {
            let new_instrument = ActiveModel {
                id: Set(0),
                symbol: Set(symbol.to_string()),
                instrument_type: Set(instrument_type.to_string()),
            };
            let res = Instrument::insert(new_instrument).exec(conn).await?;
            let inserted_instrument = Instrument::find_by_id(res.last_insert_id).one(conn).await?;
            Ok(inserted_instrument.unwrap())
        }
    }
}
