use egui::{CentralPanel, Context, RichText, Sense, TopBottomPanel, Ui};

use crate::db::Database;
use crate::theme;
use crate::ui::notes::NotesPanel;
use crate::ui::tasks::TaskPanel;

const MIN_PANEL: f32 = 240.0;

pub struct TickApp {
    db: Database,
    tasks: Vec<crate::models::Task>,
    notes: Vec<crate::models::Note>,
    task_panel: TaskPanel,
    notes_panel: NotesPanel,
    dirty: bool,
    last_save_time: f64,
    save_interval: f64,
    split_ratio: f32,
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
            split_ratio: 0.38,
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

        //  Top bar 
        TopBottomPanel::top("top_bar").show(ctx, |ui: &mut Ui| {
            ui.horizontal(|ui: &mut Ui| {
                ui.label(RichText::new("Tick").size(18.0).color(theme::TEXT_PRIMARY));
                ui.separator();
                ui.label(RichText::new("Tasks & Notes").size(13.0).color(theme::TEXT_MUTED));
            });
        });

        //  Status bar 
        TopBottomPanel::bottom("status_bar").show(ctx, |ui: &mut Ui| {
            ui.horizontal(|ui: &mut Ui| {
                let done = self.tasks.iter().filter(|t| t.completed).count();
                ui.label(
                    RichText::new(format!(
                        "Tasks {}/{} done  ·  Notes {}",
                        done,
                        self.tasks.len(),
                        self.notes.len()
                    ))
                    .size(12.0)
                    .color(theme::TEXT_MUTED),
                );
                ui.with_layout(
                    egui::Layout::right_to_left(egui::Align::Center),
                    |ui: &mut Ui| {
                        ui.label(
                            RichText::new(self.db.db_path.display().to_string())
                                .size(12.0)
                                .color(theme::TEXT_MUTED),
                        );
                    },
                );
            });
        });

        //  Content: manual draggable split 
        CentralPanel::default().show(ctx, |ui: &mut Ui| {
            let total_w = ui.available_width();
            let grip_w = 8.0;
            let left_w = (total_w * self.split_ratio)
                .max(MIN_PANEL)
                .min(total_w - MIN_PANEL - grip_w);
            let right_w = total_w - left_w - grip_w;

            ui.horizontal(|ui: &mut Ui| {
                //  Tasks (left) 
                ui.vertical(|ui: &mut Ui| {
                    ui.set_width(left_w);
                    ui.set_height(ui.available_height());
                    self.task_panel.show(ui, &mut self.tasks, &mut modified);
                    if let Some(title) = self.task_panel.pending_new_task.take() {
                        if let Ok(task) = self.db.add_task(&title) {
                            self.tasks.push(task);
                        }
                    }
                });

                //  Drag grip 
                let grip_rect = ui.vertical(|ui: &mut Ui| {
                    ui.set_width(grip_w);
                    ui.set_height(ui.available_height());
                })
                .response
                .rect;

                // Paint grip indicator
                let painter = ui.painter().clone();
                let cx = grip_rect.center().x;
                let grip_h = 60.0;
                let grip_y = grip_rect.center().y - grip_h / 2.0;
                let grip_color = theme::BORDER;
                painter.line_segment(
                    [egui::pos2(cx - 1.0, grip_y), egui::pos2(cx - 1.0, grip_y + grip_h)],
                    egui::Stroke::new(1.5, grip_color),
                );
                painter.line_segment(
                    [egui::pos2(cx + 1.0, grip_y), egui::pos2(cx + 1.0, grip_y + grip_h)],
                    egui::Stroke::new(1.5, grip_color),
                );

                let grip_resp = ui.interact(grip_rect, ui.next_auto_id(), Sense::drag());
                if grip_resp.dragged() {
                    let delta = grip_resp.drag_delta();
                    let new_left = left_w + delta.x;
                    let clamped = new_left.max(MIN_PANEL).min(total_w - MIN_PANEL - grip_w);
                    self.split_ratio = clamped / total_w;
                    ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeHorizontal);
                }
                if grip_resp.hovered() || grip_resp.dragged() {
                    ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeHorizontal);
                }

                //  Notes (right) 
                ui.vertical(|ui: &mut Ui| {
                    ui.set_width(right_w);
                    ui.set_height(ui.available_height());
                    let linked = self.task_panel.selected_task_id;
                    self.notes_panel.show(ui, &mut self.notes, &self.tasks, linked, &mut modified);
                    if let Some(title) = self.notes_panel.pending_new_note.take() {
                        let tid = self.notes_panel.pending_note_task.take();
                        if let Ok(note) = self.db.add_note(&title, tid) {
                            self.notes.push(note);
                        }
                    }
                });
            });
        });

        if modified {
            self.notes_panel.apply_edits(&mut self.notes);
            self.dirty = true;
            self.last_save_time = time;
        }
        self.save_if_needed(time);
    }
}
