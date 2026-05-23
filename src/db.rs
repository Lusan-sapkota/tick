use rusqlite::{Connection, Result, params};
use std::path::PathBuf;

use crate::models::{Note, Task};

pub struct Database {
    conn: Connection,
    pub db_path: PathBuf,
}

impl Database {
    pub fn open() -> Result<Self> {
        let data_dir = dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("tick");
        std::fs::create_dir_all(&data_dir).ok();

        let db_path = data_dir.join("tick.db");
        let conn = Connection::open(&db_path)?;

        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS tasks (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                completed INTEGER NOT NULL DEFAULT 0,
                priority INTEGER NOT NULL DEFAULT 0,
                position INTEGER NOT NULL DEFAULT 0,
                created_at TEXT NOT NULL
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS notes (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL DEFAULT 'Untitled',
                content TEXT NOT NULL DEFAULT '',
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )",
            [],
        )?;

        Ok(Self { conn, db_path })
    }

    // Tasks

    pub fn load_tasks(&self) -> Result<Vec<Task>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, completed, priority, position, created_at FROM tasks ORDER BY position, id"
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(Task {
                id: row.get(0)?,
                title: row.get(1)?,
                completed: row.get::<_, i32>(2)? != 0,
                priority: row.get(3)?,
                position: row.get(4)?,
                created_at: row.get(5)?,
            })
        })?;
        rows.collect()
    }

    pub fn add_task(&self, title: &str) -> Result<Task> {
        let now = chrono::Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();
        let max_pos: i32 = self
            .conn
            .query_row("SELECT COALESCE(MAX(position), -1) FROM tasks", [], |r| {
                r.get(0)
            })?;
        self.conn.execute(
            "INSERT INTO tasks (title, completed, priority, position, created_at) VALUES (?1, 0, 0, ?2, ?3)",
            params![title, max_pos + 1, now],
        )?;
        let id = self.conn.last_insert_rowid();
        Ok(Task {
            id,
            title: title.to_string(),
            completed: false,
            priority: 0,
            position: max_pos + 1,
            created_at: now,
        })
    }

    #[allow(dead_code)]
    pub fn update_task(&self, task: &Task) -> Result<()> {
        self.conn.execute(
            "UPDATE tasks SET title=?1, completed=?2, priority=?3, position=?4 WHERE id=?5",
            params![task.title, task.completed as i32, task.priority, task.position, task.id],
        )?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn delete_task(&self, id: i64) -> Result<()> {
        self.conn.execute("DELETE FROM tasks WHERE id=?1", params![id])?;
        Ok(())
    }

    // Notes

    pub fn load_notes(&self) -> Result<Vec<Note>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, content, created_at, updated_at FROM notes ORDER BY updated_at DESC"
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(Note {
                id: row.get(0)?,
                title: row.get(1)?,
                content: row.get(2)?,
                created_at: row.get(3)?,
                updated_at: row.get(4)?,
            })
        })?;
        rows.collect()
    }

    pub fn add_note(&self, title: &str) -> Result<Note> {
        let now = chrono::Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();
        self.conn.execute(
            "INSERT INTO notes (title, content, created_at, updated_at) VALUES (?1, '', ?2, ?2)",
            params![title, now],
        )?;
        let id = self.conn.last_insert_rowid();
        Ok(Note {
            id,
            title: title.to_string(),
            content: String::new(),
            created_at: now.clone(),
            updated_at: now,
        })
    }

    #[allow(dead_code)]
    pub fn update_note(&self, note: &Note) -> Result<()> {
        let now = chrono::Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();
        self.conn.execute(
            "UPDATE notes SET title=?1, content=?2, updated_at=?3 WHERE id=?4",
            params![note.title, note.content, now, note.id],
        )?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn delete_note(&self, id: i64) -> Result<()> {
        self.conn.execute("DELETE FROM notes WHERE id=?1", params![id])?;
        Ok(())
    }

    // Bulk-save all tasks and notes in a single transaction.
    pub fn sync_all(&self, tasks: &[Task], notes: &[Note]) -> Result<()> {
        let now = chrono::Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();
        self.conn.execute_batch("BEGIN")?;

        self.conn.execute("DELETE FROM tasks", [])?;
        for task in tasks {
            self.conn.execute(
                "INSERT INTO tasks (id, title, completed, priority, position, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![task.id, task.title, task.completed as i32, task.priority, task.position, task.created_at],
            )?;
        }

        self.conn.execute("DELETE FROM notes", [])?;
        for note in notes {
            self.conn.execute(
                "INSERT INTO notes (id, title, content, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5)",
                params![note.id, note.title, note.content, note.created_at, now],
            )?;
        }

        self.conn.execute_batch("COMMIT")?;
        Ok(())
    }
}
