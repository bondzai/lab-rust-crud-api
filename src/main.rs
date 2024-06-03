use actix_web::{web, App, HttpServer};
use std::sync::{Arc, Mutex};

mod models;
mod handlers;
mod repositories;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let repository = Arc::new(Mutex::new(repositories::TodoRepositoryImpl::new()));
    let shared_data = web::Data::new(repository);

    HttpServer::new(move || {
        App::new()
            .app_data(shared_data.clone())
            .service(
                web::scope("/todos")
                    .route("", web::post().to(handlers::create_todo))
                    .route("", web::get().to(handlers::get_all_todos))
                    .route("/{id}", web::get().to(handlers::get_todo))
                    .route("/{id}", web::put().to(handlers::update_todo))
                    .route("/{id}", web::delete().to(handlers::delete_todo)),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
