use egui::{CentralPanel, Context, RichText, SidePanel, TopBottomPanel};

use crate::db::Database;
use crate::theme;
use crate::ui::notes::NotesPanel;
use crate::ui::tasks::TaskPanel;

pub struct TickApp {
    db: Database,
    tasks: Vec<crate::models::Task>,
    notes: Vec<crate::models::Note>,
    task_panel: TaskPanel,
    notes_panel: NotesPanel,
    dirty: bool,
    last_save_time: f64,
    save_interval: f64,
}

impl TickApp {
    pub fn new(
        db: Database,
        tasks: Vec<crate::models::Task>,
        notes: Vec<crate::models::Note>,
    ) -> Self {
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

        TopBottomPanel::top("top_bar").show(ctx, |ui: &mut egui::Ui| {
            ui.add_space(2.0);
            ui.horizontal(|ui: &mut egui::Ui| {
                ui.label(RichText::new("Tick").size(17.0).color(theme::ACCENT));
                ui.separator();
                let done = self.tasks.iter().filter(|t| t.completed).count();
                ui.label(
                    RichText::new(format!(
                        "{} tasks · {} done · {} notes",
                        self.tasks.len(),
                        done,
                        self.notes.len()
                    ))
                    .color(theme::MUTED),
                );
            });
            ui.add_space(2.0);
        });

        TopBottomPanel::bottom("status_bar").show(ctx, |ui: &mut egui::Ui| {
            ui.add_space(2.0);
            ui.horizontal(|ui: &mut egui::Ui| {
                ui.label(
                    RichText::new(self.db.db_path.display().to_string())
                        .size(11.0)
                        .color(theme::MUTED),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui: &mut egui::Ui| {
                    ui.label(RichText::new("SQLite").size(11.0).color(theme::MUTED));
                });
            });
            ui.add_space(2.0);
        });

        SidePanel::left("tasks_panel")
            .resizable(true)
            .default_width(360.0)
            .min_width(220.0)
            .max_width(600.0)
            .show(ctx, |ui: &mut egui::Ui| {
                ui.add_space(4.0);
                self.task_panel.show(ui, &mut self.tasks, &mut modified);
                if let Some(title) = self.task_panel.pending_new_task.take() {
                    if let Ok(task) = self.db.add_task(&title) {
                        self.tasks.push(task);
                    }
                }
                ui.add_space(4.0);
            });

        CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {
            ui.add_space(4.0);
            self.notes_panel.show(ui, &mut self.notes, &self.tasks, self.task_panel.selected_task_id, &mut modified);
            if let Some(title) = self.notes_panel.pending_new_note.take() {
                let tid = self.notes_panel.pending_note_task.take();
                if let Ok(note) = self.db.add_note(&title, tid) {
                    self.notes.push(note);
                }
            }
            ui.add_space(4.0);
        });

        if modified {
            self.notes_panel.apply_edits(&mut self.notes);
            self.dirty = true;
            self.last_save_time = time;
        }
        self.save_if_needed(time);
    }
}
