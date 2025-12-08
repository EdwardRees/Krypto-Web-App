use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(Problem::Table)
                    .if_not_exists()
                    .col(pk_auto(Problem::Id))
                    .col(ColumnDef::new(Problem::OwnerId).uuid())
                    .col(string(Problem::Equation))
                    .col(string(Problem::Numbers)) // csv
                    .col(ColumnDef::new(Problem::Date).timestamp())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(Problem::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Problem {
    Table,
    Id,
    OwnerId,
    Date,
    Equation,
    Numbers
}
