use egui::{Align2, Color32, RichText, Window};

use crate::models::{Note, Task};
use crate::theme;

pub struct NotesPanel {
    pub selected_note_id: Option<i64>,
    pub new_note_title: String,
    pub pending_new_note: Option<String>,
    pub pending_note_task: Option<i64>,
    pub filter_task_id: Option<i64>,
    edit_title: String,
    edit_content: String,
    edit_task_id: Option<i64>,
    delete_confirm_id: Option<i64>,
}

impl NotesPanel {
    pub fn new() -> Self {
        Self {
            selected_note_id: None,
            new_note_title: "Untitled".to_string(),
            pending_new_note: None,
            pending_note_task: None,
            filter_task_id: None,
            edit_title: String::new(),
            edit_content: String::new(),
            edit_task_id: None,
            delete_confirm_id: None,
        }
    }

    pub fn sync_selected(&mut self, notes: &[Note]) {
        if let Some(id) = self.selected_note_id {
            if let Some(note) = notes.iter().find(|n| n.id == id) {
                self.edit_title = note.title.clone();
                self.edit_content = note.content.clone();
                self.edit_task_id = note.task_id;
            } else {
                self.selected_note_id = None;
                self.edit_title.clear();
                self.edit_content.clear();
                self.edit_task_id = None;
            }
        }
    }

