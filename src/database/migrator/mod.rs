mod m20231229_000001_create_collection_table;
mod m20231229_000002_create_collection_header_table;

use sea_orm_migration::prelude::*;

use super::get_database;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20231229_000001_create_collection_table::Migration),
            Box::new(m20231229_000002_create_collection_header_table::Migration),
        ]
    }
}

pub async fn run_migrations() -> Result<(), DbErr> {
    let db = get_database().await?;
    let schema_manager = SchemaManager::new(&db);
    Migrator::refresh(&db).await?;
    assert!(schema_manager.has_table("collection_header").await?);
    assert!(schema_manager.has_table("collection").await?);

    Ok(())
}
