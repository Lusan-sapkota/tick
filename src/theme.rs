use egui::Style;

pub const ACCENT: egui::Color32 = egui::Color32::from_rgb(88, 155, 255);
pub const GREEN: egui::Color32 = egui::Color32::from_rgb(52, 211, 110);
pub const YELLOW: egui::Color32 = egui::Color32::from_rgb(250, 204, 60);
pub const RED: egui::Color32 = egui::Color32::from_rgb(255, 90, 90);
pub const MUTED: egui::Color32 = egui::Color32::from_rgb(120, 125, 140);

pub fn apply_theme(ctx: &egui::Context) {
    let mut style = Style::default();
    style.visuals.dark_mode = true;
    ctx.set_style(style);
}

pub fn priority_color(level: i32) -> egui::Color32 {
    match level {
        1 => GREEN,
        2 => YELLOW,
        3 => RED,
        _ => MUTED,
    }
}

pub fn priority_label(level: i32) -> &'static str {
    match level {
        1 => "Low",
        2 => "Medium",
        3 => "High",
        _ => "None",
    }
}