    fn task_name_for(&self, task_id: Option<i64>, tasks: &[Task]) -> String {
        match task_id {
            Some(tid) => tasks.iter().find(|t| t.id == tid).map(|t| t.title.clone()).unwrap_or_default(),
            None => "None".to_string(),
        }
    }

    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        notes: &mut Vec<Note>,
        tasks: &[Task],
        linked_task_id: Option<i64>,
        modified: &mut bool,
    ) {
        //  Delete confirmation modal 
        if let Some(del_id) = self.delete_confirm_id {
            let note_title = notes
                .iter()
                .find(|n| n.id == del_id)
                .map(|n| n.title.clone())
                .unwrap_or_default();

            let mut close = false;
            let mut confirmed = false;

            Window::new("Delete note?")
                .id(egui::Id::new("confirm_delete_note"))
                .collapsible(false)
                .resizable(false)
                .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
                .fixed_size([260.0, 90.0])
                .show(ui.ctx(), |ui: &mut egui::Ui| {
                    ui.label(format!("Delete \"{}\"?", note_title));
                    ui.add_space(6.0);
                    ui.horizontal(|ui: &mut egui::Ui| {
                        if ui.button("Cancel").clicked() {
                            close = true;
                        }
                        if ui
                            .add_sized(
                                [70.0, 24.0],
                                egui::Button::new(RichText::new("Delete").color(Color32::WHITE))
                                    .fill(theme::RED),
                            )
                            .clicked()
                        {
                            confirmed = true;
                            close = true;
                        }
                    });
                });

            if close {
                if confirmed {
                    notes.retain(|n| n.id != del_id);
                    if self.selected_note_id == Some(del_id) {
                        self.selected_note_id = None;
                        self.edit_title.clear();
                        self.edit_content.clear();
                        self.edit_task_id = None;
                    }
                    *modified = true;
                }
                self.delete_confirm_id = None;
            }
        }

        // Header with filter
        ui.horizontal(|ui: &mut egui::Ui| {
            ui.heading("Notes");
            if linked_task_id.is_some() {
                let label = if self.filter_task_id.is_some() { "Linked only" } else { "All" };
                if ui.selectable_label(self.filter_task_id.is_some(), label).clicked() {
                    self.filter_task_id = if self.filter_task_id.is_some() { None } else { linked_task_id };
                }
            } else {
                self.filter_task_id = None;
            }
        });

        // Task link context
        if let Some(tid) = self.filter_task_id {
            if let Some(task) = tasks.iter().find(|t| t.id == tid) {
                ui.label(RichText::new(format!("Linked to: {}", task.title)).color(theme::ACCENT));
            }
        }

        // New note input
        ui.horizontal(|ui: &mut egui::Ui| {
            ui.add_sized(
                [ui.available_width() - 60.0, 22.0],
                egui::TextEdit::singleline(&mut self.new_note_title)
                    .hint_text("Note title..."),
            );
            if ui.button("+ New").clicked() {
                let text = self.new_note_title.trim().to_string();
                if !text.is_empty() {
                    self.pending_new_note = Some(text);
                    self.pending_note_task = self.filter_task_id;
                    *modified = true;
                }
            }
        });

        ui.add_space(6.0);

        let available = ui.available_size();
        let list_height = (available.y * 0.35).min(200.0).max(60.0);

        let visible: Vec<i64> = notes
            .iter()
            .filter(|n| {
                if let Some(ft) = self.filter_task_id { n.task_id == Some(ft) } else { true }
            })
            .map(|n| n.id)
            .collect();

        if visible.is_empty() {
            ui.label(RichText::new("No notes").color(theme::MUTED));
        } else {
            egui::ScrollArea::vertical()
                .id_source("note_list")
                .max_height(list_height)
                .auto_shrink([false, false])
                .show(ui, |ui: &mut egui::Ui| {
                    let mut note_to_select: Option<i64> = None;

                    for note in notes.iter().filter(|n| visible.contains(&n.id)) {
                        let is_selected = self.selected_note_id == Some(note.id);

                        ui.horizontal(|ui: &mut egui::Ui| {
                            if ui.selectable_label(is_selected, &note.title).clicked() {
                                note_to_select = Some(note.id);
                            }
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui: &mut egui::Ui| {
                                    if let Some(tid) = note.task_id {
                                        if let Some(task) = tasks.iter().find(|t| t.id == tid) {
                                            ui.label(
                                                RichText::new(format!("-> {}", task.title))
                                                    .size(11.0)
                                                    .color(theme::ACCENT),
                                            );
                                        }
                                    }
                                    let ts = &note.updated_at[..note.updated_at.len().min(16)];
                                    ui.label(RichText::new(ts).size(11.0).color(theme::MUTED));
                                    if ui.button("D").clicked() {
                                        self.delete_confirm_id = Some(note.id);
                                    }
                                },
                            );
                        });
                    }

                    if let Some(id) = note_to_select {
                        self.selected_note_id = Some(id);
                    }
                });
        }

        ui.add_space(4.0);

        // Editor
        if self.selected_note_id.is_some() {
            ui.horizontal(|ui: &mut egui::Ui| {
                ui.label("Title:");
                if ui
                    .add_sized(
                        [ui.available_width() - 100.0, 20.0],
                        egui::TextEdit::singleline(&mut self.edit_title),
                    )
                    .changed()
                {
                    *modified = true;
                }

                let current_link = self.task_name_for(self.edit_task_id, tasks);
                egui::ComboBox::from_id_source("link_task")
                    .selected_text(&current_link)
                    .width(90.0)
                    .show_ui(ui, |ui: &mut egui::Ui| {
                        if ui.selectable_label(self.edit_task_id.is_none(), "None").clicked() {
                            self.edit_task_id = None;
                            *modified = true;
                        }
                        if let Some(tid) = linked_task_id {
                            if let Some(task) = tasks.iter().find(|t| t.id == tid) {
                                if ui.selectable_label(self.edit_task_id == Some(tid), &task.title).clicked() {
                                    self.edit_task_id = Some(tid);
                                    *modified = true;
                                }
                            }
                        }
                        for task in tasks.iter().filter(|t| Some(t.id) != linked_task_id).take(15) {
                            if ui.selectable_label(self.edit_task_id == Some(task.id), &task.title).clicked() {
                                self.edit_task_id = Some(task.id);
                                *modified = true;
                            }
                        }
                    });
            });

            ui.add_space(2.0);

            let h = ui.available_height().max(120.0);
            egui::ScrollArea::vertical()
                .id_source("note_editor")
                .max_height(h)
                .auto_shrink([false, false])
                .show(ui, |ui: &mut egui::Ui| {
                    if ui
                        .add(
                            egui::TextEdit::multiline(&mut self.edit_content)
                                .desired_width(f32::INFINITY)
                                .desired_rows(6)
                                .hint_text("Start writing..."),
                        )
                        .changed()
                    {
                        *modified = true;
                    }
                });
        } else {
            ui.label(RichText::new("Select a note to edit").color(theme::MUTED));
        }
    }

    pub fn apply_edits(&self, notes: &mut Vec<Note>) {
        if let Some(id) = self.selected_note_id {
            if let Some(note) = notes.iter_mut().find(|n| n.id == id) {
                note.title = self.edit_title.trim().to_string();
                if note.title.is_empty() {
                    note.title = "Untitled".to_string();
                }
                note.content.clone_from(&self.edit_content);
                note.task_id = self.edit_task_id;
            }
        }
    }
}
