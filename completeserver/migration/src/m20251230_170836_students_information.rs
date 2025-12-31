use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        

        manager
            .create_table(
                Table::create()
                    .table(StudentsInformation::Table)
                    .if_not_exists()
                    .col(pk_auto(StudentsInformation::Id))
                    .col(string(StudentsInformation::Name))
                    .col(integer(StudentsInformation::Age))
                    .col(string(StudentsInformation::Course))
                    .col(string(StudentsInformation::RegNumber))
                    .col(string(StudentsInformation::AdmissionYear))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
      

        manager
            .drop_table(Table::drop().table(StudentsInformation::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum StudentsInformation {
    Table,
    Id,
    Name,
    Age,
    Course,
    RegNumber,
    AdmissionYear
}
