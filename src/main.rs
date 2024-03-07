mod models;
mod config;
mod handlers;
mod db;
mod errors;

use std::io;
use actix_web::{HttpServer, App, web};
use dotenv::dotenv;
use slog::{
    info
};
use crate::handlers::*;
use crate::models::AppState;
use crate::config::Config;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let config = Config::from_env().unwrap();


    let pool = config.configure_pool();

    let log = Config::configure_log();

    info!(log, "Server running on {}:{}", config.server.host, config.server.port);

    HttpServer::new(move || {
        App::new()
            .app_data(
                web::Data::new(AppState {
                    pool: pool.clone(),
                    log: log.clone(),
                })
            )
            .route("/", web::get().to(status))
            .route("/todos{_:/?}", web::get().to(get_todos))
            .route("/todos{_:/?}", web::post().to(create_todo))
            .route("/todos/{list_id}{_:/?}", web::delete().to(delete_todo))
            .route("/todos/{list_id}/items{_:/?}", web::post().to(create_item))
            .route("/todos/{list_id}/items{_:/?}", web::get().to(get_items))
            .route("/todos/{list_id}/items/{item_id}{_:/?}", web::delete().to(delete_item))
            .route("/todos/{list_id}/items/{item_id}{_:/?}", web::put().to(check_item))
    }).bind(format!("{}:{}", config.server.host, config.server.port))?.run().await
}

#[cfg(test)]
mod integration_tests {
    use actix_web::{App, web, test};
    use dotenv::dotenv;
    use crate::config::Config;
    use crate::handlers::*;
    use crate::models::*;

    #[actix_rt::test]
    async fn test_get_todos() {
        dotenv().ok();

        let config = Config::from_env().unwrap();


        let pool = config.configure_pool();

        let log = Config::configure_log();

        let app = App::new()
            .app_data(
                web::Data::new(AppState {
                    pool: pool.clone(),
                    log: log.clone(),
                })
            )
            .route("/todos{_:/?}", web::get().to(get_todos));

        let mut app = test::init_service(app).await;

        let req = test::TestRequest::get().uri("/todos").to_request();
        let resp = test::call_service(&mut app, req).await;

        assert!(resp.status().is_success(), "GET /todos must return status code 200");
    }

    #[actix_rt::test]
    async fn test_create_todo() {

    }
}