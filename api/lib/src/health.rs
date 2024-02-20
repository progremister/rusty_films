use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse,
};

pub fn service(cfg: &mut ServiceConfig) {
    cfg.route("/health", web::get().to(health));
}

pub const API_VERSION: &str = "0.0.1";

async fn health() -> HttpResponse {
    HttpResponse::Ok()
        .append_header(("version", API_VERSION))
        .finish()
}

#[cfg(test)]
mod tests {
    use actix_web::http::StatusCode;

    use super::*;

    #[actix_rt::test]
    async fn health_check_works() {
        let resp = health().await;

        assert!(resp.status().is_success());
        assert_eq!(resp.status(), StatusCode::OK);

        let data = resp
            .headers()
            .get("version")
            .and_then(|h| h.to_str().ok());

        assert_eq!(data, Some(API_VERSION));
    }
}
