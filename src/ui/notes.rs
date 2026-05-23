use egui::{RichText, Rounding, Widget};

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
            Some(tid) => tasks
                .iter()
                .find(|t| t.id == tid)
                .map(|t| t.title.clone())
                .unwrap_or_else(|| "Deleted task".to_string()),
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
        ui.add_space(2.0);

        // Header
        ui.horizontal(|ui: &mut egui::Ui| {
            ui.heading(RichText::new("Notes").size(17.0).color(theme::TEXT_PRIMARY));

            // Filter toggle
            if linked_task_id.is_some() {
                let filter_text = if self.filter_task_id.is_some() {
                    "Linked only"
                } else {
                    "All notes"
                };
                if ui
                    .selectable_label(
                        self.filter_task_id.is_some(),
                        RichText::new(filter_text).size(12.0).color(theme::TEXT_SECONDARY),
                    )
                    .clicked()
                {
                    self.filter_task_id = if self.filter_task_id.is_some() {
                        None
                    } else {
                        linked_task_id
                    };
                }
            } else {
                self.filter_task_id = None;
            }

            ui.with_layout(
                egui::Layout::right_to_left(egui::Align::Center),
                |ui: &mut egui::Ui| {
                    let shown = if self.filter_task_id.is_some() {
                        notes.iter().filter(|n| n.task_id == self.filter_task_id).count()
                    } else {
                        notes.len()
                    };
                    ui.label(RichText::new(format!("{} total", shown)).size(13.0).color(theme::TEXT_SECONDARY));
                },
            );
        });

        // Show link context
        if let Some(tid) = self.filter_task_id {
            if let Some(task) = tasks.iter().find(|t| t.id == tid) {
                ui.label(
                    RichText::new(format!("Linked to: {}", task.title))
                        .size(12.0)
                        .color(theme::PALETTE.accent),
                );
            }
        }

        ui.add_space(8.0);

        // New note input
        theme::input_frame().show(ui, |ui: &mut egui::Ui| {
            ui.horizontal(|ui: &mut egui::Ui| {
                let btn_w = if linked_task_id.is_some() { 68.0 } else { 72.0 };
                ui.add_sized(
                    [ui.available_width() - btn_w, 26.0],
                    egui::TextEdit::singleline(&mut self.new_note_title)
                        .hint_text("Note title...")
                        .text_color(theme::TEXT_PRIMARY)
                        .font(egui::TextStyle::Body),
                );
                let new_btn = egui::Button::new(RichText::new("+ New").size(14.0).color(egui::Color32::WHITE))
                    .fill(theme::PALETTE.accent)
                    .rounding(Rounding::same(5.0))
                    .min_size(egui::vec2(60.0, 26.0));
                if new_btn.ui(ui).clicked() {
                    let text = self.new_note_title.trim().to_string();
                    if !text.is_empty() {
                        self.pending_new_note = Some(text);
                        self.pending_note_task = self.filter_task_id;
                        *modified = true;
                    }
                }
            });
        });

        ui.add_space(8.0);

        let available = ui.available_size();
        let list_height = (available.y * 0.35).min(220.0).max(80.0);

        // Filtered note list
        let visible: Vec<i64> = notes
            .iter()
            .filter(|n| {
                if let Some(ft) = self.filter_task_id {
                    n.task_id == Some(ft)
                } else {
                    true
                }
            })
            .map(|n| n.id)
            .collect();

        if visible.is_empty() {
            ui.add_space(16.0);
            ui.vertical_centered(|ui: &mut egui::Ui| {
                let msg = if self.filter_task_id.is_some() {
                    "No notes linked to this task"
                } else {
                    "No notes yet"
                };
                ui.label(RichText::new(msg).size(14.0).color(theme::TEXT_MUTED));
                ui.label(
                    RichText::new("Enter a title above and click + New")
                        .size(12.0)
                        .color(theme::TEXT_MUTED),
                );
            });
        } else {
            egui::ScrollArea::vertical()
                .id_source("note_list")
                .max_height(list_height)
                .auto_shrink([false, false])
                .show(ui, |ui: &mut egui::Ui| {
                    let mut note_to_select: Option<i64> = None;
                    let mut note_to_delete: Option<i64> = None;

                    for note in notes.iter().filter(|n| visible.contains(&n.id)) {
                        let is_selected = self.selected_note_id == Some(note.id);
                        let bg = if is_selected { theme::SURFACE_ACTIVE } else { theme::SURFACE };

                        let card = egui::Frame::default()
                            .fill(bg)
                            .stroke(egui::Stroke::new(1.0, theme::BORDER))
                            .rounding(Rounding::same(6.0))
                            .inner_margin(egui::Margin::symmetric(10.0, 5.0))
                            .outer_margin(egui::Margin::symmetric(0.0, 2.0));

                        card.show(ui, |ui: &mut egui::Ui| {
                            ui.horizontal(|ui: &mut egui::Ui| {
                                if ui
                                    .selectable_label(
                                        is_selected,
                                        RichText::new(&note.title).size(14.0).color(theme::TEXT_PRIMARY),
                                    )
                                    .clicked()
                                {
                                    note_to_select = Some(note.id);
                                }
                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::Center),
                                    |ui: &mut egui::Ui| {
                                        // Task link badge
                                        if let Some(tid) = note.task_id {
                                            if let Some(task) = tasks.iter().find(|t| t.id == tid) {
                                                ui.label(
                                                    RichText::new(format!("→ {}", task.title))
                                                        .size(10.0)
                                                        .color(theme::PALETTE.accent),
                                                );
                                                ui.add_space(6.0);
                                            }
                                        }

                                        let ts = &note.updated_at[..note.updated_at.len().min(16)];
                                        ui.label(RichText::new(ts).size(11.0).color(theme::TEXT_MUTED));
                                        ui.add_space(6.0);
                                        if ui
                                            .add_sized(
                                                [20.0, 20.0],
                                                egui::Button::new(
                                                    RichText::new("✕").size(11.0).color(theme::TEXT_MUTED),
                                                )
                                                .fill(egui::Color32::TRANSPARENT),
                                            )
                                            .clicked()
                                        {
                                            note_to_delete = Some(note.id);
                                        }
                                    },
                                );
                            });
                        });
                    }

                    if let Some(id) = note_to_select {
                        self.selected_note_id = Some(id);
                    }
                    if let Some(id) = note_to_delete {
                        notes.retain(|n| n.id != id);
                        if self.selected_note_id == Some(id) {
                            self.selected_note_id = None;
                            self.edit_title.clear();
                            self.edit_content.clear();
                            self.edit_task_id = None;
                        }
                        *modified = true;
                    }
                });
        }

        ui.add_space(6.0);

        // Editor
        if self.selected_note_id.is_some() {
            theme::input_frame().show(ui, |ui: &mut egui::Ui| {
                // Title row
                ui.horizontal(|ui: &mut egui::Ui| {
                    ui.label(RichText::new("Title").size(12.0).color(theme::TEXT_SECONDARY));
                    if ui
                        .add_sized(
                            [ui.available_width() - 100.0, 24.0],
                            egui::TextEdit::singleline(&mut self.edit_title)
                                .text_color(theme::TEXT_PRIMARY)
                                .font(egui::TextStyle::Body),
                        )
                        .changed()
                    {
                        *modified = true;
                    }

                    // Link to task dropdown
                    let current_link = self.task_name_for(self.edit_task_id, tasks);
                    egui::ComboBox::from_id_source("link_task")
                        .selected_text(RichText::new(&current_link).size(12.0).color(theme::TEXT_SECONDARY))
                        .width(90.0)
                        .show_ui(ui, |ui: &mut egui::Ui| {
                            if ui
                                .selectable_label(self.edit_task_id.is_none(), "None")
                                .clicked()
                            {
                                self.edit_task_id = None;
                                *modified = true;
                            }
                            if let Some(tid) = linked_task_id {
                                if let Some(task) = tasks.iter().find(|t| t.id == tid) {
                                    if ui
                                        .selectable_label(
                                            self.edit_task_id == Some(tid),
                                            RichText::new(&task.title).size(12.0),
                                        )
                                        .clicked()
                                    {
                                        self.edit_task_id = Some(tid);
                                        *modified = true;
                                    }
                                }
                            }
                            for task in tasks.iter().filter(|t| Some(t.id) != linked_task_id).take(15) {
                                if ui
                                    .selectable_label(
                                        self.edit_task_id == Some(task.id),
                                        RichText::new(&task.title).size(12.0),
                                    )
                                    .clicked()
                                {
                                    self.edit_task_id = Some(task.id);
                                    *modified = true;
                                }
                            }
                        });
                });

                ui.add_space(4.0);

                // Content
                let h = ui.available_height().max(140.0);
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
                                    .hint_text("Start writing...")
                                    .text_color(theme::TEXT_PRIMARY)
                                    .font(egui::TextStyle::Body),
                            )
                            .changed()
                        {
                            *modified = true;
                        }
                    });
            });
        } else {
            ui.add_space(40.0);
            ui.vertical_centered(|ui: &mut egui::Ui| {
                ui.label(RichText::new("No note selected").size(14.0).color(theme::TEXT_MUTED));
                ui.label(
                    RichText::new("Create a new note or click one from the list above")
                        .size(12.0)
                        .color(theme::TEXT_MUTED),
                );
            });
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
