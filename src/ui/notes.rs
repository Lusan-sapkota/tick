use crate::models::Note;

pub struct NotesPanel {
    pub selected_note_id: Option<i64>,
    pub new_note_title: String,
    pub pending_new_note: Option<String>,
    edit_title: String,
    edit_content: String,
}

impl NotesPanel {
    pub fn new() -> Self {
        Self {
            selected_note_id: None,
            new_note_title: "Untitled".to_string(),
            pending_new_note: None,
            edit_title: String::new(),
            edit_content: String::new(),
        }
    }

    /// Sync the editing buffers with the currently selected note.
    pub fn sync_selected(&mut self, notes: &[Note]) {
        if let Some(id) = self.selected_note_id {
            if let Some(note) = notes.iter().find(|n| n.id == id) {
                self.edit_title = note.title.clone();
                self.edit_content = note.content.clone();
            } else {
                self.selected_note_id = None;
                self.edit_title.clear();
                self.edit_content.clear();
            }
        }
    }

    /// Render the notes panel. Returns `true` if notes were modified.
    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        notes: &mut Vec<Note>,
        modified: &mut bool,
    ) {
        ui.heading("Notes");
        ui.separator();

        // New note button
        ui.horizontal(|ui: &mut egui::Ui| {
            ui.label("Title:");
            ui.text_edit_singleline(&mut self.new_note_title);
            if ui.button("+ Note").clicked() && !self.new_note_title.trim().is_empty() {
                self.pending_new_note = Some(self.new_note_title.trim().to_string());
                *modified = true;
            }
        });

        ui.add_space(4.0);

        // Split: note list on top, editor on bottom
        let available = ui.available_size();
        let list_height = (available.y * 0.4).min(200.0);

        // Note list
        egui::ScrollArea::vertical()
            .max_height(list_height)
            .auto_shrink([false, false])
            .show(ui, |ui: &mut egui::Ui| {
                let mut note_to_select: Option<i64> = None;
                let mut note_to_delete: Option<i64> = None;

                for note in notes.iter() {
                    let is_selected = self.selected_note_id == Some(note.id);
                    let bg = if is_selected {
                        Some(egui::Color32::from_rgb(55, 55, 80))
                    } else {
                        None
                    };

                    let frame = egui::Frame::default()
                        .fill(bg.unwrap_or(egui::Color32::TRANSPARENT))
                        .inner_margin(egui::vec2(4.0, 2.0));

                    frame.show(ui, |ui: &mut egui::Ui| {
                        ui.horizontal(|ui: &mut egui::Ui| {
                            let resp = ui.selectable_label(is_selected, &note.title);
                            if resp.clicked() {
                                note_to_select = Some(note.id);
                            }
                            let ts = &note.updated_at[..note.updated_at.len().min(16)];
                            ui.label(
                                egui::RichText::new(ts)
                                    .small()
                                    .color(egui::Color32::from_rgb(140, 140, 140)),
                            );
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui: &mut egui::Ui| {
                                    if ui.button("✕").clicked() {
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
                    }
                    *modified = true;
                }
            });

        ui.add_space(4.0);

        // Editor
        if self.selected_note_id.is_some() {
            ui.separator();
            ui.horizontal(|ui: &mut egui::Ui| {
                ui.label("Title:");
                if ui
                    .text_edit_singleline(&mut self.edit_title)
                    .changed()
                {
                    *modified = true;
                }
            });

            ui.label("Content:");
            let available = ui.available_size();
            let editor_height = (available.y - 20.0).max(100.0);
            egui::ScrollArea::vertical()
                .max_height(editor_height)
                .auto_shrink([false, false])
                .show(ui, |ui: &mut egui::Ui| {
                    let text_edit = egui::TextEdit::multiline(&mut self.edit_content)
                        .desired_width(f32::INFINITY)
                        .desired_rows(6)
                        .hint_text("Start typing your note here...");
                    if ui.add(text_edit).changed() {
                        *modified = true;
                    }
                });
        } else {
            ui.label(
                egui::RichText::new("Select a note to edit")
                    .color(egui::Color32::from_rgb(140, 140, 140)),
            );
        }
    }

    /// Apply edits back into the selected note in the vector.
    pub fn apply_edits(&self, notes: &mut Vec<Note>) {
        if let Some(id) = self.selected_note_id {
            if let Some(note) = notes.iter_mut().find(|n| n.id == id) {
                note.title = self.edit_title.trim().to_string();
                if note.title.is_empty() {
                    note.title = "Untitled".to_string();
                }
                note.content.clone_from(&self.edit_content);
            }
        }
    }
}
