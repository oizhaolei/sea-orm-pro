use sea_orm_migration::{prelude::*, schema::*};

use crate::{m20241101_000001_address::Address, m20241101_000002_customer::Customer};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(SalesOrderHeader::Table)
            .col(pk_auto(SalesOrderHeader::SalesOrderId))
            .col(integer(SalesOrderHeader::RevisionNumber).default(0))
            .col(date_time(SalesOrderHeader::OrderDate))
            .col(date_time(SalesOrderHeader::DueDate))
            .col(date_time_null(SalesOrderHeader::ShipDate))
            .col(integer(SalesOrderHeader::Status).default(1))
            .col(
                boolean(SalesOrderHeader::OnlineOrderFlag)
                    .default(manager.get_database_backend().boolean_value(true)),
            )
            .col(string_len_null(SalesOrderHeader::PurchaseOrderNumber, 25))
            .col(string_len_null(SalesOrderHeader::AccountNumber, 15))
            .col(integer(SalesOrderHeader::CustomerId))
            .col(integer_null(SalesOrderHeader::ShipToAddressId))
            .col(integer_null(SalesOrderHeader::BillToAddressId))
            .col(string_len(SalesOrderHeader::ShipMethod, 50))
            .col(string_len_null(
                SalesOrderHeader::CreditCardApprovalCode,
                15,
            ))
            .col(double(SalesOrderHeader::SubTotal).default(0))
            .col(double(SalesOrderHeader::TaxAmt).default(0))
            .col(double(SalesOrderHeader::Freight).default(0))
            .col(text_null(SalesOrderHeader::Comment))
            .col(uuid_uniq(SalesOrderHeader::Rowguid))
            .col(date_time(SalesOrderHeader::CreatedDate))
            .foreign_key(
                ForeignKey::create()
                    .name("fk_sales_order_header_customer_customer_id")
                    .from(SalesOrderHeader::Table, SalesOrderHeader::CustomerId)
                    .to(Customer::Table, Customer::CustomerId),
            )
            .foreign_key(
                ForeignKey::create()
                    .name("fk_sales_order_header_address_ship_to_address_id")
                    .from(SalesOrderHeader::Table, SalesOrderHeader::ShipToAddressId)
                    .to(Address::Table, Address::AddressId),
            )
            .foreign_key(
                ForeignKey::create()
                    .name("fk_sales_order_header_address_bill_to_address_id")
                    .from(SalesOrderHeader::Table, SalesOrderHeader::BillToAddressId)
                    .to(Address::Table, Address::AddressId),
            )
            .to_owned();
        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SalesOrderHeader::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum SalesOrderHeader {
    Table,
    SalesOrderId,
    RevisionNumber,
    OrderDate,
    DueDate,
    ShipDate,
    Status,
    OnlineOrderFlag,
    PurchaseOrderNumber,
    AccountNumber,
    CustomerId,
    ShipToAddressId,
    BillToAddressId,
    ShipMethod,
    CreditCardApprovalCode,
    SubTotal,
    TaxAmt,
    Freight,
    Comment,
    Rowguid,
    CreatedDate,
}
