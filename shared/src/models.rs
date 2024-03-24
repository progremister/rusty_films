use serde::{ Deserialize, Serialize };
 
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Film {
    pub id: uuid::Uuid,
    pub title: String,
    pub director: String,
    pub year: u16,
    pub poster: String, // image url
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct CreateFilm {
    pub title: String,
    pub director: String,
    pub year: u16,
    pub poster: String
}
