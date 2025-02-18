use autosurgeon::{Hydrate, Reconcile};

#[derive(Debug, Clone, Reconcile, Hydrate, PartialEq, Eq)]
pub struct Task {
    #[key]
    pub id: u64,
    pub title: String,
    pub done: bool,
}

impl Task {
    pub fn new(title: &str, id: u64) -> Self {
        Self {
            id,
            title: title.to_string(),
            done: false,
        }
    }
}

#[derive(Debug, Clone, Reconcile, Hydrate, PartialEq, Eq)]
pub struct List {
    pub tasks: Vec<Task>,
}

impl List {
    pub fn new() -> Self {
        Self { tasks: vec![] }
    }
}
