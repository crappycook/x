use crate::model::instrument::Entity as Instrument;
use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::{ConnectionTrait, Schema, Statement};
use tracing::info;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Check if the table already exists
        let table_exists = manager
            .get_connection()
            .query_one(Statement::from_string(
                manager.get_database_backend(),
                "SELECT name FROM sqlite_master WHERE type='table' AND name='instruments'"
                    .to_owned(),
            ))
            .await?
            .is_some();

        if !table_exists {
            let schema = Schema::new(manager.get_database_backend());
            manager
                .create_table(
                    schema
                        .create_table_from_entity(Instrument)
                        .if_not_exists()
                        .to_owned(),
                )
                .await?;

            info!("Instruments table created successfully");
        } else {
            info!("Instruments table already exists, skipping creation");
        }

        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(Migration)]
    }
}
