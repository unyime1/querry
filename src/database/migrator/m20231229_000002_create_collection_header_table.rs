use super::m20231229_000001_create_collection_table::Collection;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20231229_000002_create_collection_header_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // Define how to apply this migration: Create the Collections table.
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(CollectionHeader::Table)
                    .col(
                        ColumnDef::new(CollectionHeader::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(CollectionHeader::Key).string())
                    .col(ColumnDef::new(CollectionHeader::Value).string())
                    .col(
                        ColumnDef::new(CollectionHeader::CollectionId)
                            .uuid()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-header-collection_id")
                            .from(CollectionHeader::Table, CollectionHeader::CollectionId)
                            .to(Collection::Table, Collection::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    // Define how to rollback this migration: Drop the Bakery table.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CollectionHeader::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum CollectionHeader {
    Table,
    Id,
    Key,
    Value,
    CollectionId,
}
