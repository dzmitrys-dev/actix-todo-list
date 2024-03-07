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
use serde_json::json;
use lazy_static::lazy_static;
use crate::handlers::*;
use crate::models::*;
use crate::config::Config;

lazy_static! {
    static ref APP_STATE: AppState = {
        dotenv().ok();

        let config = Config::from_env().unwrap();


        let pool = config.configure_pool();

        let log = Config::configure_log();

        AppState {
                    pool: pool.clone(),
                    log: log.clone(),
                    config: config.clone(),
                }
    };
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    info!(APP_STATE.log, "Server running on {}:{}", APP_STATE.config.server.host, APP_STATE.config.server.port);

    HttpServer::new(move || {
        App::new()
            .app_data(
                web::Data::new(APP_STATE.clone())
            )
            .route("/", web::get().to(status))
            .route("/todos{_:/?}", web::get().to(get_todos))
            .route("/todos{_:/?}", web::post().to(create_todo))
            .route("/todos/{list_id}{_:/?}", web::delete().to(delete_todo))
            .route("/todos/{list_id}/items{_:/?}", web::post().to(create_item))
            .route("/todos/{list_id}/items{_:/?}", web::get().to(get_items))
            .route("/todos/{list_id}/items/{item_id}{_:/?}", web::delete().to(delete_item))
            .route("/todos/{list_id}/items/{item_id}{_:/?}", web::put().to(check_item))
    }).bind(format!("{}:{}", APP_STATE.config.server.host, APP_STATE.config.server.port))?.run().await
}

#[cfg(test)]
mod integration_tests {
    use actix_web::{App, web, test};
    use dotenv::dotenv;
    use serde_json::json;
    use crate::APP_STATE;
    use crate::config::Config;
    use crate::handlers::*;
    use crate::models::*;

    #[actix_rt::test]
    async fn test_get_todos() {
        let app = App::new()
            .app_data(
                web::Data::new(APP_STATE.clone())
            )
            .route("/todos{_:/?}", web::get().to(get_todos));

        let mut app = test::init_service(app).await;

        let req = test::TestRequest::get().uri("/todos").to_request();
        let resp = test::call_service(&mut app, req).await;

        assert!(resp.status().is_success(), "GET /todos must return status code 200");
    }

    #[actix_rt::test]
    async fn test_create_todo() {
        let app = App::new()
            .app_data(
                web::Data::new(APP_STATE.clone())
            )
            .route("/todos{_:/?}", web::get().to(get_todos))
            .route("/todos{_:/?}", web::post().to(create_todo));

        let mut app = test::init_service(app).await;

        let todo_title = "Test todolist title";

        let create_todo_list = json!({
            "title": todo_title,
        });

        let req = test::TestRequest::post()
            .uri("/todos")
            .append_header(("Content-Type", "application/json"))
            .set_payload(create_todo_list.to_string())
            .to_request();

        let resp = test::call_service(&mut app, req).await;

        assert!(resp.status().is_success(), "POST /todos must return status code 200");

        let body = test::read_body(resp).await;

        let try_created: Result<TodoList, serde_json::Error> = serde_json::from_slice(&body);
        assert!(try_created.is_ok(), "Response body must be a TodoList");

        let created = try_created.unwrap();

        let req = test::TestRequest::get().uri(&format!("/todos")).to_request();

        let todos: Vec<TodoList> = test::read_response_json(&mut app, req).await;

        let maybe_todo = todos.iter().find(|todo| todo.id == created.id);

        assert!(maybe_todo.is_some(), "Todo must be created");
        assert!(maybe_todo.unwrap().title == todo_title, "Todo title must match");
    }
}