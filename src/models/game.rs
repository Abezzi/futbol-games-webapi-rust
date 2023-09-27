#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct GameModel {
    pub id: Uuid,
    pub field_name: String,
    pub address: String,
    pub date: chrono::DateTime<chrono::Utc>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Serialize, Deserialize Debug)]
pub struct CreateGameSchema {
    pub field_name: String,
    pub address: String,
    pub date: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize Debug)]
pub struct UpdateGameSchema {
    pub field_name: String,
    pub address: String,
    pub date: chrono::DateTime<chrono::Utc>,
}
