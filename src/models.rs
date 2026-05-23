#[derive(Debug, Clone)]
pub struct Task {
    pub id: i64,
    pub title: String,
    pub completed: bool,
    pub priority: i32,
    pub position: i32,
    pub created_at: String,
}

#[derive(Debug, Clone)]
pub struct Note {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
}
