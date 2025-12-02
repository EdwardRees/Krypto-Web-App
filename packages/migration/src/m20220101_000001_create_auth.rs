use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager.create_table(
            Table::create()
            .table(Auth::Table)
            .if_not_exists()
            .col(ColumnDef::new(Auth::Id)
                .uuid()
                .not_null()
                .primary_key()
            )
            .col(string(Auth::Email).unique_key())
            .col(string(Auth::Password))
            .to_owned(),
        ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(Auth::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Auth {
    Table,
    Id,
    Email,
    Password
}
