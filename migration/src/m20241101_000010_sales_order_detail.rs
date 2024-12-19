use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20241101_000007_product::Product, m20241101_000009_sales_order_header::SalesOrderHeader,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(SalesOrderDetail::Table)
            .col(integer(SalesOrderDetail::SalesOrderId))
            .col(integer(SalesOrderDetail::SalesOrderDetailId))
            .col(small_integer(SalesOrderDetail::OrderQty))
            .col(integer(SalesOrderDetail::ProductId))
            .col(double(SalesOrderDetail::UnitPrice))
            .col(double(SalesOrderDetail::UnitPriceDiscount).default(0))
            .col(uuid_uniq(SalesOrderDetail::Rowguid))
            .col(date_time(SalesOrderDetail::CreatedDate))
            .primary_key(
                Index::create()
                    .name("pk_sales_order_detail_id")
                    .col(SalesOrderDetail::SalesOrderId)
                    .col(SalesOrderDetail::SalesOrderDetailId),
            )
            .foreign_key(
                ForeignKey::create()
                    .name("fk_sales_order_detail_sales_order_header_sales_order_id")
                    .from(SalesOrderDetail::Table, SalesOrderDetail::SalesOrderId)
                    .to(SalesOrderHeader::Table, SalesOrderHeader::SalesOrderId)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .name("fk_sales_order_detail_product_product_id")
                    .from(SalesOrderDetail::Table, SalesOrderDetail::ProductId)
                    .to(Product::Table, Product::ProductId),
            )
            .to_owned();
        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SalesOrderDetail::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum SalesOrderDetail {
    Table,
    SalesOrderId,
    SalesOrderDetailId,
    OrderQty,
    ProductId,
    UnitPrice,
    UnitPriceDiscount,
    Rowguid,
    CreatedDate,
}
