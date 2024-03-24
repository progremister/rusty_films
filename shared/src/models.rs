pub struct Film {
    pub id: uuid::Uuid,
    pub title: String,
    pub director: String,
    pub year: u16,
    pub poster: String, // image url
    pub created_at: Option<chono::DateTime<chrono::utc>>,
    pub created_at: Option<chono::DateTime<chrono::utc>>
}

pub struct CreateFilm {
    pub title: String,
    pub director: String,
    pub year: u16,
    pub poster: String 
}
