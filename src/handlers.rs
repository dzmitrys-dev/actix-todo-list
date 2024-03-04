use actix_web::{HttpResponse, Responder, web};
use deadpool_postgres::{Client, Pool};
use slog::{crit, Drain, error, Logger, o};
use web::Json;
use crate::models::*;
use crate::db;
use crate::errors::AppError;

pub async fn get_client(pool: Pool, log: Logger) -> Result<Client, AppError> {
    pool.get().await.map_err(|err| {
        let log = log.new(o!("cause" => format!("{:?}", err)));
        crit!(log, "Error getting client from pool"; "error" => format!("{:?}", err));
        AppError::db_error(err)
    })
}

pub fn log_error(log: Logger) -> Box<dyn Fn(AppError) -> AppError> {
    Box::new(move |err| {
        let log = log.new(o!("cause" => format!("{:?}", err)));
        error!(log, "Error in handler"; "error" => format!("{:?}", err));
        err
    })
}

pub async fn status() -> impl Responder {
    HttpResponse::Ok().json(Status {
        status: "UP".to_string()
    })
}

pub async fn get_todos(state: web::Data<AppState>) -> Result<impl Responder, AppError> {
    let log = state.log.new(o!("handler" => "get_todos"));

    let client: Client = get_client(state.pool.clone(), log.clone()).await?;

    let result = db::get_todos(&client).await;

    result.map(|todos| HttpResponse::Ok().json(todos))
        .map_err(log_error(log))
}

pub async fn get_items(state: web::Data<AppState>, path: web::Path<(i32,)>) -> Result<impl Responder, AppError> {
    let log = state.log.new(o!("handler" => "get_items"));

    let client: Client = get_client(state.pool.clone(), log.clone()).await?;

    let result = db::get_items(&client, path.0).await;

    Ok(result.map(|items| HttpResponse::Ok().json(items)).map_err(log_error(log)))
}

pub async fn create_todo(state: web::Data<AppState>, json: Json<CreateTodoList>) -> Result<impl Responder, AppError> {
    let log = state.log.new(o!("handler" => "create_todo"));

    let client: Client = get_client(state.pool.clone(), log.clone()).await?;

    let result = db::create_todo(&client, json.title.clone()).await;

    result.map(|todo| HttpResponse::Ok().json(todo)).map_err(log_error(log))
}

pub async fn delete_todo(state: web::Data<AppState>, path: web::Path<(i32, )>) -> Result<impl Responder, AppError> {
    let log = state.log.new(o!("handler" => "delete_todo"));

    let client: Client = get_client(state.pool.clone(), log.clone()).await?;

    let result = db::delete_todo(&client, path.0).await;

    result.map(|todo| HttpResponse::Ok().json(todo)).map_err(log_error(log))
}

pub async fn create_item(state: web::Data<AppState>, path: web::Path<(i32,)>, json: Json<CreateTodoItem>) -> Result<impl Responder, AppError> {
    let log = state.log.new(o!("handler" => "create_item"));

    let client: Client = get_client(state.pool.clone(), log.clone()).await?;

    let result = db::create_item(&client, path.0, json.title.clone()).await;

    result.map(|item| HttpResponse::Ok().json(item)).map_err(log_error(log))
}

pub async fn delete_item(state: web::Data<AppState>, path: web::Path<(i32, i32)>) -> Result<impl Responder, AppError> {
    let log = state.log.new(o!("handler" => "delete_item"));

    let client: Client = get_client(state.pool.clone(), log.clone()).await?;

    let result = db::delete_item(&client, path.1).await;

    result.map(|item| HttpResponse::Ok().json(item)).map_err(log_error(log))
}

pub async fn check_item(state: web::Data<AppState>, path: web::Path<(i32,i32)>) -> Result<impl Responder, AppError> {
    let log = state.log.new(o!("handler" => "check_item"));

    let client: Client = get_client(state.pool.clone(), log.clone()).await?;

    let result = db::check_item(&client, path.1).await;

    result.map(|item| HttpResponse::Ok().json(item)).map_err(log_error(log))
}