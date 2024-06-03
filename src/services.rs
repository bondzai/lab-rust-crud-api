use crate::models::Todo;
use crate::repositories::TodoRepositoryImpl;
use std::sync::{Arc, Mutex};

pub struct TodoServiceImpl {
    repository: Arc<Mutex<TodoRepositoryImpl>>,
}

impl TodoServiceImpl {
    pub fn new(repo: Arc<Mutex<TodoRepositoryImpl>>) -> Self {
        TodoServiceImpl { repository: repo }
    }

    pub fn create_todo(&self, todo: Todo) -> Todo {
        let mut repo = self.repository.lock().unwrap();
        repo.create(todo)
    }

    pub fn get_all_todos(&self) -> Vec<Todo> {
        let repo = self.repository.lock().unwrap();
        repo.get_all()
    }

    pub fn get_todo(&self, id: usize) -> Option<Todo> {
        let repo = self.repository.lock().unwrap();
        repo.get(id)
    }

    pub fn update_todo(&self, id: usize, todo: Todo) -> Option<Todo> {
        let mut repo = self.repository.lock().unwrap();
        repo.update(id, todo)
    }

    pub fn delete_todo(&self, id: usize) -> bool {
        let mut repo = self.repository.lock().unwrap();
        repo.delete(id)
    }
}
