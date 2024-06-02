use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Todo {
    #[serde(default)]
    pub id: usize,
    pub title: String,
    pub completed: bool,
}
