use crate::task::{List, Task};
use automerge::AutoCommit;
use autosurgeon::{hydrate, reconcile};

pub struct Store {
    pub db_path: String,
    pub next_id: u64,
    pub doc: AutoCommit,
}

impl Store {
    pub fn new(db_path: &str) -> anyhow::Result<Self> {
        let list = List::new();

        let mut doc = AutoCommit::new();
        reconcile(&mut doc, &list)?;

        Ok(Self {
            db_path: db_path.to_string(),
            next_id: 0,
            doc,
        })
    }

    pub fn save(&mut self) -> anyhow::Result<()> {
        let bytes = self.doc.save();
        std::fs::write(&self.db_path, &bytes)?;

        Ok(())
    }

    pub fn load(&mut self) -> anyhow::Result<()> {
        let bytes = std::fs::read(&self.db_path)?;
        self.doc = AutoCommit::load(&bytes)?;

        Ok(())
    }

    pub fn save_to_file(&mut self) -> anyhow::Result<()> {
        let bytes = self.doc.save();
        std::fs::write(&self.db_path, &bytes)?;

        Ok(())
    }

    pub fn add_task(&mut self, title: &str) -> anyhow::Result<()> {
        let task = Task::new(title, self.next_id);

        let mut doc = self.doc.fork().with_actor(automerge::ActorId::random());
        let mut list: List = hydrate(&doc)?;
        list.tasks.insert(self.next_id as usize, task);
        reconcile(&mut doc, &list)?;

        self.next_id += 1;
        self.doc = doc;

        Ok(())
    }

    pub fn list_tasks(&mut self) -> anyhow::Result<List> {
        let list: List = hydrate(&self.doc)?;
        Ok(list)
    }

    pub fn do_task(&mut self, id: u64) -> anyhow::Result<()> {
        let mut doc = self.doc.fork().with_actor(automerge::ActorId::random());

        let mut list: List = hydrate(&doc)?;
        list.tasks.get_mut(id as usize).unwrap().done = true;

        reconcile(&mut doc, &list)?;
        self.doc = doc;

        Ok(())
    }

    pub fn undo_task(&mut self, id: u64) -> anyhow::Result<()> {
        let mut doc = self.doc.fork().with_actor(automerge::ActorId::random());

        let mut list: List = hydrate(&doc)?;
        list.tasks.get_mut(id as usize).unwrap().done = false;

        reconcile(&mut doc, &list)?;
        self.doc = doc;

        Ok(())
    }

    pub fn delete_task(&mut self, id: u64) -> anyhow::Result<()> {
        let mut doc = self.doc.fork().with_actor(automerge::ActorId::random());

        let mut list: List = hydrate(&doc)?;
        list.tasks.remove(id as usize);

        reconcile(&mut doc, &list)?;
        self.doc = doc;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(unused_imports)]
    use automerge_test::{assert_doc, list, map, RealizedObject};

    #[test]
    fn test_new_store() {
        let store = Store::new("test.json").unwrap();
        assert_doc!(
            &store.doc,
            map! {
                "tasks" => {list![]}
            }
        )
    }

    #[test]
    fn test_save_and_load() {
        let mut store = Store::new("test.json").unwrap();
        store.add_task("Buy groceries").unwrap();
        store.save_to_file().unwrap();

        let mut store2 = Store::new("test.json").unwrap();
        store2.load().unwrap();
        assert_doc!(
            &store2.doc,
            map! {
                "tasks" => { list! {
                    { map! {
                        "id" => { 0_u64 },
                        "title" => { "Buy groceries" },
                        "done" => { false }
                    }}
                }}
            }
        );
    }

    #[test]
    fn test_add_task() {
        let mut store = Store::new("test.json").unwrap();
        store.add_task("Buy groceries").unwrap();
        assert_doc!(
            &store.doc,
            map! {
                "tasks" => { list! {
                    { map! {
                        "id" => { 0_u64 },
                        "title" => { "Buy groceries" },
                        "done" => { false }
                    }}
                }}
            }
        );

        store.add_task("Clean room").unwrap();
        assert_doc!(
            &store.doc,
            map! {
                "tasks" => { list! {
                    { map! {
                        "id" => { 0_u64 },
                        "title" => { "Buy groceries" },
                        "done" => { false }
                    }},
                    { map! {
                        "id" => { 1_u64 },
                        "title" => { "Clean room" },
                        "done" => { false }
                    }}
                }}
            }
        );
    }

    #[test]
    fn test_do_task() {
        let mut store = Store::new("test.json").unwrap();
        store.add_task("Buy groceries").unwrap();
        store.do_task(0).unwrap();
        assert_doc!(
            &store.doc,
            map! {
                "tasks" => { list! {
                    { map! {
                        "id" => { 0_u64 },
                        "title" => { "Buy groceries" },
                        "done" => { true }
                    }}
                }}
            }
        );
    }

    #[test]
    fn test_undo_task() {
        let mut store = Store::new("test.json").unwrap();
        store.add_task("Buy groceries").unwrap();
        store.do_task(0).unwrap();
        store.undo_task(0).unwrap();
        assert_doc!(
            &store.doc,
            map! {
                "tasks" => { list! {
                    { map! {
                        "id" => { 0_u64 },
                        "title" => { "Buy groceries" },
                        "done" => { false }
                    }}
                }}
            }
        );
    }
}
