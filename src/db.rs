use crate::models::*;
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use crate::errors::*;

pub async fn get_todos(client: &Client) -> Result<Vec<TodoList>, AppError> {
    let statement = client
        .prepare("SELECT * FROM todo_list order by id desc")
        .await
        .map_err(AppError::db_error)?;
    let todos = client.query(&statement, &[])
        .await
        .expect("Error getting todo lists")
        .iter()
        .map(|row| TodoList::from_row_ref(row).unwrap())
        .collect::<Vec<TodoList>>();
    Ok(todos)
}

pub async fn get_items(client: &Client, list_id: i32) -> Result<Vec<TodoItem>, AppError> {
    let statement = client
        .prepare("SELECT * FROM todo_item WHERE list_id = $1 order by id")
        .await
        .map_err(AppError::db_error)?;
    let items = client.query(&statement, &[&list_id])
        .await
        .map_err(AppError::db_error)?
        .iter()
        .map(|row| TodoItem::from_row_ref(row).unwrap())
        .collect::<Vec<TodoItem>>();
    Ok(items)
}

pub async fn create_todo(client: &Client, title: String) -> Result<TodoList, AppError> {
    let statement = client
        .prepare("INSERT INTO todo_list (title) VALUES ($1) RETURNING id, title")
        .await
        .map_err(AppError::db_error)?;
    client.query(&statement, &[&title])
        .await
        .map_err(AppError::db_error)?
        .iter()
        .map(|row| TodoList::from_row_ref(row).unwrap())
        .collect::<Vec<TodoList>>()
        .pop()
        .ok_or(AppError{
            message: Option::from("Error creating todo list".to_string()),
            cause: None,
            error_type: AppErrorType::DbError })
}

pub async fn delete_todo(client: &Client, id: i32) -> Result<TodoList, AppError> {
    let statement = client
        .prepare("DELETE FROM todo_list WHERE id = $1 RETURNING id, title")
        .await
        .map_err(AppError::db_error)?;
    client.query(&statement, &[&id])
        .await
        .map_err(AppError::db_error)?
        .iter()
        .map(|row| TodoList::from_row_ref(row).unwrap())
        .collect::<Vec<TodoList>>()
        .pop()
        .ok_or(AppError{
            message: Option::from("Error deleting todo list".to_string()),
            cause: None,
            error_type: AppErrorType::DbError })
}

pub async fn create_item(client: &Client, list_id: i32, title: String) -> Result<TodoItem, AppError> {
    let statement = client

        .prepare("INSERT INTO todo_item (list_id, title) VALUES ($1, $2) RETURNING id, list_id, title, checked")
        .await
        .map_err(AppError::db_error)?;
    client.query(&statement, &[&list_id, &title])
        .await
        .map_err(AppError::db_error)?
        .iter()
        .map(|row| TodoItem::from_row_ref(row).unwrap())
        .collect::<Vec<TodoItem>>()
        .pop()
        .ok_or(AppError{
            message: Option::from("Error creating todo item".to_string()),
            cause: None,
            error_type: AppErrorType::DbError })
}

pub async fn delete_item(client: &Client, id: i32) -> Result<TodoItem, AppError> {
    let statement = client
        .prepare("DELETE FROM todo_item WHERE id = $1 RETURNING id, list_id, title, checked")
        .await
        .map_err(AppError::db_error)?;
    client.query(&statement, &[&id])
        .await
        .map_err(AppError::db_error)?
        .iter()
        .map(|row| TodoItem::from_row_ref(row).unwrap())
        .collect::<Vec<TodoItem>>()
        .pop()
        .ok_or(AppError{
            message: Option::from("Error deleting todo item".to_string()),
            cause: None,
            error_type: AppErrorType::DbError })
}

pub async fn check_item(client: &Client, id: i32) -> Result<TodoItem, AppError> {
    let statement = client
        .prepare("UPDATE todo_item SET checked = NOT checked WHERE id = $1 RETURNING id, list_id, title, checked")
        .await
        .map_err(AppError::db_error)?;
    client.query(&statement, &[&id])
        .await
        .map_err(AppError::db_error)?
        .iter()
        .map(|row| TodoItem::from_row_ref(row).unwrap())
        .collect::<Vec<TodoItem>>()
        .pop()
        .ok_or(AppError{
            message: Option::from("Error checking todo item".to_string()),
            cause: None,
            error_type: AppErrorType::DbError })
}