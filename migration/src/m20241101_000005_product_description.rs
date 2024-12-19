use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(ProductDescription::Table)
            .col(pk_auto(ProductDescription::ProductDescriptionId))
            .col(string_len(ProductDescription::Description, 400))
            .col(uuid_uniq(ProductDescription::Rowguid))
            .col(date_time(ProductDescription::CreatedDate))
            .to_owned();
        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ProductDescription::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum ProductDescription {
    Table,
    ProductDescriptionId,
    Description,
    Rowguid,
    CreatedDate,
}
