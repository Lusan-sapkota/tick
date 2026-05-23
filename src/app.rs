use egui::{CentralPanel, Context, RichText, TopBottomPanel};

use crate::db::Database;
use crate::models::{Note, Task};
use crate::theme;
use crate::ui::notes::NotesPanel;
use crate::ui::tasks::TaskPanel;

const MIN_PANEL: f32 = 220.0;

pub struct TickApp {
    db: Database,
    tasks: Vec<Task>,
    notes: Vec<Note>,
    task_panel: TaskPanel,
    notes_panel: NotesPanel,
    dirty: bool,
    last_save_time: f64,
    save_interval: f64,
    split_ratio: f32,
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

        // ── Top bar ──
        TopBottomPanel::top("top_bar").show(ctx, |ui: &mut egui::Ui| {
            let bar = egui::Frame::default()
                .fill(theme::BACKGROUND)
                .inner_margin(egui::Margin::symmetric(14.0, 6.0));

            bar.show(ui, |ui: &mut egui::Ui| {
                ui.horizontal(|ui: &mut egui::Ui| {
                    ui.label(
                        RichText::new("Tick")
                            .size(18.0)
                            .color(theme::TEXT_PRIMARY)
                            .strong(),
                    );
                    ui.label(
                        RichText::new("Tasks & Notes")
                            .size(12.0)
                            .color(theme::TEXT_MUTED),
                    );
                });
            });
        });

        // ── Status bar ──
        TopBottomPanel::bottom("status_bar").show(ctx, |ui: &mut egui::Ui| {
            let bar = egui::Frame::default()
                .fill(theme::BACKGROUND)
                .inner_margin(egui::Margin::symmetric(14.0, 4.0));

            bar.show(ui, |ui: &mut egui::Ui| {
                ui.horizontal(|ui: &mut egui::Ui| {
                    let tasks_done = self.tasks.iter().filter(|t| t.completed).count();
                    let tasks_total = self.tasks.len();
                    ui.label(
                        RichText::new(format!("Tasks {}/{}", tasks_done, tasks_total))
                            .size(11.0)
                            .color(theme::TEXT_MUTED),
                    );

                    ui.separator();

                    ui.label(
                        RichText::new(format!("Notes {}", self.notes.len()))
                            .size(11.0)
                            .color(theme::TEXT_MUTED),
                    );

                    ui.with_layout(
                        egui::Layout::right_to_left(egui::Align::Center),
                        |ui: &mut egui::Ui| {
                            ui.label(
                                RichText::new(self.db.db_path.display().to_string())
                                    .size(11.0)
                                    .color(theme::TEXT_MUTED),
                            );
                        },
                    );
                });
            });
        });

        // ── Content: manual horizontal split ──
        CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {
            let frame = egui::Frame::default()
                .fill(theme::BACKGROUND)
                .inner_margin(egui::Margin::symmetric(0.0, 0.0));

            frame.show(ui, |ui: &mut egui::Ui| {
                let total_w = ui.available_width();
                let grip_w = 6.0;
                let left_w = (total_w * self.split_ratio)
                    .max(MIN_PANEL)
                    .min(total_w - MIN_PANEL - grip_w);
                let right_w = total_w - left_w - grip_w;

                ui.horizontal(|ui: &mut egui::Ui| {
                    // ── Tasks panel ──
                    let task_frame = egui::Frame::default()
                        .fill(theme::BACKGROUND)
                        .inner_margin(egui::Margin::symmetric(12.0, 10.0));

                    ui.allocate_ui(
                        egui::Vec2::new(left_w, ui.available_height()),
                        |ui| {
                            task_frame.show(ui, |ui: &mut egui::Ui| {
                                self.task_panel.show(ui, &mut self.tasks, &mut modified);

                                if let Some(title) = self.task_panel.pending_new_task.take() {
                                    if let Ok(task) = self.db.add_task(&title) {
                                        self.tasks.push(task);
                                    }
                                }
                            });
                        },
                    );

                    // ── Drag handle ──
                    let grip_resp = ui.allocate_ui(
                        egui::Vec2::new(grip_w, ui.available_height()),
                        |ui: &mut egui::Ui| {
                            ui.add_space(ui.available_height() * 0.5);
                        },
                    );
                    let grip_rect = grip_resp.response.rect;
                    let painter = ui.painter().clone();
                    let cx = grip_rect.center().x;
                    painter.line_segment(
                        [
                            egui::pos2(cx, grip_rect.top() + 20.0),
                            egui::pos2(cx, grip_rect.bottom() - 20.0),
                        ],
                        egui::Stroke::new(1.5, theme::SURFACE_HOVER),
                    );
                    // Drag interaction
                    let resp = ui.interact(grip_rect, ui.next_auto_id(), egui::Sense::drag());
                    if resp.dragged() {
                        let delta = resp.drag_delta();
                        let new_left = left_w + delta.x;
                        let clamped = new_left.max(MIN_PANEL).min(total_w - MIN_PANEL - grip_w);
                        self.split_ratio = clamped / total_w;
                        ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeHorizontal);
                    }
                    if resp.hovered() || resp.dragged() {
                        ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeHorizontal);
                    }

                    // ── Notes panel ──
                    let note_frame = egui::Frame::default()
                        .fill(theme::BACKGROUND)
                        .inner_margin(egui::Margin::symmetric(12.0, 10.0));

                    ui.allocate_ui(
                        egui::Vec2::new(right_w, ui.available_height()),
                        |ui| {
                            note_frame.show(ui, |ui: &mut egui::Ui| {
                                self.notes_panel.show(ui, &mut self.notes, &mut modified);

                                if let Some(title) = self.notes_panel.pending_new_note.take() {
                                    if let Ok(note) = self.db.add_note(&title) {
                                        self.notes.push(note);
                                    }
                                }
                            });
                        },
                    );
                });
            });
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
