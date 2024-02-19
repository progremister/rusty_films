use actix_web::{get, web::ServiceConfig};
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::CustomError;
use sqlx::{Executor, PgPool};
use dotenv::dotenv;

#[get("/")]
async fn hello_world() -> &'static str {
    "Hello World!"
}

#[get("/version")]
async fn display_version(db: actix_web::web::Data<sqlx::PgPool>) -> String {
    let result: Result<String, sqlx::Error> = sqlx::query_scalar("SELECT version()")
        .fetch_one(db.get_ref())
        .await;

    match result {
        Ok(version) => version,
        Err(e) => format!("Error: {:?}", e),
    }
}

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {

    dotenv().ok();

    let db_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL is not set in environmental variables!");

    let db_connetion = PgPool::connect(&db_url)
        .await
        .map_err(CustomError::new)?;

    let pool = actix_web::web::Data::new(db_connetion.clone());

    db_connetion.execute(include_str!("../../migrations/schema.sql"))
        .await
        .map_err(CustomError::new)?;

    let config = move |cfg: &mut ServiceConfig| {
        cfg.app_data(pool)
           .service(hello_world)
           .service(display_version);
    };

    Ok(config.into())
}
