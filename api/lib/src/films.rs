use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse
};

pub fn service(config: &mut ServiceConfig) {
    config.service(
        web::scope("/v1/films")

        // get all films
        .route("", web::get().to(get_all))

        // get by id 
        .route("/{film_id}", web::get().to(get_by_id))

        // post new film 
        .route("", web::post().to(post))

        //update film
        .route("", web::put().to(put))

        //delete film
        .route("/{film_id}", web::delete().to(delete)),
    );
}

async fn get_all() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn get_by_id() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn post() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn put() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn delete() -> HttpResponse {
    HttpResponse::Ok().finish()
}
