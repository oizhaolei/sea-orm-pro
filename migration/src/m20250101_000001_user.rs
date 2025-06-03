use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(User::Table)
            .col(date_time(User::CreatedAt).default(Expr::current_timestamp()))
            .col(date_time(User::UpdatedAt).default(Expr::current_timestamp()))
            .col(pk_auto(User::Id))
            .col(uuid(User::Pid))
            .col(string_uniq(User::Email))
            .col(string(User::Password))
            .col(string(User::ApiKey).unique_key())
            .col(string(User::Name))
            .col(string_null(User::ResetToken))
            .col(timestamp_with_time_zone_null(User::ResetSentAt))
            .col(string_null(User::EmailVerificationToken))
            .col(timestamp_with_time_zone_null(User::EmailVerificationSentAt))
            .col(timestamp_with_time_zone_null(User::EmailVerifiedAt))
            .col(string_null(User::MagicLinkToken))
            .col(timestamp_with_time_zone_null(User::MagicLinkExpiration))
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
    CreatedAt,
    UpdatedAt,
    Id,
    Pid,
    Email,
    Name,
    Password,
    ApiKey,
    ResetToken,
    ResetSentAt,
    EmailVerificationToken,
    EmailVerificationSentAt,
    EmailVerifiedAt,
    MagicLinkToken,
    MagicLinkExpiration,
}
