use egui::{Color32, RichText};

use crate::models::Task;

pub struct TaskPanel {
    pub new_task_input: String,
    pub pending_new_task: Option<String>,
}

impl TaskPanel {
    pub fn new() -> Self {
        Self {
            new_task_input: String::new(),
            pending_new_task: None,
        }
    }

    /// Render the tasks panel. Returns `true` if tasks were modified.
    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        tasks: &mut Vec<Task>,
        modified: &mut bool,
    ) {
        ui.heading("Tasks");
        ui.separator();

        // Add new task input
        let submitted = ui.horizontal(|ui: &mut egui::Ui| {
            let input = ui.text_edit_singleline(&mut self.new_task_input);
            let enter = input.lost_focus() && ui.input(|i: &egui::InputState| i.key_pressed(egui::Key::Enter));
            let clicked = ui.button("+").clicked();
            if (clicked || enter) && !self.new_task_input.trim().is_empty() {
                self.pending_new_task = Some(self.new_task_input.trim().to_string());
                self.new_task_input.clear();
                return true;
            }
            false
        });
        if submitted.inner {
            *modified = true;
        }

        ui.add_space(4.0);

        // Task list
        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui: &mut egui::Ui| {
                let mut task_to_delete: Option<usize> = None;
                let mut task_to_toggle: Option<usize> = None;
                let mut task_to_cycle_prio: Option<usize> = None;

                for (i, task) in tasks.iter().enumerate() {
                    ui.horizontal(|ui: &mut egui::Ui| {
                        // Checkbox
                        let mut checked = task.completed;
                        if ui.checkbox(&mut checked, "").changed() {
                            task_to_toggle = Some(i);
                        }

                        // Priority indicator
                        let prio_color = match task.priority {
                            1 => Color32::from_rgb(100, 200, 100), // green (low)
                            2 => Color32::from_rgb(220, 180, 40),  // yellow (medium)
                            3 => Color32::from_rgb(220, 80, 80),   // red (high)
                            _ => Color32::from_rgb(120, 120, 120), // grey (none)
                        };
                        let prio_label = if task.priority > 0 {
                            RichText::new("●").color(prio_color)
                        } else {
                            RichText::new("○").color(prio_color)
                        };
                        if ui.selectable_label(false, prio_label).clicked() {
                            task_to_cycle_prio = Some(i);
                        }

                        // Title
                        let text = if task.completed {
                            RichText::new(&task.title)
                                .strikethrough()
                                .color(Color32::from_rgb(140, 140, 140))
                        } else {
                            RichText::new(&task.title)
                        };
                        ui.label(text);

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui: &mut egui::Ui| {
                            if ui.button("✕").clicked() {
                                task_to_delete = Some(i);
                            }
                        });
                    });
                }

                // Apply changes after immutable borrow
                if let Some(i) = task_to_toggle {
                    tasks[i].completed = !tasks[i].completed;
                    *modified = true;
                }
                if let Some(i) = task_to_cycle_prio {
                    tasks[i].priority = (tasks[i].priority + 1) % 4;
                    *modified = true;
                }
                if let Some(i) = task_to_delete {
                    tasks.remove(i);
                    *modified = true;
                }
            });
    }
}
