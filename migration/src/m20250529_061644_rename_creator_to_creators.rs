use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20250529_061644_rename_creator_to_creators"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let stmt = Table::rename()
            .table("creator", "creators")
            .to_owned();
        manager.rename_table(stmt).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let stmt = Table::rename()
            .table("creators", "creator")
            .to_owned();
        manager.rename_table(stmt).await
    }
}
