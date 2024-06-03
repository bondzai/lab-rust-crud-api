use actix_web::{web, HttpResponse};
use std::sync::{Arc, Mutex};
use crate::models::Todo;
use crate::services::TodoServiceImpl;

type SharedService = Arc<Mutex<TodoServiceImpl>>;

pub async fn create_todo(
    service: web::Data<SharedService>,
    todo: web::Json<Todo>,
) -> HttpResponse {
    let service = service.lock().unwrap();
    let created_todo = service.create_todo(todo.into_inner());
    HttpResponse::Ok().json(created_todo)
}

pub async fn get_all_todos(service: web::Data<SharedService>) -> HttpResponse {
    let service = service.lock().unwrap();
    let todos = service.get_all_todos();
    HttpResponse::Ok().json(todos)
}

pub async fn get_todo(
    service: web::Data<SharedService>,
    info: web::Path<usize>,
) -> HttpResponse {
    let service = service.lock().unwrap();
    let todo = service.get_todo(info.into_inner());
    match todo {
        Some(todo) => HttpResponse::Ok().json(todo),
        None => HttpResponse::NotFound().finish(),
    }
}

pub async fn update_todo(
    service: web::Data<SharedService>,
    info: web::Path<usize>,
    todo: web::Json<Todo>,
) -> HttpResponse {
    let service = service.lock().unwrap();
    let updated_todo = service.update_todo(info.into_inner(), todo.into_inner());
    match updated_todo {
        Some(todo) => HttpResponse::Ok().json(todo),
        None => HttpResponse::NotFound().finish(),
    }
}

pub async fn delete_todo(
    service: web::Data<SharedService>,
    info: web::Path<usize>,
) -> HttpResponse {
    let service = service.lock().unwrap();
    let result = service.delete_todo(info.into_inner());
    match result {
        true => HttpResponse::NoContent().finish(),
        false => HttpResponse::NotFound().finish(),
    }
}
