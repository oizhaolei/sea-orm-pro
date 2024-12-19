use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20241101_000004_product_category::ProductCategory,
    m20241101_000006_product_model::ProductModel,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(Product::Table)
            .col(pk_auto(Product::ProductId))
            .col(string_len_uniq(Product::Name, 60))
            .col(string_len_uniq(Product::ProductNumber, 25))
            .col(string_len_null(Product::Color, 15))
            .col(double(Product::StandardCost))
            .col(double(Product::ListPrice))
            .col(string_len_null(Product::Size, 5))
            .col(decimal_len_null(Product::Weight, 8, 2))
            .col(integer_null(Product::ProductCategoryId))
            .col(integer_null(Product::ProductModelId))
            .col(date_time(Product::SellStartDate))
            .col(date_time_null(Product::SellEndDate))
            .col(date_time_null(Product::DiscontinuedDate))
            .col(text_null(Product::ThumbNailPhoto))
            .col(string_len_null(Product::ThumbnailPhotoFileName, 50))
            .col(uuid_uniq(Product::Rowguid))
            .col(date_time(Product::CreatedDate))
            .foreign_key(
                ForeignKey::create()
                    .name("fk_product_product_category_product_category_id")
                    .from(Product::Table, Product::ProductCategoryId)
                    .to(ProductCategory::Table, ProductCategory::ProductCategoryId),
            )
            .foreign_key(
                ForeignKey::create()
                    .name("fk_product_product_model_product_model_id")
                    .from(Product::Table, Product::ProductModelId)
                    .to(ProductModel::Table, ProductModel::ProductModelId),
            )
            .to_owned();
        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Product::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Product {
    Table,
    ProductId,
    Name,
    ProductNumber,
    Color,
    StandardCost,
    ListPrice,
    Size,
    Weight,
    ProductCategoryId,
    ProductModelId,
    SellStartDate,
    SellEndDate,
    DiscontinuedDate,
    ThumbNailPhoto,
    ThumbnailPhotoFileName,
    Rowguid,
    CreatedDate,
}
