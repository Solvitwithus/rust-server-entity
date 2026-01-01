use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
       

        manager
            .create_table(
                Table::create()
                    .table(Login::Table)
                    .if_not_exists()
                    .col(pk_auto(Login::Id))
                    .col(string(Login::UserName))
                    .col(string(Login::Password))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
       

        manager
            .drop_table(Table::drop().table(Login::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Login {
    Table,
    Id,
    UserName,
    Password,
}
