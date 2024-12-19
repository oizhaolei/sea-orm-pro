use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(ProductModel::Table)
            .col(pk_auto(ProductModel::ProductModelId))
            .col(string_len(ProductModel::Name, 60))
            .col(text_null(ProductModel::CatalogDescription))
            .col(uuid_uniq(ProductModel::Rowguid))
            .col(date_time(ProductModel::CreatedDate))
            .to_owned();
        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ProductModel::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum ProductModel {
    Table,
    ProductModelId,
    Name,
    CatalogDescription,
    Rowguid,
    CreatedDate,
}
