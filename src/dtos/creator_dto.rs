use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateCreator {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct UpdateCreator {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
}
