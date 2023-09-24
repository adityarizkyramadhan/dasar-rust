use uuid::Uuid;
use chrono::Utc;

#[derive(Debug)]
pub struct Todo {
    id: String,
    is_done: bool,
    name: String,
    created_at: String,
}

impl Todo{
    pub fn new(name: String) -> Todo {
        Todo {
            id: Uuid::new_v4().to_string(),
            is_done: false,
            name,
            created_at: Utc::now().to_string(),
        }
    }

    pub fn mark_done(&mut self) {
        self.is_done = true;
    }

    pub fn mark_undone(&mut self) {
        self.is_done = false;
    }

    pub fn rename(&mut self, name: String) {
        self.name = name;
    }

    pub fn print(&self) {
        let status = if self.is_done { "done" } else { "not done" };
        println!("{} - {} ({}) : {}", self.id, self.name, status, self.created_at);
    }

}
