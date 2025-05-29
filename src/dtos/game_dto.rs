use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateGame {
    pub name: String,
    pub description: String,
    pub genre: String,
    pub creator_id: Uuid,
}

#[derive(Deserialize)]
pub struct UpdateGame {
    pub name: Option<String>,
    pub description: Option<String>,
    pub genre: Option<String>,
    pub creator_id: Option<Uuid>,
}
