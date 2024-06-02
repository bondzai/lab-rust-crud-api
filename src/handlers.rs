use actix_web::{web, HttpResponse};
use crate::models::Todo;
use crate::services::{TodoService, TodoServiceImpl};

pub async fn create_todo(todo: web::Json<Todo>) -> HttpResponse {
    let mut service = TodoServiceImpl::new();
    let created_todo = service.create_todo(todo.into_inner());
    HttpResponse::Ok().json(created_todo)
}

pub async fn get_all_todos() -> HttpResponse {
    let service = TodoServiceImpl::new();
    let todos = service.get_all_todos();
    HttpResponse::Ok().json(todos)
}

pub async fn get_todo(info: web::Path<usize>) -> HttpResponse {
    let service = TodoServiceImpl::new();
    let todo = service.get_todo(info.into_inner());
    match todo {
        Some(todo) => HttpResponse::Ok().json(todo),
        None => HttpResponse::NotFound().finish(),
    }
}

pub async fn update_todo(info: web::Path<usize>, todo: web::Json<Todo>) -> HttpResponse {
    let mut service = TodoServiceImpl::new();
    let updated_todo = service.update_todo(info.into_inner(), todo.into_inner());
    match updated_todo {
        Some(todo) => HttpResponse::Ok().json(todo),
        None => HttpResponse::NotFound().finish(),
    }
}

pub async fn delete_todo(info: web::Path<usize>) -> HttpResponse {
    let mut service = TodoServiceImpl::new();
    let result = service.delete_todo(info.into_inner());
    match result {
        true => HttpResponse::NoContent().finish(),
        false => HttpResponse::NotFound().finish(),
    }
}
