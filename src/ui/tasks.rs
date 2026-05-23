use egui::{RichText, Rounding, Widget};

use crate::models::Task;
use crate::theme;

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

    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        tasks: &mut Vec<Task>,
        modified: &mut bool,
    ) {
        // Header row
        ui.horizontal(|ui: &mut egui::Ui| {
            ui.heading(
                RichText::new("Tasks")
                    .size(16.0)
                    .color(theme::TEXT_PRIMARY),
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui: &mut egui::Ui| {
                ui.label(
                    RichText::new(format!("{}", tasks.len()))
                        .size(12.0)
                        .color(theme::TEXT_SECONDARY),
                );
            });
        });

        ui.add_space(6.0);

        // Add task bar
        let bar_bg = theme::SURFACE;
        let bar_frame = egui::Frame::default()
            .fill(bar_bg)
            .rounding(Rounding::same(8.0))
            .inner_margin(egui::Margin::symmetric(8.0, 4.0));

        let submitted = bar_frame.show(ui, |ui: &mut egui::Ui| {
            ui.horizontal(|ui: &mut egui::Ui| {
                let input = ui.add_sized(
                    [ui.available_width() - 40.0, 28.0],
                    egui::TextEdit::singleline(&mut self.new_task_input)
                        .hint_text("Add a new task...")
                        .text_color(theme::TEXT_PRIMARY),
                );
                let enter = input.lost_focus()
                    && ui.input(|i: &egui::InputState| i.key_pressed(egui::Key::Enter));
                let clicked = theme::accent_button("Add").ui(ui).clicked();
                if (clicked || enter) && !self.new_task_input.trim().is_empty() {
                    self.pending_new_task = Some(self.new_task_input.trim().to_string());
                    self.new_task_input.clear();
                    return true;
                }
                false
            })
            .inner
        });
        if submitted.inner {
            *modified = true;
        }

        ui.add_space(8.0);

        // Task list
        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui: &mut egui::Ui| {
                let mut task_to_delete: Option<usize> = None;
                let mut task_to_toggle: Option<usize> = None;
                let mut task_to_cycle_prio: Option<usize> = None;

                for (i, task) in tasks.iter().enumerate() {
                    let card = theme::card_frame();
                    card.show(ui, |ui: &mut egui::Ui| {
                        ui.horizontal(|ui: &mut egui::Ui| {
                            // Checkbox with accent color
                            let mut checked = task.completed;
                            let cb = ui.checkbox(&mut checked, "");
                            if cb.changed() {
                                task_to_toggle = Some(i);
                            }

                            // Title
                            let title = if task.completed {
                                RichText::new(&task.title)
                                    .strikethrough()
                                    .color(theme::TEXT_MUTED)
                                    .size(13.0)
                            } else {
                                RichText::new(&task.title)
                                    .color(theme::TEXT_PRIMARY)
                                    .size(13.0)
                            };
                            ui.label(title);

                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui: &mut egui::Ui| {
                                    // Priority badge
                                    let prio = task.priority;
                                    let prio_color = theme::priority_color(prio);
                                    let prio_text = theme::priority_label(prio);

                                    let badge_bg = egui::Color32::from_rgba_premultiplied(
                                        prio_color.r(),
                                        prio_color.g(),
                                        prio_color.b(),
                                        30,
                                    );
                                    let badge = egui::Frame::default()
                                        .fill(badge_bg)
                                        .rounding(Rounding::same(4.0))
                                        .inner_margin(egui::Margin::symmetric(6.0, 2.0));

                                    let clickable = badge.show(ui, |ui: &mut egui::Ui| {
                                        ui.selectable_label(
                                            false,
                                            RichText::new(prio_text)
                                                .size(11.0)
                                                .color(prio_color),
                                        )
                                    });
                                    if clickable.inner.clicked() {
                                        task_to_cycle_prio = Some(i);
                                    }

                                    ui.add_space(6.0);

                                    // Delete button — subtle
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
                                        task_to_delete = Some(i);
                                    }
                                },
                            );
                        });
                    });
                }

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
