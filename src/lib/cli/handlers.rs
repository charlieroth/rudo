pub fn add(title: &str) -> anyhow::Result<()> {
    println!("Adding task: {}", title);
    Ok(())
}

pub fn list() -> anyhow::Result<()> {
    println!("Listing all tasks");
    Ok(())
}

pub fn do_task(id: u64) -> anyhow::Result<()> {
    println!("Marking task as done: {}", id);
    Ok(())
}

pub fn undo_task(id: u64) -> anyhow::Result<()> {
    println!("Marking task as undone: {}", id);
    Ok(())
}

pub fn delete_task(id: u64) -> anyhow::Result<()> {
    println!("Deleting task: {}", id);
    Ok(())
}
