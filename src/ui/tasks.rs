use egui::{Align2, Color32, RichText, Window};

use crate::models::{Schedule, Task};
use crate::theme;

pub struct TaskPanel {
    pub new_task_input: String,
    pub pending_new_task: Option<String>,
    pub selected_task_id: Option<i64>,
    recent_expanded: bool,
    editing_id: Option<i64>,
    edit_buffer: String,
    delete_confirm_id: Option<i64>,
}

impl TaskPanel {
    pub fn new() -> Self {
        Self {
            new_task_input: String::new(),
            pending_new_task: None,
            selected_task_id: None,
            recent_expanded: false,
            editing_id: None,
            edit_buffer: String::new(),
            delete_confirm_id: None,
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, tasks: &mut Vec<Task>, modified: &mut bool) {
        //  Delete confirmation modal 
        if let Some(del_id) = self.delete_confirm_id {
            let task_title = tasks
                .iter()
                .find(|t| t.id == del_id)
                .map(|t| t.title.clone())
                .unwrap_or_default();

            let mut close = false;
            let mut confirmed = false;

            Window::new("Delete task?")
                .id(egui::Id::new("confirm_delete"))
                .collapsible(false)
                .resizable(false)
                .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
                .fixed_size([260.0, 90.0])
                .show(ui.ctx(), |ui: &mut egui::Ui| {
                    ui.label(format!("Delete \"{}\"?", task_title));
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
                    tasks.retain(|t| t.id != del_id);
                    if self.selected_task_id == Some(del_id) {
                        self.selected_task_id = None;
                    }
                    if self.editing_id == Some(del_id) {
                        self.editing_id = None;
                        self.edit_buffer.clear();
                    }
                    *modified = true;
                }
                self.delete_confirm_id = None;
            }
        }

        ui.heading("Tasks");

        // Add task input
        ui.horizontal(|ui: &mut egui::Ui| {
            ui.add_sized(
                [ui.available_width() - 50.0, 22.0],
                egui::TextEdit::singleline(&mut self.new_task_input)
                    .hint_text("New task..."),
            );
            let enter = ui.input(|i: &egui::InputState| i.key_pressed(egui::Key::Enter));
            if ui.button("Add").clicked() || enter {
                let text = self.new_task_input.trim().to_string();
                if !text.is_empty() {
                    self.pending_new_task = Some(text);
                    self.new_task_input.clear();
                    *modified = true;
                }
            }
        });

        ui.add_space(6.0);

        // Build groups
        let mut groups: Vec<(&str, Vec<i64>)> = Vec::new();
        let schedules = [
            Schedule::Today,
            Schedule::Tomorrow,
            Schedule::Unscheduled,
            Schedule::Later,
        ];

        for sched in &schedules {
            let ids: Vec<i64> = tasks
                .iter()
                .filter(|t| t.schedule == *sched)
                .map(|t| t.id)
                .collect();
            if !ids.is_empty() {
                groups.push((sched.label(), ids));
            }
        }

        if groups.is_empty() {
            ui.add_space(8.0);
            ui.label(RichText::new("No tasks").color(theme::MUTED));
        } else {
            egui::ScrollArea::vertical()
                .id_source("task_list")
                .auto_shrink([false, false])
                .show(ui, |ui: &mut egui::Ui| {
                    for (label, ids) in &groups {
                        Self::render_group(
                            ui,
                            label,
                            ids,
                            tasks,
                            &mut self.selected_task_id,
                            &mut self.editing_id,
                            &mut self.edit_buffer,
                            &mut self.delete_confirm_id,
                            modified,
                        );
                        ui.add_space(4.0);
                    }
                });
        }

        //  Recent 
        let completed_ids: Vec<i64> = tasks
            .iter()
            .filter(|t| t.completed)
            .map(|t| t.id)
            .collect();

        if !completed_ids.is_empty() {
            ui.add_space(6.0);
            ui.separator();

            let header = if self.recent_expanded {
                format!("▾ Recent ({})", completed_ids.len())
            } else {
                format!("▸ Recent ({})", completed_ids.len())
            };

            if ui.selectable_label(false, header).clicked() {
                self.recent_expanded = !self.recent_expanded;
            }

            if self.recent_expanded {
                for id in &completed_ids {
                    Self::render_completed_item(ui, *id, tasks, &mut self.delete_confirm_id, modified);
                }
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn render_group(
        ui: &mut egui::Ui,
        label: &str,
        ids: &[i64],
        tasks: &mut Vec<Task>,
        selected_id: &mut Option<i64>,
        editing_id: &mut Option<i64>,
        edit_buffer: &mut String,
        delete_confirm_id: &mut Option<i64>,
        modified: &mut bool,
    ) {
        ui.label(RichText::new(label).size(12.0).color(theme::MUTED));

        for &id in ids {
            let Some(idx) = tasks.iter().position(|t| t.id == id) else {
                continue;
            };

            let completed = tasks[idx].completed;
            let is_selected = *selected_id == Some(id);
            let is_editing = *editing_id == Some(id);

            ui.horizontal(|ui: &mut egui::Ui| {
                // Checkbox
                let mut checked = completed;
                if ui.checkbox(&mut checked, "").changed() {
                    tasks[idx].completed = !tasks[idx].completed;
                    *modified = true;
                }

                // Title or edit field
                if is_editing {
                    let resp = ui.add_sized(
                        [ui.available_width() - 170.0, 20.0],
                        egui::TextEdit::singleline(edit_buffer),
                    );
                    if resp.lost_focus()
                        || ui.input(|i: &egui::InputState| i.key_pressed(egui::Key::Enter))
                    {
                        let trimmed = edit_buffer.trim().to_string();
                        if !trimmed.is_empty() {
                            tasks[idx].title = trimmed;
                            *modified = true;
                        }
                        *editing_id = None;
                        edit_buffer.clear();
                    }
                    resp.request_focus();
                } else {
                    let title = if completed {
                        RichText::new(&tasks[idx].title).strikethrough().color(theme::MUTED)
                    } else {
                        RichText::new(&tasks[idx].title)
                    };
                    if ui.selectable_label(is_selected, title).clicked() {
                        *selected_id = if *selected_id == Some(id) { None } else { Some(id) };
                    }
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui: &mut egui::Ui| {
                    // Edit
                    if ui.button("✎").clicked() {
                        if is_editing {
                            let trimmed = edit_buffer.trim().to_string();
                            if !trimmed.is_empty() {
                                tasks[idx].title = trimmed;
                                *modified = true;
                            }
                            *editing_id = None;
                            edit_buffer.clear();
                        } else {
                            *editing_id = Some(id);
                            *edit_buffer = tasks[idx].title.clone();
                        }
                    }

                    // Schedule cycle
                    let sched_color = match tasks[idx].schedule {
                        Schedule::Today => theme::GREEN,
                        Schedule::Tomorrow => theme::YELLOW,
                        Schedule::Later => theme::ACCENT,
                        Schedule::Unscheduled => theme::MUTED,
                    };
                    if ui
                        .add_sized(
                            [60.0, 20.0],
                            egui::Button::new(
                                RichText::new(tasks[idx].schedule.label()).size(11.0).color(sched_color),
                            ),
                        )
                        .clicked()
                    {
                        tasks[idx].schedule = match tasks[idx].schedule {
                            Schedule::Unscheduled => Schedule::Today,
                            Schedule::Today => Schedule::Tomorrow,
                            Schedule::Tomorrow => Schedule::Later,
                            Schedule::Later => Schedule::Unscheduled,
                        };
                        *modified = true;
                    }

                    // Priority cycle
                    let prio_color = theme::priority_color(tasks[idx].priority);
                    let prio_text = theme::priority_label(tasks[idx].priority);
                    if ui
                        .add_sized(
                            [52.0, 20.0],
                            egui::Button::new(
                                RichText::new(prio_text).size(11.0).color(prio_color),
                            ),
                        )
                        .clicked()
                    {
                        tasks[idx].priority = (tasks[idx].priority + 1) % 4;
                        *modified = true;
                    }

                    // Delete
                    if ui.button("✕").clicked() {
                        *delete_confirm_id = Some(id);
                    }
                });
            });
        }
    }

    fn render_completed_item(
        ui: &mut egui::Ui,
        id: i64,
        tasks: &mut Vec<Task>,
        delete_confirm_id: &mut Option<i64>,
        modified: &mut bool,
    ) {
        let Some(idx) = tasks.iter().position(|t| t.id == id) else {
            return;
        };
        ui.horizontal(|ui: &mut egui::Ui| {
            let mut checked = true;
            if ui.checkbox(&mut checked, "").changed() {
                tasks[idx].completed = false;
                *modified = true;
            }
            ui.label(
                RichText::new(&tasks[idx].title)
                    .strikethrough()
                    .color(theme::MUTED),
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui: &mut egui::Ui| {
                if ui.button("✕").clicked() {
                    *delete_confirm_id = Some(id);
                }
            });
        });
    }
}
