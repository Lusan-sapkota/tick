#[derive(Debug, Clone, PartialEq)]
pub enum Schedule {
    Today,
    Tomorrow,
    Later,
    Unscheduled,
}

impl Schedule {
    pub fn from_i32(v: i32) -> Self {
        match v {
            1 => Schedule::Today,
            2 => Schedule::Tomorrow,
            3 => Schedule::Later,
            _ => Schedule::Unscheduled,
        }
    }

    pub fn to_i32(&self) -> i32 {
        match self {
            Schedule::Today => 1,
            Schedule::Tomorrow => 2,
            Schedule::Later => 3,
            Schedule::Unscheduled => 0,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Schedule::Today => "Today",
            Schedule::Tomorrow => "Tomorrow",
            Schedule::Later => "Scheduled",
            Schedule::Unscheduled => "Backlog",
        }
    }

    #[allow(dead_code)]
    pub fn order(&self) -> i32 {
        match self {
            Schedule::Today => 0,
            Schedule::Tomorrow => 1,
            Schedule::Unscheduled => 2,
            Schedule::Later => 3,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Task {
    pub id: i64,
    pub title: String,
    pub completed: bool,
    pub priority: i32,
    pub position: i32,
    pub schedule: Schedule,
    pub created_at: String,
}

#[derive(Debug, Clone)]
pub struct Note {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub task_id: Option<i64>,
    pub created_at: String,
    pub updated_at: String,
}
