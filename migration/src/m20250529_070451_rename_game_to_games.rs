use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20250529_070451_rename_game_to_games"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let stmt = Table::rename()
            .table("game", "games")
            .to_owned();  // stmt เป็น TableRenameStatement

        manager.rename_table(stmt).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let stmt = Table::rename()
            .table("games", "game")
            .to_owned();

        manager.rename_table(stmt).await
    }
}
