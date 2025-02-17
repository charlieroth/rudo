use crate::task::Task;
use automerge::{transaction::Transactable, AutoCommit, ObjType, ReadDoc};

pub struct Store {
    doc: AutoCommit,
}

impl Store {
    pub fn new() -> anyhow::Result<Self> {
        let mut doc = AutoCommit::new();
        let _ = doc.put_object(automerge::ROOT, "tasks", ObjType::List)?;
        Ok(Self { doc })
    }

    pub fn load(path: &str) -> anyhow::Result<Self> {
        let saved = std::fs::read(path)?;
        let doc = AutoCommit::load(&saved)?;
        Ok(Self { doc })
    }

    pub fn save(&mut self, path: &str) -> anyhow::Result<()> {
        let saved = self.doc.save();
        std::fs::write(path, &saved)?;
        Ok(())
    }

    pub fn add_task(&mut self, title: &str) -> anyhow::Result<()> {
        let task = Task::new(title);
        let tasks = self.doc.get(automerge::ROOT, "tasks")?;
        tasks.insert_object(&tasks, task);
        Ok(())
    }
}
