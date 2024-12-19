use sea_orm_migration::{prelude::*, schema::*};

use crate::{m20241101_000001_address::Address, m20241101_000002_customer::Customer};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(CustomerAddress::Table)
            .col(integer(CustomerAddress::CustomerId))
            .col(integer(CustomerAddress::AddressId))
            .col(string_len(CustomerAddress::AddressType, 60))
            .col(uuid_uniq(CustomerAddress::Rowguid))
            .col(date_time(CustomerAddress::CreatedDate))
            .primary_key(
                Index::create()
                    .name("pk_customer_address_customer_id_address_id")
                    .col(CustomerAddress::CustomerId)
                    .col(CustomerAddress::AddressId),
            )
            .foreign_key(
                ForeignKey::create()
                    .name("fk_customer_address_customer_customer_id")
                    .from(CustomerAddress::Table, CustomerAddress::CustomerId)
                    .to(Customer::Table, Customer::CustomerId),
            )
            .foreign_key(
                ForeignKey::create()
                    .name("fk_customer_address_address_address_id")
                    .from(CustomerAddress::Table, CustomerAddress::AddressId)
                    .to(Address::Table, Address::AddressId),
            )
            .to_owned();
        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CustomerAddress::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum CustomerAddress {
    Table,
    CustomerId,
    AddressId,
    AddressType,
    Rowguid,
    CreatedDate,
}
