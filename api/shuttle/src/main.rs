use actix_web::web::ServiceConfig;
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::CustomError;
use shuttle_secrets::SecretStore;
use sqlx::{Executor, PgPool};

#[shuttle_runtime::main]
async fn main(
    #[shuttle_secrets::Secrets] secret_store: SecretStore
             ) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {

    let db_url = secret_store
        .get("DATABASE_URL")
        .expect("Secret was not found: DATABASE_URL");

    let db_connetion = PgPool::connect(&db_url)
        .await
        .map_err(CustomError::new)?;

    let pool = actix_web::web::Data::new(db_connetion.clone());

    db_connetion.execute(include_str!("../../migrations/schema.sql"))
        .await
        .map_err(CustomError::new)?;

    let config = move |cfg: &mut ServiceConfig| {
        cfg.app_data(pool)
            .configure(api_lib::health::service)
            .configure(api_lib::films::service);
    };

    Ok(config.into())
}
