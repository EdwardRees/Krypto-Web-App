pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_auth;
mod m20251202_012109_create_game_stats;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_auth::Migration),
            Box::new(m20251202_012109_create_game_stats::Migration),
        ]
    }
}
