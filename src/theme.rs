use egui::{Rounding, Stroke, Style, Visuals};

pub struct Palette {
    pub accent: egui::Color32,
    pub green: egui::Color32,
    pub yellow: egui::Color32,
    pub red: egui::Color32,
}

pub const PALETTE: Palette = Palette {
    accent: egui::Color32::from_rgb(88, 155, 255),
    green: egui::Color32::from_rgb(52, 211, 110),
    yellow: egui::Color32::from_rgb(250, 204, 60),
    red: egui::Color32::from_rgb(255, 90, 90),
};

pub const SURFACE: egui::Color32 = egui::Color32::from_rgb(42, 46, 58);
pub const SURFACE_HOVER: egui::Color32 = egui::Color32::from_rgb(52, 56, 68);
pub const SURFACE_ACTIVE: egui::Color32 = egui::Color32::from_rgb(45, 55, 90);
pub const BACKGROUND: egui::Color32 = egui::Color32::from_rgb(28, 30, 38);
pub const BORDER: egui::Color32 = egui::Color32::from_rgb(72, 76, 90);
pub const TEXT_PRIMARY: egui::Color32 = egui::Color32::from_rgb(238, 240, 245);
pub const TEXT_SECONDARY: egui::Color32 = egui::Color32::from_rgb(170, 174, 190);
pub const TEXT_MUTED: egui::Color32 = egui::Color32::from_rgb(125, 130, 145);

pub fn apply_theme(ctx: &egui::Context) {
    let mut style = Style {
        visuals: Visuals::dark(),
        ..Default::default()
    };

    let v = &mut style.visuals;
    v.window_fill = BACKGROUND;
    v.panel_fill = BACKGROUND;
    v.faint_bg_color = BACKGROUND;
    v.extreme_bg_color = egui::Color32::from_rgb(18, 20, 26);
    v.code_bg_color = SURFACE;
    v.window_rounding = Rounding::same(10.0);

    v.widgets.noninteractive.bg_fill = SURFACE;
    v.widgets.noninteractive.fg_stroke.color = TEXT_SECONDARY;
    v.widgets.noninteractive.rounding = Rounding::same(6.0);

    v.widgets.inactive.bg_fill = SURFACE;
    v.widgets.inactive.fg_stroke.color = TEXT_PRIMARY;
    v.widgets.inactive.rounding = Rounding::same(6.0);
    v.widgets.inactive.weak_bg_fill = SURFACE;

    v.widgets.hovered.bg_fill = SURFACE_HOVER;
    v.widgets.hovered.fg_stroke.color = TEXT_PRIMARY;
    v.widgets.hovered.rounding = Rounding::same(6.0);
    v.widgets.hovered.weak_bg_fill = SURFACE_HOVER;

    v.widgets.active.bg_fill = SURFACE_ACTIVE;
    v.widgets.active.fg_stroke.color = TEXT_PRIMARY;
    v.widgets.active.rounding = Rounding::same(6.0);
    v.widgets.active.weak_bg_fill = SURFACE_ACTIVE;

    v.widgets.open.bg_fill = SURFACE;
    v.widgets.open.fg_stroke.color = TEXT_PRIMARY;
    v.widgets.open.rounding = Rounding::same(6.0);
    v.widgets.open.weak_bg_fill = SURFACE;

    v.selection.bg_fill = egui::Color32::from_rgba_premultiplied(88, 155, 255, 50);
    v.selection.stroke.color = PALETTE.accent;
    v.hyperlink_color = PALETTE.accent;
    v.warn_fg_color = PALETTE.yellow;
    v.error_fg_color = PALETTE.red;

    style.spacing.item_spacing = egui::vec2(10.0, 6.0);
    style.spacing.button_padding = egui::vec2(14.0, 7.0);
    style.spacing.indent = 12.0;

    // Larger default body size
    style.text_styles.insert(
        egui::TextStyle::Body,
        egui::FontId::new(15.0, egui::FontFamily::Proportional),
    );
    style.text_styles.insert(
        egui::TextStyle::Button,
        egui::FontId::new(15.0, egui::FontFamily::Proportional),
    );
    style.text_styles.insert(
        egui::TextStyle::Heading,
        egui::FontId::new(19.0, egui::FontFamily::Proportional),
    );
    style.text_styles.insert(
        egui::TextStyle::Small,
        egui::FontId::new(13.0, egui::FontFamily::Proportional),
    );

    ctx.set_style(style);
}

pub fn card_frame() -> egui::Frame {
    egui::Frame::default()
        .fill(SURFACE)
        .stroke(Stroke::new(1.0, BORDER))
        .rounding(Rounding::same(6.0))
        .inner_margin(egui::Margin::symmetric(10.0, 6.0))
        .outer_margin(egui::Margin::symmetric(0.0, 2.0))
}

pub fn input_frame() -> egui::Frame {
    egui::Frame::default()
        .fill(SURFACE)
        .stroke(Stroke::new(1.0, BORDER))
        .rounding(Rounding::same(7.0))
        .inner_margin(egui::Margin::symmetric(10.0, 6.0))
}

pub fn section_header(text: &str) -> egui::RichText {
    egui::RichText::new(text)
        .size(12.0)
        .color(TEXT_SECONDARY)
        .strong()
}

pub fn priority_color(level: i32) -> egui::Color32 {
    match level {
        1 => PALETTE.green,
        2 => PALETTE.yellow,
        3 => PALETTE.red,
        _ => TEXT_MUTED,
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
