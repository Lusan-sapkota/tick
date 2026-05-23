use egui::{Context, TopBottomPanel, SidePanel, CentralPanel};

use crate::db::Database;
use crate::models::{Note, Task};
use crate::ui::notes::NotesPanel;
use crate::ui::tasks::TaskPanel;

pub struct TickApp {
    db: Database,
    tasks: Vec<Task>,
    notes: Vec<Note>,
    task_panel: TaskPanel,
    notes_panel: NotesPanel,
    dirty: bool,
    last_save_time: f64,
    save_interval: f64,
}

impl TickApp {
    pub fn new(db: Database, tasks: Vec<Task>, notes: Vec<Note>) -> Self {
        let mut notes_panel = NotesPanel::new();
        notes_panel.sync_selected(&notes);

        Self {
            db,
            tasks,
            notes,
            task_panel: TaskPanel::new(),
            notes_panel,
            dirty: false,
            last_save_time: 0.0,
            save_interval: 0.5,
        }
    }

    fn save_if_needed(&mut self, time: f64) {
        if self.dirty && (time - self.last_save_time) > self.save_interval {
            self.last_save_time = time;
            self.dirty = false;
            if let Err(e) = self.db.sync_all(&self.tasks, &self.notes) {
                eprintln!("Failed to save: {e}");
            }
        }
    }
}

impl eframe::App for TickApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        let time = ctx.input(|i| i.time);
        let mut modified = false;

        // Top bar
        TopBottomPanel::top("top_bar").show(ctx, |ui: &mut egui::Ui| {
            ui.horizontal(|ui: &mut egui::Ui| {
                ui.heading("Tick");
                ui.separator();
            });
        });

        // Status bar
        TopBottomPanel::bottom("status_bar").show(ctx, |ui: &mut egui::Ui| {
            ui.horizontal(|ui: &mut egui::Ui| {
                let tasks_done = self.tasks.iter().filter(|t| t.completed).count();
                ui.label(format!(
                    "Tasks: {}/{} done | Notes: {} | {}",
                    tasks_done,
                    self.tasks.len(),
                    self.notes.len(),
                    self.db.db_path.display(),
                ));
            });
        });

        // Left panel: Tasks
        SidePanel::left("tasks_panel")
            .resizable(true)
            .default_width(320.0)
            .min_width(200.0)
            .show(ctx, |ui: &mut egui::Ui| {
                self.task_panel.show(ui, &mut self.tasks, &mut modified);

                // Handle pending new task
                if let Some(title) = self.task_panel.pending_new_task.take() {
                    if let Ok(task) = self.db.add_task(&title) {
                        self.tasks.push(task);
                    }
                }
            });

        // Central panel: Notes
        CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {
            self.notes_panel.show(ui, &mut self.notes, &mut modified);

            // Handle pending new note
            if let Some(title) = self.notes_panel.pending_new_note.take() {
                if let Ok(note) = self.db.add_note(&title) {
                    self.notes.push(note);
                }
            }
        });

        // Apply note edits to the notes vec
        if modified {
            self.notes_panel.apply_edits(&mut self.notes);
            self.dirty = true;
            self.last_save_time = time;
        }

        self.save_if_needed(time);
    }
}
