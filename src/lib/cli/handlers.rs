use crate::{store::Store, task::List};

pub fn add(store: &mut Store, title: &str) -> anyhow::Result<()> {
    store.add_task(title)?;
    store.save()?;
    Ok(())
}

pub fn list(store: &mut Store) -> anyhow::Result<()> {
    let list = store.list_tasks()?;
    print_list(&list);
    Ok(())
}

pub fn do_task(store: &mut Store, id: u64) -> anyhow::Result<()> {
    store.do_task(id)?;
    store.save()?;
    Ok(())
}

pub fn undo_task(store: &mut Store, id: u64) -> anyhow::Result<()> {
    store.undo_task(id)?;
    store.save()?;
    Ok(())
}

pub fn delete_task(store: &mut Store, id: u64) -> anyhow::Result<()> {
    store.delete_task(id)?;
    store.save()?;
    Ok(())
}

fn print_list(list: &List) {
    if list.tasks.is_empty() {
        println!("No tasks");
        return;
    }

    for (id, task) in list.tasks.iter().enumerate() {
        if task.done {
            println!("{}: [X] {}", id, task.title);
        } else {
            println!("{}: [ ] {}", id, task.title);
        }
    }
}
