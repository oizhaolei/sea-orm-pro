use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20241101_000005_product_description::ProductDescription,
    m20241101_000006_product_model::ProductModel,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(ProductModelProductDescription::Table)
            .col(integer(ProductModelProductDescription::ProductModelId))
            .col(integer(
                ProductModelProductDescription::ProductDescriptionId,
            ))
            .col(char_len(ProductModelProductDescription::Culture, 6))
            .col(uuid_uniq(ProductModelProductDescription::Rowguid))
            .col(date_time(ProductModelProductDescription::CreatedDate))
            .primary_key(
                Index::create()
                    .name("pk_product_model_product_description_id")
                    .col(ProductModelProductDescription::ProductModelId)
                    .col(ProductModelProductDescription::ProductDescriptionId)
                    .col(ProductModelProductDescription::Culture),
            )
            .foreign_key(
                ForeignKey::create()
                    .name("fk_product_model_product_model_id")
                    .from(
                        ProductModelProductDescription::Table,
                        ProductModelProductDescription::ProductModelId,
                    )
                    .to(ProductModel::Table, ProductModel::ProductModelId),
            )
            .foreign_key(
                ForeignKey::create()
                    .name("fk_product_description_id")
                    .from(
                        ProductModelProductDescription::Table,
                        ProductModelProductDescription::ProductDescriptionId,
                    )
                    .to(
                        ProductDescription::Table,
                        ProductDescription::ProductDescriptionId,
                    ),
            )
            .to_owned();
        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(ProductModelProductDescription::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(Iden)]
pub enum ProductModelProductDescription {
    Table,
    ProductModelId,
    ProductDescriptionId,
    Culture,
    Rowguid,
    CreatedDate,
}
