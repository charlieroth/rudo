pub struct Task {
    id: uuid::Uuid,
    title: String,
    done: bool,
}

impl Task {
    pub fn new(title: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            title: title.to_string(),
            done: false,
        }
    }
}
