use egui::{RichText, Rounding, Widget};

use crate::models::Note;
use crate::theme;

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

    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        notes: &mut Vec<Note>,
        modified: &mut bool,
    ) {
        // Header row
        ui.horizontal(|ui: &mut egui::Ui| {
            ui.heading(
                RichText::new("Notes")
                    .size(16.0)
                    .color(theme::TEXT_PRIMARY),
            );
            ui.with_layout(
                egui::Layout::right_to_left(egui::Align::Center),
                |ui: &mut egui::Ui| {
                    ui.label(
                        RichText::new(format!("{}", notes.len()))
                            .size(12.0)
                            .color(theme::TEXT_SECONDARY),
                    );
                },
            );
        });

        ui.add_space(6.0);

        // New note bar
        let bar_frame = egui::Frame::default()
            .fill(theme::SURFACE)
            .rounding(Rounding::same(8.0))
            .inner_margin(egui::Margin::symmetric(8.0, 4.0));

        bar_frame.show(ui, |ui: &mut egui::Ui| {
            ui.horizontal(|ui: &mut egui::Ui| {
                ui.add_sized(
                    [ui.available_width() - 90.0, 28.0],
                    egui::TextEdit::singleline(&mut self.new_note_title)
                        .hint_text("Note title...")
                        .text_color(theme::TEXT_PRIMARY),
                );
                if theme::accent_button("+ New")
                    .ui(ui)
                    .clicked()
                    && !self.new_note_title.trim().is_empty()
                {
                    self.pending_new_note = Some(self.new_note_title.trim().to_string());
                    *modified = true;
                }
            });
        });

        ui.add_space(8.0);

        let available = ui.available_size();
        let list_height = (available.y * 0.4).min(220.0);

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
                        theme::SURFACE_ACTIVE
                    } else {
                        theme::SURFACE
                    };

                    let card = egui::Frame::default()
                        .fill(bg)
                        .rounding(Rounding::same(6.0))
                        .inner_margin(egui::Margin::symmetric(10.0, 6.0))
                        .outer_margin(egui::Margin::symmetric(0.0, 2.0));

                    card.show(ui, |ui: &mut egui::Ui| {
                        ui.horizontal(|ui: &mut egui::Ui| {
                            let sel = ui.selectable_label(
                                is_selected,
                                RichText::new(&note.title)
                                    .size(13.0)
                                    .color(theme::TEXT_PRIMARY),
                            );
                            if sel.clicked() {
                                note_to_select = Some(note.id);
                            }

                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui: &mut egui::Ui| {
                                    let ts = &note.updated_at[..note.updated_at.len().min(16)];
                                    ui.label(
                                        RichText::new(ts)
                                            .size(11.0)
                                            .color(theme::TEXT_MUTED),
                                    );

                                    ui.add_space(8.0);

                                    let del = ui.add_sized(
                                        [22.0, 22.0],
                                        egui::Button::new(
                                            RichText::new("✕")
                                                .size(11.0)
                                                .color(theme::TEXT_MUTED),
                                        )
                                        .fill(egui::Color32::TRANSPARENT)
                                        .rounding(Rounding::same(4.0)),
                                    );
                                    if del.clicked() {
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

        ui.add_space(8.0);

        // Editor
        if self.selected_note_id.is_some() {
            let editor_frame = egui::Frame::default()
                .fill(theme::SURFACE)
                .rounding(Rounding::same(8.0))
                .inner_margin(egui::Margin::symmetric(10.0, 8.0));

            editor_frame.show(ui, |ui: &mut egui::Ui| {
                ui.horizontal(|ui: &mut egui::Ui| {
                    ui.label(
                        RichText::new("Title")
                            .size(12.0)
                            .color(theme::TEXT_SECONDARY),
                    );
                    if ui
                        .add_sized(
                            [ui.available_width(), 24.0],
                            egui::TextEdit::singleline(&mut self.edit_title)
                                .text_color(theme::TEXT_PRIMARY),
                        )
                        .changed()
                    {
                        *modified = true;
                    }
                });

                ui.add_space(6.0);

                let available = ui.available_size();
                let editor_height = (available.y - 10.0).max(100.0);

                egui::ScrollArea::vertical()
                    .max_height(editor_height)
                    .auto_shrink([false, false])
                    .show(ui, |ui: &mut egui::Ui| {
                        let text_edit = egui::TextEdit::multiline(&mut self.edit_content)
                            .desired_width(f32::INFINITY)
                            .desired_rows(8)
                            .hint_text("Start writing...")
                            .text_color(theme::TEXT_PRIMARY);
                        if ui.add(text_edit).changed() {
                            *modified = true;
                        }
                    });
            });
        } else {
            // Empty state
            ui.vertical_centered(|ui: &mut egui::Ui| {
                ui.add_space(40.0);
                ui.label(
                    RichText::new("No note selected")
                        .size(14.0)
                        .color(theme::TEXT_MUTED),
                );
                ui.label(
                    RichText::new("Create a new note or select one from the list")
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
            }
        }
    }
}
