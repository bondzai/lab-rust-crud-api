use std::collections::HashMap;
use crate::models::Todo;

pub trait TodoRepository {
    fn new() -> Self;
    fn create(&mut self, todo: Todo) -> Todo;
    fn get_all(&self) -> Vec<Todo>;
    fn get(&self, id: usize) -> Option<Todo>;
    fn update(&mut self, id: usize, todo: Todo) -> Option<Todo>;
    fn delete(&mut self, id: usize) -> bool;
}

pub struct TodoRepositoryImpl {
    todos: HashMap<usize, Todo>,
    counter: usize,
}

impl TodoRepository for TodoRepositoryImpl {
    fn new() -> Self {
        TodoRepositoryImpl {
            todos: HashMap::new(),
            counter: 1, // Start counting from 1
        }
    }

    fn create(&mut self, mut todo: Todo) -> Todo {
        todo.id = self.counter;
        self.counter += 1;
        self.todos.insert(todo.id, todo.clone());
        todo
    }

    fn get_all(&self) -> Vec<Todo> {
        self.todos.values().cloned().collect()
    }

    fn get(&self, id: usize) -> Option<Todo> {
        self.todos.get(&id).cloned()
    }

    fn update(&mut self, id: usize, todo: Todo) -> Option<Todo> {
        if let Some(_) = self.todos.get(&id) {
            self.todos.insert(id, todo.clone());
            Some(todo)
        } else {
            None
        }
    }

    fn delete(&mut self, id: usize) -> bool {
        self.todos.remove(&id).is_some()
    }
}
