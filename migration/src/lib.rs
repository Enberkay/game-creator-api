pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20250529_055640_create_creator_and_game_tables;
mod m20250529_061644_rename_creator_to_creators;
mod m20250529_070451_rename_game_to_games;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20250529_055640_create_creator_and_game_tables::Migration),
            Box::new(m20250529_061644_rename_creator_to_creators::Migration),
            Box::new(m20250529_070451_rename_game_to_games::Migration),
        ]
    }
}
