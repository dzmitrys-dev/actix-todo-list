mod models;
mod config;
mod handlers;
mod db;
mod errors;

use std::io;
use actix_web::{HttpServer, App, web, Responder};
use dotenv::dotenv;
use

slog::{
    Logger,
    Drain,
    o,
    info
};
use tokio_postgres::NoTls;
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
                web::Data::new(AppState{
                    pool: pool.clone(),
                    log: log.clone()
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
    use actix_web::{App, web};
    use dotenv::dotenv;
    use tokio_postgres::NoTls;
    use crate::config::Config;
    use crate::models::AppState;

    #[actix_rt::test]
    async fn test_get_todos() {
        dotenv().ok();

        let config = Config::from_env().unwrap();


        let pool = config.configure_pool();

        let log = Config::configure_log();

        // App::new()
        //     .app_data(
        //         web::Data::new(AppState{
        //             pool: pool.clone(),
        //             log: log.clone()
        //         })
        //     )
    }
}