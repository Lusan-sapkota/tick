use egui::{Align2, Color32, RichText, Rounding, Widget, Window};

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
        ui.add_space(2.0);

        //  Delete confirmation modal 
        if let Some(del_id) = self.delete_confirm_id {
            let task_title = tasks
                .iter()
                .find(|t| t.id == del_id)
                .map(|t| t.title.clone())
                .unwrap_or_default();

            let modal = Window::new("Delete task?")
                .id(egui::Id::new("confirm_delete"))
                .collapsible(false)
                .resizable(false)
                .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
                .fixed_size([280.0, 110.0]);

            let mut close = false;
            let mut confirmed = false;

            modal.show(ui.ctx(), |ui: &mut egui::Ui| {
                ui.label(format!("Delete \"{}\"?", task_title));
                ui.label(
                    RichText::new("This action cannot be undone.")
                        .size(12.0)
                        .color(theme::TEXT_MUTED),
                );
                ui.add_space(8.0);
                ui.horizontal(|ui: &mut egui::Ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui: &mut egui::Ui| {
                        if ui
                            .add_sized(
                                [70.0, 28.0],
                                egui::Button::new(RichText::new("Cancel").size(13.0)),
                            )
                            .clicked()
                        {
                            close = true;
                        }
                        if ui
                            .add_sized(
                                [70.0, 28.0],
                                egui::Button::new(
                                    RichText::new("Delete").size(13.0).color(Color32::WHITE),
                                )
                                .fill(theme::PALETTE.red)
                                .rounding(Rounding::same(5.0)),
                            )
                            .clicked()
                        {
                            confirmed = true;
                            close = true;
                        }
                    });
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

        // Header
        ui.horizontal(|ui: &mut egui::Ui| {
            ui.heading(RichText::new("Tasks").size(17.0).color(theme::TEXT_PRIMARY));
            ui.with_layout(
                egui::Layout::right_to_left(egui::Align::Center),
                |ui: &mut egui::Ui| {
                    let active = tasks.iter().filter(|t| !t.completed).count();
                    let done = tasks.iter().filter(|t| t.completed).count();
                    ui.label(
                        RichText::new(format!("{} active · {} done", active, done))
                            .size(13.0)
                            .color(theme::TEXT_SECONDARY),
                    );
                },
            );
        });

        ui.add_space(8.0);

        // Add task input
        theme::input_frame().show(ui, |ui: &mut egui::Ui| {
            ui.horizontal(|ui: &mut egui::Ui| {
                ui.add_sized(
                    [ui.available_width() - 52.0, 26.0],
                    egui::TextEdit::singleline(&mut self.new_task_input)
                        .hint_text("New task...")
                        .text_color(theme::TEXT_PRIMARY)
                        .font(egui::TextStyle::Body),
                );
                let enter = ui.input(|i: &egui::InputState| i.key_pressed(egui::Key::Enter));
                let add = egui::Button::new(RichText::new("Add").size(14.0).color(Color32::WHITE))
                    .fill(theme::PALETTE.accent)
                    .rounding(Rounding::same(5.0))
                    .min_size(egui::vec2(46.0, 26.0));
                if add.ui(ui).clicked() || enter {
                    let text = self.new_task_input.trim().to_string();
                    if !text.is_empty() {
                        self.pending_new_task = Some(text);
                        self.new_task_input.clear();
                        *modified = true;
                    }
                }
            });
        });

        ui.add_space(10.0);

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
            ui.add_space(12.0);
            ui.vertical_centered(|ui: &mut egui::Ui| {
                ui.label(RichText::new("No tasks").size(14.0).color(theme::TEXT_MUTED));
                ui.label(
                    RichText::new("Type a task above and press Add")
                        .size(12.0)
                        .color(theme::TEXT_MUTED),
                );
            });
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
                        ui.add_space(6.0);
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
                RichText::new(format!("▾ Recent ({})", completed_ids.len()))
                    .size(12.0)
                    .color(theme::TEXT_SECONDARY)
            } else {
                RichText::new(format!("▸ Recent ({})", completed_ids.len()))
                    .size(12.0)
                    .color(theme::TEXT_MUTED)
            };

            if ui.selectable_label(false, header).clicked() {
                self.recent_expanded = !self.recent_expanded;
            }

            if self.recent_expanded {
                ui.add_space(4.0);
                for id in &completed_ids {
                    Self::render_completed_item(
                        ui,
                        *id,
                        tasks,
                        &mut self.selected_task_id,
                        &mut self.delete_confirm_id,
                        modified,
                    );
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
        ui.label(theme::section_header(label));
        ui.add_space(2.0);

        for &id in ids {
            let Some(idx) = tasks.iter().position(|t| t.id == id) else {
                continue;
            };

            let completed = tasks[idx].completed;
            let is_selected = *selected_id == Some(id);
            let is_editing = *editing_id == Some(id);

            let card_bg = if is_selected {
                theme::SURFACE_ACTIVE
            } else {
                theme::SURFACE
            };

            let card = egui::Frame::default()
                .fill(card_bg)
                .stroke(egui::Stroke::new(1.0, theme::BORDER))
                .rounding(Rounding::same(6.0))
                .inner_margin(egui::Margin::symmetric(10.0, 5.0))
                .outer_margin(egui::Margin::symmetric(0.0, 2.0));

            card.show(ui, |ui: &mut egui::Ui| {
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
                            [ui.available_width() - 160.0, 24.0],
                            egui::TextEdit::singleline(edit_buffer)
                                .text_color(theme::TEXT_PRIMARY)
                                .font(egui::TextStyle::Body),
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
                        // Grab focus on first frame
                        if is_editing {
                            resp.request_focus();
                        }
                    } else {
                        let title_text = if completed {
                            RichText::new(&tasks[idx].title)
                                .strikethrough()
                                .color(theme::TEXT_MUTED)
                                .size(14.0)
                        } else {
                            RichText::new(&tasks[idx].title)
                                .color(theme::TEXT_PRIMARY)
                                .size(14.0)
                        };
                        if ui.selectable_label(is_selected, title_text).clicked() {
                            if *selected_id == Some(id) {
                                *selected_id = None;
                            } else {
                                *selected_id = Some(id);
                            }
                        }
                    }

                    ui.with_layout(
                        egui::Layout::right_to_left(egui::Align::Center),
                        |ui: &mut egui::Ui| {
                            // Edit button
                            if ui
                                .add_sized(
                                    [22.0, 22.0],
                                    egui::Button::new(
                                        RichText::new("✎")
                                            .size(12.0)
                                            .color(theme::TEXT_MUTED),
                                    )
                                    .fill(Color32::TRANSPARENT),
                                )
                                .clicked()
                            {
                                if is_editing {
                                    // Save current edit
                                    let trimmed = edit_buffer.trim().to_string();
                                    if !trimmed.is_empty() {
                                        tasks[idx].title = trimmed;
                                        *modified = true;
                                    }
                                    *editing_id = None;
                                    edit_buffer.clear();
                                } else {
                                    // Start editing
                                    *editing_id = Some(id);
                                    *edit_buffer = tasks[idx].title.clone();
                                }
                            }

                            // Schedule badge
                            let sched = &tasks[idx].schedule;
                            let sched_color = match sched {
                                Schedule::Today => theme::PALETTE.green,
                                Schedule::Tomorrow => theme::PALETTE.yellow,
                                Schedule::Later => theme::PALETTE.accent,
                                Schedule::Unscheduled => theme::TEXT_MUTED,
                            };
                            let sched_bg = Color32::from_rgba_premultiplied(
                                sched_color.r(),
                                sched_color.g(),
                                sched_color.b(),
                                25,
                            );
                            let badge = egui::Frame::default()
                                .fill(sched_bg)
                                .rounding(Rounding::same(3.0))
                                .inner_margin(egui::Margin::symmetric(5.0, 1.0));
                            if badge
                                .show(ui, |ui: &mut egui::Ui| {
                                    ui.selectable_label(
                                        false,
                                        RichText::new(sched.label())
                                            .size(10.0)
                                            .color(sched_color),
                                    )
                                })
                                .inner
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

                            ui.add_space(2.0);

                            // Priority badge
                            let prio_color = theme::priority_color(tasks[idx].priority);
                            let prio_text = theme::priority_label(tasks[idx].priority);
                            let prio_bg = Color32::from_rgba_premultiplied(
                                prio_color.r(),
                                prio_color.g(),
                                prio_color.b(),
                                25,
                            );
                            let prio_badge = egui::Frame::default()
                                .fill(prio_bg)
                                .rounding(Rounding::same(3.0))
                                .inner_margin(egui::Margin::symmetric(5.0, 1.0));
                            if prio_badge
                                .show(ui, |ui: &mut egui::Ui| {
                                    ui.selectable_label(
                                        false,
                                        RichText::new(prio_text).size(10.0).color(prio_color),
                                    )
                                })
                                .inner
                                .clicked()
                            {
                                tasks[idx].priority = (tasks[idx].priority + 1) % 4;
                                *modified = true;
                            }

                            ui.add_space(4.0);

                            // Delete button → opens confirmation
                            if ui
                                .add_sized(
                                    [22.0, 22.0],
                                    egui::Button::new(
                                        RichText::new("✕")
                                            .size(12.0)
                                            .color(theme::TEXT_MUTED),
                                    )
                                    .fill(Color32::TRANSPARENT),
                                )
                                .clicked()
                            {
                                *delete_confirm_id = Some(id);
                            }
                        },
                    );
                });
            });
        }
    }

    fn render_completed_item(
        ui: &mut egui::Ui,
        id: i64,
        tasks: &mut Vec<Task>,
        _selected_id: &mut Option<i64>,
        delete_confirm_id: &mut Option<i64>,
        modified: &mut bool,
    ) {
        let Some(idx) = tasks.iter().position(|t| t.id == id) else {
            return;
        };
        theme::card_frame().show(ui, |ui: &mut egui::Ui| {
            ui.horizontal(|ui: &mut egui::Ui| {
                let mut checked = true;
                if ui.checkbox(&mut checked, "").changed() {
                    tasks[idx].completed = false;
                    *modified = true;
                }
                ui.label(
                    RichText::new(&tasks[idx].title)
                        .strikethrough()
                        .color(theme::TEXT_MUTED)
                        .size(14.0),
                );
                ui.with_layout(
                    egui::Layout::right_to_left(egui::Align::Center),
                    |ui: &mut egui::Ui| {
                        if ui
                            .add_sized(
                                [22.0, 22.0],
                                egui::Button::new(
                                    RichText::new("✕")
                                        .size(12.0)
                                        .color(theme::TEXT_MUTED),
                                )
                                .fill(Color32::TRANSPARENT),
                            )
                            .clicked()
                        {
                            *delete_confirm_id = Some(id);
                        }
                    },
                );
            });
        });
    }
}
