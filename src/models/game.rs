use sea_orm::entity::prelude::*;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "games")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub genre: String,
    pub creator_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Creator,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Creator => Entity::belongs_to(super::creator::Entity)
                .from(Column::CreatorId)
                .to(super::creator::Column::Id)
                .into(),
        }
    }
}

impl Related<super::creator::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Creator.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
