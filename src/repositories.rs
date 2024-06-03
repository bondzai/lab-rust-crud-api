use std::collections::HashMap;
use crate::models::Todo;

pub struct TodoRepositoryImpl {
    todos: HashMap<usize, Todo>,
    counter: usize,
}

impl TodoRepositoryImpl {
    pub fn new() -> Self {
        TodoRepositoryImpl {
            todos: HashMap::new(),
            counter: 1,
        }
    }

    pub fn create(&mut self, mut todo: Todo) -> Todo {
        todo.id = self.counter;
        self.counter += 1;
        self.todos.insert(todo.id, todo.clone());
        todo
    }

    pub fn get_all(&self) -> Vec<Todo> {
        self.todos.values().cloned().collect()
    }

    pub fn get(&self, id: usize) -> Option<Todo> {
        self.todos.get(&id).cloned()
    }

    pub fn update(&mut self, id: usize, todo: Todo) -> Option<Todo> {
        if let Some(existing_todo) = self.todos.get_mut(&id) {
            existing_todo.title = todo.title;
            existing_todo.completed = todo.completed;
            Some(existing_todo.clone())
        } else {
            None
        }
    }

    pub fn delete(&mut self, id: usize) -> bool {
        self.todos.remove(&id).is_some()
    }
}
