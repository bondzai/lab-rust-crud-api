use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Todo {
    pub id: usize,
    pub title: String,
    pub completed: bool,
}
