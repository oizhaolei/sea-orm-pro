use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(Customer::Table)
            .col(pk_auto(Customer::CustomerId))
            .col(
                boolean(Customer::NameStyle)
                    .default(manager.get_database_backend().boolean_value(false)),
            )
            .col(string_len_null(Customer::Title, 8))
            .col(string_len(Customer::FirstName, 60))
            .col(string_len_null(Customer::MiddleName, 60))
            .col(string_len(Customer::LastName, 60))
            .col(string_len_null(Customer::Suffix, 10))
            .col(string_len_null(Customer::CompanyName, 128))
            .col(string_len_null(Customer::SalesPerson, 256))
            .col(string_len_null(Customer::EmailAddress, 50))
            .col(string_len(Customer::Phone, 25))
            .col(string_len(Customer::PasswordHash, 128))
            .col(string_len(Customer::PasswordSalt, 10))
            .col(uuid_uniq(Customer::Rowguid))
            .col(date_time(Customer::CreatedDate))
            .to_owned();
        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Customer::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Customer {
    Table,
    CustomerId,
    NameStyle,
    Title,
    FirstName,
    MiddleName,
    LastName,
    Suffix,
    CompanyName,
    SalesPerson,
    EmailAddress,
    Phone,
    PasswordHash,
    PasswordSalt,
    Rowguid,
    CreatedDate,
}
