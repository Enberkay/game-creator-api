use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Creator::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Creator::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Creator::FirstName).string().not_null())
                    .col(ColumnDef::new(Creator::LastName).string().not_null())
                    .col(ColumnDef::new(Creator::Email).string().not_null())
                    .col(ColumnDef::new(Creator::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Creator::UpdatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Game::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Game::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Game::Name).string().not_null())
                    .col(ColumnDef::new(Game::Description).string().not_null())
                    .col(ColumnDef::new(Game::Genre).string().not_null())
                    .col(ColumnDef::new(Game::CreatorId).uuid().not_null())
                    .col(ColumnDef::new(Game::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Game::UpdatedAt).timestamp_with_time_zone().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-game-creator_id")
                            .from(Game::Table, Game::CreatorId)
                            .to(Creator::Table, Creator::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Game::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Creator::Table).to_owned()).await?;
        Ok(())
    }
}

#[derive(Iden)]
enum Creator {
    Table,
    Id,
    FirstName,
    LastName,
    Email,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum Game {
    Table,
    Id,
    Name,
    Description,
    Genre,
    CreatorId,
    CreatedAt,
    UpdatedAt,
}
