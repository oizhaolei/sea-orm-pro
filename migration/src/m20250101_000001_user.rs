use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = table_auto(User::Table)
            .col(pk_auto(User::Id))
            .col(uuid(User::Pid))
            .col(string_uniq(User::Email))
            .col(string(User::Password))
            .col(string(User::ApiKey).unique_key())
            .col(string(User::Name))
            .to_owned();
        manager.create_table(table).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum User {
    Table,
    Id,
    Pid,
    Email,
    Name,
    Password,
    ApiKey,
}
