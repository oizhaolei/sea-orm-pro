use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(Address::Table)
            .col(pk_auto(Address::AddressId))
            .col(string_len(Address::AddressLine1, 60))
            .col(string_len_null(Address::AddressLine2, 60))
            .col(string_len(Address::City, 30))
            .col(string_len(Address::StateProvince, 50))
            .col(string_len(Address::CountryRegion, 50))
            .col(string_len(Address::PostalCode, 15))
            .col(uuid_uniq(Address::Rowguid))
            .col(date_time(Address::CreatedDate))
            .to_owned();
        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Address::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Address {
    Table,
    AddressId,
    AddressLine1,
    AddressLine2,
    City,
    StateProvince,
    CountryRegion,
    PostalCode,
    Rowguid,
    CreatedDate,
}
