pub struct Film {
    pub id: uuid::Uuid,
    pub title: String,
    pub director: String,
    pub year: u16,
    pub poster: String, // image url
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>
}

pub struct CreateFilm {
    pub title: String,
    pub director: String,
    pub year: u16,
    pub poster: String
}
