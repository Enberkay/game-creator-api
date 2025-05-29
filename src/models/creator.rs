use sea_orm::entity::prelude::*;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "creators")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Game,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Game => Entity::has_many(super::game::Entity).into(),
        }
    }
}

impl Related<super::game::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Game.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
