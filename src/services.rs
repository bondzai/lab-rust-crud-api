use crate::models::Todo;
use crate::repositories::{TodoRepository, TodoRepositoryImpl};

pub trait TodoService {
    fn new() -> Self;
    fn create_todo(&mut self, todo: Todo) -> Todo;
    fn get_all_todos(&self) -> Vec<Todo>;
    fn get_todo(&self, id: usize) -> Option<Todo>;
    fn update_todo(&mut self, id: usize, todo: Todo) -> Option<Todo>;
    fn delete_todo(&mut self, id: usize) -> bool;
}

pub struct TodoServiceImpl;

impl TodoService for TodoServiceImpl {
    fn new() -> Self {
        TodoServiceImpl
    }

    fn create_todo(&mut self, todo: Todo) -> Todo {
        let mut repository = TodoRepositoryImpl::new();
        repository.create(todo)
    }

    fn get_all_todos(&self) -> Vec<Todo> {
        let repository = TodoRepositoryImpl::new();
        repository.get_all()
    }

    fn get_todo(&self, id: usize) -> Option<Todo> {
        let repository = TodoRepositoryImpl::new();
        repository.get(id)
    }

    fn update_todo(&mut self, id: usize, todo: Todo) -> Option<Todo> {
        let mut repository = TodoRepositoryImpl::new();
        repository.update(id, todo)
    }

    fn delete_todo(&mut self, id: usize) -> bool {
        let mut repository = TodoRepositoryImpl::new();
        repository.delete(id)
    }
}
