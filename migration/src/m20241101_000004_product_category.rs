use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(ProductCategory::Table)
            .col(pk_auto(ProductCategory::ProductCategoryId))
            .col(integer_null(ProductCategory::ParentProductCategoryId))
            .col(string_len_uniq(ProductCategory::Name, 60))
            .col(uuid_uniq(ProductCategory::Rowguid))
            .col(date_time(ProductCategory::CreatedDate))
            .foreign_key(
                ForeignKey::create()
                    .name("fk_parent_product_category_id")
                    .from(
                        ProductCategory::Table,
                        ProductCategory::ParentProductCategoryId,
                    )
                    .to(ProductCategory::Table, ProductCategory::ProductCategoryId),
            )
            .to_owned();
        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ProductCategory::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum ProductCategory {
    Table,
    ProductCategoryId,
    ParentProductCategoryId,
    Name,
    Rowguid,
    CreatedDate,
}
