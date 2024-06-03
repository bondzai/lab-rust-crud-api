use actix_web::{web, HttpResponse};
use std::sync::{Arc, Mutex};
use crate::models::Todo;
use crate::repositories::TodoRepositoryImpl;

type SharedRepo = Arc<Mutex<TodoRepositoryImpl>>;

pub async fn create_todo(
    repo: web::Data<SharedRepo>,
    todo: web::Json<Todo>,
) -> HttpResponse {
    let mut repo = repo.lock().unwrap();
    let created_todo = repo.create(todo.into_inner());
    HttpResponse::Ok().json(created_todo)
}

pub async fn get_all_todos(repo: web::Data<SharedRepo>) -> HttpResponse {
    let repo = repo.lock().unwrap();
    let todos = repo.get_all();
    HttpResponse::Ok().json(todos)
}

pub async fn get_todo(
    repo: web::Data<SharedRepo>,
    info: web::Path<usize>,
) -> HttpResponse {
    let repo = repo.lock().unwrap();
    let todo = repo.get(info.into_inner());
    match todo {
        Some(todo) => HttpResponse::Ok().json(todo),
        None => HttpResponse::NotFound().finish(),
    }
}

pub async fn update_todo(
    repo: web::Data<SharedRepo>,
    info: web::Path<usize>,
    todo: web::Json<Todo>,
) -> HttpResponse {
    let mut repo = repo.lock().unwrap();
    let updated_todo = repo.update(info.into_inner(), todo.into_inner());
    match updated_todo {
        Some(todo) => HttpResponse::Ok().json(todo),
        None => HttpResponse::NotFound().finish(),
    }
}

pub async fn delete_todo(
    repo: web::Data<SharedRepo>,
    info: web::Path<usize>,
) -> HttpResponse {
    let mut repo = repo.lock().unwrap();
    let result = repo.delete(info.into_inner());
    match result {
        true => HttpResponse::NoContent().finish(),
        false => HttpResponse::NotFound().finish(),
    }
}
